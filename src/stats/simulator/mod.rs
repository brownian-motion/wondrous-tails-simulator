pub mod computed;
pub use computed::*;

// pub mod cached;
// pub use cached::*;

use crate::stats::{BoardState, BoardMatchCounter};

pub trait WondrousTailsSimulator {
	fn simulate_until_9_stickers(&self, board: BoardState) -> Result<BoardMatchCounter, ()>;
}

pub fn new() -> impl WondrousTailsSimulator {
    computed::ComputedSimulator::new()
}

// thanks to https://stackoverflow.com/a/65563202/929708
fn count_combinations(n: u64, r: u64) -> u64 {
    (n - r + 1..=n).product::<u64>() / factorial(r)
}
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
