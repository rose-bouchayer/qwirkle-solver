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

    players
}

fn main() {
    // create bag to draw
    let mut bag = Bag::new();

    // create players to play
    let [mut player1, mut player2] = create_players(&mut bag);
    println!("Player 1: {:?}", player1.hand);
    println!("Player 2: {:?}", player2.hand);

    // create board to play on
    let mut board = Board::new();

    for turn in 1..=TURNS {
        println!("=== Turn {turn} ===");

        let player1_can_play = player1.play(&mut board, &mut bag);
        println!("Player 1 - {} points\n{:?}", player1.points, board);
        if !player1_can_play {
            println!("Player 1 ended the game");
            break;
        }

        let player2_can_play = player2.play(&mut board, &mut bag);
        println!("Player 2 - {} points\n{:?}", player2.points, board);
        if !player2_can_play {
            println!("Player 1 ended the game");
            break;
        }

        println!("=== End of turn {turn} ===\n\n\n");

        sleep(time::Duration::from_millis(250));
    }

    println!("Player 1 score: {}", player1.points);
    println!("Player 2 score: {}", player2.points);
}
