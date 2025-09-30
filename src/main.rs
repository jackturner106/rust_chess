use std::env;

use crate::controller::Controller;
mod controller;
mod engine;
mod model;
mod player;
mod user_input;
fn main() {
    /*
    let mut board = make_board();
    println!("{board}");
    let mut moves = board.get_all_moves(Color::White);
    for mv in moves {
        println!("{mv}");
    }
    let hu: Human = Human{};
    let mvt = hu.take_turn(board);
    println!("{mvt}");
    */
    env::set_var("RUST_BACKTRACE", "1");

    let board = model::make_board();
    let game = controller::HumanGame {};
    game.play_game(board);
}
