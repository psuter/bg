[![Rust](https://github.com/psuter/bg/actions/workflows/rust.yml/badge.svg)](https://github.com/psuter/bg/actions/workflows/rust.yml)

# `bg`

A backgammon engine (eventually, possibly). Primarily an excuse to learn some Rust.

## Concepts

While the two players are largely interchangeable, code is simpler to organize if they're distinguished. Throughout the codebase, we assume an `o` and an `x` player.

When applicable, the `o` player is the "main" player. E.g. the board is internally represented with `o` bearing off at point `0` and `x` bearing off at point `25`, games are parsed assuming the `o` player is reported first, console representation has `o` bearing off at the bottom, etc.
