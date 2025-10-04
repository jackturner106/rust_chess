use crate::model::board::Board;

pub(crate) struct Node {
    board: Board,
    depth_below_top: u8,
    max_child_depth: u8,
    children: Vec<Node>,
}
