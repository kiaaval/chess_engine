use std::fmt;

pub enum MoveTurns {
    White,
    Black,
}

#[derive()]
pub struct MoveTurn {
    turn: MoveTurns,
}
impl fmt::Display for MoveTurn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.turn {
            MoveTurns::White => "White",
            MoveTurns::Black => "Black",
        };
        write!(f, "{}", s)
    }
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
