use std::{fs::read_to_string, str::FromStr};

trait Score {
    fn score(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Score for Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Defeat,
    Draw,
    Victory,
}

impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Defeat => 0,
            Outcome::Draw => 3,
            Outcome::Victory => 6,
        }
    }
}

struct Game(Move, Outcome);

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
        assert_eq!(bytes.len(), 3);

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

fn main() {
    let result = read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .filter(|&line| line != "")
        .map(|game| game.parse::<Game>().expect("Invalid game").score())
        .sum::<u32>();

    dbg!(result);
}
