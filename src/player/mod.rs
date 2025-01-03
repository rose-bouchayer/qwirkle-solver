mod moves;

use rand::{thread_rng, Rng};
use std::fmt::Debug;

use crate::bag::Bag;
use crate::board::{location::Location, position::Position, r#move::Move, Board};
use crate::rules::validate_tiles;
use crate::tile::{Tile, Tiles};
use moves::get_combination_moves;

pub type Points = i32;
pub type Combination = Tiles;
pub type Combinations = Vec<Tiles>;

#[derive(Debug)]
pub struct Player {
    pub points: Points,
    pub hand: Tiles,
    pub combinations: Combinations,
}

impl Player {
    /// Constructs a new player with a hand full of 6 random tiles.
    pub fn new(bag: &mut Bag) -> Player {
        let mut player = Player {
            points: 0,
            hand: Vec::new(),
            combinations: Vec::new(),
        };

        player.draw(bag, 6);

        player
    }

    /// Draws a `number` of tiles from `bag` and stores them in player's hand.
    fn draw(&mut self, bag: &mut Bag, number: u8) {
        let mut rng = thread_rng();

        // make sure to not draw more than what the bag contains
        let range = number.min(bag.tiles().len() as u8);
        for _ in 0..range {
            let max = bag.tiles().len();

            // prevent `gen_range()` to panic
            if max == 0 {
                break;
            }

            // generate a random index and pick the associated tile
            let index = rng.gen_range(0..max);
            if let Some(new_tile) = bag.remove(index) {
                self.hand.push(new_tile);
            };
        }

        // update combinations with new tiles in hand
        self.update_combinations();
    }

    /// Removes the `tile` within player's hand and draws a new tile.
    fn remove_tile(&mut self, bag: &mut Bag, tile: Tile) {
        if let Some(index) = self.hand.iter().position(|&local_tile| local_tile == tile) {
            self.hand.remove(index);
            self.draw(bag, 1);
        };
    }

    /// Removes `tiles` within player's hand and draws new tiles.
    fn remove_tiles(&mut self, bag: &mut Bag, combination: Combination) {
        for tile in combination {
            self.remove_tile(bag, tile);
        }
    }

    /// Removes from hand a random number of tiles,
    /// draws as many new tiles and put back removed tiles in `bag`.
    fn replace(&mut self, bag: &mut Bag) {
        let mut rng = thread_rng();

        // generate number of tiles to replace
        let length = (self.hand.len() as u8).max(1);
        let number = rng.gen_range(1..=length);

        let mut tiles = Vec::new();
        for _ in 0..number {
            let length = self.hand.len();
            if length == 0 {
                break;
            }

            // generate a random index and remove a random tile from hand
            let index = rng.gen_range(0..length);
            let tile = self.hand.remove(index);
            tiles.push(tile);
        }

        // draw new tiles from bag
        self.draw(bag, number);

        // add replaced tiles to bag
        bag.add(tiles);
    }

    /// Finds the best move and plays it.
    /// TODO: split this in two sub-methods
    pub fn play(&mut self, board: &mut Board, bag: &mut Bag) {
        if board.tiles().len() == 0 {
            // if board is empty, start in the center
            // TODO: replace playing 1 tile by playing the longest combination
            let tile = self.hand[0];
            board.add_tile(Location {
                position: Position { x: 0, y: 0 },
                tile,
            });
            self.points += 1;
            self.remove_tile(bag, tile);
        } else {
            // get every possible moves
            let moves = self.get_moves(board);

            // get last tiles from possible moves
            // last = best move, highest amount of points
            // TODO: based the latest highest score, randomly select a move with the same score
            if let Some(last_move) = moves.last() {
                let (partial_move, points) = last_move.into_partial();

                // play move by adding tiles to the board
                board.add_tiles(&partial_move);

                // increase points
                self.points += points;

                // remove combination from hand
                self.remove_tiles(bag, partial_move.combination);
            } else {
                // can't find any tile to play, replace some tiles
                self.replace(bag);
            };
        }
    }

    /// Finds all playable locations with associated points to gain.
    fn get_moves(&self, board: &Board) -> Vec<Move> {
        // for every combination from player's hand
        let mut moves = self
            .combinations
            .iter()
            .filter_map(|combination| get_combination_moves(board, combination))
            .flatten()
            .collect::<Vec<Move>>();

        // sort `moves` to find best (= latest)
        moves.sort();

        moves
    }

    /// Updates player's `combinations` based on current `hand` state.
    /// Must be call after every hand update (e.g. `.draw()`).
    pub fn update_combinations(&mut self) {
        // for each tile in hand, compute combinations
        // wrap the tile in `Vec<Tile>` to create a one tile combination
        self.combinations = self
            .hand
            .iter()
            .flat_map(|&tile| self.compute_combinations(&vec![tile], &self.hand))
            .collect::<Combinations>();
    }

    /// For one `combination`, recursively computes every possible combination with `tiles`.
    /// It returns entry `combination` in addition with found combinations.
    /// So if no combination is found, returned vector has at least one element.
    fn compute_combinations(&self, combination: &Combination, tiles: &Tiles) -> Combinations {
        let combination_clone = vec![combination.clone()];

        // for each tile
        let new_combinations: Combinations = tiles
            .iter()
            // validate that `tile` can be added to the `combination`
            .filter(|&tile| combination.iter().all(|t| validate_tiles(t, tile)))
            .flat_map(|tile| {
                // add `tile` to `combination` by creating a `new_combination`
                let mut new_combination: Combination = combination.clone();
                new_combination.push(*tile);

                // keep checking if `new_combination` can create more combinations
                self.compute_combinations(&new_combination, &tiles)
            })
            .collect::<Combinations>();

        // concat entry `combination` with `new_combinations`
        [combination_clone, new_combinations].concat()
    }
}
