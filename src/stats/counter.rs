use super::board::*;
use core::iter::Sum;
use core::ops::Add;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct BoardMatchCounter {
    counts: [u32; 4],
    total: u32,
    count: u32,
}

impl BoardMatchCounter {
    pub const fn empty() -> Self {
        BoardMatchCounter {
            counts: [0; 4],
            total: 0,
            count: 0,
        }
    }

    pub const fn from_counts(counts: [u32; 4]) -> Self {
        let count = counts[0] + counts[1] + counts[2] + counts[3];
        let total = counts[1] + counts[2] * 2 + counts[3] * 3;
        BoardMatchCounter {
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

    pub fn average(&self) -> f32 {
        (self.total as f32) / (self.count as f32)
    }

    pub fn distribution(&self) -> Option<[f32; 4]> {
        if self.count == 0 {
            None
        } else {
            Some([
                (self.counts[0] as f32) / (self.count as f32),
                (self.counts[1] as f32) / (self.count as f32),
                (self.counts[2] as f32) / (self.count as f32),
                (self.counts[3] as f32) / (self.count as f32),
            ])
        }
    }

    pub const fn num_boards(&self) -> u32 {
        self.count
    }

    pub const fn total_matches(&self) -> u32 {
        self.total
    }

    pub const fn match_counts(&self) -> [u32; 4] {
        self.counts
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl FromIterator<BoardState> for BoardMatchCounter {
    fn from_iter<I: IntoIterator<Item = BoardState>>(iter: I) -> Self {
        let mut counter = BoardMatchCounter::default();
        for board in iter {
            counter.count(board);
        }

        counter
    }
}

impl FromIterator<BoardMatchCounter> for BoardMatchCounter {
    fn from_iter<I: IntoIterator<Item = BoardMatchCounter>>(iter: I) -> Self {
        iter.into_iter().fold(BoardMatchCounter::empty(), |a,b| a+b)
    }
}


impl Add for BoardMatchCounter {
    type Output = BoardMatchCounter;

    fn add(self, other: Self) -> Self {
        BoardMatchCounter {
            counts: [
                self.counts[0] + other.counts[0],
                self.counts[1] + other.counts[1],
                self.counts[2] + other.counts[2],
                self.counts[3] + other.counts[3],
            ],
            total: self.total + other.total,
            count: self.count + other.count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_boards_with_9_stickers() {
        use crate::stats::shuffle_results::*;

        let board_counter: BoardMatchCounter = (0..0xFFFFu16)
            .map(|state| BoardState::new(state))
            .filter(|board| board.count_stickers() == 9)
            .collect();

        assert_eq!(SHUFFLED_BOARD_COUNTS, board_counter.match_counts());
        assert_eq!(SHUFFLED_BOARD_COUNT, board_counter.num_boards());
        assert_eq!(SHUFFLED_BOARD_TOTAL_MATCHES, board_counter.total_matches());
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            BoardMatchCounter::empty(),
            BoardMatchCounter::empty() + BoardMatchCounter::empty()
        );
        assert_eq!(
            BoardMatchCounter::from_counts([1, 3, 3, 7]),
            BoardMatchCounter::from_counts([0, 2, 1, 6])
                + BoardMatchCounter::from_counts([1, 1, 2, 1])
        )
    }
}
