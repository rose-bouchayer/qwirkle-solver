use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::bag::Bag;
use crate::board::{Board, Location, Position};
use crate::rules::{get_points, validate_combination, validate_locations};
use crate::tile::{Tile, Tiles};

type Combination = Tiles;
type Combinations = Vec<Tiles>;

#[derive(Debug)]
pub struct Player {
    pub points: i32,
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
    fn remove(&mut self, bag: &mut Bag, tile: Tile) {
        if let Some(index) = self.hand.iter().position(|&local_tile| local_tile == tile) {
            self.hand.remove(index);
            self.draw(bag, 1);
        };
    }

    /// Removes from hand a random number of tiles,
    /// draws as many new tiles and put back removed tiles in `bag`.
    fn replace(&mut self, bag: &mut Bag) {
        let mut rng = thread_rng();

        // generate number of tiles to replace
        let number = rng.gen_range(1..=self.hand.len()) as u8;

        let mut tiles = Vec::new();
        for _ in 0..number {
            // generate a random index and remove a random tile from hand
            let index = rng.gen_range(0..self.hand.len());
            let tile = self.hand.remove(index);
            tiles.push(tile);
        }

        // draw new tiles from bag
        self.draw(bag, number);

        // add replaced tiles to bag
        bag.add(tiles);
    }

    /**
     * 1. find where to play with which tile
     * 2. add tiles to board to the found location
     * 3. remove played tile from player's hand and draw
     */
    pub fn play(&mut self, board: &mut Board, bag: &mut Bag) {
        if board.tiles().len() == 0 {
            // if board is empty, start in the center
            let tile = self.hand[0];
            board.add_tile(Location {
                position: Position { x: 0, y: 0 },
                tile,
            });
            self.points += 1;
            self.remove(bag, tile);
        } else {
            // get every possible moves
            let moves = self.get_moves(board);

            // get last tiles from possible moves
            // last = best move, highest amount of points
            if let Some(last_move) = moves.last() {
                let Move { location, points } = *last_move;

                // add tile to board
                board.add_tile(location);

                // increase pointes
                self.points += points;

                // remove tile from hand
                self.remove(bag, location.tile);
            } else {
                // can't find any tile to play, replace some tiles
                self.replace(bag);
            };
        }
    }

    /// Finds all playable locations with associated points to gain.
    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = self
            .hand
            .iter()
            // for every tile in players hand
            .filter_map(|tile| {
                let moves = board
                    .tiles()
                    .iter()
                    // compare tile from hand to every tile on board
                    .filter_map(|location| {
                        let new_locations = validate_locations(board, tile, location);

                        // no new locations has been found
                        if new_locations.is_empty() {
                            return None;
                        }

                        // compute points for each location and map them together
                        let moves = new_locations
                            .iter()
                            .map(|&location| Move {
                                location,
                                points: get_points(board, location),
                            })
                            .collect::<Vec<Move>>();

                        // returns `moves` for this `location`
                        Some(moves)
                    })
                    .flatten()
                    .collect::<Vec<Move>>();

                // returns `moves` for this `tile`
                Some(moves)
            })
            .flatten()
            .collect::<Vec<Move>>();

        // sort `moves` to find best (= latest)
        moves.sort();

        moves
    }

    /// Updates player's `combinations` based on current `hand` state.
    /// Must be call after every hand update (e.g. `.draw()`).
    fn update_combinations(&mut self) {
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
            .filter(|&tile| combination.iter().all(|t| validate_combination(t, tile)))
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

/// An association of how many `points` players gain if they play at `Location`.
#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    location: Location,
    points: i32,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.points.cmp(&other.points))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points.cmp(&other.points)
    }
}
