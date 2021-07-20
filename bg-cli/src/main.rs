use std::env;
use std::fs;
use std::time;

use bg_core::dice::Dice;
use bg_core::game::Match;
use bg_core::movegen::{generate_o_moves, generate_x_moves};
use bg_core::position::Position;
use bg_core::rollout;

use bg_parser::parse_match;

use rand::Rng;

fn load_match(filename: &str) -> Match {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    parse_match(&contents).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mtch = load_match(&args[1]);

    let g1 = &mtch.games[0];

    let mut p = Position::initial();
    println!("{}", p);

    for turn in &g1.turns {
        let o_move = &turn.0;
        let x_move = &turn.1;

        for om in o_move {
            p = p.apply_o_move(om);
            println!("o -> {}", om);
            println!("{}", p);
        }

        for xm in x_move {
            p = p.apply_x_move(xm);
            println!("x -> {}", xm);
            println!("{}", p);
        }
    }

    if 2 < 3 {
        return;
    }

    let start = time::Instant::now();
    let p = Position::initial();
    let stats = rollout::rollout_o(&p);
    let finish = time::Instant::now();
    let duration: time::Duration = finish.duration_since(start);
    let micros = duration.as_micros();

    println!("Total time: {}", micros as f64 / 1e6);
    println!("Micros per game: {}", micros / u128::from(stats.rolls));

    println!("{:?}", stats);

    let mut p = Position::initial();
    let mut o_turn = true;

    loop {
        if rand::thread_rng().gen_range(0..20) == 0 {
            println!("Rollout requested...");
            println!("{}", p);
            let s = rollout::rollout_o(&p);
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
