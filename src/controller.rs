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

            let mut p1: AI = AI::new();
            let mut p2: Human = Human{};

            while turns < 100 {

                println!("Current Board:");
                println!("{board}");

                println!("White to move. Asking the AI:");
                //moves = board.get_all_moves(Color::White);
                //for mv in moves {
                //    println!("{mv}");
                //}
                //now = Instant::now();
                //current_move = p1.take_turn(board, Color::White);
                //elapsed = now.elapsed();
                //println!("Take Turn (no threads) took: {:.2?}", elapsed);
                //println!("And got move {current_move}");

                //now = Instant::now();
                current_move = p1.take_turn_threaded(board, Color::White);
                //elapsed = now.elapsed();
                //println!("Take Turn (threads) took: {:.2?}", elapsed);
                //println!("And got move {current_move}");

                board.make_move(current_move);
                println!("Got move {current_move}");

                println!("Current Board:");
                println!("{board}");

                println!("Black to move. Avaliable moves are:");
                moves = board.get_all_moves(Color::Black);
                for mv in moves {
                    println!("{mv}");
                }

                current_move = p2.take_turn(board, Color::Black);
                board.make_move(current_move);
                turns += 1;
                
            }
        }
    }

}