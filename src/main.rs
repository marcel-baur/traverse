use crate::board::Square;
use crate::logic::{GameState, Move};

mod board;
mod logic;
fn main() {
    let mut board = GameState::create();
    println!("{}", board);
}
