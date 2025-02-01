use anyhow::{bail, Error};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CastleRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
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
