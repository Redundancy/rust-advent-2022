#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

pub const LOSS: usize = 0;
pub const DRAW: usize = 3;
pub const WIN: usize = 6;

impl Into<RPS> for &str {
    fn into(self) -> RPS {
        use RPS::*;
        match self.chars().nth(0) {
            Some('A') => ROCK,
            Some('B') => PAPER,
            Some('C') => SCISSORS,
            Some('X') => ROCK,
            Some('Y') => PAPER,
            Some('Z') => SCISSORS,
            _ => unreachable!("invalid character"),
        }
    }
}
impl RPS {
    pub fn score(&self) -> usize {
        use RPS::*;
        match self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
        }
    }

    pub fn result(&self, other: &RPS) -> usize {
        use RPS::*;

        return match (self, other) {
            (x, y) if x == y => DRAW,
            (&ROCK, &SCISSORS) => WIN,
            (&PAPER, &ROCK) => WIN,
            (&SCISSORS, &PAPER) => WIN,
            _ => LOSS,
        };
    }

    pub fn choose_for_result(&self, result: &WinDrawLose) -> RPS {
        use RPS::*;
        use WinDrawLose::*;

        match (self, result) {
            (_, DRAW) => *self,
            (ROCK, WIN) => PAPER,
            (PAPER, WIN) => SCISSORS,
            (SCISSORS, WIN) => ROCK,
            (ROCK, LOSE) => SCISSORS,
            (PAPER, LOSE) => ROCK,
            (SCISSORS, LOSE) => PAPER,
        }
    }
}

pub enum WinDrawLose {
    WIN,
    DRAW,
    LOSE,
}

impl WinDrawLose {
    pub fn score(&self) -> usize {
        match self {
            WinDrawLose::WIN => WIN,
            WinDrawLose::DRAW => DRAW,
            WinDrawLose::LOSE => LOSS,
        }
    }
}

impl Into<WinDrawLose> for &str {
    fn into(self) -> WinDrawLose {
        use WinDrawLose::*;

        match self.chars().nth(0) {
            Some('X') => LOSE,
            Some('Y') => DRAW,
            Some('Z') => WIN,
            _ => unreachable!("invalid character"),
        }
    }
}