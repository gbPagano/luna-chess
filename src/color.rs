use crate::rank::Rank;
use std::ops::Not;

/// Enum representing the two colors in chess
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the index [0-1] of the color as a `usize`.
    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn promotion_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Eighth,
            Color::Black => Rank::First,
        }
    }

    pub fn pre_promotion_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Seventh,
            Color::Black => Rank::Second,
        }
    }

    pub fn starting_rank(&self) -> Rank {
        match self {
            Color::White => Rank::First,
            Color::Black => Rank::Eighth,
        }
    }
}
impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
