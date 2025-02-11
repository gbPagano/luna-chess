#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub const ALL_FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    pub fn from_index(i: usize) -> Self {
        match i & 7 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn left(&self) -> Self {
        Self::from_index(self.to_index().wrapping_sub(1))
    }

    pub fn right(&self) -> Self {
        Self::from_index(self.to_index() + 1)
    }

    pub fn is_edge(&self) -> bool {
        *self == File::A || *self == File::H
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_from_index() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(7), File::H);
        assert_eq!(File::from_index(10), File::C);
        assert_eq!(File::from_index(usize::MAX), File::H);
    }

    #[test]
    fn test_file_to_index() {
        assert_eq!(0, File::A.to_index());
        assert_eq!(7, File::H.to_index());
        assert_eq!(2, File::C.to_index());
    }

    #[test]
    fn test_file_right_left() {
        assert_eq!(File::A.right(), File::B);
        assert_eq!(File::A.left(), File::H);
        assert_eq!(File::H.right(), File::A);
    }
}
