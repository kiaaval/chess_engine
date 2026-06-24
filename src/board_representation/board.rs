use crate::board_representation::{
    bb_pieces::{BitBoard, EveryPiece, EverySide},
    game_state::GameState,
    side_to_move::MoveTurn,
};

pub struct Board {
    pub bb_pieces: EverySide<EveryPiece<BitBoard>>,
    pub side_to_mode: MoveTurn,
    pub game_stage: GameState,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bb_pieces: EverySide::new(EveryPiece::new(BitBoard::empty())),
            side_to_mode: MoveTurn::default(),
            game_stage: GameState::new()
        }
    }
}
