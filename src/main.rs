mod bag;
mod board;
mod player;
mod rules;
mod tile;

use bag::Bag;
use board::Board;
use player::Player;
use tile::{Color, Shape, Tile};

fn create_players(mut bag: Bag) -> [Player; 2] {
    let player1 = Player::new(&mut bag);
    let player2 = Player::new(&mut bag);
    let players_unsorted = [&player1, &player2];
    let players_combinations_length = get_players_combinations_length(players_unsorted);
    let players = if players_combinations_length == 0 {
        [player1, player2]
    } else {
        [player2, player1]
    };
    println!("{players:?}");

    players
}

fn main() {
    let bag = Bag::new();

    let players = create_players(bag);

    let mut board = Board::new();
    /* board.add_tile(
        0,
        0,
        Tile {
            color: Color::Blue,
            shape: Shape::Circle,
        },
    );
    board.add_tile(
        10,
        10,
        Tile {
            color: Color::Green,
            shape: Shape::Club,
        },
    ); */

    let [player1, player2] = players;

    board = player1.play(board);
    println!("player1\n{:?}", board);

    board = player2.play(board);
    println!("player2\n{:?}", board);
}

// ! TODO: rename and document
fn get_players_combinations_length(players: [&Player; 2]) -> usize {
    let result = players
        .iter()
        .enumerate()
        .map(|(index, player)| {
            let length = player.get_longest_combinations_length();
            (index, length)
        })
        .max_by(|(_, length0), (_, length1)| length0.cmp(length1))
        .map(|(index, _)| index);

    if let Some(i) = result {
        i
    } else {
        0
    }
}
