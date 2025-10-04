use crate::model::{board::Board, color::Color, move_::Move};

pub(crate) trait Player {
    fn take_turn(&mut self, board: Board, color: Color) -> Move;
}
// mod players
