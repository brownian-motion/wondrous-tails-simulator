use wondrous_tails_simulator::*;
use wondrous_tails_simulator::stats::*;

fn generate_9_sticker_boards() -> impl Iterator<Item = stats::BoardState> {
	(0u16..0xFFFFu16).map(|d|BoardState::new(d)).filter(|b| b.count_stickers() == 9)
}

// used to compute the stats in crate::stats::shuffle_results
fn main() {
	let counter: BoardMatchCounter = generate_9_sticker_boards().collect();
	println!("{:?}", counter);
	println!("average: {:}", counter.average());
	println!("distribution: {:?}", counter.distribution());
}