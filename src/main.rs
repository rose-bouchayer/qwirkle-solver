mod bag;
mod player;
mod rules;
mod tile;

use bag::Bag;
use player::Player;

fn main() {
    let mut bag = Bag::new();

    let player1 = Player::new(&mut bag);
    let player2 = Player::new(&mut bag);
}
