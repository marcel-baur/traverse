use crate::board::Square;
use crate::logic::GameState;

mod board;
mod logic;
fn main() {
    let board = GameState::create();
    println!("{}", board);
}
