use std::cell::RefCell;
use std::collections::HashMap;
use std::{
    collections::HashSet,
    fmt::Display,
    rc::Rc
};
use MoveType::{Random, Adjacent};
use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
enum MoveType {
    Random,
    Adjacent,
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    symbol: char,
    move_type: MoveType,
    active: RefCell<bool>,
}

impl Token {
    fn new(symbol: char, move_type: MoveType) -> Self {
        Self { move_type, symbol, active: RefCell::new(true) }
    }

    fn is_active(&self) -> bool {
        *self.active.borrow()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

#[derive(Debug)]
struct Board {
    token_positions: HashMap<Rc<Token>, (usize, usize)>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize, tokens: &Vec<Rc<Token>>) -> Self {
        let mut token_positions = HashMap::new();

        for token in tokens {
            token_positions.insert(Rc::clone(&token), (0,0));
        }

        Self {
            token_positions,
            width,
            height,
        }       
    }

    fn try_get_active_token_at(&self, target_row: usize, target_col: usize) -> Option<Rc<Token>> {
        for (token, (row, col)) in self.token_positions.iter() {
            if token.is_active() && (*row, *col) == (target_row, target_col) {
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
            output.push('\n')
        }
        write!(f, "{output}")
    }
}

fn main() {
    const HEIGHT: usize = 4;
    const WIDTH : usize = 8;

    let tokens = vec![
        Rc::new(Token::new('@', Random)),
        Rc::new(Token::new('&', Random)),
        Rc::new(Token::new('$', Adjacent)),
    ];

    let mut board = Board::new(8, 4, &tokens);

    let mut rng = rand::thread_rng();

    {
        // generate unique coordinate pairs
        let mut init_positions: HashSet::<(usize, usize)> = HashSet::new();
        while init_positions.len() < tokens.len() {
            init_positions.insert((rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH)));
        }

        dbg!(&init_positions);

        // place those tokens !
        for (i, (row, col)) in init_positions.iter().enumerate() {
            let token = Rc::clone(tokens.get(i).unwrap());
            println!("Tried to place {token} at row {row} col {col}");
            board.token_positions.entry(token).and_modify(|p| *p = (*row, *col));
        }
    }

    print!("Starting game!\n{board}");

    let capture = |t: &Rc<Token>| *t.active.borrow_mut() = false;

    let mut token_queue = tokens.iter().cycle();

    while tokens.iter().filter(|t| t.is_active()).count() > 1 {
        let this_token = loop {
            let token = token_queue.next().unwrap();
            if token.is_active() { break token }
        };

        let (current_row, current_col) = board.token_positions.get(&Rc::clone(&this_token)).unwrap().to_owned();
        println!("{this_token} is moving.");

        // TODO: consider MoveType
        let (target_row, target_col) = loop {
            let (row, col) = (rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH));
            if (row, col) != (current_row, current_col) { break (row, col) }
        };

        if let Some(other_token) = board.try_get_active_token_at(target_row, target_col) {
            capture(&other_token);
            println!("{other_token} has been captured!");
        }

        board.token_positions.entry(Rc::clone(this_token)).and_modify(|p| *p = (target_row, target_col));

        print!("{board}");
    }

    let winning_token = tokens.iter().find(|t| t.is_active()).unwrap();
    println!("{winning_token} wins!");
}
