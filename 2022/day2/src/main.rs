use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
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

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Defeat => 0,
            Outcome::Draw => 3,
            Outcome::Victory => 6,
        }
    }
}

struct Game(Move, Move);

impl Game {
    fn score(&self) -> u32 {
        let outcome = match self {
            Game(Move::Rock, Move::Paper) => Outcome::Victory,
            Game(Move::Paper, Move::Scissors) => Outcome::Victory,
            Game(Move::Scissors, Move::Rock) => Outcome::Victory,
            Game(a, b) if a == b => Outcome::Draw,
            _ => Outcome::Defeat,
        };

        let Game(_, my_move) = self;

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
            b'X' => Move::Rock,
            b'Y' => Move::Paper,
            b'Z' => Move::Scissors,
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
