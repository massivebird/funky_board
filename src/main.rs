use std::{
    collections::HashSet,
    fmt::Display,
    rc::Rc
};
use MoveType::{Random, Adjacent};
use rand::Rng;

#[derive(Debug)]
enum MoveType {
    Random,
    Adjacent,
}

#[derive(Debug)]
struct Token {
    symbol: char,
    move_type: MoveType,
    active: bool,
}

impl Token {
    fn new(symbol: char, move_type: MoveType) -> Self {
        Self { move_type, symbol, active: true }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Option<Rc<Token>>>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self {
            board: vec![vec![None; width]; height],
            width,
            height,
        }       
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                unsafe {
                    match self.board.get_unchecked(row).get_unchecked(col) {
                        Some(token) => output.push(token.symbol),
                        None => output.push('.'),
                    }
                }
            }
            output.push('\n')
        }
        write!(f, "{output}")
    }
}

fn main() {
    const HEIGHT: usize = 4;
    const WIDTH : usize = 8;

    let mut board = Board::new(WIDTH, HEIGHT);

    let mut tokens = vec![
        Rc::new(Token::new('@', Random)),
        Rc::new(Token::new('&', Random)),
        Rc::new(Token::new('$', Adjacent)),
    ];

    let mut rng = rand::thread_rng();

    {
        // place pieces randomly
        let mut init_positions: HashSet::<(usize, usize)> = HashSet::new();
        while init_positions.len() < tokens.len() {
            init_positions.insert((rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT)));
        }

        for (i, (col, row)) in init_positions.iter().enumerate() {
            unsafe {
                let token = Rc::clone(tokens.get_unchecked(i));
                *board.board.get_unchecked_mut(*row).get_unchecked_mut(*col) = Some(token);
            }
        }
    }

    print!("{board}");
}
