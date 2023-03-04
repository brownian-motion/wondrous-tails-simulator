use crate::stats::{BoardState, BoardCounter};

pub type SimulationResult = Result<BoardCounter, ()>;
pub fn simulate_until_9_stickers(board: BoardState) -> SimulationResult {
	if board.count_stickers() > 9 {
		return Err(());
	}
	let mut boards = vec![board];
	for _ in board.count_stickers()..9 {
		boards = boards.iter().flat_map(|b| b.add_sticker()).collect()
	}
	Ok(boards.into_iter().collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_7_sticker_simulation() {
		// _***
		// *_*_
		// **__
		// ____
		let board = crate::stats::BoardState::new(0b0111_1010_1100_0000);
		let board_counter = simulate_until_9_stickers(board).unwrap();
		assert_eq!([34, 36, 0, 2], board_counter.match_counts());
	}


	#[test]
	fn test_8_sticker_simulation() {
		// _***
		// ***_
		// **__
		// ____
		let board = crate::stats::BoardState::new(0b0111_1110_1100_0000);
		let board_counter = simulate_until_9_stickers(board).unwrap();
		assert_eq!([4, 4, 0, 0], board_counter.match_counts());


		// _***
		// *_*_
		// **__
		// *___
		let board = crate::stats::BoardState::new(0b0111_1010_1100_1000);
		let board_counter = simulate_until_9_stickers(board).unwrap();
		assert_eq!([0, 7, 0, 1], board_counter.match_counts());
	}

	// #[test]
	// fn test_1_sticker_simulation() {
	// 	let top_left_sim = simulate_until_9_stickers(crate::stats::BoardState::new(0b1000_0000_0000_0000));
	// 	let center_bottom_right_sim = simulate_until_9_stickers(crate::stats::BoardState::new(0b0000_0000_0010_0000));
	// 	assert_eq!(top_left_sim, center_bottom_right_sim);
	// }

	#[test]
	fn test_3_sticker_simulation() {
		let top_left_sim = simulate_until_9_stickers(crate::stats::BoardState::new(0b1100_1000_0000_0000));
		let bottom_right_sim = simulate_until_9_stickers(crate::stats::BoardState::new(0b0000_0000_0001_0011));
		assert_eq!(top_left_sim, bottom_right_sim);
	}
}