use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::token::Token;

#[derive(Debug)]
pub struct Board {
    pub token_positions: HashMap<Rc<Token>, RefCell<(usize, usize)>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, tokens: &[Rc<Token>]) -> Self {
        let token_positions = {
            let mut temp_map: HashMap<Rc<Token>, RefCell<(usize, usize)>> = HashMap::new();
            for token in tokens {
                temp_map.insert(Rc::clone(token), RefCell::new((0,0)));
            }
            temp_map
        };

        Self {
            token_positions,
            width,
            height,
        }       
    }

    pub fn try_get_active_token_at(&self, target_row: usize, target_col: usize) -> Option<Rc<Token>> {
        for (token, pos) in &self.token_positions {
            let (row, col) = (pos.borrow().0, pos.borrow().1);
            if token.is_active() && (row, col) == (target_row, target_col) {
                return Some(Rc::clone(token))
            }
        }
        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                match self.try_get_active_token_at(row, col) {
                    Some(token) => output.push(token.symbol),
                    None => output.push('.'),
                }
            }
            output.push('\n');
        }
        write!(f, "{output}")
    }
}