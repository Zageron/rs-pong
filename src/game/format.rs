pub use super::types::Wall;

use std::fmt;

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.point_a, self.point_b)
    }
}
