/// Enum representing the two colors in chess
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the index [0-1] of the color as a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}
