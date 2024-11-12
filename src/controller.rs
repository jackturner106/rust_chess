pub mod controller {
    
    use crate::{player::players::{Human, AI, Player}, Board, Color, Move};
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
            let mut moves: Vec<Move>;
            let mut now: Instant;
            let mut elapsed: Duration;

            let mut p1 = Human{};
            let mut p2: AI = AI::new();

            while turns < 100 {

                if board.checkmatep(current_turn) {
                    println!("Game Over");
                    return;
                }

                println!("Current Board:");
                println!("{board}");

                if current_turn == Color::White {

                    println!("White to move");

                    //now = Instant::now();
                    current_move = p1.take_turn(board, Color::White);
                    //elapsed = now.elapsed();
                    //println!("Take Turn (threads) took: {:.2?}", elapsed);
                    //println!("And got move {current_move}");
                    //current_move = p1.take_turn(board, Color::White);
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