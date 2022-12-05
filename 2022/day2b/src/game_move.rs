use crate::score::Score;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Move {
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
