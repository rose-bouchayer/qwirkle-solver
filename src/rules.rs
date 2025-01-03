use crate::board::{position::Position, r#move::Move, r#move::PartialMove, Board};
use crate::player::Points;
use crate::tile::{Tile, Tiles};

/// Validates combination between two tiles.
///
/// To be valid, a combination must have either the same color or the same shape
/// but not both at the same time.
pub fn validate_tiles(tile0: &Tile, tile1: &Tile) -> bool {
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

/// Validates a list of alignements of tiles. It checks that every tile can be combined together.
///
/// An alignement can't contains a tile twice
/// and tiles can only have the same color or the same shape.
/// > Note: an alignement can contains only 6 tiles.
fn validate_alignements(alignements: &Vec<Tiles>) -> bool {
    alignements.iter().all(|alignement| {
        alignement.iter().enumerate().all(|(index, tile)| {
            alignement[(index + 1)..]
                .iter()
                .all(|next_tile| validate_tiles(tile, next_tile))
        })
    })
}

/// Validates a `partial_move` and returns how many points it gives.
/// Returns a `Move`, i.e. `partial_move` + `points`.
///
/// First it checks that there is free space to place it.
/// Then it builds every alignement created by the move and validates them.
pub fn validate_partial_move(board: &Board, partial_move: PartialMove) -> Option<Move> {
    let PartialMove {
        ref combination,
        position,
        direction,
    } = partial_move;

    // first validate that the combination can be placed from this `position` in this `direction`
    let is_placeable = combination.iter().enumerate().all(|(index, _)| {
        let step = index as i8;
        let x = position.x + direction.0 * step;
        let y = position.y + direction.1 * step;
        if let Some(_) = board.get(x, y) {
            false
        } else {
            true
        }
    });
    if !is_placeable {
        return None;
    }

    // build main alignement which is following the move's direction
    let main_alignement = {
        let opposite = direction.opposite();
        let before = board.get_tiles(position, opposite);

        let length = (combination.len() - 1) as i8;
        let last_position = Position {
            x: position.x + direction.0 * length,
            y: position.y + direction.1 * length,
        };
        let after = board.get_tiles(last_position, direction);

        let alignement = [before, combination.clone(), after].concat();

        alignement
    };

    // for every tile of the combination, build perpendiculars alignements (left + right)
    let perpendicular_alignements = {
        let perpendicular = direction.perpendicular();
        let perpendicular_opposite = perpendicular.opposite();

        let alignements = combination
            .iter()
            .enumerate()
            .filter_map(|(index, &tile)| {
                let step = index as i8;
                let tile_position = Position {
                    x: position.x + direction.0 * step,
                    y: position.y + direction.1 * step,
                };

                let before = board.get_tiles(tile_position, perpendicular_opposite);
                let after = board.get_tiles(tile_position, perpendicular);
                let alignement = [before, vec![tile], after].concat();

                if alignement.len() == 1 {
                    // if the tile isn't aligned with anything,
                    // don't return it to avoid useless vec and computations
                    None
                } else {
                    Some(alignement)
                }
            })
            .collect::<Vec<Tiles>>();

        alignements
    };

    // validate all alignements to validate the move
    let alignements = [vec![main_alignement], perpendicular_alignements].concat();
    let is_valid_move = validate_alignements(&alignements);

    if !is_valid_move {
        return None;
    }

    let points = alignements.iter().fold(0, |acc, alignement| {
        let length = alignement.len() as Points;
        let mul = if length == 6 { 2 } else { 1 }; // QWIRKLE!

        acc + length * mul
    });

    Some(partial_move.into_move(points))
}
