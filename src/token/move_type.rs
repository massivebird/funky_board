use crate::dimensions::{Dimensions, BOARD_SIZE};
use rand::Rng;
use std::cmp::{max, min};

pub trait MoveType {
    fn generate(&self, coords: Option<Dimensions>) -> Dimensions;
    fn descriptor(&self) -> String;
}

pub struct Random;
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

pub struct Adjacent;
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
