use std::rc::Rc;
use funky_board::{
    run,
    token::{Token, MoveType::{Random, Adjacent}}
};

mod token;
mod board;

fn main() {
    let tokens = vec![
        Rc::new(Token::new('@', Random)),
        Rc::new(Token::new('&', Random)),
        Rc::new(Token::new('$', Adjacent)),
        Rc::new(Token::new('#', Adjacent)),
    ];

    run(&tokens);
}
