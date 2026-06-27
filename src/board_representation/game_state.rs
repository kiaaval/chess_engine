pub enum Record {
    Inplay,
    Draw,
    WhiteWon,
    BlackWon,
}

pub enum CastlingRights {
    All,
    Long,
    Short,
    None,
}

pub struct GameState {
    record: Record,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
}
impl GameState {
    pub fn empty() -> Self {
        GameState {
            record: Record::Inplay,
            white_castling_rights: CastlingRights::None,
            black_castling_rights: CastlingRights::None,
        }
    }

    pub fn new() -> Self {
        GameState {
            record: Record::Inplay,
            white_castling_rights: CastlingRights::All,
            black_castling_rights: CastlingRights::All,
        }
    }
}
