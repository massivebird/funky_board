use colored::Colorize;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::cmp::{max, min};

use crate::board::Board;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveType {
    Random,
    Adjacent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Cyan,
    Magenta,
    Red,
    Yellow,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub symbol: char,
    pub move_type: MoveType,
    is_alive: RefCell<bool>,
    pub color: Color,
}

impl Token {
    pub const fn new(symbol: char, move_type: MoveType, color: Color) -> Self {
        Self {
            move_type,
            symbol,
            is_alive: RefCell::new(true),
            color,
        }
    }

    pub fn is_alive(&self) -> bool {
        *self.is_alive.borrow()
    }

    pub fn kill(&self) {
        *self.is_alive.borrow_mut() = false;
    }

    pub fn print_move_msg(&self) {
        let adverb = match self.move_type {
            MoveType::Random => "randomly",
            MoveType::Adjacent => "adjacently",
        };
        println!("{self} is moving {adverb}.");
    }

    pub fn generate_target_coords(
        &self,
        board: &Board,
        current_row: usize,
        current_col: usize,
        rng: &mut ThreadRng,
    ) -> (usize, usize) {
        let bounded_subtract = |x: usize| max(1, x) - 1;
        let bounded_add_row = |x: usize| min(board.height - 2, x) + 1;
        let bounded_add_col = |x: usize| min(board.width - 2, x) + 1;

        loop {
            let (row, col) = match &self.move_type {
                MoveType::Random => (
                    rng.gen_range(0..board.height),
                    rng.gen_range(0..board.width),
                ),
                MoveType::Adjacent => match rng.gen_range(1..=4) {
                    // up
                    1 => (bounded_subtract(current_row), current_col),
                    // right
                    2 => (current_row, bounded_add_col(current_col)),
                    // down
                    3 => (bounded_add_row(current_row), current_col),
                    // left
                    4 => (current_row, bounded_subtract(current_col)),
                    _ => unreachable!(),
                },
            };

            // loop again if trying to move out of bounds
            if (row, col) != (current_row, current_col) {
                break (row, col);
            }
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let print_me = match self.color {
            Color::Blue => self.symbol.to_string().blue(),
            Color::Cyan => self.symbol.to_string().cyan(),
            Color::Magenta => self.symbol.to_string().magenta(),
            Color::Red => self.symbol.to_string().red(),
            Color::Yellow => self.symbol.to_string().yellow(),
        };
        write!(f, "{print_me}")
    }
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}
