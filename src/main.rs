mod board_representation;

use board_representation::bb_pieces::{Color, Piece};
use board_representation::board;

fn main() {
    let mut board = board::Board::new();
    println!("{}", board.bb_occupancy.all);
    println!("{}", board.side_to_move);
}

