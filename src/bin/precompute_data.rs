use std::fmt::*;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::collections::{VecDeque, HashMap};
use wondrous_tails_simulator::stats::precomputed::PrecomputedResult;
use wondrous_tails_simulator::stats::simulator::computing::*;
use wondrous_tails_simulator::stats::{BoardMatchCounter, BoardState, WondrousTailsSimulator, count_combinations};

fn get_valid_boards() -> impl Iterator<Item = BoardState> {
    (0..0xFFFFu16)
        .map(|id| BoardState::new(id))
        .filter(|board| board.count_stickers() >= 3)
        .filter(|board| board.count_stickers() < 9)
}

fn generate_9_sticker_boards() -> impl Iterator<Item = BoardState> {
    (0..0xFFFFu16)
        .map(|id| BoardState::new(id))
        .filter(|board| board.count_stickers() == 9)
}
fn expected_count() -> u64 {
	(1..=8).map(|d| count_combinations(16,d)).sum()
}

struct PrecomputeGenerator {
	buf: HashMap<BoardState, BoardMatchCounter>,
	next: VecDeque<BoardState>,
}

impl PrecomputeGenerator {
	pub fn new() -> Self {
		let mut buf =  HashMap::new();
		buf.reserve(expected_count() as usize);
		PrecomputeGenerator {
			buf,
			next: VecDeque::from_iter(generate_9_sticker_boards()),
		}
	}
}

impl PrecomputeGenerator {
	fn next_new_board(&mut self) -> Option<BoardState> {
		loop {	
			let board = match self.next.pop_front() {
				Some(b) => b,
				None => return None,
			};
			if !self.buf.contains_key(&board) {
				return Some(board);
			}
		}
	}
}

impl Iterator for PrecomputeGenerator {
	type Item = PrecomputedResult;

	fn next(&mut self) -> Option<Self::Item> {
		let board = match self.next_new_board() {
			Some(b) => b,
			None => return None,
		};
		let score: BoardMatchCounter = if board.count_stickers() == 9 {
			board.iter().collect()
		} else {
			board.clone().add_sticker()
				.map(|b|{
					match self.buf.get(&b) {
						Some(score) => score.clone(),
						None => panic!("board {:?} should already exist!", board),
					}
				})
				.collect()
		};
		self.buf.insert(board, score);
		for b in board.remove_sticker() {
			self.next.push_back(b);
		}
		Some(PrecomputedResult::new(board, score))
	}
}

fn write_progress_bar<W: Write>(writer: &mut W, label: &str, progress: u64, max: u64) -> std::io::Result<()> {
	let mut w = BufWriter::new(writer);
	w.write_all("\r".as_bytes())?;
	w.write_all(label.as_bytes())?;
	w.write_all(" [".as_bytes())?;
	for i in 0..20 {
		w.write_all(if (progress * 20) > (i * max) { "*".as_bytes() } else { " ".as_bytes() })?;
	}
	w.write_all("] ".as_bytes())?;

	w.write_all(progress.to_string().as_bytes())?;
	w.write_all("/".as_bytes())?;
	w.write_all(max.to_string().as_bytes())?;
	if progress == max {
		w.write_all(" Done!\n".as_bytes())?;
	} else if progress >= max {
		w.write_all(" Exceeded!! ".as_bytes())?;
	}
	w.flush()
}

fn compute_all_state_scores() -> Vec<PrecomputedResult> {
	let mut count = 0;
    let mut stdout = std::io::stdout().lock();
    let expected_count = expected_count();
    let mut results: Vec<PrecomputedResult> = PrecomputeGenerator::new()
    	 // those are trivial to compute so we don't need to save them!
    	.filter(|result| result.board().count_stickers() < 9 && result.board().count_stickers() > 0)
    	.map(move |result| { 
    		count += 1;
    		write_progress_bar(&mut stdout, "Computing...", count, expected_count);
    		result
    	})
    	.collect();

    println!("sorting...");
    results.sort_by_key(|result| result.0);

    results
}

fn write_precomputed_results(results: &[PrecomputedResult]) {
	let file = File::create("results.dat").unwrap();
	let mut writer = BufWriter::new(file);
    let mut count = 0;
    let mut stdout = std::io::stdout().lock();
    let expected_count = expected_count();
    for r in results {
    	count += 1;
    	write_progress_bar(&mut stdout, "Writing to file...", count, expected_count);
    	let bytes = r.to_bytes();
    	writer.write_all(&bytes);
    }
    writer.flush().unwrap();
}

fn main() {
    let results = compute_all_state_scores();
    write_precomputed_results(&results);
}
