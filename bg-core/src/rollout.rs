use rand::Rng;

use super::dice::Dice;
use super::movegen::generate_x_moves;
use super::movegen::generate_o_moves;
use super::position::Position;

#[derive(Debug)]
pub struct RolloutStats {
    pub rolls: u64,
    pub o_win: f64,
    pub o_gammon: f64,
    pub o_backgammon: f64,
    pub x_win: f64,
    pub x_gammon: f64,
    pub x_backgammon: f64,
}

const ROLLOUT_ROLLS: u64 = 1000;

pub fn rollout_o(position: &Position) -> RolloutStats {
    let mut o_wins = 0;
    let mut o_gammons = 0;
    let mut o_backgammons = 0;
    let mut x_wins = 0;
    let mut x_gammons = 0;
    let mut x_backgammons = 0;

    for _ in 0..ROLLOUT_ROLLS {
        let mut p = *position;
        let mut o_turn: bool = true;

        loop {
            let roll = Dice::roll();

            let moves: Vec<Position> = if o_turn {
                generate_o_moves(&p, &roll).into_iter().collect()
            } else {
                generate_x_moves(&p, &roll).into_iter().collect()
            };

            if !moves.is_empty() {
                p = moves[rand::thread_rng().gen_range(0..moves.len())];
            }

            if p.o_has_won() {
                o_wins += 1;
                if p.o_has_gammoned() {
                    o_gammons += 1;
                    if p.o_has_backgammoned() {
                        o_backgammons += 1;
                    }
                }
                break;
            }

            if p.x_has_won() {
                x_wins += 1;
                if p.x_has_gammoned() {
                    x_gammons += 1;
                    if p.x_has_backgammoned() {
                        x_backgammons += 1;
                    }
                }
                break;
            }

            o_turn = !o_turn;
            // i += 1;
        }
    }

    let rolls = ROLLOUT_ROLLS as f64;

    RolloutStats {
        rolls: ROLLOUT_ROLLS,
        o_win: f64::from(o_wins) / rolls,
        o_gammon: f64::from(o_gammons) / rolls,
        o_backgammon: f64::from(o_backgammons) / rolls,
        x_win: f64::from(x_wins) / rolls,
        x_gammon: f64::from(x_gammons) / rolls,
        x_backgammon: f64::from(x_backgammons) / rolls
    }
}