use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use qwirkle_solver::{
    board::{location::Location, Board},
    player::Player,
    tile::Tiles,
};

pub fn setup_board(board: &mut Board, tiles: Vec<Location>) {
    for location in tiles {
        board.add_tile(location);
    }
}

pub fn create_player(hand: Tiles) -> Player {
    let mut player = Player {
        points: 0,
        hand,
        combinations: Vec::new(),
    };
    player.update_combinations();

    player
}

pub fn format(board: Board, player: Player) -> String {
    format!("{board:?}\nPoints: {}", player.points)
}

pub fn save(filename: &str, text: String) {
    let full_path = "tests/results/".to_owned() + filename + ".ans";
    let path = Path::new(&full_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
