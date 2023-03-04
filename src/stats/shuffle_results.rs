pub const SHUFFLED_BOARD_COUNTS : [u32;4] = [4752, 5480, 1184, 24];
pub const SHUFFLED_BOARD_TOTAL_MATCHES : u32 = 7920;
pub const SHUFFLED_BOARD_COUNT : u32 = 11440;

pub fn percentage_shuffled_boards_with_matches(num_matches: usize) -> f32 {
	(SHUFFLED_BOARD_COUNTS[num_matches] as f32) / (SHUFFLED_BOARD_COUNT as f32)
}