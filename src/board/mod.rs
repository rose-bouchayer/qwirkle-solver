pub mod direction;
pub mod location;
pub mod r#move;
pub mod position;

use std::fmt::{Debug, Formatter, Result};

use direction::Direction;
use location::Location;
use position::Position;
use r#move::PartialMove;

use crate::tile::{Tile, Tiles};

#[derive(Clone)]
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
    /// Adds a tile to the board at `(x, y)` position.
    pub fn add_tile(&mut self, location: Location) {
        self.tiles.push(location);
    }

    /// Adds multiple tiles from a `partial_move`.
    /// Starts at `position`, goes to `direction` and places tiles from `combination`.
    pub fn add_tiles(&mut self, partial_move: &PartialMove) {
        let PartialMove {
            combination,
            position,
            direction,
        } = partial_move;

        for (index, &tile) in combination.iter().enumerate() {
            let step = index as i8;
            let location = Location {
                tile,
                position: Position {
                    x: position.x + direction.0 * step,
                    y: position.y + direction.1 * step,
                },
            };
            self.add_tile(location)
        }
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
    pub fn get_tiles(&self, position: Position, direction: Direction) -> Tiles {
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
