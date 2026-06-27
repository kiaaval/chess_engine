use crate::board_representation::{
    bb_pieces::{BitBoard, Color, EveryPiece, EverySide, Piece},
    game_state::GameState,
    side_to_move::{MoveTurn, MoveTurns},
};

pub struct Board {
    pub bb_pieces: EverySide<EveryPiece<BitBoard>>,
    pub side_to_move: MoveTurn,
    pub game_stage: GameState,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            bb_pieces: EverySide::new(EveryPiece::new(BitBoard::empty())),
            side_to_move: MoveTurn::default(),
            game_stage: GameState::empty(),
        }
    }

    pub fn new() -> Self {
        let mut bb = EverySide::new(EveryPiece::new(BitBoard::empty()));

        {
            let white = &mut bb[Color::White];
            white[Piece::Pawn] = BitBoard(0x0000_0000_0000_FF00);
            white[Piece::Knight] = BitBoard(0x0000_0000_0000_0042);
            white[Piece::Bishop] = BitBoard(0x0000_0000_0000_0024);
            white[Piece::Rook] = BitBoard(0x0000_0000_0000_0081);
            white[Piece::Queen] = BitBoard(0x0000_0000_0000_0008);
            white[Piece::King] = BitBoard(0x0000_0000_0000_0010);
        }
        {
            let black = &mut bb[Color::Black];
            black[Piece::Pawn] = BitBoard(0x00FF_0000_0000_0000);
            black[Piece::Knight] = BitBoard(0x0042_0000_0000_0000);
            black[Piece::Bishop] = BitBoard(0x0024_0000_0000_0000);
            black[Piece::Rook] = BitBoard(0x8100_0000_0000_0000);
            black[Piece::Queen] = BitBoard(0x0800_0000_0000_0000);
            black[Piece::King] = BitBoard(0x1000_0000_0000_0000);
        }
        Self {
            bb_pieces: bb,
            side_to_move: MoveTurn::default(),
            game_stage: GameState::new(),
        }
    }
}
