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
    pub fn new() -> Self {
        GameState {
            record: Record::Inplay,
            white_castling_rights: CastlingRights::None,
            black_castling_rights: CastlingRights::None,
        }
    }
}
