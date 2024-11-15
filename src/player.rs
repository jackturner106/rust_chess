pub mod players {

    use crate::Board;
    use crate::Color;
    use crate::Move;
    use crate::Piece;
    use crate::PieceType;
    use crate::Position;
    use std::cmp;
    use std::io;
    use std::thread;
    use std::thread::JoinHandle;
    pub trait Player {
        fn take_turn(&mut self, board: Board, color: Color) -> Move;
    }

    pub struct Human {

    }

    impl Player for Human {
        fn take_turn(&mut self, board: Board, color: Color) -> Move {
            println!("Your Move:");
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
                temp_score = self.alphabeta(temp_board, 4, -32768, 32767, false, color.opponent_color());

                println!("{mov}: {temp_score}");

                if temp_score > score {
                    mv = mov;
                    score = temp_score;
                }
            }

            let evals:u64 = self.pos_evaluated;
            println!("Evaluted: {evals} ");
            println!("Best Position: {score}");

            temp_board = board.clone();
            temp_board.make_move(mv);
            self.print_evaluate(temp_board, color);

            return mv;
        }
    }

    impl AI {

        pub fn take_turn_threaded(&mut self, board: Board, color: Color) -> Move {
            let mut mv: Move;
            let mut score: i16 = -32768;
            let mut temp_board;
            self.pos_evaluated = 0;
            let mut temp_score: i16;

            let mut moves = board.get_all_moves(color);
            mv = moves[0];
            let moves2 = moves.split_off(moves.len()/2);

            let handle: JoinHandle<(i16, Move, u64)> = thread::spawn(move || {
                let mut thread_temp_board: Board;
                let mut ai: AI = AI{pos_evaluated:0};
                let mut thread_temp_score: i16;
                let mut thread_score: i16 = -32768;
                let mut thread_mv: Move = moves2[0];

                for mov in moves2 {
                    thread_temp_board = board.clone();
                    thread_temp_board.make_move(mov);
                    thread_temp_score = ai.alphabeta(thread_temp_board, 5, -32768, 32767, false, color.opponent_color());
    
                    if thread_temp_score > thread_score {
                        thread_mv = mov;
                        thread_score = thread_temp_score;
                    }
                }

                return (thread_score, thread_mv, ai.pos_evaluated);
            });

            for mov in moves {
                temp_board = board.clone();
                temp_board.make_move(mov);
                temp_score = self.alphabeta(temp_board, 5, -32768, 32767, false, color.opponent_color());

                if temp_score > score {
                    mv = mov;
                    score = temp_score;
                }
            }

            let res = handle.join();
            let val = res.unwrap();
            println!("thread: {val:?}");

            let evals:u64 = self.pos_evaluated + val.2;
            println!("Evaluted: {evals} ");
            println!("Best Position: {score}");

            return if val.0 > score {val.1} else {mv};
        }

        pub fn new() -> AI {
            return AI { pos_evaluated: 0};
        }

        fn alphabeta(&mut self, board: Board, depth: u8, al: i16, be: i16, max: bool, color: Color) -> i16 {

            self.pos_evaluated += 1;

            if board.checkmatep(color) {
                // This possible introduces a bug, could be just return -32768?
                return if max {-32768} else {32767};
            } else if depth == 0 {
                return if max {self.evaluate(board, color)} else {self.evaluate(board, color) * -1};
            }

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
            // Piece points (knights in center and forward, bishops on long files, rooks on 7th rank)
            // center control
            // rooks on open/semi open files
            // doubled pawns/pawn structure
            // fewer moves for the opponent, more moves for me
            let mut score = 0;
            score += AI::points(board, color);
            score += AI::moves(board, color) * 3;
            score += AI::doubled_pawns(board, color) * 3;
            return score;
        }

        fn print_evaluate(&self, board: Board, color: Color) {
            let pts = AI::points(board, color);
            let mvs = AI::moves(board, color) * 3;
            let dps = AI::doubled_pawns(board, color) * 3;
            let tts = pts + mvs + dps;
            println!("Eval for {color:?}:: points: {pts}, moves: {mvs}, doubled: {dps}, total: {tts}");
        }

        fn moves(board: Board, color: Color) -> i16 {
            return ((board.get_all_moves(color).len() as isize) - (board.get_all_moves(color.opponent_color()).len() as isize)).try_into().unwrap();
        }

        fn doubled_pawns(board: Board, color: Color) -> i16 {
            let mut pawns: i16;
            let mut total: i16 = 0;
            let mut piece: Piece;

            for col in 0..8 {
                pawns = 0;
                for row in 0..8 {
                    piece = board.get_piece(Position{x:col,y:row});
                    if piece.piece_type == PieceType::Pawn && piece.color == color {
                        pawns += 1;
                    }
                }
                total += if pawns > 1 {pawns - 1} else {0};
            }
            return total;
        }

        fn points(board: Board, color: Color) -> i16 {
            let mut points: i16 = 0;
            let mut piece: Piece;
            let mut pos: Position;

            for i in 0..8 {
                for l in 0..8 {
                    pos = Position{x:i,y:l};
                    piece = board.get_piece(pos);
                    if piece.color == color {
                        points += AI::piece_points(piece.piece_type, pos, color)
                    } else if piece.color != Color::None {
                        points -= AI::piece_points(piece.piece_type, pos, color.opponent_color());
                    }
                }
            }

            return points;
        }

        fn piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {

            let pawn_pos: [[i16; 8]; 8] = [[100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 105, 110, 110, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100],
                                            [100, 100, 100, 100, 100, 100, 100, 100]];
            let knight_pos: [[i16; 8]; 8] = [[300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 330, 330, 330, 330, 300, 300],
                                            [300, 300, 320, 320, 320, 320, 300, 300],
                                            [300, 300, 310, 310, 310, 310, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300]];
            let bishop_pos: [[i16; 8]; 8] = [[300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300],
                                            [300, 310, 300, 300, 300, 300, 310, 300],
                                            [310, 300, 310, 300, 300, 310, 300, 310],
                                            [300, 310, 300, 300, 300, 300, 310, 300],
                                            [300, 300, 300, 300, 300, 300, 300, 300]];

            match piece {
                PieceType::Bishop=>return AI::get_pos_points(pos, color, bishop_pos),
                PieceType::Knight=>return AI::get_pos_points(pos, color, knight_pos),
                PieceType::Rook=>return 500,
                PieceType::King=>return 0,
                PieceType::Queen=>return 900,
                PieceType::Pawn=>return AI::get_pos_points(pos, color, pawn_pos),
                PieceType::Empty=>return 0,
            };
        }

        fn get_pos_points(pos: Position, color: Color, mut grid: [[i16; 8]; 8]) -> i16 {
            if color == Color::White {
                grid.reverse();
                grid[pos.y as usize].reverse();
            }
            return grid[pos.y as usize][pos.x as usize];
        }
    }
}