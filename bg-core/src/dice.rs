use rand::Rng;
use std::cmp::{max, min};
use std::fmt;

// const DICE_CHARS: [char; 6] = ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'];
const DICE_CHARS: [char; 6] = ['1', '2', '3', '4', '5', '6'];

#[derive(Debug, PartialEq)]
pub struct Dice {
    first: u8,
    second: u8,
}

impl Dice {
    pub fn make(first: u8, second: u8) -> Dice {
        if !(1..=6).contains(&first) {
            panic!("Invalid die value: {}", first);
        }

        if !(1..=6).contains(&second) {
            panic!("Invalid die value: {}", second);
        }

        Dice {
            first: max(first, second),
            second: min(first, second),
        }
    }

    pub fn roll() -> Dice {
        let r = rand::thread_rng().gen_range(0..36);
        Dice {
            first: r % 6 + 1,
            second: r / 6 + 1,
        }
    }

    pub fn high(&self) -> u8 {
        self.first
    }

    pub fn low(&self) -> u8 {
        self.second
    }

    pub fn is_double(&self) -> bool {
        self.first == self.second
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            DICE_CHARS[usize::from(self.high() - 1)],
            DICE_CHARS[usize::from(self.low() - 1)]
        )
    }
}

impl Eq for Dice {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ROLLS: u32 = 1_000_000;

    #[test]
    fn test_dist_1() -> () {
        let mut d1_counts: [u32; 6] = [0; 6];
        let mut d2_counts: [u32; 6] = [0; 6];
        let mut sum_counts: [u32; 11] = [0; 11];

        for _ in 0..TEST_ROLLS {
            let r = Dice::roll();

            d1_counts[usize::from(r.first - 1)] += 1;
            d2_counts[usize::from(r.second - 1)] += 1;
            sum_counts[usize::from(r.first + r.second) - 2] += 1;
        }

        for i in 0..6 {
            let d1_ratio = f64::from(d1_counts[i]) / f64::from(TEST_ROLLS);
            let d2_ratio = f64::from(d2_counts[i]) / f64::from(TEST_ROLLS);

            assert!(d1_ratio >= 1.0 / 6.5 && d1_ratio <= 1.0 / 5.5);
            assert!(d2_ratio >= 1.0 / 6.5 && d2_ratio <= 1.0 / 5.5);
        }

        let expected_dist: [f64; 11] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

        for i in 0..11 {
            let freq = 36.0 * f64::from(sum_counts[i]) / f64::from(TEST_ROLLS);
            assert!(f64::abs(freq - expected_dist[i]) < 0.1);
        }
    }
}
