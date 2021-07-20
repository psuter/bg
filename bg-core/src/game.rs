use super::moves::Move;

#[derive(Debug)]
pub struct Game {
    /** The game turns, with `o` move then `x` move. */
    pub turns: Vec<(Option<Move>, Option<Move>)>,
}

impl Game {
    pub fn make(turns: Vec<(Option<Move>, Option<Move>)>) -> Game {
        Game { turns }
    }
}

#[derive(Debug)]
pub struct Match {
    pub games: Vec<Game>,
}

impl Match {
    pub fn make(games: Vec<Game>) -> Match {
        Match { games }
    }
}
