#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
use bg_core::dice::Dice;
#[cfg(test)]
use bg_core::moves::CheckerMove;
#[cfg(test)]
use bg_core::moves::Move;

lalrpop_mod!(#[allow(clippy::all)] pub bg_game);

#[test]
fn test_parse_player_identifier() {
    let player_identifier_parser = bg_game::PlayerIdentifierParser::new();
    assert!(player_identifier_parser.parse("").is_err());
    assert!(player_identifier_parser.parse("123").is_err());
    assert!(player_identifier_parser.parse("Fribourg?").is_err());
    assert!(player_identifier_parser.parse("Jordan").is_ok());
    assert!(player_identifier_parser.parse("psuter").is_ok());
}

#[test]
fn test_parse_int_literal() {
    let literal_parser = bg_game::IntLiteralParser::new();
    let non_zero_parser = bg_game::NonZeroIntLiteralParser::new();
    assert!(literal_parser.parse("").is_err());
    assert!(literal_parser.parse("-1").is_err());
    assert_eq!(0, literal_parser.parse("0").unwrap());
    assert_eq!(1, literal_parser.parse("1").unwrap());
    assert_eq!(123, literal_parser.parse("123").unwrap());
    assert!(non_zero_parser.parse("").is_err());
    assert!(non_zero_parser.parse("-1").is_err());
    assert!(non_zero_parser.parse("0").is_err());
    assert_eq!(1, non_zero_parser.parse("1").unwrap());
    assert_eq!(123, non_zero_parser.parse("123").unwrap());
}

#[test]
fn test_parse_match_header() {
    let match_header_parser = bg_game::MatchHeaderParser::new();
    assert!(match_header_parser.parse("").is_err());
    assert!(match_header_parser.parse("5").is_err());
    assert!(match_header_parser.parse("point match").is_err());
    assert_eq!(5, match_header_parser.parse("5 point match").unwrap());
}

#[test]
fn test_parse_game_header() {
    let game_header_parser = bg_game::GameHeaderParser::new();
    assert!(game_header_parser.parse("").is_err());
    assert!(game_header_parser.parse("Game 2").is_err());
    assert!(game_header_parser
        .parse("Game 2\nPlayer: 0\t\t\tOther : 1")
        .is_ok());
}

#[test]
fn test_parse_roll() {
    let roll_parser = bg_game::RollParser::new();
    assert!(roll_parser.parse("64").is_err());
    assert_eq!(Dice::make(4, 3), roll_parser.parse("43:").unwrap());
}

#[test]
fn test_parse_checker_move() {
    let move_parser = bg_game::CheckerMoveParser::new();

    // This style is currently unsupported.
    assert!(move_parser.parse("13/12(2)").is_err());

    assert_eq!(
        CheckerMove::make(13, 12, false),
        move_parser.parse("13/12").unwrap()
    );
    assert_eq!(
        CheckerMove::make(13, 12, true),
        move_parser.parse("13/12*").unwrap()
    );
    assert_eq!(
        CheckerMove::make(4, 0, false),
        move_parser.parse("4/0").unwrap()
    );
}

#[test]
fn test_parse_move() {
    let move_parser = bg_game::MoveParser::new();

    assert_eq!(
        Move::make(
            Dice::make(1, 1),
            vec![
                CheckerMove::make(13, 12, true),
                CheckerMove::make(13, 12, false),
                CheckerMove::make(4, 3, false),
                CheckerMove::make(3, 2, false),
            ]
        ),
        move_parser.parse("11: 13/12* 13/12 4/3 3/2").unwrap()
    );

    assert_eq!(
        Move::make(Dice::make(5, 4), vec![]),
        move_parser.parse("54:").unwrap()
    )
}

#[test]
fn test_parse_turn() {
    let turn_parser = bg_game::TurnParser::new();

    assert!(turn_parser.parse("1)\t\t21: 24/23 13/11").is_ok());
    assert!(turn_parser.parse("7) 62: 13/7 7/5\t\t54: 8/3 7/3").is_ok());
    assert!(turn_parser.parse("16) 64: 20/16 16/10*\t\t52:").is_ok());
    assert!(turn_parser
        .parse("5) 21: 13/11 11/10\t\tDoubles => 2")
        .is_ok());
    assert!(turn_parser.parse("2) Doubles => 2\t\tTakes").is_ok());
}
