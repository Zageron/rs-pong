pub use super::types::{GameState, Wall};

use std::fmt;

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.point_a, self.point_b)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(P1: {:?}, P2: {:?})",
            self.player0_score, self.player1_score
        )
    }
}
