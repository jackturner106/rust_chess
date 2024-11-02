use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    White,
    Black,
    None,
}

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Empty,
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
        for b in self.vec() {
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
    println!("Hello, world!");
    let board = make_board();
    println!("{board}");
}
