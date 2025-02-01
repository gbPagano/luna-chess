use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitBoard {
    pub fn new(val: u64) -> BitBoard {
        BitBoard(val)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn from_square(tile: Square) -> BitBoard {
        BitBoard(1u64 << tile.to_index())
    }

    pub fn set(rank: Rank, file: File) -> BitBoard {
        BitBoard::from_square(Square::new(rank, file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_bitwise_ops() {
        let a = BitBoard(0b1100);
        let b = BitBoard(0b1010);

        assert_eq!(a & b, BitBoard(0b1000));
        assert_eq!(a | b, BitBoard(0b1110));
        assert_eq!(a ^ b, BitBoard(0b0110));
        assert_eq!(!a, BitBoard(!0b1100));
    }

    #[test]
    fn test_bitboard_from_square() {
        let tile = Square::new(Rank::First, File::H);
        assert_eq!(BitBoard::from_square(tile), BitBoard::new(0b10000000));
    }
}
