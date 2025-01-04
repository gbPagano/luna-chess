use crate::file::File;
use crate::rank::Rank;

#[derive(Debug, PartialEq, Eq)]
pub struct Square(u8);
impl Square {
    pub fn new(rank: Rank, file: File) -> Self {
        Square((rank.to_index() << 3 ^ file.to_index()) as u8)
    }

    pub fn from_index(idx: u8) -> Self {
        Square(idx & 63)
    }

    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    pub fn get_rank(&self) -> Rank {
        Rank::from_index((self.0 >> 3) as usize)
    }

    pub fn get_file(&self) -> File {
        File::from_index((self.0 & 7) as usize)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new(Rank::First, File::A)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_square() {
        assert_eq!(Square::new(Rank::First, File::A), Square::from_index(0));
        assert_eq!(Square::new(Rank::Third, File::C), Square::from_index(18));
        assert_eq!(Square::new(Rank::Seventh, File::G), Square::from_index(54));
    }

    #[test]
    fn test_rank_and_file_from_square() {
        assert_eq!(Square::new(Rank::First, File::A).get_rank(), Rank::First);
        assert_eq!(Square::new(Rank::Seventh, File::G).get_file(), File::G);
    }
}
