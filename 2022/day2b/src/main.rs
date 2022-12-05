mod game;
mod game_move;
mod outcome;
mod score;

use std::fs::read_to_string;

use crate::{game::Game, score::Score};

fn main() {
    let result = read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .filter(|&line| line != "")
        .map(|game| game.parse::<Game>().expect("Invalid game").score())
        .sum::<u32>();

    dbg!(result);
}
