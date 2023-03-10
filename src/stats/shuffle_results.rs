use super::counter::BoardMatchCounter;

pub const SHUFFLED_BOARD_COUNTS: [u32; 4] = [4752, 5480, 1184, 24];
pub const SHUFFLED_BOARD_TOTAL_MATCHES: u32 = 7920;
pub const SHUFFLED_BOARD_COUNT: u32 = 11440;
pub const SHUFFLED_BOARD_STATS: BoardMatchCounter =
    BoardMatchCounter::from_counts(SHUFFLED_BOARD_COUNTS);

pub fn percentage_shuffled_boards_with_matches(num_matches: usize) -> f32 {
    (SHUFFLED_BOARD_COUNTS[num_matches] as f32) / (SHUFFLED_BOARD_COUNT as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_constants_are_equal() {
        assert_eq!(SHUFFLED_BOARD_COUNT, SHUFFLED_BOARD_STATS.num_boards());
        assert_eq!(
            SHUFFLED_BOARD_TOTAL_MATCHES,
            SHUFFLED_BOARD_STATS.total_matches()
        );
        assert_eq!(SHUFFLED_BOARD_COUNTS, SHUFFLED_BOARD_STATS.match_counts());
    }
}
