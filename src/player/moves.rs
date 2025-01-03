use crate::board::{
    direction::Direction,
    location::Location,
    position::Position,
    r#move::{Move, Moves, PartialMove},
    Board,
};
use crate::rules::{validate_partial_move, validate_tiles};

use super::Combination;

/// Returns `moves` for a given `combination`.
/// Compares the `combination` to every tile on the booard.
pub fn get_combination_moves(board: &Board, combination: &Combination) -> Option<Moves> {
    let combination_moves = board
        .tiles()
        .iter()
        .filter_map(|location| {
            // if the first tile of the combination isn't valid with the current tile,
            // return `None`
            let first_tile = combination[0];
            if !validate_tiles(&first_tile, &location.tile) {
                return None;
            }

            // returns `moves` for this `location`
            get_combination_moves_near_location(board, combination, location)
        })
        .flatten()
        .collect::<Vec<Move>>();

    // wrap and returns `moves` for this `combination`
    Some(combination_moves)
}

/// Returns `moves` for a given `combination` next to a given `location`.
/// Finds every position close to `location` and tries to place the `combination`.
fn get_combination_moves_near_location(
    board: &Board,
    combination: &Combination,
    location: &Location,
) -> Option<Moves> {
    let moves = Direction::values()
        .iter()
        .filter_map(|location_direction| {
            let position = Position {
                x: location.position.x + location_direction.0,
                y: location.position.y + location_direction.1,
            };

            // find neighbor to check if it's free to drop a tile
            let neighbor = board.get(position.x, position.y);
            if neighbor.is_some() {
                return None;
            }

            // returns `moves` for this `position`
            get_combination_moves_near_location_at_position(board, combination, location, position)
        })
        .flatten()
        .collect::<Moves>();

    // wrap and returns `moves` for at this `location` for this `combination`
    Some(moves)
}

/// Returns `moves` for a given `combination` at a given `position`, which is next to a given `location`.
fn get_combination_moves_near_location_at_position(
    board: &Board,
    combination: &Combination,
    location: &Location,
    position: Position,
) -> Option<Moves> {
    let get_one_move = || -> Moves {
        let direction = Direction(
            position.x - location.position.x,
            position.y - location.position.y,
        );

        let Some(one_move) = get_move(board, combination, position, direction) else {
            return Vec::new();
        };

        vec![one_move]
    };

    let get_all_moves = || -> Moves {
        Direction::values()
            .iter()
            .filter_map(|&direction| get_move(board, combination, position, direction))
            .collect::<Moves>()
    };

    let moves = if combination.len() == 1 {
        // if combination has one tile, check only one direction
        get_one_move()
    } else {
        // for each direction of the combination
        get_all_moves()
    };

    Some(moves)
}

/// Validates a (partial) move and get points from it.
fn get_move(
    board: &Board,
    combination: &Combination,
    position: Position,
    direction: Direction,
) -> Option<Move> {
    let partial_move = PartialMove {
        combination: combination.clone(),
        position,
        direction,
    };
    validate_partial_move(board, partial_move)
}
