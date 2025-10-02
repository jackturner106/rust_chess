mod evaluator;
use crate::model::{Board, Color, Move, PieceType, Position};
use crate::player;
use std::cmp;

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

const THREADED: bool = true;
const TIME_LIMIT: Duration = Duration::from_secs(60);
const NUM_THREADS: usize = 8;

pub struct AI {
    pos_evaluated: u64,
}

pub struct MoveScore {
    board: Board,
    score: i16,
}

impl PartialEq for MoveScore {
    fn eq(&self, other: &Self) -> bool {
        return self.board == other.board && self.score == other.score;
    }
}

impl Eq for MoveScore {}

impl PartialOrd for MoveScore {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for MoveScore {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        return self.score.cmp(&other.score);
    }
}

impl player::Player for AI {
    fn take_turn(&mut self, board: Board, color: Color) -> Move {
        if THREADED {
            return self.take_turn_threaded(board, color);
        }
        return self.take_turn_private(board, color);
    }
}

impl AI {
    fn take_turn_threaded(&mut self, board: Board, color: Color) -> Move {
        let start_time: Instant = Instant::now();

        let depth = 4;
        let mut moves = board.get_all_moves(color);
        let mut new_moves: Vec<MoveScore> = Vec::new();

        if moves.len() == 1 {
            return moves[0];
        } else if moves.len() < 1 {
            return Move {
                start: Position { x: 0, y: 0 },
                end: Position { x: 0, y: 0 },
            };
        }

        let mut threads: Vec<JoinHandle<(i16, Board, u64)>> = Vec::new();

        while start_time.elapsed() < TIME_LIMIT {
            while !moves.is_empty() {
                if threads.len() < NUM_THREADS {
                    threads.push(self.new_thread(moves.pop().unwrap(), board, color, depth));
                } else if threads.iter().any(|f| -> bool { f.is_finished() }) {
                    let index = threads
                        .iter()
                        .position(|f| -> bool { f.is_finished() })
                        .unwrap();
                    let (score, board, pos_eval) = threads.remove(index).join().unwrap();
                    self.pos_evaluated += pos_eval;

                    let move_score = MoveScore {
                        board: board,
                        score: score,
                    };
                    match new_moves.binary_search(&move_score) {
                        Ok(_) => {}
                        Err(pos) => new_moves.insert(
                            pos,
                            MoveScore {
                                board: board,
                                score: score,
                            },
                        ),
                    }
                }
            }
            // Explore board threaded...
        }
        return board.get_all_moves(color)[0];
    }

    fn take_turn_private(&mut self, board: Board, color: Color) -> Move {
        let now: Instant;
        let elapsed: Duration;
        now = Instant::now();

        let mut mv: Move;
        let mut score: i16 = -32768;
        self.pos_evaluated = 0;
        let mut temp_score: i16;

        let moves = board.get_all_moves(color);

        if moves.len() == 1 {
            return moves[0];
        } else if moves.len() == 0 {
            return Move {
                start: Position { x: 0, y: 0 },
                end: Position { x: 0, y: 0 },
            };
        }

        mv = moves[0];
        let mut depth: u8 = 1;

        while now.elapsed() < TIME_LIMIT {
            for mov in &moves {
                let mut nb = board.clone();
                nb.make_move(*mov);
                temp_score = self
                    .alphabeta_trace(nb, depth, -32768, 32767, false, color.opponent_color())
                    .0;

                println!("{mov}: {temp_score}");

                if temp_score > score {
                    mv = *mov;
                    score = temp_score;
                }
            }
            depth += 1;
        }

        let evals = self.pos_evaluated;
        elapsed = now.elapsed();
        let per_second = (evals as f64) / elapsed.as_secs_f64();
        println!("Evaluted {evals} positions in {elapsed:?} for a speed of {per_second} positions per second");
        println!("Best Position: {score}");

        return mv;
    }

    fn new_thread(
        &self,
        move_: Move,
        board: Board,
        color: Color,
        depth: u8,
    ) -> JoinHandle<(i16, Board, u64)> {
        let mut ai = AI { pos_evaluated: 0 };
        let mut nb = board.clone();
        nb.make_move(move_);
        return thread::spawn(move || {
            let (rscore, rboard) = ai.alphabeta_trace(nb, depth, -32768, 32767, false, color);
            return (rscore, rboard, ai.pos_evaluated);
        });
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
    ) -> (i16, Board) {
        self.pos_evaluated += 1;

        if board.checkmatep(color) {
            // This possible introduces a bug, could be just return -32768?
            return if max { (-32768, board) } else { (32767, board) };
        } else if depth == 0 {
            return if max {
                (evaluator::evaluate(board, color), board)
            } else {
                (evaluator::evaluate(board, color.opponent_color()), board)
            };
        }

        let mut best_score: i16;
        let mut cur_score: i16;
        let mut ret_board: Board;
        let mut temp_ret_board: Board;
        let mut temp_board: Board;
        let op: Color = color.opponent_color();

        let mut a = al;
        let mut b = be;

        let moves: Vec<Move> = board.get_all_moves(color);
        ret_board = board.clone();
        ret_board.make_move(moves[0]);
        //let mut mvv_lva_moves: Vec<(Move, i16)> = moves.into_iter().map(|mva| (mva, self.mvv_lva_score(board, mva))).collect();
        if max {
            best_score = -32768;

            //for i in 0..mvv_lva_moves.len() {
            //self.pick_move(&mut mvv_lva_moves, i);
            //mv = mvv_lva_moves[i].0;
            for mv in moves {
                temp_board = board.clone();
                temp_board.make_move(mv);
                (cur_score, temp_ret_board) =
                    self.alphabeta_trace(temp_board, depth - 1, a, b, false, op);

                if cur_score > best_score {
                    best_score = cur_score;
                    ret_board = temp_ret_board;
                }

                a = cmp::max(a, best_score);

                if best_score > b {
                    return (best_score, ret_board);
                }
            }

            return (best_score, board);
        } else {
            best_score = 32767;

            //for i in 0..mvv_lva_moves.len() {
            //self.pick_move(&mut mvv_lva_moves, i);
            //mv = mvv_lva_moves[i].0;
            for mv in moves {
                temp_board = board.clone();
                temp_board.make_move(mv);
                (cur_score, temp_ret_board) =
                    self.alphabeta_trace(temp_board, depth - 1, a, b, true, op);

                if cur_score < best_score {
                    best_score = cur_score;
                    ret_board = temp_ret_board;
                }

                b = cmp::min(b, best_score);

                if best_score < a {
                    return (best_score, ret_board);
                }
            }

            return (best_score, ret_board);
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
