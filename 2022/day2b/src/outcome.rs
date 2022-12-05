use crate::score::Score;

pub(crate) enum Outcome {
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
