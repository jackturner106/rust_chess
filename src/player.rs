pub mod players {

    use crate::Board;
    use crate::Color;
    use crate::Move;
    use crate::Piece;
    use crate::Position;
    use std::io;
    pub trait Player {
        fn take_turn(&mut self, board: Board, color: Color) -> Move;
    }

    pub struct Human {

    }

    impl Player for Human {
        fn take_turn(&mut self, board: Board, color: Color) -> Move {
            println!("Starting position:");
            let mut start = String::new();

            io::stdin()
                .read_line(&mut start)
                .expect("Failed to read line");
            
            let end = start.split_off(2);

            return Move {start:Position::from_string(start), end:Position::from_string(end)}
        }
    }

    pub struct AI {
        pos_evaluated: u64
    }

    impl Player for AI {
        fn take_turn(&mut self, board: Board, color: Color) -> Move {
            let mv: Move;
            let score: i8;
            self.pos_evaluated = 0;

            (mv, score) = self.search(board, 0, 3, color);

            let evals:u64 = self.pos_evaluated;
            println!("Evaluted: {evals} ");
            println!("Best Position: {score}");

            return mv;
        }
    }

    impl AI {

        pub fn new() -> AI {
            return AI { pos_evaluated: 0};
        }
        
        // negamax
        fn search(&mut self, board: Board, depth: u8, max_depth: u8, color: Color) -> (Move, i8) {
            
            let moves: Vec<Move> = board.get_all_moves(color);
            let op: Color = color.opponent_color();

            let mut temp_board: Board = board.clone();
            temp_board.make_move(moves[0]);

            let mut best_move: Move = moves[0];
            let mut best_score: i8 = if depth == max_depth {self.evaluate(temp_board, color)} else {(self.search(temp_board, depth + 1, max_depth, op).1) * -1};

            let mut temp_score: i8;

            for mv in moves {
                temp_board = board.clone();
                temp_board.make_move(mv);
                temp_score = if depth == max_depth {self.evaluate(temp_board, color)} else {(self.search(temp_board, depth + 1, max_depth, op).1) * -1};

                if temp_score > best_score {
                    best_score = temp_score;
                    best_move = mv;
                }

                self.pos_evaluated += 1;
            }

            return (best_move, best_score);
        }

        fn evaluate(&self, board: Board, color: Color) -> i8 {
            return AI::points(board, color);
        }

        fn points(board: Board, color: Color) -> i8 {
            let mut points: i8 = 0;
            let mut piece: Piece;

            for i in 0..8 {
                for l in 0..8 {
                    piece = board.get_piece(Position{x:i,y:l});
                    if piece.color == color {
                        points += piece.points()
                    } else if piece.color != Color::None {
                        points -= piece.points();
                    }
                }
            }

            return points;
        }
    }
}