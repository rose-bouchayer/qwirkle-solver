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
    let players_unsorted = [&player1, &player2];
    let players_combinations_length = get_players_combinations_length(players_unsorted);
    let players = if players_combinations_length == 0 {
        [player1, player2]
    } else {
        [player2, player1]
    };

    println!("{players:?}");
}

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
