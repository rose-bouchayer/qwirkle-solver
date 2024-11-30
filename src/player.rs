use rand::{thread_rng, Rng};
use std::fmt::Debug;

use crate::bag::Bag;
use crate::tile::Tile;

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Tile>,
}

impl Player {
    pub fn new(bag: &mut Bag) -> Player {
        let tiles = Player::draw(bag, 6);

        Player { hand: tiles }
    }

    pub fn draw(bag: &mut Bag, number: u8) -> Vec<Tile> {
        let mut rng = thread_rng();

        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 0..number {
            let max = bag.tiles.len();
            let index = rng.gen_range(0..max);
            let new_tile = bag.tiles.remove(index);
            tiles.push(new_tile);
        }

        tiles
    }
}
