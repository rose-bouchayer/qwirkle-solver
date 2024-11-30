mod bag;
mod board;
mod player;
mod rules;
mod tile;

use bag::Bag;
use board::Board;
use player::Player;

fn create_players(mut bag: Bag) -> [Player; 2] {
    let player1 = Player::new(&mut bag);
    let player2 = Player::new(&mut bag);
    let players = [player1, player2];
    println!("{players:?}");

    players
}

fn main() {
    // create bag to draw
    let bag = Bag::new();

    // create players to play
    let [player1, player2] = create_players(bag);

    // create board to play on
    let mut board = Board::new();

    player1.play(&mut board);
    println!("player1\n{:?}", board);

    player2.play(&mut board);
    println!("player2\n{:?}", board);
}
