mod bag;
mod board;
mod player;
mod rules;
mod tile;

use bag::Bag;
use board::Board;
use player::Player;

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

    player1.play(&mut board, &mut bag);
    println!("player1\n{:?}", board);

    player2.play(&mut board, &mut bag);
    println!("player2\n{:?}", board);
}
