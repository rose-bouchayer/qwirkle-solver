use std::fmt::Debug;

use crate::tile::{Color, Shape, Tile, COLORS_NUMBER, SHAPES_NUMBER, SHAPES_REPETITION};

/// 6 shapes * 6 colors * 3 tiles = 108 tiles
pub const BAG_SIZE: usize = 108;

#[derive(Debug)]
pub struct Bag {
    // ? TODO: find a way to define maximum length?
    tiles: Vec<Tile>,
}

impl Bag {
    /// Constructs a new filled bag with 108 tiles. Tiles are sorted.
    pub fn new() -> Bag {
        let mut tiles = Vec::new();

        for index in 0..BAG_SIZE {
            tiles.push(Tile {
                color: get_tile_color(index),
                shape: get_tile_shape(index),
            });
        }

        Bag { tiles }
    }

    /// Returns all tiles in the bag. Tiles are sorted.
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    /// Removes and returns the tile at position `index` within the bag.
    pub fn remove(&mut self, index: usize) -> Option<Tile> {
        if index < self.tiles.len() {
            Some(self.tiles.remove(index))
        } else {
            None
        }
    }
}

fn get_tile_color(index: usize) -> Color {
    match (index / (COLORS_NUMBER * 3)) % COLORS_NUMBER {
        0 => Color::Red,
        1 => Color::Orange,
        2 => Color::Yellow,
        3 => Color::Green,
        4 => Color::Blue,
        5 => Color::Purple,
        _ => Color::Red,
    }
}

fn get_tile_shape(index: usize) -> Shape {
    match (index / SHAPES_REPETITION) % SHAPES_NUMBER {
        0 => Shape::Square,
        1 => Shape::Circle,
        2 => Shape::Diamond,
        3 => Shape::Club,
        4 => Shape::Star,
        5 => Shape::Cross,
        _ => Shape::Square,
    }
}
