use std::io::BufReader;
use std::fs::File;

pub mod computing;
pub use computing::*;

pub mod precomputed;
pub use precomputed::*;

// pub mod memoizing;
// pub use memoizing::*;

use crate::stats::{BoardState, BoardMatchCounter};

pub trait WondrousTailsSimulator {
	fn simulate_until_9_stickers(&self, board: BoardState) -> Result<BoardMatchCounter, ()>;
}

pub fn new() -> impl WondrousTailsSimulator {
    computing::new()
}

pub fn load_precomputed_simulator() -> std::io::Result<PrecomputedSimulator> {
    let file = File::open("results.dat")?;
    let read = BufReader::new(file);
    PrecomputedSimulator::from_bytes(read)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_precomputed_data() {
        let _precomputed_sim = load_precomputed_simulator().expect("Can't load precomputed data!");
    }

    #[test]
    fn precomputed_data_is_valid() {
        let sim =  load_precomputed_simulator().unwrap();

        // _***
        // *_*_
        // **__
        // ____
        let board = crate::stats::BoardState::new(0b0111_1010_1100_0000);
        let board_counter = sim.simulate_until_9_stickers(board).unwrap();
        assert_eq!([34, 36, 0, 2], board_counter.match_counts());

        // _***
        // ***_
        // **__
        // ____
        let board = crate::stats::BoardState::new(0b0111_1110_1100_0000);
        let board_counter = sim.simulate_until_9_stickers(board).unwrap();
        assert_eq!([4, 4, 0, 0], board_counter.match_counts());


        // _***
        // *_*_
        // **__
        // *___
        let board = crate::stats::BoardState::new(0b0111_1010_1100_1000);
        let board_counter = sim.simulate_until_9_stickers(board).unwrap();
        assert_eq!([0, 7, 0, 1], board_counter.match_counts());

        let top_left_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b1100_1000_0000_0000));
        let bottom_right_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b0000_0000_0001_0011));
        assert_eq!(top_left_sim, bottom_right_sim);
    }

    #[test]
    fn random_boards_scoring_equivalency() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let random_boards = (0..20)
            .map(move |_| rng.gen::<u16>())
            .map(|seed| BoardState(seed))
            .filter(|board| board.count_stickers() >= 3 && board.count_stickers() < 9);

        let precomputed_sim = load_precomputed_simulator().unwrap();

        for board in random_boards {
            let computing_sim = computing::new();
            assert_eq!(
                precomputed_sim.simulate_until_9_stickers(board.clone()),
                computing_sim.simulate_until_9_stickers(board.clone()),
            );
        }
    }

}