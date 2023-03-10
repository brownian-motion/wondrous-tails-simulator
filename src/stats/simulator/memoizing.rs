use std::collections::HashMap;
use super::{WondrousTailsSimulator};
use crate::stats::{BoardState, BoardMatchCounter};

pub struct MemoizingSimulator<T>{
	delegate: T,
	memos: HashMap<BoardState, BoardMatchCounter>,
}

impl<T> WondrousTailsSimulator for MemoizingSimulator<T> where T: WondrousTailsSimulator {
	fn simulate_until_9_stickers(&mut self, board: BoardState) -> Result<BoardMatchCounter, ()> {
		if let Some(matches) = self.memos.get(&board) {
			return Ok(matches.clone());
		}
		let matches = self.delegate.simulate_until_9_stickers(board)?;
		self.memos.insert(board, matches);
		Ok(matches)
	}
}

impl<T> MemoizingSimulator<T> {
	pub fn new(delegate: T) -> MemoizingSimulator<T> {
		MemoizingSimulator {
			delegate,
			memos: HashMap::new(),
		}
	}
}