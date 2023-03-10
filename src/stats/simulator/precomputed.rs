use std::fs::File;
use std::io::{Read, BufReader};
use super::{BoardMatchCounter, BoardState, WondrousTailsSimulator};

// mod data;
// use data::PRECOMPUTED_DATA;
const PRECOMPUTED_DATA: [PrecomputedResult; 0] = [];

#[derive(Debug, Clone, Copy)]
pub struct PrecomputedResult(pub u16, pub u32, pub u32, pub u32);
impl PrecomputedResult {
    pub const fn new(board: BoardState, result: BoardMatchCounter) -> Self {
        let counts = result.match_counts();
        PrecomputedResult(board.0, counts[1], counts[2], counts[3])
    }

    pub const fn to_bytes(&self) -> [u8; 14] {
        let board_bytes = self.0.to_le_bytes();
        let match_one_bytes = self.1.to_le_bytes();
        let match_two_bytes = self.2.to_le_bytes();
        let match_three_bytes = self.3.to_le_bytes();
        [
            board_bytes[0],
            board_bytes[1],
            match_one_bytes[0],
            match_one_bytes[1],
            match_one_bytes[2],
            match_one_bytes[3],
            match_two_bytes[0],
            match_two_bytes[1],
            match_two_bytes[2],
            match_two_bytes[3],
            match_three_bytes[0],
            match_three_bytes[1],
            match_three_bytes[2],
            match_three_bytes[3],
        ]
    }

    pub const fn from_bytes(bytes: [u8; 14]) -> Self {
        PrecomputedResult(
            u16::from_le_bytes([bytes[0], bytes[1]]),
            u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
            u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]),
            u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]),
        )
    }

    #[inline]
    pub fn board(&self) -> BoardState {
        BoardState(self.0)
    }

    #[inline]
    pub fn counts(&self) -> BoardMatchCounter {
        let num_stickers = self.board().count_stickers();
        let num_possible_boards: u32 = (num_stickers..9).map(|n|16-n).product();
        let num_zeroes = num_possible_boards - self.1 - self.2 - self.3;
        BoardMatchCounter::from_counts([num_zeroes, self.1, self.2, self.3])
    }

    #[inline]
    pub fn extract(&self) -> (BoardState, BoardMatchCounter) {
        return (self.board(), self.counts());
    }
}

struct ChunkedIterator<const N: usize, I>(I);

impl<const N: usize, E, I: Iterator<Item=Result<u8, E>>> Iterator for ChunkedIterator<N, I> {
	type Item=Result<[u8;N], E>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut buf = [0u8;N];
		for i in 0..buf.len() {
			buf[i] = match self.0.next() {
				Some(Ok(b)) => b,
				Some(Err(e)) => return Some(Err(e)),
				None => return None, // might drop a few bytes, but it works for our single use case
			}
		}
		Some(Ok(buf))
	}
}

fn chunked<const N: usize, I>(iterator: I) -> ChunkedIterator<N,I> {
	ChunkedIterator::<N,I>(iterator)
}

pub struct PrecomputedSimulator {
	results: Vec<PrecomputedResult>,
}

impl WondrousTailsSimulator for PrecomputedSimulator {
    fn simulate_until_9_stickers(&self, board: BoardState) -> Result<BoardMatchCounter, ()> {
        // unimplemented!()
        let result = match self.results.binary_search_by_key(&board.0, |result| result.0) {
            Ok(idx) => self.results[idx],
            Err(_) => return Err(()),
        };
        Ok(result.counts())
    }
}

impl PrecomputedSimulator {
    pub fn new(results: Vec<PrecomputedResult>) -> Self {
        PrecomputedSimulator { results }
    }

    pub fn from_bytes<R: Read>(read: R) -> std::io::Result<Self> {
    	let results : std::io::Result<Vec<PrecomputedResult>> = chunked(read.bytes())
    		.map(|chunk: std::io::Result<[u8;14]>| chunk.map(|c| PrecomputedResult::from_bytes(c)))
    		.collect();
    	let mut results = results?;
    	results.sort_by_key(|result| result.0);
    	Ok(PrecomputedSimulator{ results })
    }
}