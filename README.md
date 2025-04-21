# massivebird's funky board game

__Funky board__ is a [zero-player board game](https://en.wikipedia.org/wiki/Zero-player_game)!

🦀 written in Rust

## The rules

Tokens exist inside a game board of fixed size.

Tokens take turns moving around the board. Tokens declare their movement types before the game starts. They are:

1. _Random_: teleports to some random space, and
2. _Adjacent_: moves to some adjacent space.

A piece is captured when another piece moves to its current position.

A winner is declared when all but one piece have been captured!

## Building

To manually build the project, you must first [install Rust](https://www.rust-lang.org/tools/install).

Once you have Rust installed, run the following commands:

```bash
$ git clone https://github.com/massivebird/funky_board
$ cd funky_board
$ cargo run # runs unoptimized build
```

## Configuration

> Coming soon???
