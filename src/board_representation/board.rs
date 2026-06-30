use crate::board_representation::{
    bb_pieces::{BitBoard, Color, EveryPiece, EverySide, Occupancy, Piece},
    game_state::GameState,
};

pub struct Board {
    pub bb_pieces: EverySide<EveryPiece<BitBoard>>,
    pub bb_occupancy: Occupancy,
    pub side_to_move: Color,
    pub state: GameState,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            bb_pieces: EverySide::new(EveryPiece::new(BitBoard::empty())),
            bb_occupancy: Occupancy::new(),
            side_to_move: Color::White,
            state: GameState::empty(),
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
            black[Piece::Knight] = BitBoard(0x4200_0000_0000_0000);
            black[Piece::Bishop] = BitBoard(0x2400_0000_0000_0000);
            black[Piece::Rook] = BitBoard(0x8100_0000_0000_0000);
            black[Piece::Queen] = BitBoard(0x0800_0000_0000_0000);
            black[Piece::King] = BitBoard(0x1000_0000_0000_0000);
        }

        let white = bb[Color::White][Piece::Pawn]
            | bb[Color::White][Piece::Knight]
            | bb[Color::White][Piece::Bishop]
            | bb[Color::White][Piece::Rook]
            | bb[Color::White][Piece::Queen]
            | bb[Color::White][Piece::King];

        let black = bb[Color::Black][Piece::Pawn]
            | bb[Color::Black][Piece::Knight]
            | bb[Color::Black][Piece::Bishop]
            | bb[Color::Black][Piece::Rook]
            | bb[Color::Black][Piece::Queen]
            | bb[Color::Black][Piece::King];

        let bb_occupancy = Occupancy::from_parts(BitBoard(white.0 | black.0), white, black);

        Self {
            bb_pieces: bb,
            bb_occupancy,
            side_to_move: Color::White,
            state: GameState::new(),
        }
    }
}
