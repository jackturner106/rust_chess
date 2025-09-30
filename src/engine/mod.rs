mod evaluator;
use crate::model::{Board, Color, Move, PieceType, Position};
use crate::player;
use std::cmp;

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

pub struct AI {
    pos_evaluated: u64,
}

impl player::Player for AI {
    fn take_turn(&mut self, board: Board, color: Color) -> Move {
        let now: Instant;
        let elapsed: Duration;
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
            (temp_score, move_list) =
                self.alphabeta_trace(mov.0, depth, -32768, 32767, false, color.opponent_color());

            mv_string = move_list.iter().map(|m| m.to_string()).collect();
            cur_move = mov.1;
            println!("{cur_move}: {temp_score} from {mv_string:?}");

            if temp_score > score {
                mv = mov.1;
                score = temp_score;
            }
        }

        let evals: u64 = self.pos_evaluated;
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
        let mut mv_boards: Vec<(Board, Move, i16)> = moves
            .iter()
            .map(|mv| {
                let mut temp_board = board.clone();
                temp_board.make_move(*mv);
                (temp_board, *mv, evaluator::evaluate(temp_board, color))
            })
            .collect();
        mv_boards.sort_by(|mva, mvb| return mvb.2.cmp(&mva.2));
        return mv_boards;
    }

    pub fn take_turn_threaded(&mut self, board: Board, color: Color) -> Move {
        evaluator::print_evaluate(board, color);

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
            .map(|i| {
                mv_boards.split_off(
                    (((mv_boards.len() as f64) / ((THREADS - i) as f64))
                        * ((THREADS - i - 1) as f64))
                        .round() as usize,
                )
            })
            .collect();

        let mut handles: Vec<JoinHandle<(i16, Move, u64)>> = mv_board_splits
            .into_iter()
            .map(|mvb| {
                thread::spawn(move || {
                    if mvb.len() == 0 {
                        return (
                            i16::MIN,
                            Move {
                                start: Position { x: 0, y: 0 },
                                end: Position { x: 0, y: 0 },
                            },
                            0,
                        );
                    }

                    let mut ai: AI = AI { pos_evaluated: 0 };
                    let mut ab_res;
                    let mut thread_temp_score: i16;
                    let mut thread_score: i16 = -32768;
                    let mut thread_mv: Move = mvb[0].1;
                    let mut tmvl: Vec<String>;
                    let thread_mv_boards = mvb;

                    for mov in thread_mv_boards {
                        ab_res = ai.alphabeta_trace(
                            mov.0,
                            depth,
                            -32768,
                            32767,
                            false,
                            color.opponent_color(),
                        );
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
                })
            })
            .collect();

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
        return AI { pos_evaluated: 0 };
    }

    fn alphabeta(
        &mut self,
        board: Board,
        depth: u8,
        al: i16,
        be: i16,
        max: bool,
        color: Color,
    ) -> i16 {
        self.pos_evaluated += 1;

        if board.checkmatep(color) {
            // This possible introduces a bug, could be just return -32768?
            return if max { -32768 } else { 32767 };
        } else if depth == 0 {
            return if max {
                evaluator::evaluate(board, color)
            } else {
                evaluator::evaluate(board, color) * -1
            };
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

    fn alphabeta_trace(
        &mut self,
        board: Board,
        depth: u8,
        al: i16,
        be: i16,
        max: bool,
        color: Color,
    ) -> (i16, Vec<Move>) {
        self.pos_evaluated += 1;

        if board.checkmatep(color) {
            // This possible introduces a bug, could be just return -32768?
            return if max {
                (-32768, vec![])
            } else {
                (32767, vec![])
            };
        } else if depth == 0 {
            return if max {
                (evaluator::evaluate(board, color), vec![])
            } else {
                (evaluator::evaluate(board, color.opponent_color()), vec![])
            };
        }

        let mut best_score: i16;
        let mut cur_score: i16;
        let mut move_list: Vec<Move> = vec![];
        let mut temp_move_list: Vec<Move>;
        let mut temp_board: Board;
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
                (cur_score, temp_move_list) =
                    self.alphabeta_trace(temp_board, depth - 1, a, b, false, op);

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
                (cur_score, temp_move_list) =
                    self.alphabeta_trace(temp_board, depth - 1, a, b, true, op);

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
            PieceType::Bishop => 30,
            PieceType::Knight => 20,
            PieceType::Rook => 40,
            PieceType::King => 0,
            PieceType::Queen => 50,
            PieceType::Pawn => 10,
            PieceType::Empty => 0,
        } + match board.get_piece(mv.start).piece_type {
            PieceType::Bishop => 3,
            PieceType::Knight => 4,
            PieceType::Rook => 2,
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Pawn => 5,
            PieceType::Empty => 0,
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
}
