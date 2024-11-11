use std::{env, fmt};

use player::players::{Human, Player};
mod player;

use controller::controller::{Controller, HumanGame};
mod controller;

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

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self.x {
            0=>return "A".to_owned() + &(self.y + 1).to_string(),
            1=>return "B".to_owned() + &(self.y + 1).to_string(),
            2=>return "C".to_owned() + &(self.y + 1).to_string(),
            3=>return "D".to_owned() + &(self.y + 1).to_string(),
            4=>return "E".to_owned() + &(self.y + 1).to_string(),
            5=>return "F".to_owned() + &(self.y + 1).to_string(),
            6=>return "G".to_owned() + &(self.y + 1).to_string(),
            7=>return "H".to_owned() + &(self.y + 1).to_string(),
            _=>return String::from("")
        }
    }
}

impl Position {

    fn up(&self) -> Position {
        return Position{x: self.x, y: self.y + 1}
    }

    fn down(&self) -> Position {
        return Position{x: self.x, y: self.y - 1}
    }

    fn left(&self) -> Position {
        return Position{x: self.x - 1, y: self.y}
    }

    fn right(&self) -> Position {
        return Position{x: self.x + 1, y: self.y}
    }

    fn validp(&self) -> bool {
        return self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7;
    }

    fn from_string(str: String) -> Position {
        
        let x: isize;
        match str.chars().nth(0).unwrap() {
            'a'=>x=0,
            'b'=>x=1,
            'c'=>x=2,
            'd'=>x=3,
            'e'=>x=4,
            'f'=>x=5,
            'g'=>x=6,
            'h'=>x=7,
            _=>x=0,
        }

        let y: isize = (str.chars().nth(1).unwrap().to_digit(10).unwrap() - 1).try_into().unwrap();

        Position { x: x, y: y }
    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Move {
    start: Position,
    end: Position
}

impl fmt::Display for Move {
    fn fmt(&self,  fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.start.to_string())?;
        fmt.write_str("->")?;
        fmt.write_str(&self.end.to_string())?;
        Ok(())
    }
}

impl Move {
    fn kingside_castlep(&self) -> bool {
        return (self.start == Position{x:4,y:0} && self.end == Position{x:6,y:0}) || (self.start == Position{x:4,y:7} && self.end == Position{x:6,y:7});
    }

    fn queenside_castlep(&self) -> bool {
        return (self.start == Position{x:4,y:0} && self.end == Position{x:2,y:0}) || (self.start == Position{x:4,y:7} && self.end == Position{x:2,y:7});
    }
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self.piece_type{
            PieceType::Bishop=>return if self.color == Color::White {String::from("♗")} else {String::from("♝")},
            PieceType::Knight=>return if self.color == Color::White {String::from("♘")} else {String::from("♞")},
            PieceType::Rook=>return if self.color == Color::White {String::from("♖")} else {String::from("♜")},
            PieceType::King=>return if self.color == Color::White {String::from("♔")} else {String::from("♚")},
            PieceType::Queen=>return if self.color == Color::White {String::from("♕")} else {String::from("♛")},
            PieceType::Pawn=>return if self.color == Color::White {String::from("♙")} else {String::from("♟")},
            PieceType::Empty=>return if self.color == Color::White {String::from(" ")} else {String::from(" ")},
           }
    }
}

#[derive(Clone, Copy)]
struct Board {
    // Stored as an array of arrays. The first array corresponds to the first row. 
    // This means Y IS FIRST!! 
    // board[y][x] is the correct way to access
    board: [[Piece; 8]; 8],
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool
}

impl fmt::Display for Board {
    fn fmt(&self,  fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut flag: bool = false;
        let mut l: u8 = 8;
        for b in self.vec().iter().rev() {
            fmt.write_str(&(l.to_string() + " "))?;
            for i in 0..8 {

                if flag {fmt.write_str("\x1b[40m")?;}
                fmt.write_str(&(b[i].to_string() + " "))?;
                if flag { fmt.write_str("\x1b[0m")?; }
            
                flag = !flag;
            }
            fmt.write_str("\n")?;
            flag = !flag;
            l -= 1;
        }
        fmt.write_str("  A B C D E F G H")?;
        Ok(())
    }
}

impl Board {
    fn vec(&self) -> Vec<[Piece; 8]> {
        return self.board.to_vec();
    }

    fn make_move(&mut self, new_move: Move) -> () {

        let piece: Piece = self.get_piece(new_move.start);

        if piece.color == Color::None {
            return;
        }

        //if !(self.get_all_moves(piece.color).contains(&new_move)) {
        //    return;
        //}

        self.put_piece(new_move.end, self.get_piece(new_move.start));
        self.put_piece(new_move.start, Piece {piece_type: PieceType::Empty, color: Color::None});

        if piece.piece_type == PieceType::King && (new_move.kingside_castlep() || new_move.queenside_castlep()) {
            if piece.color == Color::White && new_move.kingside_castlep() {
                self.white_kingside = false;
                self.put_piece(Position{y:0,x:5}, self.get_piece(Position{y:0,x:7}));
                self.put_piece(Position{y:0,x:7}, Piece {piece_type: PieceType::Empty, color: Color::None});
            } else if piece.color == Color::White && new_move.queenside_castlep() {
                self.white_queenside = false;
                self.put_piece(Position{y:0,x:3}, self.get_piece(Position{y:0,x:0}));
                self.put_piece(Position{y:0,x:0}, Piece {piece_type: PieceType::Empty, color: Color::None});
            } else if piece.color == Color::Black && new_move.kingside_castlep() {
                self.black_kingside = false;
                self.put_piece(Position{y:7,x:5}, self.get_piece(Position{y:7,x:7}));
                self.put_piece(Position{y:7,x:7}, Piece {piece_type: PieceType::Empty, color: Color::None});
            }else if piece.color == Color::Black && new_move.queenside_castlep() {
                self.black_queenside = false;
                self.put_piece(Position{y:7,x:3}, self.get_piece(Position{y:7,x:0}));
                self.put_piece(Position{y:7,x:0}, Piece {piece_type: PieceType::Empty, color: Color::None});
            }
        }
    }

    fn put_piece(&mut self, pos: Position, piece: Piece) {
        self.board[pos.y as usize][pos.x as usize] = piece;
    }

    fn get_piece(&self, pos: Position) -> Piece {
        return self.board[pos.y as usize][pos.x as usize];
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

    fn get_all_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        let mut row = 0;
        let mut col;
        for r in self.board {
            col = 0;
            for p in r {
                if p.color == color {
                    moves.append(&mut self.get_moves(Position{x: col, y: row}));
                }
                col += 1;
            }
            row += 1
        }

        return moves;
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

        if me == Color::White && self.white_kingside && pos.right().right().validp() && self.get_piece(pos.right()).piece_type == PieceType::Empty && self.get_piece(pos.right().right()).piece_type == PieceType::Empty {
            moves.push(Move{ start:pos, end: pos.right().right() });
        }
        if me == Color::White && self.white_queenside && pos.left().left().left().validp() && self.get_piece(pos.left()).piece_type == PieceType::Empty && self.get_piece(pos.left().left()).piece_type == PieceType::Empty && self.get_piece(pos.left().left().left()).piece_type == PieceType::Empty {
            moves.push(Move{ start:pos, end: pos.left().left() });
        }
        if me == Color::Black && self.black_kingside && pos.right().right().validp() && self.get_piece(pos.right()).piece_type == PieceType::Empty && self.get_piece(pos.right().right()).piece_type == PieceType::Empty {
            moves.push(Move{ start:pos, end: pos.right().right() });
        }
        if me == Color::Black && self.black_queenside && pos.left().left().left().validp() && self.get_piece(pos.left()).piece_type == PieceType::Empty && self.get_piece(pos.left().left()).piece_type == PieceType::Empty && self.get_piece(pos.left().left().left()).piece_type == PieceType::Empty {
            moves.push(Move{ start:pos, end: pos.left().left() });
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
                    if first_double.validp() && self.get_piece(first_double).piece_type == PieceType::Empty {
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
                    if first_double.validp() && self.get_piece(first_double).piece_type == PieceType::Empty {
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
        board: [[Piece {piece_type: PieceType::Rook, color: Color::White}, Piece {piece_type: PieceType::Knight, color: Color::White}, Piece {piece_type: PieceType::Bishop, color: Color::White}, Piece {piece_type: PieceType::Queen, color: Color::White}, Piece {piece_type: PieceType::King, color: Color::White}, Piece {piece_type: PieceType::Bishop, color: Color::White}, Piece {piece_type: PieceType::Knight, color: Color::White}, Piece {piece_type: PieceType::Rook, color: Color::White}],
        [Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}, Piece {piece_type: PieceType::Pawn, color: Color::White}],
        [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        [Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}, Piece {piece_type: PieceType::Empty, color: Color::None}],
        [Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}, Piece {piece_type: PieceType::Pawn, color: Color::Black}],
        [Piece {piece_type: PieceType::Rook, color: Color::Black}, Piece {piece_type: PieceType::Knight, color: Color::Black}, Piece {piece_type: PieceType::Bishop, color: Color::Black}, Piece {piece_type: PieceType::Queen, color: Color::Black}, Piece {piece_type: PieceType::King, color: Color::Black}, Piece {piece_type: PieceType::Bishop, color: Color::Black}, Piece {piece_type: PieceType::Knight, color: Color::Black}, Piece {piece_type: PieceType::Rook, color: Color::Black}]],
        white_kingside: true,
        black_kingside: true,
        white_queenside: true,
        black_queenside: true
    };
}

fn main() {
    /* 
    let mut board = make_board();
    println!("{board}");
    let mut moves = board.get_all_moves(Color::White);
    for mv in moves {
        println!("{mv}");
    }
    let hu: Human = Human{};
    let mvt = hu.take_turn(board);
    println!("{mvt}");
    */
    env::set_var("RUST_BACKTRACE", "1");

    let board = make_board();
    let game = HumanGame{};
    game.play_game(board);

}
