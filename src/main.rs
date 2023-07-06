use std::{
    cell::RefCell,
    cmp::{min, max},
    collections::HashMap,
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
    token_positions: HashMap<Rc<Token>, RefCell<(usize, usize)>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize, tokens: &Vec<Rc<Token>>) -> Self {
        let token_positions = {
            let mut temp_map: HashMap<Rc<Token>, RefCell<(usize, usize)>> = HashMap::new();
            for token in tokens {
                temp_map.insert(Rc::clone(token), RefCell::new((0,0)));
            }
            temp_map
        };

        Self {
            token_positions,
            width,
            height,
        }       
    }

    fn try_get_active_token_at(&self, target_row: usize, target_col: usize) -> Option<Rc<Token>> {
        for (token, pos) in &self.token_positions {
            let (row, col) = (pos.borrow().0, pos.borrow().1);
            if token.is_active() && (row, col) == (target_row, target_col) {
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
            output.push('\n');
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
        Rc::new(Token::new('#', Adjacent)),
    ];

    let mut board = Board::new(8, 4, &tokens);

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

    while tokens.iter().filter(|t| t.is_active()).count() > 1 {
        let this_token = loop {
            if let Some(token) = token_queue.next().filter(|t| t.is_active()) { break token }
        };

        let pos = board.token_positions.get(&Rc::clone(this_token)).unwrap().clone();
        let (current_row, current_col) = (pos.borrow().0, pos.borrow().1);
        println!("{this_token} is moving.");

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
