mod board_representation;

use board_representation::bb_pieces::{Color, Piece};
use board_representation::board;

fn main() {
    let mut board = board::Board::new();
    print_bitboard(board.bb_occupancy.all.0);
    println!("{}", board.side_to_move);
}

fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
}
