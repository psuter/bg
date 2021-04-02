use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;

use super::dice::Dice;
use super::position::Position;

pub fn generate_o_moves(position: &Position, dice: &Dice) -> HashSet<Position> {
    if dice.is_double() {
        let one_move: HashSet<Position> = HashSet::from_iter(
            generate_one_die_moves(position, dice.high())
                .iter()
                .copied(),
        );

        if one_move.is_empty() {
            return HashSet::new();
        }

        let two_moves: HashSet<Position> = one_move
            .iter()
            .map(|&pos| generate_one_die_moves(&pos, dice.high()))
            .flatten()
            .collect();

        if two_moves.is_empty() {
            return one_move;
        }

        let three_moves: HashSet<Position> = two_moves
            .iter()
            .map(|&pos| generate_one_die_moves(&pos, dice.high()))
            .flatten()
            .collect();

        if three_moves.is_empty() {
            return two_moves;
        }

        let four_moves: HashSet<Position> = three_moves
            .iter()
            .map(|&pos| generate_one_die_moves(&pos, dice.high()))
            .flatten()
            .collect();

        if four_moves.is_empty() {
            return three_moves;
        }

        four_moves
    } else {
        let mut final_results: HashSet<Position> = HashSet::new();

        let high_roll_positions = generate_one_die_moves(position, dice.high());
        let low_roll_positions = generate_one_die_moves(position, dice.low());

        for pos1 in high_roll_positions.iter() {
            for pos2 in generate_one_die_moves(pos1, dice.low()).iter() {
                final_results.insert(*pos2);
            }
        }

        for pos1 in low_roll_positions.iter() {
            for pos2 in generate_one_die_moves(pos1, dice.high()).iter() {
                final_results.insert(*pos2);
            }
        }

        // If we can't use both dice, we have to use one if possible...
        if final_results.is_empty() {
            // ...and should always prefer the higher roll.
            if !high_roll_positions.is_empty() {
                for pos in high_roll_positions {
                    final_results.insert(pos);
                }
            } else if !low_roll_positions.is_empty() {
                for pos in low_roll_positions {
                    final_results.insert(pos);
                }
            }
        }

        final_results
    }
}

pub fn generate_x_moves(position: &Position, dice: &Dice) -> HashSet<Position> {
    generate_o_moves(&position.flip(), dice)
        .iter()
        .map(|&pos| pos.flip())
        .collect()
}

fn generate_one_die_moves(position: &Position, die: u8) -> Vec<Position> {
    let mut results: Vec<Position> = Vec::new();

    // Players has at least one checker on bar that needs to come home.
    if position.o_bar_value() > 0 {
        if position.point_x_value(24 - die) <= 1 {
            let new_position = position.with_o_entering(24 - die);
            results.push(new_position);
        }
    } else {
        // Go over every point. If there is at least one of ours there, see what's n moves away.
        for point in (0..24).rev() {
            if position.point_o_value(point) > 0 {
                // Move is legal.
                if point >= die && position.point_x_value(point - die) <= 1 {
                    let new_position = position.with_o_move(point, point - die);
                    results.push(new_position);
                }
            }
        }

        // We have already considered all non-bearing off moves. They may be more.
        if position.o_can_bear_off() {
            let mut seen_higher: bool = false;

            for point in (0..6).rev() {
                if position.point_o_value(point) > 0 {
                    match (point + 1).cmp(&die) {
                        Ordering::Greater => (),

                        Ordering::Equal => {
                            // It's always possible to bear that off.
                            // But we must also remember we saw it.
                            let new_position = position.with_o_bearing_off(point);
                            results.push(new_position)
                        }

                        Ordering::Less => {
                            // We can only move if we haven't seen anything higher.
                            if !seen_higher {
                                let new_position = position.with_o_bearing_off(point);
                                results.push(new_position);
                            }
                        }
                    }

                    // We must always recall we've seen a checker before considering lower points.
                    seen_higher = true;

                    /*
                    if point + 1 == die {
                        // It's always possible to bear that off.
                        // But we must also remember we saw it.
                        let new_position = position.with_o_bearing_off(point);
                        results.push(new_position);
                    } else if point + 1 < die {
                        // We can only move if we haven't seen anything higher.
                        if !seen_higher {
                            let new_position = position.with_o_bearing_off(point);
                            results.push(new_position);
                        }
                    }
                    */
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_both() -> () {
        // Example from https://www.bkgm.com/faq/BasicRules.html#moving_the_checkers
        let x_checkers = [(2, 2), (3, 2), (9, 2), (19, 3), (20, 2), (22, 2), (23, 2)];

        let start = Position::make(&[(6, 5), (7, 3), (13, 5), (24, 2)], &x_checkers, 0, 0, 0, 0);

        let roll = Dice::make(6, 4);

        let expected = Position::make(
            &[(6, 5), (7, 3), (13, 5), (14, 1), (24, 1)],
            &x_checkers,
            0,
            0,
            0,
            0,
        );

        let moves = generate_o_moves(&start, &roll);

        assert_eq!(1, moves.len());
        assert_eq!(moves.iter().next(), Some(&expected));
    }

    #[test]
    fn test_movegen_play_highest() -> () {
        // Example from https://www.bkgm.com/faq/BasicRules.html#moving_the_checkers
        let x_checkers = [
            (2, 2),
            (7, 2),
            (9, 2),
            (11, 2),
            (14, 2),
            (19, 2),
            (20, 1),
            (21, 2),
        ];

        let start = Position::make(
            &[(6, 5), (13, 5), (15, 4), (24, 1)],
            &x_checkers,
            0,
            0,
            0,
            0,
        );

        let roll = Dice::make(6, 4);

        let expected = Position::make(
            &[(6, 5), (13, 5), (15, 4), (18, 1)],
            &x_checkers,
            0,
            0,
            0,
            0,
        );

        let moves = generate_o_moves(&start, &roll);

        assert_eq!(1, moves.len());
        assert_eq!(moves.iter().next(), Some(&expected));
    }

    #[test]
    fn test_movegen_entering() -> () {
        let start = Position::make(
            &[(6, 5), (8, 3), (13, 5), (24, 1)],
            &[(1, 2), (7, 2), (12, 2), (17, 2), (18, 2), (19, 5)],
            1,
            0,
            0,
            0,
        );

        let roll = Dice::make(6, 6);
        let moves = generate_o_moves(&start, &roll);
        assert_eq!(0, moves.len());

        let roll = Dice::make(6, 5);

        let expected_1 = Position::make(
            &[(2, 1), (6, 5), (8, 2), (13, 5), (20, 1), (24, 1)],
            &[(1, 2), (7, 2), (12, 2), (17, 2), (18, 2), (19, 5)],
            0,
            0,
            0,
            0,
        );

        let expected_2 = Position::make(
            &[(6, 5), (8, 3), (13, 5), (14, 1), (24, 1)],
            &[(1, 2), (7, 2), (12, 2), (17, 2), (18, 2), (19, 5)],
            0,
            0,
            0,
            0,
        );

        let expected: HashSet<Position> = vec![expected_1, expected_2].into_iter().collect();

        let moves = generate_o_moves(&start, &roll);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_movegen_bearing_off_1() -> () {
        // Example from https://www.bkgm.com/faq/BasicRules.html#can_i_play_low_number_first_when_bearing_off_
        let start = Position::make(
            &[(2, 2), (4, 1)],
            &[(1, 1), (19, 4), (20, 2), (21, 2), (22, 2), (23, 2), (24, 2)],
            0,
            0,
            12,
            0,
        );

        let roll = Dice::make(4, 1);

        let expected_1 = Position::make(
            &[(1, 1), (2, 1)],
            &[(19, 4), (20, 2), (21, 2), (22, 2), (23, 2), (24, 2)],
            0,
            1,
            13,
            0,
        );

        let expected_2 = Position::make(
            &[(2, 2)],
            &[(1, 1), (19, 4), (20, 2), (21, 2), (22, 2), (23, 2), (24, 2)],
            0,
            0,
            13,
            0,
        );

        let expected: HashSet<Position> = vec![expected_1, expected_2].into_iter().collect();

        let moves = generate_o_moves(&start, &roll);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_movegen_bearing_off_2() -> () {
        // Example from https://www.bkgm.com/faq/BasicRules.html#can_i_play_low_number_first_when_bearing_off_
        let start = Position::make(&[(1, 1), (3, 3), (6, 1)], &[(23, 3), (24, 2)], 0, 0, 10, 10);

        let roll = Dice::make(5, 4);

        let expected_1 = Position::make(&[(1, 2), (3, 2)], &[(23, 3), (24, 2)], 0, 0, 11, 10);

        let expected_2 =
            Position::make(&[(1, 1), (2, 1), (3, 2)], &[(23, 3), (24, 2)], 0, 0, 11, 10);

        let expected: HashSet<Position> = vec![expected_1, expected_2].into_iter().collect();

        let moves = generate_o_moves(&start, &roll);

        assert_eq!(moves, expected);
    }
}
