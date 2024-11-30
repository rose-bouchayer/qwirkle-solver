use rand::{thread_rng, Rng};
use std::fmt::Debug;

use crate::bag::Bag;
use crate::board::{Board, Location, Position};
use crate::rules::find_position;
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

        // FIXME: add exception if bag is smaller than draw
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 0..number {
            let max = bag.tiles.len();
            let index = rng.gen_range(0..max);
            let new_tile = bag.tiles.remove(index);
            tiles.push(new_tile);
        }

        tiles
    }

    /**
     * 1. find where to play
     *  1.a. find better combinations
     *  1.b. find location to play it
     * 2. add tiles to board to the correct location
     * 3. remove played tiles from player's hand
     * 4. return played combinations with locations
     * ? draw?
     */
    pub fn play(&self, board: &mut Board) {
        if board.tiles().len() == 0 {
            // if board is empty, start in the center
            let tile = &self.hand[0];
            board.add_tile(0, 0, tile);
        } else {
            // find location to play
            // find first combinable tile with tiles in the board
            if let Some(location) = self.find_location(&board) {
                // add found tile to found position
                let Location { position, tile } = location;
                board.add_tile(position.x, position.y, &tile);
            } else {
                // can't find any tile to play, draw new tiles
                // FIXME: replace a random number of tiles
                println!("Can't play!");
            };
        }

        // TODO: remove played tile from hand
    }

    fn find_location(&self, board: &Board) -> Option<Location> {
        let mut location = None;
        for tile in self.hand.iter() {
            println!("\nchecking {tile:?}");

            let position = self.find_position(board, tile);
            if let Some(p) = position {
                println!("found position {p:?} to play");
                location = Some(Location {
                    position: p,
                    tile: *tile,
                });
                break;
            } else {
                println!("can't find position...");
            }
        }

        location
    }

    fn find_position(&self, board: &Board, tile: &Tile) -> Option<Position> {
        let mut position = None;

        for location in board.tiles() {
            position = find_position(board, tile, location);
            if position.is_some() {
                break;
            };
        }

        position
    }
}
