# massivebird's funky board

__Funky board__ is a board game played by the computer!

🦀 written in Rust

## The rules

Within the game board are several game pieces.

Pieces take turns to move about the board.

Each piece is categorized under one of two movement types:

1. _Random_: teleporting to some random space, and
2. _Adjacent_: only able to move to some adjacent space.

A piece is captured when another piece moves to its current space.

A winner is declared when all but one piece have been captured!

## Building

To manually build the project, you must first [install Rust](https://www.rust-lang.org/tools/install).

Once you have Rust installed, run the following commands:

```bash
git clone https://github.com/massivebird/funky_board
cd funky_board
cargo run # runs unoptimized build
```

## Configuration

Currently, game configuration is configurable only by modifying source code.

Sorry! But feel free to tinker with `main.rs` and `lib.rs` if you can and want to :3
