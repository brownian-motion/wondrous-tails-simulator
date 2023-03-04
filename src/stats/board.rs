

#[derive(PartialEq, Clone, Copy)]
pub struct BoardState(u16);

impl BoardState {
	pub fn new(state: u16) -> BoardState {
		BoardState(state)
	}

	pub fn empty() -> BoardState {
		BoardState(0u16)
	}

	const fn bit_idx(row: usize, col:usize) -> usize {
    // 0b0000_0000_0000_0000 
    //   ^ (0,0)          ^
    //              (3,2) |

		15-(row*4+col)
	}

	pub fn is_sticker(&self, row: usize, col: usize) -> bool {
		self.is_bit_set(Self::bit_idx(row,col))
	}

	fn is_bit_set(&self, bit_idx: usize) -> bool {
		let mask = 1 << bit_idx;
		(self.0 & mask) != 0
	}

	pub fn toggle(&self, row: usize, col: usize) -> BoardState {
		self.toggle_bit(Self::bit_idx(row, col))
	}

	fn toggle_bit(&self, bit_idx: usize) -> BoardState {
		let mut c: BoardState = self.clone();
		c.0 ^= 1 << bit_idx;
		c
	}

	pub fn count_stickers(&self) -> u32 {
		self.0.count_ones()
	}

	pub fn add_sticker(&self) -> impl Iterator<Item=BoardState> {
			let board = self.clone();
			(0..16).filter_map(move |bit_idx| if board.is_bit_set(bit_idx) { None } else { Some(board.toggle_bit(bit_idx))})
	}

	fn is_row_full(&self, row: usize) -> bool {
		self.is_sticker(row,0) && self.is_sticker(row, 1) && self.is_sticker(row,2) && self.is_sticker(row, 3)
	}

	fn is_col_full(&self, col: usize) -> bool {
		self.is_sticker(0, col) && self.is_sticker( 1, col) && self.is_sticker(2, col) && self.is_sticker( 3, col)
	}

	fn is_top_left_diag_full(&self ) -> bool {
		self.is_sticker(0,0) && self.is_sticker(1,1) && self.is_sticker(2,2) && self.is_sticker(3,3)
	}

	fn is_top_right_diag_full(&self ) -> bool {
		self.is_sticker(3,0) && self.is_sticker(2,1) && self.is_sticker(1,2) && self.is_sticker(0,3)
	}

	pub fn count_matches(&self) -> u32 {
		let mut total = 0u32;

		for r in 0..4 {
			if self.is_row_full(r) { total += 1 }
		}

		for c in 0..4 {
			if self.is_col_full(c) { total += 1 }
		}

		if self.is_top_left_diag_full() { total += 1 }
		if self.is_top_right_diag_full() { total += 1 }

		total
	}

	pub fn iter(&self) -> impl Iterator<Item = BoardState> {
		Some(self.clone()).into_iter()
	}
}

impl std::fmt::Debug for BoardState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BoardState({:04X})", self.0))
    }
}

impl IntoIterator for BoardState {
	type Item = BoardState;
	type IntoIter = std::option::IntoIter<BoardState>;

	fn into_iter(self) -> Self::IntoIter {
		Some(self).into_iter()
	}
}

#[derive(Default,Debug,PartialEq)]
pub struct BoardCounter {
	counts: [u32;4],
	total: u32,
	count: u32,
}

impl BoardCounter {
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
	fn test_add_sticker_empty_board() {
		let board = BoardState(0u16);
		let next_states: Vec<BoardState> = board.add_sticker().collect();
		assert_eq!(next_states, vec![
			BoardState(0x0001u16), BoardState(0x0002u16), BoardState(0x0004u16), BoardState(0x0008u16),
			BoardState(0x0010u16), BoardState(0x0020u16), BoardState(0x0040u16), BoardState(0x0080u16),
			BoardState(0x0100u16), BoardState(0x0200u16), BoardState(0x0400u16), BoardState(0x0800u16),
			BoardState(0x1000u16), BoardState(0x2000u16), BoardState(0x4000u16), BoardState(0x8000u16),
		]);
	}


	#[test]
	fn test_add_sticker_single_move() {
		let board = BoardState(0x0400u16);
		let next_states: Vec<BoardState> = board.add_sticker().collect();
		assert_eq!(next_states, vec![
			BoardState(0x0401u16), BoardState(0x0402u16), BoardState(0x0404u16), BoardState(0x0408u16),
			BoardState(0x0410u16), BoardState(0x0420u16), BoardState(0x0440u16), BoardState(0x0480u16),
			BoardState(0x0500u16), BoardState(0x0600u16),                        BoardState(0x0C00u16),
			BoardState(0x1400u16), BoardState(0x2400u16), BoardState(0x4400u16), BoardState(0x8400u16),
		]);
	}

	#[test]
	fn test_three_sticker_steps_count() {
		let board = BoardState(0u16);
		let first_step = board.add_sticker();
		let second_step = first_step.flat_map(|b| b.add_sticker());

		assert_eq!(16*15, second_step.count());
	}


	#[test]
	fn test_four_sticker_steps_count() {
		let board = BoardState(0u16);
		let first_step = board.add_sticker();
		let second_step = first_step.flat_map(|b| b.add_sticker());
		let third_step = second_step.flat_map(|b| b.add_sticker());
		let fourth_step = third_step.flat_map(|b| b.add_sticker());

		assert_eq!(16*15*14*13, fourth_step.count());
	}

	#[test]
	fn test_count_matches() {
		assert_eq!(0, BoardState(0u16).count_matches());
		assert_eq!(1, BoardState(0b1000_1000_1000_1000u16).count_matches());
		assert_eq!(2, BoardState(0b1111_0000_1111_0000u16).count_matches());
		assert_eq!(3, BoardState(0b1100_0100_0110_1111u16).count_matches());
		assert_eq!(1, BoardState(0b0001_0010_0100_1000u16).count_matches());
	}

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

	#[test]
	fn toggle_flips_sticker() {
		assert_eq!(BoardState::new(0b1000_0000_0000_0000u16), BoardState::empty().toggle(0,0));
		assert_eq!(BoardState::new(0b0000_0000_0001_0000u16), BoardState::empty().toggle(2,3));
		assert_eq!(BoardState::empty(), BoardState::empty().toggle(3,1).toggle(3,1));
	}
}