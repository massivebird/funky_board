use funky_board::{
    run,
    token::{
        Token,
        MoveType::{ Random, Adjacent },
        Color::{
            Red,
            Blue,
            Magenta,
            Yellow,
        }
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
