use std::fmt::{Debug, Formatter, Result};

use crate::tile::{Tile, Tiles};

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn value(&self) -> DirectionVector {
        match *self {
            Direction::North => DirectionVector(0, 1),
            Direction::East => DirectionVector(1, 0),
            Direction::South => DirectionVector(0, -1),
            Direction::West => DirectionVector(-1, 0),
        }
    }

    pub fn values() -> [DirectionVector; 4] {
        [
            Direction::North.value(),
            Direction::East.value(),
            Direction::South.value(),
            Direction::West.value(),
        ]
    }

    pub fn alignements() -> [(DirectionVector, DirectionVector); 2] {
        [
            (Direction::South.value(), Direction::North.value()),
            (Direction::West.value(), Direction::East.value()),
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DirectionVector(pub i8, pub i8);

pub struct Board {
    tiles: Vec<Location>,
}

impl Board {
    /// Constructs a new, empty board to play on.
    pub fn new() -> Board {
        Board { tiles: Vec::new() }
    }

    /// Returns all tiles on the board. Tiles aren't sorted.
    pub fn tiles(&self) -> &Vec<Location> {
        &self.tiles
    }

    // TODO: check if location x/y is free before pushing
    /// Add a tile to the board at `(x, y)` position.
    pub fn add_tile(&mut self, location: Location) {
        self.tiles.push(location);
    }

    /// Searches for a tile at `(x, y)` position.
    /// If something is found `Some(Tile)` is returned, otherwise`None`.
    pub fn get(&self, x: i8, y: i8) -> Option<Tile> {
        let location = self
            .tiles
            .iter()
            .find(|Location { position, .. }| position.x == x && position.y == y);

        let tile = match location {
            Some(location) => Some(location.tile),
            None => None,
        };

        tile
    }

    /// Returns tiles next to a given position, for a given direction,
    /// until an empty location is reached.
    pub fn get_tiles(&self, position: Position, direction: DirectionVector) -> Tiles {
        let mut tiles = Vec::new();

        let mut step = 1;
        while let Some(tile) = self.get(
            position.x + direction.0 * step,
            position.y + direction.1 * step,
        ) {
            tiles.push(tile);

            step += 1;
        }

        tiles
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // ? TODO: find a smaller way to write this to avoid repetition
        let top = self
            .tiles
            .iter()
            .map(|location| location.position.y)
            .max()
            .unwrap_or(0);
        let right = self
            .tiles
            .iter()
            .map(|location| location.position.x)
            .max()
            .unwrap_or(0);
        let bottom = self
            .tiles
            .iter()
            .map(|location| location.position.y)
            .min()
            .unwrap_or(0);
        let left = self
            .tiles
            .iter()
            .map(|location| location.position.x)
            .min()
            .unwrap_or(0);

        let mut str = String::new();
        for y in (bottom..=top).rev() {
            for x in left..=right {
                let tile = self.get(x, y);
                let tile_str = match tile {
                    Some(tile) => format!("{tile:?}"),
                    None => String::from("   "),
                };
                str.push_str(&format!("{tile_str}"));
            }
            str.push_str("\n");
        }

        write!(f, "{str}")
    }
}
