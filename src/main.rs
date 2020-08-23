use rand::Rng;

fn main() {
    let roll = rand::thread_rng().gen_range(1, 7);
    println!("Hello, you rolled {}.", roll);
}
