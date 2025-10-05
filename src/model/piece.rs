use crate::model::{color::Color, piece_type::PieceType};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self.piece_type {
            PieceType::Bishop => {
                return if self.color == Color::White {
                    "♗".to_owned()
                } else {
                    String::from("♝")
                }
            }
            PieceType::Knight => {
                return if self.color == Color::White {
                    String::from("♘")
                } else {
                    String::from("♞")
                }
            }
            PieceType::Rook => {
                return if self.color == Color::White {
                    String::from("♖")
                } else {
                    String::from("♜")
                }
            }
            PieceType::King => {
                return if self.color == Color::White {
                    String::from("♔")
                } else {
                    String::from("♚")
                }
            }
            PieceType::Queen => {
                return if self.color == Color::White {
                    String::from("♕")
                } else {
                    String::from("♛")
                }
            }
            PieceType::Pawn => {
                return if self.color == Color::White {
                    String::from("♙")
                } else {
                    String::from("♟")
                }
            }
            PieceType::Empty => {
                return if self.color == Color::White {
                    String::from(" ")
                } else {
                    String::from(" ")
                }
            }
        }
    }
}

impl Piece {
    pub(crate) fn fen_string(&self) -> String {
        match self.piece_type {
            PieceType::Bishop => {
                return if self.color == Color::White {
                    "B".to_owned()
                } else {
                    String::from("b")
                }
            }
            PieceType::Knight => {
                return if self.color == Color::White {
                    String::from("N")
                } else {
                    String::from("n")
                }
            }
            PieceType::Rook => {
                return if self.color == Color::White {
                    String::from("R")
                } else {
                    String::from("r")
                }
            }
            PieceType::King => {
                return if self.color == Color::White {
                    String::from("K")
                } else {
                    String::from("k")
                }
            }
            PieceType::Queen => {
                return if self.color == Color::White {
                    String::from("Q")
                } else {
                    String::from("q")
                }
            }
            PieceType::Pawn => {
                return if self.color == Color::White {
                    String::from("P")
                } else {
                    String::from("p")
                }
            }
            PieceType::Empty => {
                return if self.color == Color::White {
                    String::from("")
                } else {
                    String::from("")
                }
            }
        }
    }
}
