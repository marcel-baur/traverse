use crate::board::Square;
use crate::logic::{GameState, Move};

mod board;
mod logic;
fn main() {
    let mut board = GameState::create();
    println!("{}", board);
    if let Err(e) = board.handle_move(Move {
        from: Square(6, 2),
        to: Square(5, 3),
    }) {
        println!("{}", e);
        return;
    }
    println!("{}", board);
}
