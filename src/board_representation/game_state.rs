pub const WK: u8 = 0b0001;
pub const WQ: u8 = 0b0010;
pub const BK: u8 = 0b0100;
pub const BQ: u8 = 0b1000;

pub struct GameState {
    castling_rights: u8,
    en_passant: Option<u8>,
    halfmove_clock: u16,
    fullmove_number: u16,
}
impl GameState {
    pub fn empty() -> Self {
        GameState {
            castling_rights: 0b0000,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn new() -> Self {
        GameState {
            castling_rights: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}
