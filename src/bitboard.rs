use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;
use rand::Rng;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shr};

/// Represents a 64-bit bitboard, where each bit corresponds to a square on a chessboard.
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

impl Mul for BitBoard {
    type Output = BitBoard;

    fn mul(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(rhs.0))
    }
}

impl Shr<u8> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl BitBoard {
    pub fn new(val: u64) -> BitBoard {
        BitBoard(val)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Converts a `Square` into a `BitBoard` with a single bit set corresponding to the square.
    pub fn from_square(tile: Square) -> BitBoard {
        BitBoard(1u64 << tile.to_index())
    }

    /// Creates a `BitBoard` from a specific rank and file.
    pub fn set(rank: Rank, file: File) -> BitBoard {
        BitBoard::from_square(Square::new(rank, file))
    }

    /// Returns a `Vec<Square>` containing all the squares that are set in the `BitBoard`.
    pub fn get_squares(&self) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut bb = self.0;

        while bb != 0 {
            let idx = bb.trailing_zeros() as u8;
            squares.push(Square::from_index(idx));
            bb &= bb - 1;
        }

        squares
    }

    /// Generates a random `BitBoard` by combining random `u64` values.
    pub fn random<R: Rng>(rng: &mut R) -> BitBoard {
        BitBoard::new(rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>())
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
