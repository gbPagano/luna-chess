#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

pub const ALL_RANKS: [Rank; 8] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth,
];

impl Rank {
    pub fn from_index(i: usize) -> Self {
        match i & 7 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn down(&self) -> Self {
        Self::from_index(self.to_index().wrapping_sub(1))
    }

    pub fn up(&self) -> Self {
        Self::from_index(self.to_index() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_from_index() {
        assert_eq!(Rank::from_index(0), Rank::First);
        assert_eq!(Rank::from_index(7), Rank::Eighth);
        assert_eq!(Rank::from_index(10), Rank::Third);
        assert_eq!(Rank::from_index(usize::MAX), Rank::Eighth);
    }

    #[test]
    fn test_rank_to_index() {
        assert_eq!(0, Rank::First.to_index());
        assert_eq!(7, Rank::Eighth.to_index());
        assert_eq!(2, Rank::Third.to_index());
    }

    #[test]
    fn test_rank_up_down() {
        assert_eq!(Rank::First.up(), Rank::Second);
        assert_eq!(Rank::First.down(), Rank::Eighth);
        assert_eq!(Rank::Eighth.up(), Rank::First);
    }
}
