pub mod board;
pub use board::*;

pub mod shuffle_results;
pub use shuffle_results::*;

pub mod simulator;
pub use simulator::*;

pub mod counter;
pub use counter::*;

// thanks to https://stackoverflow.com/a/65563202/929708
pub fn count_combinations(n: u64, r: u64) -> u64 {
    (n - r + 1..=n).product::<u64>() / factorial(r)
}
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}