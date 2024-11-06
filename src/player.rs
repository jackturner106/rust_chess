pub mod players {

    use crate::Board;
    use crate::Move;
    use crate::Position;
    use std::io;
    pub trait Player {
        fn take_turn(&self, board: Board) -> Move;
    }

    pub struct Human {

    }

    impl Player for Human {
        fn take_turn(&self, board: Board) -> Move {
            println!("Starting position:");
            let mut start = String::new();

            io::stdin()
                .read_line(&mut start)
                .expect("Failed to read line");

            println!("Ending position:");
            let mut end = String::new();

            io::stdin()
                .read_line(&mut end)
                .expect("Failed to read line");

            return Move {start:Position::from_string(start), end:Position::from_string(end)}
        }
    }
}