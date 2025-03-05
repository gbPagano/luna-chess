use crate::board::Board;
use crate::file::File;
use crate::movegen::MoveGen;
use crate::pieces::Piece;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ChessMove {
    pub source: Square,
    pub dest: Square,
    pub promotion: Option<Piece>,
}
