use super::board::*;

#[derive(Default,Debug,PartialEq)]
pub struct BoardCounter {
	counts: [u32;4],
	total: u32,
	count: u32,
}

impl BoardCounter {
	pub const fn empty() -> Self{
		BoardCounter {
			counts: [0;4],
			total: 0,
			count: 0,
		}
	}

	pub const fn from_counts(counts: [u32;4]) -> Self {
		let count = counts[0] + counts[1] + counts[2] + counts[3];
		let total = counts[1] + counts[2] * 2 + counts[3] * 3;
		BoardCounter {
			counts,
			total,
			count,
		}
	}

	pub fn count(&mut self, board: BoardState) {
		let matches = board.count_matches();
		self.counts[matches as usize] += 1;
		self.total += matches;
		self.count += 1;
	}

	pub fn average(&self) -> f32{
		(self.total as f32)/(self.count as f32)
	}

	pub fn distribution(&self) -> [f32;4] {
		[
			(self.counts[0] as f32)/(self.count as f32),
			(self.counts[1] as f32)/(self.count as f32),
			(self.counts[2] as f32)/(self.count as f32),
			(self.counts[3] as f32)/(self.count as f32)
		]
	}

	pub fn num_boards(&self) -> u32 {
		self.count
	}

	pub fn total_matches(&self) -> u32 {
		self.total
	}

	pub fn match_counts(&self) -> [u32; 4] {
		self.counts
	}
}

impl FromIterator<BoardState> for BoardCounter {
	fn from_iter<I: IntoIterator<Item=BoardState>>(iter: I) -> Self {
		let mut counter = BoardCounter::default();
		for board in iter {
			counter.count(board);
		}

		counter
	}
}

#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn test_counting_boards_with_9_stickers() {
		let board_counter: BoardCounter = 
			(0..0xFFFFu16)
				.map(|state| BoardState::new(state))
		    	.filter(|board| board.count_stickers() == 9)
		    	.collect();

		assert_eq!(crate::stats::shuffle_results::SHUFFLED_BOARD_COUNTS, board_counter.match_counts());
		assert_eq!(crate::stats::shuffle_results::SHUFFLED_BOARD_COUNT, board_counter.num_boards());
		assert_eq!(crate::stats::shuffle_results::SHUFFLED_BOARD_TOTAL_MATCHES, board_counter.total_matches());
	}
}