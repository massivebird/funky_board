use crate::{dimensions::Dimensions, move_type::MoveType};
use colored::{Color, Colorize};
use std::cell::Cell;

pub struct Token {
    pub symbol: char,
    pub color: colored::Color,
    pub move_type: Box<dyn MoveType>,

    // These being Cells makes the game logic easier. Cycling through all
    // of the tokens in a loop prevents creation of mutable refs. There is
    // probably a non-IM solution, but this is perfectly fine.
    pub pos: Cell<Dimensions>,
    pub is_alive: Cell<bool>,
}

impl Token {
    pub fn new<T: MoveType + 'static>(symbol: char, move_type: T, color: Color) -> Self {
        Self {
            symbol,
            move_type: Box::new(move_type),
            color,
            is_alive: Cell::new(true),
            // Placeholder position until assigned a real position before the game.
            pos: Cell::new(Dimensions::new(0, 0)),
        }
    }

    pub fn relocate(&self) {
        let new_pos = self.move_type.generate(Some(self.pos.get()));
        self.pos.set(new_pos);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.get()
    }

    pub fn kill(&self) {
        self.is_alive.set(false);
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            colored::ColoredString::from(self.symbol.to_string()).color(self.color)
        )
    }
}
