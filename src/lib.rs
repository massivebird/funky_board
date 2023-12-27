use std::{
    cmp::{min, max},
    collections::HashSet,
    rc::Rc
};
use rand::Rng;
use crate::board::Board;
use self::token::{Token, MoveType::{Random, Adjacent}};

pub mod token;
mod board;

pub fn run(tokens: &[Rc<Token>]) {
    const HEIGHT: usize = 4;
    const WIDTH : usize = 8;

    let mut board = Board::new(8, 4, tokens);

    let mut rng = rand::thread_rng();

    {
        // generate unique coordinate pairs
        let mut init_positions: HashSet::<(usize, usize)> = HashSet::new();
        while init_positions.len() < tokens.len() {
            init_positions.insert((rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH)));
        }

        // place those tokens !
        for (i, (row, col)) in init_positions.iter().enumerate() {
            let token = Rc::clone(tokens.get(i).unwrap());
            println!("Tried to place {token} at row {row} col {col}");
            board.token_positions.entry(token).and_modify(|p| *p.borrow_mut() = (*row, *col));
        }
    }

    print!("Starting game!\n{board}");

    let capture = |t: &Rc<Token>| *t.active.borrow_mut() = false;

    let bounded_subtract = |x: usize| max(1, x) - 1;
    let bounded_add_row  = |x: usize| min(HEIGHT - 1, x) + 1;
    let bounded_add_col  = |x: usize| min(WIDTH - 1 , x) + 1;

    let mut token_queue = tokens.iter().cycle();

    let the_battle_is_yet_won = || tokens.iter().filter(|t| t.is_active()).count() > 1;
    while the_battle_is_yet_won() {
        let this_token = loop {
            if let Some(token) = token_queue.next().filter(|t| t.is_active()) { break token }
        };

        let pos = board.token_positions.get(&Rc::clone(this_token)).unwrap().clone();
        let (current_row, current_col) = (pos.borrow().0, pos.borrow().1);
        this_token.print_move_msg();

        let (target_row, target_col) = loop {
            let (row, col) = match this_token.move_type {
                Random => (rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH)),
                Adjacent => match rng.gen_range(1..=4) {
                    // up
                    1 => (bounded_subtract(current_row), current_col),
                    // right
                    2 => (current_row, bounded_add_col(current_col)),
                    // down
                    3 => (bounded_add_row(current_row), current_col),
                    // left
                    4 => (current_row, bounded_subtract(current_col)),
                    _ => unreachable!(),
                }
            };

            // loop again if trying to move out of bounds
            if (row, col) != (current_row, current_col) { break (row, col) }
        };

        if let Some(other_token) = board.try_get_active_token_at(target_row, target_col) {
            capture(&other_token);
            println!("{other_token} has been captured!");
        }

        // update this token's position
        board.token_positions.entry(Rc::clone(this_token))
            .and_modify(|p| *p.borrow_mut() = (target_row, target_col));

        print!("{board}");
    }

    let winning_token = tokens.iter().find(|t| t.is_active()).unwrap();
    println!("{winning_token} wins!");
}
