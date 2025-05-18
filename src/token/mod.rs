use self::move_strategy::MoveStrategy;
use crate::dimensions::Dimensions;
use colored::Colorize;
use std::{
    cell::Cell,
    sync::atomic::{AtomicU32, Ordering},
};

pub mod move_strategy;

// Used to generate unique token IDs.
static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct Token {
    pub symbol: char,
    pub color: colored::Color,
    pub move_strategy: Box<dyn MoveStrategy>,

    // Unique token identifier.
    id: u32,

    // These being Cells makes the game logic easier. Cycling through all
    // of the tokens in a loop prevents creation of mutable refs. There is
    // probably a non-IM solution, but this is perfectly fine.
    pub pos: Cell<Dimensions>,
    pub is_alive: Cell<bool>,
}

impl Token {
    pub fn new<T: MoveStrategy + 'static>(
        symbol: char,
        move_strategy: T,
        color: colored::Color,
    ) -> Self {
        Self {
            symbol,
            color,
            is_alive: Cell::new(true),
            move_strategy: Box::new(move_strategy),

            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),

            // Placeholder position until this token is placed on the board.
            pos: Cell::new(Dimensions::new(0, 0)),
        }
    }

    /// Moves this token to a new position on the board.
    pub fn relocate(&self) {
        let dest = self.move_strategy.generate(Some(self.pos.get()));
        self.pos.set(dest);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.get()
    }

    /// 25 to life
    pub fn kill(&self) {
        self.is_alive.set(false);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
