use crate::model::{
    board::Board, color::Color, piece::Piece, piece_type::PieceType, position::Position,
};

pub(crate) fn make_board() -> Board {
    return Board {
        board: [
            [
                Piece {
                    piece_type: PieceType::Rook,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Knight,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Queen,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::King,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Knight,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Rook,
                    color: Color::White,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::White,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
                Piece {
                    piece_type: PieceType::Empty,
                    color: Color::None,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    color: Color::Black,
                },
            ],
            [
                Piece {
                    piece_type: PieceType::Rook,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Knight,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Queen,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::King,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Bishop,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Knight,
                    color: Color::Black,
                },
                Piece {
                    piece_type: PieceType::Rook,
                    color: Color::Black,
                },
            ],
        ],
        turn: Color::White,
        white_kingside: true,
        black_kingside: true,
        white_queenside: true,
        black_queenside: true,
        black_king: Position { y: 7, x: 4 },
        white_king: Position { y: 0, x: 4 },
        en_pessant: Position { x: -1, y: -1 },
    };
}
