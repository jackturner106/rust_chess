use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    White,
    Black,
    None,
}

impl Color {
    fn opponent_color(&self) -> Color {
        if *self == Color::None {
            return Color::None
        }
        return if *self == Color::White {Color::Black} else {Color::White};
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Empty,
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize
}

impl Position {

    fn up(&self) -> Position {
        return Position{x: self.x, y: self.y + 1}
    }

    fn down(&self) -> Position {
        return Position{x: self.x, y: self.y - 1}
    }

    fn left(&self) -> Position {
        return Position{x: self.x + 1, y: self.y}
    }

    fn right(&self) -> Position {
        return Position{x: self.x - 1, y: self.y}
    }

    fn validp(&self) -> bool {
        return self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7;
    }

}

#[derive(Debug)]
struct Move {
    start: Position,
    end: Position
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self.piece_type{
            PieceType::Bishop=>return if self.color == Color::White {String::from("b")} else {String::from("B")},
            PieceType::Knight=>return if self.color == Color::White {String::from("h")} else {String::from("H")},
            PieceType::Rook=>return if self.color == Color::White {String::from("r")} else {String::from("R")},
            PieceType::King=>return if self.color == Color::White {String::from("k")} else {String::from("K")},
            PieceType::Queen=>return if self.color == Color::White {String::from("q")} else {String::from("Q")},
            PieceType::Pawn=>return if self.color == Color::White {String::from("p")} else {String::from("P")},
            PieceType::Empty=>return if self.color == Color::White {String::from(" ")} else {String::from(" ")},
           }
    }
}

#[derive(Clone, Copy)]
struct Board {
    H: [Piece; 8],
    G: [Piece; 8],
    F: [Piece; 8],
    E: [Piece; 8],
    D: [Piece; 8],
    C: [Piece; 8],
    B: [Piece; 8],
    A: [Piece; 8],
}

impl fmt::Display for Board {
    fn fmt(&self,  fmt: &mut fmt::Formatter) -> fmt::Result {
        for b in self.vec().iter().rev() {
            for i in 0..8 {
                fmt.write_str(&b[i].to_string())?;
            }
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn vec(&self) -> Vec<[Piece; 8]> {
        return vec![self.A, self.B, self.C, self.D, self.E, self.F, self.G, self.H];
    }

    fn make_move(&mut self, new_move: Move) -> () {
        self.put_piece(new_move.end, self.get_piece(new_move.start));
        self.put_piece(new_move.start, Piece {piece_type: PieceType::Empty, color: Color::None});
    }

    fn put_piece(&mut self, pos: Position, piece: Piece) {
        match pos.y {
            0=>self.A[pos.x as usize] = piece,
            1=>self.B[pos.x as usize] = piece,
            2=>self.C[pos.x as usize] = piece,
            3=>self.D[pos.x as usize] = piece,
            4=>self.E[pos.x as usize] = piece,
            5=>self.F[pos.x as usize] = piece,
            6=>self.G[pos.x as usize] = piece,
            7=>self.H[pos.x as usize] = piece,
            _=>()
        }
    }

    fn get_piece(&self, pos: Position) -> Piece {
        match pos.y {
            0=>return self.A[pos.x as usize],
            1=>return self.B[pos.x as usize],
            2=>return self.C[pos.x as usize],
            3=>return self.D[pos.x as usize],
            4=>return self.E[pos.x as usize],
            5=>return self.F[pos.x as usize],
            6=>return self.G[pos.x as usize],
            7=>return self.H[pos.x as usize],
            _=>return Piece{piece_type:PieceType::Empty, color:Color::None},
        }
    }

    fn get_moves(&self, pos: Position) -> Vec<Move> {
        match self.get_piece(pos).piece_type {
            PieceType::Bishop=>return self.get_bishop_moves(pos),
            PieceType::Knight=>return self.get_knight_moves(pos),
            PieceType::Rook=>return self.get_rook_moves(pos),
            PieceType::King=>return self.get_king_moves(pos),
            PieceType::Queen=>return self.get_queen_moves(pos),
            PieceType::Pawn=>return self.get_pawn_moves(pos),
            PieceType::Empty=>return vec![],
        }
    }

    fn get_bishop_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let op: Color = self.get_piece(pos).color.opponent_color();

        let dirs = [|pos: Position| pos.up().left(), |pos: Position| pos.up().right(), |pos: Position| pos.down().left(),|pos: Position| pos.down().right()];
        for dir in dirs {

            let mut temp: Position = dir(pos);
            while temp.validp() && self.get_piece(temp).color == Color::None {
                moves.push(Move {start: pos, end: temp});
                temp = dir(temp);
            }
            if temp.validp() && self.get_piece(temp).color == op {
                moves.push(Move {start: pos, end: temp})
            }
            
        }

        return moves;
    }

    fn get_knight_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let me: Color = self.get_piece(pos).color;

        for p in [pos.up().up().left(), pos.up().up().right(), pos.up().left().left(), pos.up().right().right(), pos.down().down().left(), pos.down().down().right(), pos.down().left().left(), pos.down().right().right()] {
            if p.validp() && self.get_piece(p).color != me {
                moves.push(Move {start:pos, end:p});
            }
        }

        return moves;
    }

    fn get_rook_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let op: Color = self.get_piece(pos).color.opponent_color();

        let dirs = [|pos: Position| pos.up(), |pos: Position| pos.down(), |pos: Position| pos.left(),|pos: Position| pos.right()];
        for dir in dirs {

            let mut temp: Position = dir(pos);
            while temp.validp() && self.get_piece(temp).color == Color::None {
                moves.push(Move {start: pos, end: temp});
                temp = dir(temp);
            }
            if temp.validp() && self.get_piece(temp).color == op {
                moves.push(Move {start: pos, end: temp})
            }

        }

        return moves;
    }

    fn get_king_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let mut board = self.clone();
        let me: Color = self.get_piece(pos).color;

        for loc in [pos.up(), pos.down(), pos.left(), pos.right(), pos.up().left(), pos.up().right(), pos.down().left(), pos.down().right()] {
            if loc.validp() && self.get_piece(loc).color != me {
                board.make_move(Move{start:pos, end:loc});
                if !(board.checkp(loc)) {
                    moves.push(Move {start: pos, end: loc})
                }
                board.make_move(Move{start:loc, end:pos});
            }
        }

        return moves;
    }

    fn get_queen_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let op: Color = self.get_piece(pos).color.opponent_color();

        let dirs = [|pos: Position| pos.up().left(), |pos: Position| pos.up().right(), |pos: Position| pos.down().left(),|pos: Position| pos.down().right(), |pos: Position| pos.up(), |pos: Position| pos.down(), |pos: Position| pos.left(), |pos: Position| pos.right()];
        for dir in dirs {

            let mut temp: Position = dir(pos);
            while temp.validp() && self.get_piece(temp).color == Color::None {
                moves.push(Move {start: pos, end: temp});
                temp = dir(temp);
            }
            if temp.validp() && self.get_piece(temp).color == op {
                moves.push(Move {start: pos, end: temp})
            }
            
        }

        return moves;
    }

    fn get_pawn_moves(&self, pos: Position) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let piece: Piece = self.get_piece(pos);

        if piece.color == Color::White {
            let mut temp: Position = pos.up();
            if temp.validp() && self.get_piece(temp).piece_type == PieceType::Empty {
                moves.push(Move {start: pos, end: temp});
                if temp.y == 2 {
                    let first_double: Position = temp.up();
                    if first_double.validp() && self.get_piece(temp).piece_type == PieceType::Empty {
                        moves.push(Move {start: pos, end: first_double});
                    }
                }
            }
            
            temp = temp.left();
            if temp.validp() && self.get_piece(temp).color == Color::Black {
                moves.push(Move {start: pos, end: temp});
            }

            temp = temp.right().right();
            if temp.validp() && self.get_piece(temp).color == Color::Black {
                moves.push(Move {start: pos, end: temp});
            }
        }

        if piece.color == Color::Black {
            let mut temp: Position = pos.down();
            if temp.validp() && self.get_piece(temp).piece_type == PieceType::Empty {
                moves.push(Move {start: pos, end: temp});
                if temp.y == 5 {
                    let first_double: Position = temp.down();
                    if first_double.validp() && self.get_piece(temp).piece_type == PieceType::Empty {
                        moves.push(Move {start: pos, end: first_double});
                    }
                }
            }
            
            temp = temp.left();
            if temp.validp() && self.get_piece(temp).color == Color::White {
                moves.push(Move {start: pos, end: temp});
            }

            temp = temp.right().right();
            if temp.validp() && self.get_piece(temp).color == Color::White {
                moves.push(Move {start: pos, end: temp});
            }
        }

        return moves;
    }

    fn checkp(&self, pos: Position) -> bool {
        let op: Color = self.get_piece(pos).color.opponent_color();
        let mut found_piece: Piece;

        // Check Diagonals for Bishops or Queens
        let mut dirs = [|pos: Position| pos.up().left(), |pos: Position| pos.up().right(), |pos: Position| pos.down().left(),|pos: Position| pos.down().right()];
        for dir in dirs {

            let mut temp: Position = dir(pos);
            while temp.validp() && self.get_piece(temp).color == Color::None {
                temp = dir(temp);
            }
            found_piece = if temp.validp() {self.get_piece(temp)} else {Piece{piece_type:PieceType::Empty, color:Color::None}};
            if temp.validp() && found_piece.color == op && (found_piece.piece_type == PieceType::Bishop || found_piece.piece_type == PieceType::Queen) {
                return true;
            }
        }

        // Check horizontal/vertical for Rooks or Queens
        dirs =  [|pos: Position| pos.up(), |pos: Position| pos.down(), |pos: Position| pos.left(), |pos: Position| pos.right()];
        for dir in dirs {

            let mut temp: Position = dir(pos);
            while temp.validp() && self.get_piece(temp).color == Color::None {
                temp = dir(temp);
            }
            found_piece = if temp.validp() {self.get_piece(temp)} else {Piece{piece_type:PieceType::Empty, color:Color::None}};
            if temp.validp() && found_piece.color == op && (found_piece.piece_type == PieceType::Rook || found_piece.piece_type == PieceType::Queen) {
                return true;
            }
        }

        // Check knight moves        
        for p in [pos.up().up().left(), pos.up().up().right(), pos.up().left().left(), pos.up().right().right(), pos.down().down().left(), pos.down().down().right(), pos.down().left().left(), pos.down().right().right()] {
            found_piece = if p.validp() {self.get_piece(p)} else {Piece{piece_type:PieceType::Empty, color:Color::None}};
            if p.validp() && found_piece.color == op && found_piece.piece_type == PieceType::Knight {
                return true;
            }
        }

        // Check Pawn moves
        let locs: [Position; 2] = if self.get_piece(pos).color == Color::Black {[pos.down().left(), pos.down().right()]} else {[pos.up().left(), pos.up().right()]};
        for p in locs {
            found_piece = if p.validp() {self.get_piece(p)} else {Piece{piece_type:PieceType::Empty, color:Color::None}};
            if p.validp() && found_piece.color == op && found_piece.piece_type == PieceType::Pawn {
                return true;
            }
        }

        return false;
    }
}

fn make_board() -> Board {
    return Board {
        H: [Piece {piece_type: PieceType::Rook, color: Color::Black}, Piece {piece_type: PieceType::Knight, color: Color::Black}, Piece {piece_type: PieceType::Bishop, color: Color::Black}, Piece {piece_type: PieceType::Queen, color: Color::Black}, Piece {piece_type: PieceType::King, color: Color::Black}, Piece {piece_type: PieceType::Bishop, color: Color::Black}, Piece {piece_type: PieceType::Knight, color: Color::Black}, Piece {piece_type: PieceType::Rook, color: Color::Black}],
        G: [Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}],
        F: [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        E: [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        D: [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        C: [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        B: [Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}],
        A: [Piece {piece_type: PieceType::Rook, color: Color::White}, Piece {piece_type: PieceType::Knight, color: Color::White}, Piece {piece_type: PieceType::Bishop, color: Color::White}, Piece {piece_type: PieceType::Queen, color: Color::White}, Piece {piece_type: PieceType::King, color: Color::White}, Piece {piece_type: PieceType::Bishop, color: Color::White}, Piece {piece_type: PieceType::Knight, color: Color::White}, Piece {piece_type: PieceType::Rook, color: Color::White}]
    };
}

fn main() {
    let mut board = make_board();
    println!("{board}");
    let mut moves = board.get_moves(Position{x:4, y:0});
    println!("{moves:?}");
    board.make_move(Move{start:Position{x:4, y:0}, end:Position{x:0,y:2}});
    moves = board.get_moves(Position{x:0, y:2});
    println!("{moves:?}");
}
