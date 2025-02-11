use crate::file::File;
use crate::rank::Rank;
use anyhow::{bail, Error};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    pub fn up(&self) -> Option<Square> {
        if self.get_rank() == Rank::Eighth {
            None
        } else {
            Some(Square::new(self.get_rank().up(), self.get_file()))
        }
    }

    pub fn down(&self) -> Option<Square> {
        if self.get_rank() == Rank::First {
            None
        } else {
            Some(Square::new(self.get_rank().down(), self.get_file()))
        }
    }

    pub fn left(&self) -> Option<Square> {
        if self.get_file() == File::A {
            None
        } else {
            Some(Square::new(self.get_rank(), self.get_file().left()))
        }
    }

    pub fn right(&self) -> Option<Square> {
        if self.get_file() == File::H {
            None
        } else {
            Some(Square::new(self.get_rank(), self.get_file().right()))
        }
    }

    pub fn is_edge(&self) -> bool {
        self.get_file().is_edge() || self.get_rank().is_edge()
    }

    pub fn all_squares() -> impl Iterator<Item = Square> {
        (0..64).map(Square::from_index)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new(Rank::First, File::A)
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            bail!("error");
        }

        let mut chars = s.chars();
        let file_char = chars.next().unwrap();
        let rank_char = chars.next().unwrap();

        let file = match file_char {
            'a'..='h' => File::from_index(file_char as usize - b'a' as usize),
            _ => bail!("error"),
        };

        let rank = match rank_char.to_digit(10) {
            Some(n @ 1..=8) => Rank::from_index((n - 1) as usize),
            _ => bail!("error"),
        };

        Ok(Square::new(rank, file))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + ((self.0 & 7) as u8)) as char,
            (('1' as u8) + ((self.0 >> 3) as u8)) as char
        )
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

    #[test]
    fn test_rank_from_str() {
        assert_eq!(
            Square::from_str("a1").unwrap(),
            Square::new(Rank::First, File::A)
        );
        assert_eq!(
            Square::from_str("e3").unwrap(),
            Square::new(Rank::Third, File::E)
        );
    }

    #[test]
    fn test_rank_fmt() {
        assert_eq!(format!("{}", Square::new(Rank::First, File::A)), "a1");
        assert_eq!(format!("{}", Square::new(Rank::Third, File::E)), "e3");
    }
}
