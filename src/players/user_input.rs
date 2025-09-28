use crate::players;
use crate::Board;
use crate::Color;
use crate::Move;
use crate::Position;
use std::io;
pub struct Human {}

impl players::Player for Human {
    fn take_turn(&mut self, board: Board, color: Color) -> Move {
        let mut my_move;
        let moves: Vec<Move> = board.get_all_moves(color);
        println!("Your Move:");
        let mut start = String::new();

        io::stdin()
            .read_line(&mut start)
            .expect("Failed to read line");

        let end = start.split_off(2);

        my_move = Move {
            start: Position::from_string(start),
            end: Position::from_string(end),
        };

        while !(moves.contains(&my_move)) {
            println!("Thats not a valid move, try again");
            let mut start = String::new();

            io::stdin()
                .read_line(&mut start)
                .expect("Failed to read line");

            let end = start.split_off(2);

            my_move = Move {
                start: Position::from_string(start),
                end: Position::from_string(end),
            };
        }

        return my_move;
    }
}
// mod user_input
