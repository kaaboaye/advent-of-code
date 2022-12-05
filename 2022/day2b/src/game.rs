use std::str::FromStr;

use crate::{game_move::Move, outcome::Outcome, score::Score};

pub(crate) struct Game(Move, Outcome);

impl Score for Game {
    fn score(&self) -> u32 {
        let Game(_, outcome) = self;

        let my_move = match self {
            Game(Move::Rock, Outcome::Victory) => Move::Paper,
            Game(Move::Paper, Outcome::Victory) => Move::Scissors,
            Game(Move::Scissors, Outcome::Victory) => Move::Rock,
            Game(Move::Rock, Outcome::Defeat) => Move::Scissors,
            Game(Move::Paper, Outcome::Defeat) => Move::Rock,
            Game(Move::Scissors, Outcome::Defeat) => Move::Paper,
            Game(opponents_move, Outcome::Draw) => *opponents_move,
        };

        outcome.score() + my_move.score()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != 3 {
            return Err(());
        }

        let a = match bytes[0] {
            b'A' => Move::Rock,
            b'B' => Move::Paper,
            b'C' => Move::Scissors,
            _ => panic!("Player 1 made invalid move"),
        };

        let b = match bytes[2] {
            b'X' => Outcome::Defeat,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Victory,
            _ => panic!("Player 2 made invalid move"),
        };

        Ok(Game(a, b))
    }
}
