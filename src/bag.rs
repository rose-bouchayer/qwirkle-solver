use crate::tile::{Color, Shape, Tile, COLORS_NUMBER, SHAPES_NUMBER, SHAPES_REPETITION};

pub const BAG_SIZE: usize = 108;

#[derive(Debug)]
pub struct Bag {
    // ? TODO: find a way to define maximum length?
    pub tiles: Vec<Tile>,
}

impl Bag {
    pub fn new() -> Bag {
        let all_tiles: [Tile; BAG_SIZE] = core::array::from_fn(|index| Tile {
            color: get_tile_color(index),
            shape: get_tile_shape(index),
        });
        let tiles: Vec<Tile> = all_tiles.to_vec();

        Bag { tiles }
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
