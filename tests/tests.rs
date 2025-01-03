mod common;

use common::format;
use qwirkle_solver::{
    bag::Bag,
    board::{location::Location, position::Position, Board},
    tile::{Color, Shape, Tile},
};

#[test]
pub fn play_1() {
    let mut bag = Bag::new();
    let mut board = Board::new();
    common::setup_board(
        &mut board,
        vec![Location {
            tile: Tile {
                color: Color::Blue,
                shape: Shape::Circle,
            },
            position: Position { x: 0, y: 0 },
        }],
    );

    let mut player = common::create_player(vec![Tile {
        color: Color::Blue,
        shape: Shape::Club,
    }]);

    player.play(&mut board, &mut bag);

    common::save("play_1", format(board, player));
}

#[test]
pub fn play_2() {
    let mut bag = Bag::new();
    let mut board = Board::new();
    common::setup_board(
        &mut board,
        vec![Location {
            tile: Tile {
                color: Color::Blue,
                shape: Shape::Circle,
            },
            position: Position { x: 0, y: 0 },
        }],
    );

    let mut player = common::create_player(vec![
        Tile {
            color: Color::Blue,
            shape: Shape::Diamond,
        },
        Tile {
            color: Color::Blue,
            shape: Shape::Cross,
        },
    ]);

    player.play(&mut board, &mut bag);
    common::save("play_2", format(board, player));
}

#[test]
pub fn play_3() {
    let mut bag = Bag::new();
    let mut board = Board::new();
    common::setup_board(
        &mut board,
        vec![
            Location {
                tile: Tile {
                    color: Color::Yellow,
                    shape: Shape::Square,
                },
                position: Position { x: 0, y: 0 },
            },
            Location {
                tile: Tile {
                    color: Color::Yellow,
                    shape: Shape::Club,
                },
                position: Position { x: 1, y: 0 },
            },
            Location {
                tile: Tile {
                    color: Color::Blue,
                    shape: Shape::Club,
                },
                position: Position { x: 1, y: 1 },
            },
            Location {
                tile: Tile {
                    color: Color::Yellow,
                    shape: Shape::Club,
                },
                position: Position { x: 2, y: 1 },
            },
        ],
    );

    let mut player = common::create_player(vec![
        Tile {
            color: Color::Yellow,
            shape: Shape::Cross,
        },
        Tile {
            color: Color::Purple,
            shape: Shape::Cross,
        },
    ]);

    player.play(&mut board, &mut bag);
    common::save("play_3", format(board, player));
}
