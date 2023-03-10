use crate::stats::{BoardState, BoardMatchCounter};
use super::WondrousTailsSimulator;

pub type SimulationResult = Result<BoardMatchCounter, ()>;

pub struct ComputingSimulator {}

impl ComputingSimulator {
	pub fn new() -> Self {
		ComputingSimulator{}
	}
}

pub fn new() -> impl WondrousTailsSimulator {
	ComputingSimulator::new()
}

impl WondrousTailsSimulator for ComputingSimulator {
	fn simulate_until_9_stickers(&self, board: BoardState) -> Result<BoardMatchCounter, ()> {
		let result = match board.count_stickers() {
			0 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			1 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			2 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			3 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			4 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			5 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			6 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			7 => board.iter()
				.flat_map(|b| b.add_sticker())
				.flat_map(|b| b.add_sticker())
				.collect(),
			8 => board.iter()
				.flat_map(|b| b.add_sticker())
				.collect(),
			9 => board.iter().collect(),
			_ => return Err(()),
		};
		Ok(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::stats::WondrousTailsSimulator;

	#[test]
	fn test_7_sticker_simulation() {
		let sim = ComputingSimulator::new();
		// _***
		// *_*_
		// **__
		// ____
		let board = crate::stats::BoardState::new(0b0111_1010_1100_0000);
		let board_counter = sim.simulate_until_9_stickers(board).unwrap();
		assert_eq!([34, 36, 0, 2], board_counter.match_counts());
	}


	#[test]
	fn test_8_sticker_simulation() {
		let sim = ComputingSimulator::new();
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
	}

	// #[test]
	// fn test_1_sticker_simulation() {
	// 	let top_left_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b1000_0000_0000_0000));
	// 	let center_bottom_right_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b0000_0000_0010_0000));
	// 	assert_eq!(top_left_sim, center_bottom_right_sim);
	// }

	#[test]
	fn test_3_sticker_simulation() {
		let sim = ComputingSimulator::new();
		let top_left_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b1100_1000_0000_0000));
		let bottom_right_sim = sim.simulate_until_9_stickers(crate::stats::BoardState::new(0b0000_0000_0001_0011));
		assert_eq!(top_left_sim, bottom_right_sim);
	}
}