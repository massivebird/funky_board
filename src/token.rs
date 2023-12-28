use colored::Colorize;
use std::cell::RefCell;

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
    pub active: RefCell<bool>,
    pub color: Color,
}

impl Token {
    pub const fn new(symbol: char, move_type: MoveType, color: Color) -> Self {
        Self {
            move_type,
            symbol,
            active: RefCell::new(true),
            color,
        }
    }

    pub fn is_active(&self) -> bool {
        *self.active.borrow()
    }

    pub fn print_move_msg(&self) {
        let adverb = match self.move_type {
            MoveType::Random => "randomly",
            MoveType::Adjacent => "adjacently"
        };
        println!("{self} is moving {adverb}.");
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
