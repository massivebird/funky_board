use std::{
    collections::HashSet,
    rc::Rc
};
use rand::Rng;
use crate::board::Board;
use self::token::Token;

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
            board.token_positions.entry(token).and_modify(|p| *p.borrow_mut() = (*row, *col));
        }
    }

    print!("Starting game!\n{board}");

    let mut token_queue = tokens.iter().cycle();

    let mut next_alive_token = || -> &Rc<Token> {
        loop {
            if let Some(alive_token) = token_queue.next().filter(|t| t.is_alive()) {
                break alive_token
            }
        }
    };

    let the_battle_is_not_yet_won = || tokens.iter().filter(|t| t.is_alive()).count() > 1;

    while the_battle_is_not_yet_won() {
        let this_token = next_alive_token();

        let (current_row, current_col) = board.get_row_col(this_token);

        let (target_row, target_col) = this_token.generate_target_coords(
            &board,
            current_row,
            current_col,
            &mut rng
        );

        this_token.print_move_msg();

        board.try_kill_and_print_if_killing(target_row, target_col);

        board.update_position(this_token, target_row, target_col);

        print!("{board}");
    }

    let winning_token = tokens.iter().find(|t| t.is_alive()).unwrap();
    println!("{winning_token} wins!");
}
