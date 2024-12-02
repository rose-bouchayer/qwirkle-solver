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

/**
 * Takes a Location, loops over each direction
 * and checks that alignement is valid (for connected tiles).
 */
fn validate_alignements(board: &Board, new_location: Location) -> bool {
    // check that each direction (north, east, south, west) is valid
    let is_valid_alignement = Direction::values().iter().all(|&direction| {
        // while there is a tile AND that it's a valid combination with `new_location`
        let mut step = 1;
        let is_valid = loop {
            let Some(neighbor) = board.get(
                new_location.position.x + direction.0 * step,
                new_location.position.y + direction.1 * step,
            ) else {
                break true;
            };

            if !validate_combination(&new_location.tile, &neighbor) {
                // not a valid combination
                break false;
            }

            step += 1;
        };

        is_valid
    });

    is_valid_alignement
}

pub fn find_position(board: &Board, tile: &Tile, location: &Location) -> Option<Position> {
    if !validate_combination(tile, &location.tile) {
        return None;
    };

    let position = location.position;
    // check each direction
    let direction = Direction::values().into_iter().find(|&direction| {
        let new_position = Position {
            x: position.x + direction.0,
            y: position.y + direction.1,
        };
        let neighbor = board.get(new_position.x, new_position.y);

        neighbor.is_none()
            && validate_alignements(
                board,
                Location {
                    position: new_position,
                    tile: *tile,
                },
            )
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
