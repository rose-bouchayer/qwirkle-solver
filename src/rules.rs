use crate::{
    board::{Board, Direction, Location, Position},
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

pub fn find_position(board: &Board, tile: &Tile, location: &Location) -> Option<Position> {
    if !validate_combination(tile, &location.tile) {
        return None;
    };

    println!("checking neighbors of {:?}", location);
    let position = location.position;
    // check each direction
    let direction = Direction::value_array().into_iter().find(|&direction| {
        let x = position.x + direction.0;
        let y = position.y + direction.1;
        let neighbor = board.get(x, y);

        neighbor.is_none()
    });

    if let Some(offset) = direction {
        Some(Position {
            x: position.x + offset.0,
            y: position.y + offset.1,
        })
    } else {
        None
    }
}
