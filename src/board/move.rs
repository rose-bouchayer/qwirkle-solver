use std::fmt::{Debug, Formatter, Result};

use crate::player::{Combination, Points};

use super::direction::Direction;
use super::position::Position;

pub type Moves = Vec<Move>;

#[derive(Clone)]
pub struct PartialMove {
    pub combination: Combination,
    pub position: Position,
    pub direction: Direction,
}

impl PartialMove {
    pub fn into_move(&self, points: Points) -> Move {
        let PartialMove {
            combination,
            position,
            direction,
        } = self.clone();

        Move {
            combination,
            position,
            direction,
            points,
        }
    }
}

impl Debug for PartialMove {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let PartialMove {
            combination,
            position,
            direction,
        } = self;
        write!(f, "{combination:?} 📍{position:?} {direction:?}")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Move {
    pub combination: Combination,
    pub position: Position,
    pub direction: Direction,
    pub points: Points,
}

impl Move {
    pub fn into_partial(&self) -> (PartialMove, Points) {
        let Move {
            combination,
            position,
            direction,
            points,
        } = self.clone();

        (
            PartialMove {
                combination,
                position,
                direction,
            },
            points,
        )
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Move {
            combination,
            position,
            direction,
            points,
        } = self;
        write!(f, "{combination:?} 📍{position:?} {direction:?} 🔢{points}")
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.points.cmp(&other.points))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points.cmp(&other.points)
    }
}
