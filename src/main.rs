use self::{
    dimensions::{Dimensions, BOARD_SIZE},
    move_type::MoveType,
    token::move_type,
};
use crate::token::Token;
use colored::Color;

mod dimensions;
mod token;

fn main() {
    let tokens = vec![
        Token::new('@', move_type::Random, Color::Red),
        Token::new('&', move_type::Random, Color::Blue),
        Token::new('$', move_type::Adjacent, Color::Magenta),
        Token::new('#', move_type::Adjacent, Color::Yellow),
    ];

    // Place all tokens in random, unique positions.
    let mut occupied: Vec<Dimensions> = Vec::new();
    for token in &tokens {
        loop {
            let pos = move_type::Random.generate(None);

            if !occupied.iter().any(|&other| other == pos) {
                token.pos.set(pos);

                occupied.push(pos);

                break;
            }
        }
    }

    println!("Starting game!");
    display_board(&tokens);

    let mut turn_queue = tokens.iter().cycle().filter(|t| t.is_alive());

    // Game continues until only one token is alive.
    let the_battle_rages_on = || tokens.iter().filter(|t| t.is_alive()).count() >= 2;

    while the_battle_rages_on() {
        let this = turn_queue.next().unwrap();

        println!("{this} is moving {}.", this.move_type.descriptor());

        this.relocate();

        let new_pos = this.pos.get();

        if let Some(kill_this_guy) = tokens
            .iter()
            .filter(|other| other.symbol != this.symbol && other.is_alive())
            .find(|other| other.pos.get() == new_pos)
        {
            println!("{kill_this_guy} has been captured!");
            kill_this_guy.kill();
        }

        display_board(&tokens);
    }

    let winner = tokens.iter().find(|t| t.is_alive()).unwrap();

    println!("{winner} is the winner!");
}

fn display_board(tokens: &[Token]) {
    for row in 0..BOARD_SIZE.row {
        for col in 0..BOARD_SIZE.col {
            let here = Dimensions::new(row, col);

            if let Some(token) = tokens
                .iter()
                .filter(|t| t.is_alive())
                .find(|t| t.pos.get() == here)
            {
                print!("{token}");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
}
