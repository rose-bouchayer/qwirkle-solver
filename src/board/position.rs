use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
/// Test of a documentation
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Position { x, y } = self;
        write!(f, "({x}, {y})")
    }
}
