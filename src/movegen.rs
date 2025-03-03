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
            //    PawnMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
            //    KnightMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
            BishopMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
            RookMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
            QueenMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
        //    KingMoves::legals(&mut movelist, &board, mask, CheckStatus::NotInCheck);
        } else if checkers.0.count_ones() == 1 {
            //    PawnMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
            //    KnightMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
            BishopMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
            RookMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
            QueenMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
        //    KingMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
        } else {
            //    KingMoves::legals(&mut movelist, &board, mask, CheckStatus::InCheck);
        }
        //
        movelist
    }
}
