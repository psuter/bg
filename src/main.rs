mod dice;
mod movegen;
mod position;

use rand::Rng;

use movegen::{generate_o_moves, generate_x_moves};
use position::Position;

fn main() {
    //println!("{}", p);

    let mut i = 0;

    let mut o_victories = 0;
    let mut x_victories = 0;

    for _ in 0..1 {
        let mut p = Position::initial();
        let mut o_turn: bool = rand::thread_rng().gen_range(0, 2) == 0;

        loop {
            let roll = dice::Dice::roll();
            println!("{}", roll);

            let moves: Vec<Position> = if o_turn {
                generate_o_moves(&p, &roll).into_iter().collect()
            } else {
                generate_x_moves(&p, &roll).into_iter().collect()
            };

            if !moves.is_empty() {
                p = moves[rand::thread_rng().gen_range(0, moves.len())];
            }

            println!("{}", i);
            println!("{}", p);

            if p.o_has_won() {
                o_victories += 1;
                break;
            }

            if p.x_has_won() {
                x_victories += 1;
                break;
            }

            o_turn = !o_turn;
            i += 1;
        }
    }
    println!("{} o victories", o_victories);
    println!("{} x victories", x_victories);
}
