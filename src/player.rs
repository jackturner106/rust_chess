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
    use std::time::Duration;
    use std::time::Instant;
    pub trait Player {
        fn take_turn(&mut self, board: Board, color: Color) -> Move;
    }

    const PAWN_POS: [[i16; 8]; 8] = [[100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 105, 110, 110, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100],
                                    [100, 100, 100, 100, 100, 100, 100, 100]];
    const KNIGHT_POS: [[i16; 8]; 8] = [[300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 330, 330, 330, 330, 300, 300],
                                    [300, 300, 320, 320, 320, 320, 300, 300],
                                    [300, 300, 310, 310, 310, 310, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300]];
    const BISHOP_POS: [[i16; 8]; 8] = [[300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300],
                                    [300, 310, 300, 300, 300, 300, 310, 300],
                                    [310, 300, 310, 300, 300, 310, 300, 310],
                                    [300, 310, 300, 300, 300, 300, 310, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300]];

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

            let mut now: Instant;
            let mut elapsed: Duration;
            now = Instant::now();

            let mut mv: Move;
            let mut cur_move: Move;
            let mut score: i16 = -32768;
            let mut temp_board;
            self.pos_evaluated = 0;
            let mut temp_score: i16;
            let mut move_list: Vec<Move>;
            let mut mv_string: Vec<String>;

            let moves = board.get_all_moves(color);
            let mut mv_boards: Vec<(Board, Move)> = moves.iter().map(|mv| {
                let mut temp_board = board.clone();
                temp_board.make_move(*mv);
                (temp_board, *mv)}).collect();
            mv_boards.sort_by(|mva, mvb| {
                return self.evaluate(mvb.0, color).cmp(&self.evaluate(mva.0, color))
            });
            mv = mv_boards[0].1;

            for mov in mv_boards {
                (temp_score, move_list) = self.alphabeta_trace(mov.0, 6, -32768, 32767, false, color.opponent_color());

                mv_string = move_list.iter().map(|m| m.to_string()).collect();
                cur_move = mov.1;
                println!("{cur_move}: {temp_score} from {mv_string:?}");

                if temp_score > score {
                    mv = mov.1;
                    score = temp_score;
                }
            }

            let evals:u64 = self.pos_evaluated;
            elapsed = now.elapsed();
            let per_second = (evals as f64) / elapsed.as_secs_f64();
            println!("Evaluted {evals} positions in {elapsed:?} for a speed of {per_second} positions per second");
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

        fn alphabeta_trace(&mut self, board: Board, depth: u8, al: i16, be: i16, max: bool, color: Color) -> (i16, Vec<Move>) {

            self.pos_evaluated += 1;

            if board.checkmatep(color) {
                // This possible introduces a bug, could be just return -32768?
                return if max {(-32768, vec![])} else {(32767, vec![])};
            } else if depth == 0 {
                return if max {(self.evaluate(board, color), vec![])} else {(self.evaluate(board, color) * -1, vec![])};
            }

            let mut best_score: i16; 
            let mut cur_score: i16;
            let mut temp_board: Board;
            let mut move_list: Vec<Move> = vec![];
            let mut temp_move_list: Vec<Move>;
            let op: Color = color.opponent_color();

            let mut a = al;
            let mut b = be;

            let moves: Vec<Move> = board.get_all_moves(color);
            if max {
                best_score = -32768;

                for mv in moves {
                    temp_board = board.clone();
                    temp_board.make_move(mv);
                    (cur_score, temp_move_list) = self.alphabeta_trace(temp_board, depth - 1, a, b, false, op);

                    if cur_score > best_score {
                        best_score = cur_score;
                        move_list = temp_move_list;
                        move_list.insert(0, mv);
                    }

                    a = cmp::max(a, best_score);

                    if best_score >= b {
                        return (best_score, move_list);
                    }
                }

                return (best_score, move_list);

            } else {
                best_score = 32767;

                for mv in moves {
                    temp_board = board.clone();
                    temp_board.make_move(mv);
                    (cur_score, temp_move_list) = self.alphabeta_trace(temp_board, depth - 1, a, b, true, op);
                    
                    if cur_score < best_score {
                        best_score = cur_score;
                        move_list = temp_move_list;
                        move_list.insert(0, mv);
                    }

                    b = cmp::min(b, best_score);

                    if best_score <= a {
                        return (best_score, move_list);
                    }
                }

                return (best_score, move_list);
            }
        }

        fn evaluate(&self, board: Board, color: Color) -> i16 {
            // Piece points (knights in center and forward, bishops on long files, rooks on 7th rank)
            // center control
            // rooks on open/semi open files
            // doubled pawns/pawn structure
            // fewer moves for the opponent, more moves for me
            let mut score = 0;
            score += AI::points(board, color) - AI::points(board, color.opponent_color());
            score += AI::moves(board, color) * 3;
            score += (AI::doubled_pawns(board, color) - AI::doubled_pawns(board, color.opponent_color())) * 0;
            return score;
        }

        fn print_evaluate(&self, board: Board, color: Color) {
            let pts = AI::points(board, color);
            //let mvs = AI::moves(board, color) * 3;
            let mvs = 0;
            let dps = AI::doubled_pawns(board, color) * 3;
            let tts = pts + mvs + dps;
            println!("Eval for {color:?}:: points: {pts}, moves: {mvs}, doubled: {dps}, total: {tts}");
        }

        fn moves(board: Board, color: Color) -> i16 {
            return 0;
            //return ((board.get_all_moves(color).len() as isize) - (board.get_all_moves(color.opponent_color()).len() as isize)).try_into().unwrap();
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
                        points += AI::piece_points(piece.piece_type, pos, color);
                    }
                }
            }

            return points;
        }

        fn piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {

            match piece {
                PieceType::Bishop=>return AI::get_pos_points(pos, color, BISHOP_POS),
                PieceType::Knight=>return AI::get_pos_points(pos, color, KNIGHT_POS),
                PieceType::Rook=>return 500,
                PieceType::King=>return 0,
                PieceType::Queen=>return 900,
                PieceType::Pawn=>return AI::get_pos_points(pos, color, PAWN_POS),
                PieceType::Empty=>return 0,
            };
        }

        fn get_pos_points(pos: Position, color: Color, grid: [[i16; 8]; 8]) -> i16 {
            let mp:Position = if color == Color::Black {pos} else {Position{x:7-pos.x,y:7-pos.y}};
            return grid[mp.y as usize][mp.x as usize];
        }
    }
}