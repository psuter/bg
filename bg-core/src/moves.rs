use super::dice::Dice;

#[derive(Debug, PartialEq)]
pub struct CheckerMove {
    from: u8,
    to: u8,
    hits: bool,
}

impl CheckerMove {
    pub fn make(from: u8, to: u8, hits: bool) -> CheckerMove {
        CheckerMove { from, to, hits }
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    roll: Dice,
    checker_moves: Vec<CheckerMove>,
}

impl Move {
    pub fn make(roll: Dice, checker_moves: Vec<CheckerMove>) -> Move {
        Move {
            roll,
            checker_moves,
        }
    }
}
