mod bag;
mod board;
mod player;
mod rules;
mod tile;

use rand::Rng;
use std::{thread::sleep, time};

use bag::Bag;
use board::Board;
use player::Player;

const TURNS: i8 = 51;

fn create_players(bag: &mut Bag) -> Vec<Player> {
    let mut rng = rand::rng();
    let number = rng.random_range(2..=4);

    let mut players = vec![];

    for id in 1..=number {
        let new_player = Player::new(id, bag);
        players.push(new_player);
    }

    players
}

fn main() {
    // create bag to draw
    let mut bag = Bag::new();

    // create players to play
    let mut players = create_players(&mut bag);
    for player in players.iter() {
        println!("Player {}: {:?}", player.id, player.hand);
    }

    // create board to play on
    let mut board = Board::new();

    'game_loop: for turn in 1..=TURNS {
        println!("=== Turn {turn} ===");

        for player in players.iter_mut() {
            let player_can_play = player.play(&mut board, &mut bag);
            println!(
                "Player {} - {} points\n{:?}",
                player.id, player.points, board
            );
            if !player_can_play {
                println!("Player {} ended the game", player.id);
                break 'game_loop;
            }
        }

        println!("=== End of turn {turn} ===\n\n\n");

        sleep(time::Duration::from_millis(50));
    }

    for player in players {
        println!("Player {} score: {}", player.id, player.points);
    }
}
