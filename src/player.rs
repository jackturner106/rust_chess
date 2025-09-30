use crate::model::{Board, Color, Move};

pub(crate) trait Player {
    fn take_turn(&mut self, board: Board, color: Color) -> Move;
}
// mod players
