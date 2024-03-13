use funky_board::{
    run,
    token::{
        Color::{Blue, Magenta, Red, Yellow},
        MoveType::{Adjacent, Random},
        Token,
    },
};
use std::rc::Rc;

fn main() {
    let tokens = vec![
        Rc::new(Token::new('@', Random, Red)),
        Rc::new(Token::new('&', Random, Blue)),
        Rc::new(Token::new('$', Adjacent, Magenta)),
        Rc::new(Token::new('#', Adjacent, Yellow)),
    ];

    run(&tokens);
}
