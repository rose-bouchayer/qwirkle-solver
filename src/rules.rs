use crate::board::{Board, Direction, DirectionVector, Location, Position};
use crate::tile::Tile;

/// Validates combination between two tiles.
///
/// To be valid, a combination must have either the same color or the same shape
/// but not both at the same time.
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

/// Validates a `location` on an alignement (North-South or West-East).
///
/// An alignement can't contains a tile twice
/// and tiles can only have the same color or the same shape.
/// > Note: an alignement can contains only 6 tiles.
pub fn validate_alignement(
    board: &Board,
    location: Location,
    prev: DirectionVector,
    next: DirectionVector,
) -> bool {
    let Location { position, tile } = location;

    // gather current tile + all previous/next tiles
    let mut tiles: Vec<Tile> = vec![tile];
    tiles.extend(board.get_tiles(position, prev));
    tiles.extend(board.get_tiles(position, next));

    // check all tiles altogehter one by one
    let is_valid_alignement = tiles.iter().enumerate().all(|(index, tile)| {
        tiles[(index + 1)..]
            .iter()
            .all(|next_tile| validate_combination(tile, next_tile))
    });

    is_valid_alignement
}

/// Validates if a `tile` can be played near `location`.
/// If the `tile` is a valid combination and had a valid spot,
/// returns the new location `Some(Location)`.
/// If no combination is found at every close position, returns `None`.
pub fn validate_location(board: &Board, tile: &Tile, location: &Location) -> Option<Location> {
    if !validate_combination(tile, &location.tile) {
        return None;
    };

    let position = location.position;
    // check each direction and find:
    //   - the first empty slot
    //   - which is valid wih other tiles
    let new_location = Direction::values().into_iter().find_map(|direction| {
        // find neighbor to check if it's free to drop a tile
        let new_position = Position {
            x: position.x + direction.0,
            y: position.y + direction.1,
        };
        let neighbor = board.get(new_position.x, new_position.y);

        // validate the combination with other tiles
        let new_location = Location {
            position: new_position,
            tile: *tile,
        };
        let is_valid_location = [
            (Direction::South.value(), Direction::North.value()),
            (Direction::West.value(), Direction::East.value()),
        ]
        .iter()
        .all(|&(prev, next)| validate_alignement(board, new_location, prev, next));

        if neighbor.is_none() && is_valid_location {
            Some(new_location)
        } else {
            None
        }
    });

    new_location
}
