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
        // first thing we'll do later is update these positions to match
        // each token's auto-generated initial position
        let token_positions = {
            let mut temp_map = HashMap::new();
            for token in tokens {
                temp_map.insert(Rc::clone(token), RefCell::new((0, 0)));
            }
            temp_map
        };

        Self {
            token_positions,
            width,
            height,
        }
    }

    pub fn try_get_alive_token(&self, target_row: usize, target_col: usize) -> Option<Rc<Token>> {
        let is_here = |row: usize, col: usize| row == target_row && col == target_col;

        self.token_positions
            .iter()
            .find(|(token, coords)| {
                let (row, col) = (coords.borrow().0, coords.borrow().1);
                is_here(row, col) && token.is_alive()
            })
            .map(|(token, _)| Rc::clone(token))
    }

    pub fn get_token_at(&self, target_row: usize, target_col: usize) -> Option<Rc<Token>> {
        self.try_get_alive_token(target_row, target_col)
    }

    pub fn take_row_col(&self, token: &Rc<Token>) -> (usize, usize) {
        self.token_positions.get(&Rc::clone(token)).unwrap().take()
    }

    pub fn update_position(
        &mut self,
        this_token: &Rc<Token>,
        target_row: usize,
        target_col: usize,
    ) {
        self.token_positions
            .entry(Rc::clone(this_token))
            .and_modify(|p| *p.borrow_mut() = (target_row, target_col));
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                match self.try_get_alive_token(row, col) {
                    Some(token) => output.push_str(&token.to_string()),
                    None => output.push('.'),
                }
            }
            output.push('\n');
        }
        write!(f, "{output}")
    }
}
