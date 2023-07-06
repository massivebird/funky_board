use std::cell::RefCell;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveType {
    Random,
    Adjacent,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub symbol: char,
    pub move_type: MoveType,
    pub active: RefCell<bool>,
}

impl Token {
    pub fn new(symbol: char, move_type: MoveType) -> Self {
        Self { move_type, symbol, active: RefCell::new(true) }
    }

    pub fn is_active(&self) -> bool {
        *self.active.borrow()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

