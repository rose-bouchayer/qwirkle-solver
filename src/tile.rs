use std::fmt::{Debug, Formatter, Result};

pub const COLORS_NUMBER: usize = 6;
pub const SHAPES_NUMBER: usize = 6;
// How many of the same shape there are in one set of color.
pub const SHAPES_REPETITION: usize = 3;

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub color: Color,
    pub shape: Shape,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Tile { color, shape } = self;

        write!(f, "{color:?}{shape:?}")
    }
}

pub type Tiles = Vec<Tile>;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Red,    // 🟥
    Orange, // 🟧
    Yellow, // 🟨
    Green,  // 🟩
    Blue,   // 🟦
    Purple, // 🟪
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let color: &str = match self {
            Color::Red => "\x1b[41m",
            Color::Orange => "\x1b[48;2;255;127;0m",
            Color::Yellow => "\x1b[43m",
            Color::Green => "\x1b[42m",
            Color::Blue => "\x1b[44m",
            Color::Purple => "\x1b[45m",
        };
        write!(f, "{}", color)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Square,  // □
    Circle,  // ◯
    Diamond, // ◇
    Club,    // ♣️
    Star,    // *
    Cross,   // +
}

impl Debug for Shape {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let shape: &str = match self {
            Shape::Square => " □ \x1b[39;49m",
            Shape::Circle => " ◯ \x1b[39;49m",
            Shape::Diamond => " ◇ \x1b[39;49m",
            Shape::Club => " ♣️ \x1b[39;49m",
            Shape::Star => " * \x1b[39;49m",
            Shape::Cross => " + \x1b[39;49m",
        };
        write!(f, "{}", shape)
    }
}
