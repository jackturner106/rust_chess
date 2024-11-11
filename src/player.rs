pub mod players {

    use crate::Board;
    use crate::Color;
    use crate::Move;
    use crate::Piece;
    use crate::PieceType;
    use crate::Position;
    use std::cmp;
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
            let mut mv: Move;
            let mut score: i16 = -32768;
            let mut temp_board;
            self.pos_evaluated = 0;
            let mut temp_score: i16;

            let moves = board.get_all_moves(color);
            mv = moves[0];
            for mov in moves {
                temp_board = board.clone();
                temp_board.make_move(mov);
                temp_score = self.alphabeta(temp_board, 3, -32768, 32767, false, color.opponent_color());

                if temp_score > score {
                    mv = mov;
                    score = temp_score;
                }
            }
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

        fn alphabeta(&mut self, board: Board, depth: u8, al: i16, be: i16, max: bool, color: Color) -> i16 {

            if depth == 0 {
                return if max {self.evaluate(board, color)} else {self.evaluate(board, color) * -1};
            }

            self.pos_evaluated += 1;
            let mut best_score: i16; 
            let mut cur_score: i16;
            let mut temp_board: Board;
            let op: Color = color.opponent_color();

            let mut a = al;
            let mut b = be;

            let moves: Vec<Move> = board.get_all_moves(color);
            if max {
                best_score = -32768;

                for mv in moves {
                    temp_board = board.clone();
                    temp_board.make_move(mv);
                    cur_score = self.alphabeta(temp_board, depth - 1, a, b, false, op);

                    if cur_score > best_score {
                        best_score = cur_score;
                    }

                    a = cmp::max(a, best_score);

                    if best_score >= b {
                        return best_score;
                    }
                }

                return best_score;

            } else {
                best_score = 32767;

                for mv in moves {
                    temp_board = board.clone();
                    temp_board.make_move(mv);
                    cur_score = self.alphabeta(temp_board, depth - 1, a, b, true, op);
                    
                    if cur_score < best_score {
                        best_score = cur_score;
                    }

                    b = cmp::min(b, best_score);

                    if best_score <= a {
                        return best_score;
                    }
                }

                return best_score;
            }
        }

        fn evaluate(&self, board: Board, color: Color) -> i16 {
            return AI::points(board, color);
        }

        fn points(board: Board, color: Color) -> i16 {
            let mut points: i16 = 0;
            let mut piece: Piece;

            for i in 0..8 {
                for l in 0..8 {
                    piece = board.get_piece(Position{x:i,y:l});
                    if piece.color == color {
                        points += AI::piece_points(piece.piece_type)
                    } else if piece.color != Color::None {
                        points -= AI::piece_points(piece.piece_type);
                    }
                }
            }

            return points;
        }

        fn piece_points(piece: PieceType) -> i16 {
            match piece {
                PieceType::Bishop=>return 3,
                PieceType::Knight=>return 3,
                PieceType::Rook=>return 5,
                PieceType::King=>return 0,
                PieceType::Queen=>return 9,
                PieceType::Pawn=>return 1,
                PieceType::Empty=>return 0,
            };
        }
    }
}