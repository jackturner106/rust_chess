pub mod players {

    use crate::Board;
    use crate::Color;
    use crate::Move;
    use crate::Piece;
    use crate::PieceType;
    use crate::Position;
    use std::any::Any;
    use std::cmp;
    use std::io;
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use std::time::Instant;
    pub trait Player {
        fn take_turn(&mut self, board: Board, color: Color) -> Move;
    }

    const PAWN_POS: [[i16; 8]; 8] = [[106, 106, 107, 108, 108, 107, 106, 106],
                                    [105, 105, 106, 107, 107, 106, 105, 105],
                                    [104, 104, 105, 106, 106, 105, 104, 104],
                                    [103, 103, 104, 105, 105, 104, 103, 103],
                                    [102, 102, 103, 104, 104, 103, 102, 102],
                                    [101, 101, 102, 103, 103, 102, 101, 101],
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
                                    [300, 313, 300, 300, 300, 300, 313, 300],
                                    [300, 310, 312, 300, 300, 312, 310, 300],
                                    [310, 300, 310, 311, 311, 310, 300, 310],
                                    [300, 310, 300, 300, 300, 300, 310, 300],
                                    [300, 300, 300, 300, 300, 300, 300, 300]];

    pub struct Human {

    }

    impl Player for Human {
        fn take_turn(&mut self, board: Board, color: Color) -> Move {
            let mut my_move;
            let moves: Vec<Move> = board.get_all_moves(color);
            println!("Your Move:");
            let mut start = String::new();

            io::stdin()
                .read_line(&mut start)
                .expect("Failed to read line");
            
            let end = start.split_off(2);

            my_move = Move {start:Position::from_string(start), end:Position::from_string(end)};
            
            while !(moves.contains(&my_move)) {
                println!("Thats not a valid move, try again");
                let mut start = String::new();
    
                io::stdin()
                    .read_line(&mut start)
                    .expect("Failed to read line");
                
                let end = start.split_off(2);
    
                my_move = Move {start:Position::from_string(start), end:Position::from_string(end)};
            }

            return my_move;
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
            let mv_boards: Vec<(Board, Move, i16)> = self.make_boards(moves, board, color);
            mv = mv_boards[0].1;

            let mut depth: u8 = 1;
            while i32::pow(mv_boards.len() as i32, depth as u32) < 45000000 {
                depth += 1;
            }
            println!("Searching to depth {depth}");

            for mov in mv_boards {
                (temp_score, move_list) = self.alphabeta_trace(mov.0, depth, -32768, 32767, false, color.opponent_color());

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

            return mv;
        }
    }

    impl AI {

        fn make_boards(&self, moves: Vec<Move>, board: Board, color: Color) -> Vec<(Board, Move, i16)> {
            let mut mv_boards: Vec<(Board, Move, i16)> = moves.iter().map(|mv| {
                let mut temp_board = board.clone();
                temp_board.make_move(*mv);
                (temp_board, *mv, self.evaluate(temp_board, color))}).collect();
            mv_boards.sort_by(|mva, mvb| {
                return mvb.2.cmp(&mva.2)
            });
            return mv_boards;
        }

        pub fn take_turn_threaded(&mut self, board: Board, color: Color) -> Move {
            self.print_evaluate(board, color);

            let now: Instant = Instant::now();
            let mut depth = 2;
            let mut mv: Move = self.run_threads_to_depth(board, color, 1);
            
            while now.elapsed() < Duration::new(0, 500000000) {
                mv = self.run_threads_to_depth(board, color, depth);
                depth += 1;
            }
            mv = self.run_threads_to_depth(board, color, depth);

            println!("Got to depth {depth}");

            return mv;
        }

        fn run_threads_to_depth(&mut self, board: Board, color: Color, depth: u8) -> Move {
            let now: Instant = Instant::now();
            let elapsed: Duration;
            const THREADS: usize = 8;

            let mut mv: Move;
            let mut score: i16 = -32768;
            self.pos_evaluated = 0;

            let moves = board.get_all_moves(color);
            let mut mv_boards = self.make_boards(moves, board, color);
            mv = mv_boards[0].1;

            let mv_board_splits: Vec<Vec<(Board, Move, i16)>> = (0..THREADS)
            .map(|i| mv_boards.split_off((((mv_boards.len() as f64)/((THREADS - i) as f64)) * ((THREADS - i - 1)as f64)).round() as usize))
            .collect();

            let mut handles: Vec<JoinHandle<(i16, Move, u64)>> = mv_board_splits.into_iter().map(|mvb|
                thread::spawn(move || {
                    if mvb.len() == 0 {
                        return (i16::MIN, Move{start:Position{x:0,y:0},end:Position{x:0,y:0}}, 0);
                    }

                    let mut ai: AI = AI{pos_evaluated:0};
                    let mut ab_res;
                    let mut thread_temp_score: i16;
                    let mut thread_score: i16 = -32768;
                    let mut thread_mv: Move = mvb[0].1;
                    let mut tmvl: Vec<String>;
                    let thread_mv_boards = mvb;
    
                    for mov in thread_mv_boards {
                        ab_res = ai.alphabeta_trace(mov.0, depth, -32768, 32767, false, color.opponent_color());
                        let tmv = mov.1;
                        let tsc = ab_res.0;
                        tmvl = ab_res.1.iter().map(|m| m.to_string()).collect();
                        println!("Move {tmv} got score {tsc} with {tmvl:?}");
                        thread_temp_score = ab_res.0;
        
                        if thread_temp_score > thread_score {
                            thread_mv = mov.1;
                            thread_score = thread_temp_score;
                        }
                    }
    
                    return (thread_score, thread_mv, ai.pos_evaluated);
                })).collect();

            let mut evals: u64 = 0;
            let mut val: (i16, Move, u64);
            handles.reverse();
            for handle in handles {
                val = handle.join().unwrap();
                let tmv = val.1;
                let tsc = val.0;
                println!("Best from thread: {tmv} with score {tsc}");
                evals += val.2;
                if val.0 > score {
                    score = val.0;
                    mv = val.1;
                }
            }

            println!("Evaluted: {evals} ");
            elapsed = now.elapsed();
            let per_second = (evals as f64) / elapsed.as_secs_f64();
            println!("Evaluted {evals} positions in {elapsed:?} for a speed of {per_second} positions per second");
            println!("Best Position: {score}");

            return mv;
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
                        return best_score
                    }
                }

                return best_score
            }
        }

        fn alphabeta_trace(&mut self, board: Board, depth: u8, al: i16, be: i16, max: bool, color: Color) -> (i16, Vec<Move>) {

            self.pos_evaluated += 1;

            if board.checkmatep(color) {
                // This possible introduces a bug, could be just return -32768?
                return if max {(-32768, vec![])} else {(32767, vec![])};
            } else if depth == 0 {
                return if max {(self.evaluate(board, color), vec![])} else {(self.evaluate(board, color.opponent_color()), vec![])};
            }

            let mut best_score: i16; 
            let mut cur_score: i16;
            let mut move_list: Vec<Move> = vec![];
            let mut temp_move_list: Vec<Move>;
            let mut temp_board: Board;
            let mut mv: Move;
            let op: Color = color.opponent_color();

            let mut a = al;
            let mut b = be;

            let moves: Vec<Move> = board.get_all_moves(color);
            //let mut mvv_lva_moves: Vec<(Move, i16)> = moves.into_iter().map(|mva| (mva, self.mvv_lva_score(board, mva))).collect();
            if max {
                best_score = -32768;

                //for i in 0..mvv_lva_moves.len() {
                    //self.pick_move(&mut mvv_lva_moves, i);
                    //mv = mvv_lva_moves[i].0;
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

                    if best_score > b {
                        return (best_score, move_list);
                    }
                }

                return (best_score, move_list);

            } else {
                best_score = 32767;

                //for i in 0..mvv_lva_moves.len() {
                    //self.pick_move(&mut mvv_lva_moves, i);
                    //mv = mvv_lva_moves[i].0;
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

                    if best_score < a {
                        return (best_score, move_list);
                    }
                }

                return (best_score, move_list);
            }
        }

        fn mvv_lva_score(&self, board: Board, mv: Move) -> i16 {
           return match board.get_piece(mv.end).piece_type {
                PieceType::Bishop=>30,
                PieceType::Knight=>20,
                PieceType::Rook=>40,
                PieceType::King=>0,
                PieceType::Queen=>50,
                PieceType::Pawn=>10,
                PieceType::Empty=>0,
            } + match board.get_piece(mv.start).piece_type {
                PieceType::Bishop=>3,
                PieceType::Knight=>4,
                PieceType::Rook=>2,
                PieceType::King=>0,
                PieceType::Queen=>1,
                PieceType::Pawn=>5,
                PieceType::Empty=>0,
            };
        }

        fn pick_move(&self, move_list: &mut Vec<(Move, i16)>, i: usize) {
            let mut max_ind = i;
            let mut max_val = move_list[i].1;
            let mut iter = i;
            while iter < move_list.len() {
                if move_list[iter].1 > max_val {
                    max_ind = iter;
                    max_val = move_list[iter].1;
                }
                iter += 1;
            }
            let temp = move_list[i];
            move_list[i] = move_list[max_ind];
            move_list[max_ind] = temp;
        }

        fn evaluate(&self, board: Board, color: Color) -> i16 {
            // Tapered evaluation: Chess boards start with 39 * 2 = 78 points
            // after 4 pawns + 4 pieces captured middlegame, so 78 - 16 = 62
            let mut score = 0;
            let my_points = AI::points(board, color);
            let op_points = AI::points(board, color.opponent_color());
            let (my_double_p, my_rook_open, my_rook_semi, my_bishops) = AI::doubled_pawns(board, color);
            let (op_double_p, op_rook_open, op_rook_semi, op_bishops) = AI::doubled_pawns(board, color.opponent_color());

            let total_points = my_points + op_points;

            score += my_points - op_points;
            score += ((op_double_p - my_double_p) as i16) * 3;
            score += ((my_rook_open - op_rook_open) as i16) * 40;
            score += ((my_rook_semi - op_rook_semi) as i16) * 10;

            score += if my_bishops {10} else {0};
            score -= if op_bishops {10} else {0};

            return score;
        }

        fn print_evaluate(&self, board: Board, color: Color) -> i16 {
            // Tapered evaluation: Chess boards start with 39 * 2 = 78 points
            // after 4 pawns + 4 pieces captured middlegame, so 78 - 16 = 62
            // (n - 62) / 16
            let mut early_score = 0;
            let mut middle_score = 0;
            let mut late_score = 0;

            let my_points = AI::points(board, color);
            let op_points = AI::points(board, color.opponent_color());

            let my_ep = AI::early_points(board, color);
            let op_ep = AI::early_points(board, color.opponent_color());
            let my_lp = AI::late_points(board, color);
            let op_lp = AI::late_points(board, color.opponent_color());

            let (my_double_p, my_rook_open, my_rook_semi, my_bishops) = AI::doubled_pawns(board, color);
            let (op_double_p, op_rook_open, op_rook_semi, op_bishops) = AI::doubled_pawns(board, color.opponent_color());
            println!("{my_points}, {my_double_p}, {my_rook_open}, {my_rook_semi}, {my_bishops}");
            println!("{op_points}, {op_double_p}, {op_rook_open}, {op_rook_semi}, {op_bishops}");

            let total_points = my_points + op_points;

            early_score += my_ep - op_ep;
            early_score += ((op_double_p - my_double_p) as i16) * 3;
            early_score += ((my_rook_open - op_rook_open) as i16) * 40;
            early_score += ((my_rook_semi - op_rook_semi) as i16) * 10;
            early_score += if my_bishops {10} else {0};
            early_score -= if op_bishops {10} else {0};

            late_score += my_ep - op_ep;
            late_score += ((op_double_p - my_double_p) as i16) * 3;
            late_score += ((my_rook_open - op_rook_open) as i16) * 40;
            late_score += ((my_rook_semi - op_rook_semi) as i16) * 10;
            late_score += if my_bishops {10} else {0};
            late_score -= if op_bishops {10} else {0};

            let score = ((early_score as f32 * total_points as f32 / 78.0).round() + (early_score as f32 * (1.0 - (total_points as f32 / 78.0))).round()) as i16;

            println!("Score: {score}");

            return score;
        }

        // Returns: (number of doubled pawns, number of rooks on open files, number of rooks on semi open files, 
        //           bishop pair)
        fn doubled_pawns(board: Board, color: Color) -> (u8, u8, u8, bool) {
            let mut pawns: u8;
            let mut opawns: u8;
            let mut rooks: u8;
            let mut piece: Piece;
            let mut bishops: u8 = 0;
            let mut total_doubled_pawns: u8 = 0;
            let mut total_rooks_open: u8 = 0;
            let mut total_rooks_semi: u8 = 0;

            for col in 0..8 {
                pawns = 0;
                rooks = 0;
                opawns = 0;
                for row in 0..8 {
                    piece = board.get_piece(Position{x:col,y:row});
                    if piece.piece_type == PieceType::Pawn {
                        if piece.color == color {
                            pawns += 1;
                        }
                        else {
                            opawns += 1;
                        }
                    } else if piece.piece_type == PieceType::Rook && piece.color == color {
                        rooks += 1;
                    } else if piece.piece_type == PieceType::Bishop && piece.color == color {
                        bishops += 1;
                    }
                }
                total_doubled_pawns += if pawns > 1 {pawns} else {0};
                if pawns + opawns == 1 {
                    total_rooks_semi += rooks;
                } else if pawns + opawns == 0 {
                    total_rooks_open += rooks;
                }
            }
            return (total_doubled_pawns, total_rooks_open, total_rooks_semi, bishops > 1);
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

        fn early_points(board: Board, color: Color) -> i16 {
            let mut points: i16 = 0;
            let mut piece: Piece;
            let mut pos: Position;

            for i in 0..8 {
                for l in 0..8 {
                    pos = Position{x:i,y:l};
                    piece = board.get_piece(pos);
                    if piece.color == color {
                        points += AI::early_piece_points(piece.piece_type, pos, color);
                    }
                }
            }

            return points;
        }

        fn late_points(board: Board, color: Color) -> i16 {
            let mut points: i16 = 0;
            let mut piece: Piece;
            let mut pos: Position;

            for i in 0..8 {
                for l in 0..8 {
                    pos = Position{x:i,y:l};
                    piece = board.get_piece(pos);
                    if piece.color == color {
                        points += AI::late_piece_points(piece.piece_type, pos, color);
                    }
                }
            }

            return points;
        }

        fn piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {

            match piece {
                PieceType::Bishop=>return 300,
                PieceType::Knight=>return 300,
                PieceType::Rook=>return 500,
                PieceType::King=>return 0,
                PieceType::Queen=>return 900,
                PieceType::Pawn=>return 100,
                PieceType::Empty=>return 0,
            };
        }

        fn early_piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {

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

        fn late_piece_points(piece: PieceType, pos: Position, color: Color) -> i16 {

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