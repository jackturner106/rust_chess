pub mod controller {
    
    use crate::{player::players::{Human, Player, AI}, Board, Color, Move, Position};
    use std::time::{Duration, Instant};

    pub trait Controller {
        fn play_game(&self, board: Board) {

        }
    }

    pub struct HumanGame {

    }

    impl Controller for HumanGame {
        fn play_game(&self, mut board: Board) {
            let mut turns: u8 = 0;
            let mut current_turn: Color = Color::White;
            let mut current_move: Move;

            //let mut p2: AI = AI::new();
            //let mut p2: Human=Human{};
            //let mut p1: AI = AI::new();
            let mut p1: AI = AI::new();
            let mut p2: Human=Human{};

            while turns < 100 {

                if board.checkmatep(current_turn) {
                    println!("Game Over");
                    return;
                }

                println!("Current Board:");
                println!("{board}");

                if current_turn == Color::White {

                    println!("White to move");

                    current_move = p1.take_turn_threaded(board, Color::White);
                    current_turn = Color::Black;
                } else {
                    println!("Black to move.");
                    current_move = p2.take_turn(board, Color::Black);
                    current_turn = Color::White;
                }

                println!("Got move {current_move}");
                board.make_move(current_move);

                turns += 1;
                
            }
        }
    }

}