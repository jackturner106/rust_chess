use crate::model::{Board, Color, Piece, PieceType, Position};

const PAWN_POS: [[i16; 8]; 8] = [
    [106, 106, 107, 108, 108, 107, 106, 106],
    [105, 105, 106, 107, 107, 106, 105, 105],
    [104, 104, 105, 106, 106, 105, 104, 104],
    [103, 103, 104, 105, 105, 104, 103, 103],
    [102, 102, 103, 104, 104, 103, 102, 102],
    [101, 101, 102, 103, 103, 102, 101, 101],
    [100, 100, 100, 100, 100, 100, 100, 100],
    [100, 100, 100, 100, 100, 100, 100, 100],
];
const KNIGHT_POS: [[i16; 8]; 8] = [
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 330, 330, 330, 330, 300, 300],
    [300, 300, 320, 320, 320, 320, 300, 300],
    [300, 300, 310, 310, 310, 310, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
];
const BISHOP_POS: [[i16; 8]; 8] = [
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
    [300, 313, 300, 300, 300, 300, 313, 300],
    [300, 310, 312, 300, 300, 312, 310, 300],
    [310, 300, 310, 311, 311, 310, 300, 310],
    [300, 310, 300, 300, 300, 300, 310, 300],
    [300, 300, 300, 300, 300, 300, 300, 300],
];

pub(crate) fn evaluate(board: Board, color: Color) -> i16 {
    // Tapered evaluation: Chess boards start with 39 * 2 = 78 points
    // after 4 pawns + 4 pieces captured middlegame, so 78 - 16 = 62
    let mut score: i16 = 0;
    let my_points = points(board, color);
    let op_points = points(board, color.opponent_color());
    let (my_double_p, my_rook_open, my_rook_semi, my_bishops) = doubled_pawns(board, color);
    let (op_double_p, op_rook_open, op_rook_semi, op_bishops) =
        doubled_pawns(board, color.opponent_color());

    score += my_points - op_points;
    score += ((op_double_p - my_double_p) as i16) * 3;
    score += ((my_rook_open - op_rook_open) as i16) * 40;
    score += ((my_rook_semi - op_rook_semi) as i16) * 10;

    score += if my_bishops { 10 } else { 0 };
    score -= if op_bishops { 10 } else { 0 };

    return score;
}

pub(crate) fn print_evaluate(board: Board, color: Color) -> i16 {
    // Tapered evaluation: Chess boards start with 39 * 2 = 78 points
    // after 4 pawns + 4 pieces captured middlegame, so 78 - 16 = 62
    // (n - 62) / 16
    let mut early_score = 0;
    let middle_score = 0;
    let mut late_score = 0;

    let my_points = points(board, color);
    let op_points = points(board, color.opponent_color());

    let my_ep = early_points(board, color);
    let op_ep = early_points(board, color.opponent_color());

    let (my_double_p, my_rook_open, my_rook_semi, my_bishops) = doubled_pawns(board, color);
    let (op_double_p, op_rook_open, op_rook_semi, op_bishops) =
        doubled_pawns(board, color.opponent_color());
    println!("{my_points}, {my_double_p}, {my_rook_open}, {my_rook_semi}, {my_bishops}");
    println!("{op_points}, {op_double_p}, {op_rook_open}, {op_rook_semi}, {op_bishops}");

    let total_points = my_points + op_points;

    early_score += my_ep - op_ep;
    early_score += (op_double_p - my_double_p) * 3;
    early_score += (my_rook_open - op_rook_open) * 40;
    early_score += (my_rook_semi - op_rook_semi) * 10;
    early_score += if my_bishops { 10 } else { 0 };
    early_score -= if op_bishops { 10 } else { 0 };

    late_score += my_ep - op_ep;
    late_score += (op_double_p - my_double_p) * 3;
    late_score += (my_rook_open - op_rook_open) * 40;
    late_score += (my_rook_semi - op_rook_semi) * 10;
    late_score += if my_bishops { 10 } else { 0 };
    late_score -= if op_bishops { 10 } else { 0 };

    let score = (((early_score * total_points) as f32 / 78.0).round()
        + (early_score as f32 * (1.0 - (total_points as f32 / 78.0))).round())
        as i16;

    return score;
}

// Returns: (number of doubled pawns, number of rooks on open files, number of rooks on semi open files,
//           bishop pair)
fn doubled_pawns(board: Board, color: Color) -> (i16, i16, i16, bool) {
    let mut pawns: i16;
    let mut opawns: i16;
    let mut rooks: i16;
    let mut piece: Piece;
    let mut bishops: i16 = 0;
    let mut total_doubled_pawns: i16 = 0;
    let mut total_rooks_open: i16 = 0;
    let mut total_rooks_semi: i16 = 0;

    for col in 0..8 {
        pawns = 0;
        rooks = 0;
        opawns = 0;
        for row in 0..8 {
            piece = board.get_piece(Position { x: col, y: row });
            if piece.piece_type == PieceType::Pawn {
                if piece.color == color {
                    pawns += 1;
                } else {
                    opawns += 1;
                }
            } else if piece.piece_type == PieceType::Rook && piece.color == color {
                rooks += 1;
            } else if piece.piece_type == PieceType::Bishop && piece.color == color {
                bishops += 1;
            }
        }
        total_doubled_pawns += if pawns > 1 { pawns } else { 0 };
        if pawns + opawns == 1 {
            total_rooks_semi += rooks;
        } else if pawns + opawns == 0 {
            total_rooks_open += rooks;
        }
    }
    return (
        total_doubled_pawns,
        total_rooks_open,
        total_rooks_semi,
        bishops > 1,
    );
}

fn points(board: Board, color: Color) -> i16 {
    let mut points: i16 = 0;
    let mut piece: Piece;
    let mut pos: Position;

    for i in 0..8 {
        for l in 0..8 {
            pos = Position { x: i, y: l };
            piece = board.get_piece(pos);
            if piece.color == color {
                points += piece_points(piece.piece_type, pos, color);
            }
        }
    }

    return points;
}

fn early_points(board: Board, color: Color) -> i16 {
    let mut points: i16 = 0;
    let mut piece: Piece;
    let mut pos: Position;

    for i in 0..8 {
        for l in 0..8 {
            pos = Position { x: i, y: l };
            piece = board.get_piece(pos);
            if piece.color == color {
                points += early_piece_points(piece.piece_type, pos, color);
            }
        }
    }

    return points;
}

fn late_points(board: Board, color: Color) -> i16 {
    let mut points: i16 = 0;
    let mut piece: Piece;
    let mut pos: Position;

    for i in 0..8 {
        for l in 0..8 {
            pos = Position { x: i, y: l };
            piece = board.get_piece(pos);
            if piece.color == color {
                points += late_piece_points(piece.piece_type, pos, color);
            }
        }
    }

    return points;
}

fn piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {
    match piece {
        PieceType::Bishop => return 300,
        PieceType::Knight => return 300,
        PieceType::Rook => return 500,
        PieceType::King => return 0,
        PieceType::Queen => return 900,
        PieceType::Pawn => return 100,
        PieceType::Empty => return 0,
    };
}

fn early_piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {
    match piece {
        PieceType::Bishop => return get_pos_points(pos, color, BISHOP_POS),
        PieceType::Knight => return get_pos_points(pos, color, KNIGHT_POS),
        PieceType::Rook => return 500,
        PieceType::King => return 0,
        PieceType::Queen => return 900,
        PieceType::Pawn => return get_pos_points(pos, color, PAWN_POS),
        PieceType::Empty => return 0,
    };
}

fn late_piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {
    match piece {
        PieceType::Bishop => return get_pos_points(pos, color, BISHOP_POS),
        PieceType::Knight => return get_pos_points(pos, color, KNIGHT_POS),
        PieceType::Rook => return 500,
        PieceType::King => return 0,
        PieceType::Queen => return 900,
        PieceType::Pawn => return get_pos_points(pos, color, PAWN_POS),
        PieceType::Empty => return 0,
    };
}

fn get_pos_points(pos: Position, color: Color, grid: [[i16; 8]; 8]) -> i16 {
    let mp: Position = if color == Color::Black {
        pos
    } else {
        Position {
            x: 7 - pos.x,
            y: 7 - pos.y,
        }
    };
    return grid[mp.y as usize][mp.x as usize];
}
