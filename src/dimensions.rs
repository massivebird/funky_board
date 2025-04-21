pub static BOARD_SIZE: Dimensions = Dimensions { row: 4, col: 8 };

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dimensions {
    pub row: usize,
    pub col: usize,
}

impl Dimensions {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

