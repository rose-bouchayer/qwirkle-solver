mod bag;
mod board;
mod player;
mod rules;
mod tile;

use std::{thread::sleep, time};

use bag::Bag;
use board::Board;
use player::Player;

const TURNS: i8 = 5;

fn create_players(bag: &mut Bag) -> [Player; 2] {
    let player1 = Player::new(bag);
    let player2 = Player::new(bag);
    let players = [player1, player2];
    println!("{players:?}");

    players
}

fn main() {
    // create bag to draw
    let mut bag = Bag::new();

    // create players to play
    let [mut player1, mut player2] = create_players(&mut bag);

    // create board to play on
    let mut board = Board::new();

    for turn in 1..=TURNS {
        println!("=== Turn {turn} ===");

        player1.play(&mut board, &mut bag);
        println!("Player 1 - {} points\n{:?}", player1.points, board);

        player2.play(&mut board, &mut bag);
        println!("Player 2 - {} points\n{:?}", player2.points, board);

        println!("=== End of turn {turn} ===\n\n\n");

        sleep(time::Duration::from_millis(250));
    }

    println!("Player 1 score: {}", player1.points);
    println!("Player 2 score: {}", player2.points);
}
