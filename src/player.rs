use rand::{thread_rng, Rng};
use std::fmt::Debug;

use crate::bag::Bag;
use crate::rules::validate_combination;
use crate::tile::Tile;

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Tile>,
    combinations: Vec<Vec<Tile>>,
}

impl Player {
    pub fn new(bag: &mut Bag) -> Player {
        let tiles = Player::draw(bag, 6);
        let combinations = Player::get_combinations(tiles.clone());

        Player {
            hand: tiles,
            combinations,
        }
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

    /** Get all possible combinations for a set of tiles. */
    pub fn get_combinations(tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
        let combination = Vec::new();
        let combinations = compute_combinations(combination, tiles);

        combinations
    }

    pub fn get_longest_combinations_length(&self) -> usize {
        // map every combinations lengths
        let lengths = self
            .combinations
            .iter()
            .map(|combination| combination.len());

        // returns the higher length
        match lengths.max() {
            Some(max) => max,
            None => 0,
        }
    }
}

// TODO: write unit tests to validate results for multiple entries + multiple sizes
/** Recursively computes combinations for an input combination and a set of tiles. */
fn compute_combinations(combination: Vec<Tile>, tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
    let mut combinations: Vec<Vec<Tile>> = Vec::new();

    for (index, tile1) in tiles.iter().enumerate() {
        let is_valid = combination
            .iter()
            .all(|tile| validate_combination(tile, tile1));

        if is_valid {
            let new_combination: Vec<Tile> = combination.iter().chain([tile1]).cloned().collect();
            combinations.push(new_combination.clone());

            let mut new_combinations =
                compute_combinations(new_combination, tiles[index + 1..].to_vec());
            if new_combinations.len() > 0 {
                combinations.append(&mut new_combinations);
            }
        }
    }

    combinations
}
