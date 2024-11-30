use crate::{
    board::{Board, Direction, Location},
    tile::Tile,
};

pub fn validate_combination(tile0: &Tile, tile1: &Tile) -> bool {
    let Tile {
        color: color0,
        shape: shape0,
    } = tile0;
    let Tile {
        color: color1,
        shape: shape1,
    } = tile1;

    let is_combinable = color0 == color1 || shape0 == shape1;
    let is_same_tile = color0 == color1 && shape0 == shape1;

    is_combinable && !is_same_tile
}

pub fn validate_placement(board: &Board, combination: &Vec<Tile>, location: &Location) -> bool {
    let first_tile = combination[0].clone();
    if !validate_combination(&first_tile, &location.tile) {
        return false;
    };

    println!("\nChecking neighbors of {:?}", location);
    // check each direction
    for direction in Direction::value_array() {
        let x = location.position.x + direction.0;
        let y = location.position.y + direction.1;
        let neighbor = board.get(x, y);

        // if `neighbor` is a tile, skip this iteration
        if let Some(tile) = neighbor {
            println!("tile {tile:?} in ({x}, {y})");
            continue;
        } else {
            println!("no tile in ({x}, {y})",);
        };
    }

    true
}
