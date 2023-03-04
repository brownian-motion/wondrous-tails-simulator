use std::fmt::Debug;

#[derive(PartialEq, Clone, Copy)]
pub struct BoardState(u16);

impl BoardState {
	const fn bit_idx(row: usize, col:usize) -> usize {
		row*4+col
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
		AdditionalStickerIterator { state: self.clone(), curr_bit: 0 }
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

	fn count_matches(&self) -> u8 {
		let mut total = 0u8;

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
}

impl std::fmt::Debug for BoardState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BoardState({:04X})", self.0))
    }
}

struct AdditionalStickerIterator {
	state: BoardState,
	curr_bit: usize,
}

impl Iterator for AdditionalStickerIterator {
	type Item = BoardState;
	
	fn next(&mut self) -> Option<BoardState> {
		loop {
			if self.curr_bit >= 16 { return None }
			if !self.state.is_bit_set(self.curr_bit) {
				break;
			}
			self.curr_bit += 1;
		}

		let out = Some(self.state.toggle_bit(self.curr_bit));
		self.curr_bit += 1;
		out
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
}