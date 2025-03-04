use anyhow::{bail, Error};
use std::fmt;
use std::str::FromStr;

use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::magic::{KINGSIDE_CASTLE_SQUARES, QUEENSIDE_CASTLE_SQUARES};

/// A struct representing the castling rights for both players in chess.
/// It tracks whether each player has kingside and queenside castling rights.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CastleRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastleRights {
    pub fn has_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_kingside,
            Color::Black => self.black_kingside,
        }
    }

    pub fn has_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_queenside,
            Color::Black => self.black_queenside,
        }
    }

    pub fn kingside_squares(&self, color: Color) -> BitBoard {
        unsafe { *KINGSIDE_CASTLE_SQUARES.get_unchecked(color.to_index()) }
    }

    pub fn queenside_squares(&self, color: Color) -> BitBoard {
        unsafe { *QUEENSIDE_CASTLE_SQUARES.get_unchecked(color.to_index()) }
    }
}

impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        if *self == CastleRights::default() {
            result = "-".to_string();
        } else {
            if self.white_kingside {
                result.push('K');
            }

            if self.white_queenside {
                result.push('Q');
            }
            if self.black_kingside {
                result.push('k');
            }
            if self.black_queenside {
                result.push('q');
            }
        }
        write!(f, "{}", result)
    }
}

impl FromStr for CastleRights {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rights = Self::default();

        match s {
            "-" => return Ok(rights),
            s if s.len() > 4 => bail!("error"),
            _ => {}
        }

        for c in s.chars() {
            match c {
                'K' => rights.white_kingside = true,
                'Q' => rights.white_queenside = true,
                'k' => rights.black_kingside = true,
                'q' => rights.black_queenside = true,
                _ => bail!("error"),
            }
        }

        Ok(rights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castle_rights_from_str() {
        let mut rights = CastleRights::default();
        assert_eq!(CastleRights::from_str("").unwrap(), rights);
        assert_eq!(CastleRights::from_str("-").unwrap(), rights);
        rights.white_queenside = true;
        rights.black_kingside = true;
        assert_eq!(CastleRights::from_str("Qk").unwrap(), rights);
        assert!(CastleRights::from_str("abc").is_err());
    }

    #[test]
    fn test_castle_rights_fmt() {
        let mut rights = CastleRights::default();
        assert_eq!(format!("{}", rights), "-");
        rights.white_queenside = true;
        rights.black_kingside = true;
        assert_eq!(format!("{}", rights), "Qk");
    }
}
