use crate::dimensions::{Dimensions, BOARD_SIZE};
use rand::Rng;

pub trait MoveStrategy {
    fn generate(&self, old_pos: Option<Dimensions>) -> Dimensions;

    fn descriptor(&self) -> String;
}

/// Moves a token to a new, random position on the board.
pub struct Random;
impl MoveStrategy for Random {
    fn generate(&self, old_pos: Option<Dimensions>) -> Dimensions {
        let mut rng = rand::rng();

        loop {
            let dest = Dimensions::new(
                rng.random_range(0..BOARD_SIZE.row),
                rng.random_range(0..BOARD_SIZE.col),
            );

            // Loop again if the destination is the current position.
            if old_pos.as_ref().is_none_or(|old_pos| *old_pos != dest) {
                break dest;
            }
        }
    }

    fn descriptor(&self) -> String {
        "randomly".to_string()
    }
}

/// Moves a token to some new, adjacent (orthogonal or diagonal) space.
///
/// # Panics
///
/// This move strategy requires an existing position, and will panic if this
/// token does not provide one.
pub struct Adjacent;
impl MoveStrategy for Adjacent {
    fn generate(&self, old_pos: Option<Dimensions>) -> Dimensions {
        // This move type requires an existing position.
        let old_pos = old_pos.unwrap();

        let mut rng = rand::rng();

        // Returns the input value with one of the following effects:
        //
        // 1. `input + 1`
        // 2. `input - 1`
        // 3. `input` (unchanged)
        //
        // Limits output based on some maximum bound.
        let mut random_shift = |x: usize, bound: usize| {
            let candidate = match rng.random_range(0..=2) {
                0 => usize::saturating_sub(x, 1),
                1 => x,
                2 => usize::saturating_add(x, 1),
                _ => unreachable!(),
            };

            // Limit candidate within board index bounds.
            usize::min(bound - 1, candidate)
        };

        loop {
            let new_pos = Dimensions::new(
                random_shift(old_pos.row, BOARD_SIZE.row),
                random_shift(old_pos.col, BOARD_SIZE.col),
            );

            // Loop again if we generated the old position.
            if new_pos != old_pos {
                break new_pos;
            }
        }
    }

    fn descriptor(&self) -> String {
        "adjacently".to_string()
    }
}
