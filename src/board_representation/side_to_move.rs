pub enum MoveTurns {
    White,
    Black,
}

pub struct MoveTurn {
    turn: MoveTurns,
}
impl MoveTurn {
    pub fn default() -> Self {
        MoveTurn {
            turn: MoveTurns::White,
        }
    }

    pub fn set_turn(&mut self, t: MoveTurns) {
        self.turn = t;
    }
}
