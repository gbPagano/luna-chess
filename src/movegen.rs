use super::piece_moves::*;
use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::pieces::Piece;
use crate::square::Square;

#[derive(Copy, Clone)]
pub struct BitBoardMove {
    square: Square,
    bitboard: BitBoard,
    promotion: bool,
}

impl BitBoardMove {
    pub fn new(square: Square, bitboard: BitBoard, promotion: bool) -> Self {
        Self {
            square,
            bitboard,
            promotion,
        }
    }
}

pub type MoveList = Vec<BitBoardMove>;

pub struct MoveGen {
    moves: MoveList,
    promotion_idx: usize,
    iter_mask: BitBoard,
    idx: usize,
}

impl MoveGen {
    fn enumerate_moves(board: &Board) -> MoveList {
        let checkers = board.get_checkers_bitboard();
        let mask = !board.get_color_bitboard(board.side_to_move());
        let mut movelist: MoveList = Vec::new();

        if checkers.is_empty() {
            PawnMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
            KnightMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
            BishopMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
            RookMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
            QueenMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
        //    KingMoves::legals::<NotInCheck>(&mut movelist, &board, mask);
        } else if checkers.0.count_ones() == 1 {
            PawnMoves::legals::<InCheck>(&mut movelist, &board, mask);
            KnightMoves::legals::<InCheck>(&mut movelist, &board, mask);
            BishopMoves::legals::<InCheck>(&mut movelist, &board, mask);
            RookMoves::legals::<InCheck>(&mut movelist, &board, mask);
            QueenMoves::legals::<InCheck>(&mut movelist, &board, mask);
        //    KingMoves::legals::<InCheck>(&mut movelist, &board, mask);
        } else {
            //    KingMoves::legals::<InCheck>(&mut movelist, &board, mask);
        }
        //
        movelist
    }
}
