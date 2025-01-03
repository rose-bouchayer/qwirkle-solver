use std::fmt::{Debug, Formatter, Result};

use crate::tile::Tile;

use super::position::Position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub position: Position,
    pub tile: Tile,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Location {
            position: Position { x, y },
            tile,
        } = self;
        write!(f, "{tile:?} ({x}, {y})")
    }
}
