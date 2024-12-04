use rand::{thread_rng, Rng};
use std::fmt::Debug;

use crate::bag::Bag;
use crate::board::{Board, Location};
use crate::rules::validate_location;
use crate::tile::{Tile, Tiles};

#[derive(Debug)]
pub struct Player {
    pub hand: Tiles,
}

impl Player {
    /// Constructs a new player with a hand full of 6 random tiles.
    pub fn new(bag: &mut Bag) -> Player {
        let mut player = Player { hand: Vec::new() };

        player.draw(bag, 6);

        player
    }

    /// Draws a `number` of tiles from `bag` and stores them in player's hand.
    fn draw(&mut self, bag: &mut Bag, number: u8) {
        let mut rng = thread_rng();

        let range = number.min(bag.tiles().len() as u8);
        for _ in 0..range {
            let max = bag.tiles().len();

            // prevent `gen_range()` to panic
            if max == 0 {
                break;
            }

            let index = rng.gen_range(0..max);
            if let Some(new_tile) = bag.remove(index) {
                self.hand.push(new_tile);
            };
        }
    }

    /// Removes the `tile` within player's hand and draws a new tile.
    fn remove(&mut self, bag: &mut Bag, tile: Tile) {
        if let Some(index) = self.hand.iter().position(|&local_tile| local_tile == tile) {
            self.hand.remove(index);
            self.draw(bag, 1);
        };
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
            board.add_tile(0, 0, &tile);
            self.remove(bag, tile);
        } else {
            // find location to play
            // find first combinable tile with tiles in the board
            if let Some(location) = self.find_location(&board) {
                // add found tile to found position
                let Location { position, tile } = location;
                board.add_tile(position.x, position.y, &tile);
                self.remove(bag, tile);
            } else {
                // can't find any tile to play, draw new tiles
                // FIXME: replace a random number of tiles
                println!("Can't play!");
            };
        }
    }

    /// Finds a location to play. If no location is found, returns `None`.
    fn find_location(&self, board: &Board) -> Option<Location> {
        let location = self.hand.iter().find_map(|tile| {
            board
                .tiles()
                .iter()
                .find_map(|location| validate_location(board, tile, location))
        });

        location
    }
}
