use colored::{Color, Colorize};
use rand::Rng;
use std::{
    cell::Cell,
    cmp::{max, min},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Dimensions {
    row: usize,
    col: usize,
}

impl Dimensions {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

static BOARD_SIZE: Dimensions = Dimensions { row: 4, col: 8 };

trait MoveType {
    fn generate(&self, coords: Option<Dimensions>) -> Dimensions;
    fn descriptor(&self) -> String;
}

struct Random;
impl MoveType for Random {
    fn generate(&self, coords: Option<Dimensions>) -> Dimensions {
        let mut rng = rand::rng();

        loop {
            let (row, col) = (
                rng.random_range(0..BOARD_SIZE.row),
                rng.random_range(0..BOARD_SIZE.col),
            );

            // Loop again if we generated the same dimensions.
            if coords
                .as_ref()
                .is_none_or(|coords| (row, col) != (coords.row, coords.col))
            {
                break Dimensions::new(row, col);
            }
        }
    }

    fn descriptor(&self) -> String {
        "randomly".to_string()
    }
}

struct Adjacent;
impl MoveType for Adjacent {
    fn generate(&self, coords: Option<Dimensions>) -> Dimensions {
        let coords = coords.unwrap();

        let bounded_subtract = |x: usize| max(1, x) - 1;
        let bounded_add_row = |x: usize| min(BOARD_SIZE.row - 2, x) + 1;
        let bounded_add_col = |x: usize| min(BOARD_SIZE.col - 2, x) + 1;

        let mut rng = rand::rng();

        loop {
            let (row, col) = match rng.random_range(1..=4) {
                // up
                1 => (bounded_subtract(coords.row), coords.col),
                // right
                2 => (coords.row, bounded_add_col(coords.col)),
                // down
                3 => (bounded_add_row(coords.row), coords.col),
                // left
                4 => (coords.row, bounded_subtract(coords.col)),
                _ => unreachable!(),
            };

            // Loop again if we generated the same dimensions.
            if (row, col) != (coords.row, coords.col) {
                break Dimensions::new(row, col);
            }
        }
    }

    fn descriptor(&self) -> String {
        "adjacently".to_string()
    }
}

struct Token {
    symbol: char,
    color: colored::Color,
    move_type: Box<dyn MoveType>,

    // These being Cells makes the game logic easier. Cycling through all
    // of the tokens in a loop prevents creation of mutable refs. There is
    // probably a non-IM solution, but this is perfectly fine.
    pos: Cell<Dimensions>,
    is_alive: Cell<bool>,
}

impl Token {
    fn new<T: MoveType + 'static>(symbol: char, move_type: T, color: Color) -> Self {
        Self {
            symbol,
            move_type: Box::new(move_type),
            color,
            is_alive: Cell::new(true),
            // Placeholder position until assigned a real position before the game.
            pos: Cell::new(Dimensions::new(0, 0)),
        }
    }

    fn relocate(&self) {
        let new_pos = self.move_type.generate(Some(self.pos.get()));
        self.pos.set(new_pos);
    }

    fn is_alive(&self) -> bool {
        self.is_alive.get()
    }

    fn kill(&self) {
        self.is_alive.set(false);
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            colored::ColoredString::from(self.symbol.to_string()).color(self.color)
        )
    }
}

fn main() {
    let tokens = vec![
        Token::new('@', Random, Color::Red),
        Token::new('&', Random, Color::Blue),
        Token::new('$', Adjacent, Color::Magenta),
        Token::new('#', Random, Color::Yellow),
    ];

    // Place all tokens in random, unique positions.
    let mut taken: Vec<Dimensions> = Vec::new();

    for token in &tokens {
        loop {
            let pos = Random.generate(None);

            if !taken.iter().any(|&other| other == pos) {
                token.pos.set(pos);
                taken.push(pos);
                break;
            }
        }
    }

    println!("Starting game!");
    display_board(&tokens);

    let mut turn_queue = tokens.iter().cycle().filter(|t| t.is_alive());

    while tokens.iter().filter(|t| t.is_alive()).count() >= 2 {
        let this = turn_queue.next().unwrap();

        this.relocate();

        let new_pos = this.pos.get();

        println!("{this} is moving {}.", this.move_type.descriptor());

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
}

fn display_board(tokens: &[Token]) {
    for row in 0..BOARD_SIZE.row {
        for col in 0..BOARD_SIZE.col {
            if let Some(token) = tokens
                .iter()
                .filter(|t| t.is_alive())
                .find(|t| t.pos.get().row == row && t.pos.get().col == col)
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
