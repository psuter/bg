use bg_core::dice::Dice;
use bg_core::movegen::{generate_o_moves, generate_x_moves};
use bg_core::position::Position;
use bg_core::rollout::rollout_o;

use rand::Rng;
// use std::time::{Duration, Instant};



fn main() {
    print!("hello world");
    /*
    let start = Instant::now();
    let p = Position::initial();
    let stats = rollout::rollout_o(&p);
    let finish = Instant::now();
    let duration: Duration = finish.duration_since(start);
    let micros = duration.as_micros();

    println!("Total time: {}", micros as f64 / 1e6);
    println!("Micros per game: {}", micros / u128::from(stats.rolls));

    println!("{:?}", stats);
    */

    let mut p = Position::initial();
    let mut o_turn = true;

    loop {
        if rand::thread_rng().gen_range(0..20) == 0 {
            println!("Rollout requested...");
            println!("{}", p);
            let s = rollout_o(&p);
            println!("{:?}", s);
        }

        let roll = Dice::roll();

        let moves: Vec<Position> = if o_turn {
            generate_o_moves(&p, &roll).into_iter().collect()
        } else {
            generate_x_moves(&p, &roll).into_iter().collect()
        };

        if !moves.is_empty() {
            p = moves[rand::thread_rng().gen_range(0..moves.len())];
        }

        if p.o_has_won() || p.x_has_won() {
            break;
        }

        o_turn = !o_turn;
    }

    println!("{}", p);
}
