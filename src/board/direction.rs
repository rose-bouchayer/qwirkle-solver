use rand::{thread_rng, Rng};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Direction(pub i8, pub i8);

impl Direction {
    pub fn rand() -> Direction {
        let index = thread_rng().gen_range(0..4);
        match index {
            0 => Direction(0, 1),  // north ^
            1 => Direction(1, 0),  // east >
            2 => Direction(0, -1), // south v
            3 => Direction(-1, 0), // west <
            _ => Direction(0, 1),
        }
    }

    pub fn values() -> [Direction; 4] {
        [
            Direction(0, 1),  // north ^
            Direction(1, 0),  // east >
            Direction(0, -1), // south v
            Direction(-1, 0), // west <
        ]
    }

    pub fn opposite(&self) -> Direction {
        let Direction(x, y) = self;
        Direction(x * -1, y * -1)
    }

    pub fn perpendicular(&self) -> Direction {
        let Direction(x, y) = *self;
        let right_angle = match (x, y) {
            (0, 1) => Direction(1, 0),   // north to east
            (1, 0) => Direction(0, -1),  // east to south
            (0, -1) => Direction(-1, 0), // south to west
            (-1, 0) => Direction(0, 1),  // west to north
            _ => Direction(0, 1),
        };
        right_angle
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Direction(x, y) = self;
        let character = match (x, y) {
            (0, 1) => "^",
            (1, 0) => ">",
            (0, -1) => "v",
            (-1, 0) => "<",
            _ => "",
        };
        write!(f, "{character}")
    }
}
