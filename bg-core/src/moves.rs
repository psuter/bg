use std::fmt;
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

    pub fn src_point(&self) -> u8 {
        self.from - 1
    }

    pub fn dst_point(&self) -> u8 {
        self.to - 1
    }

    pub fn is_entering(&self) -> bool {
        self.from == 25
    }

    pub fn is_bearing_off(&self) -> bool {
        self.to == 0
    }
}

impl fmt::Display for CheckerMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}{}", self.from, self.to, if self.hits {"*"} else {""})
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

    pub fn moves(&self) -> &Vec<CheckerMove> {
        &self.checker_moves
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.roll)?;

        for mv in &self.checker_moves {
            write!(f, " {}", mv)?;
        }

        write!(f, "")
    }
}
