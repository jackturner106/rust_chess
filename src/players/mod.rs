pub mod engine;
pub mod user_input;

use crate::Board;
use crate::Color;
use crate::Move;

pub trait Player {
    fn take_turn(&mut self, board: Board, color: Color) -> Move;
}
// mod players
