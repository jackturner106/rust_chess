use std::env;

use crate::controller::Controller;
mod controller;
mod engine;
mod model;
mod player;
mod starting_board;
mod user_input;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let board = starting_board::make_board();
    let game = controller::HumanGame {};
    game.play_game(board);
}
