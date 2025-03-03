use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::color::Color;
use crate::pieces::Piece;
use crate::square::Square;

use super::movegen::{BitBoardMove, MoveList};

use crate::magic::{get_between, get_bishop_moves, get_line, get_rook_moves};

#[derive(PartialEq)]
pub enum CheckStatus {
    InCheck,
    NotInCheck,
}

pub trait AsPiece {
    const PIECE: Piece;
}

pub trait PieceMoves: AsPiece {
    fn pseudo_legals(square: Square, color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard;

    fn legals(movelist: &mut MoveList, board: &Board, mask: BitBoard, status: CheckStatus) {
        let combined = board.get_combined_bitboard();
        let color = board.side_to_move();
        let my_pieces = board.get_color_bitboard(color);
        let king_square = board.get_king_square(color);

        let pieces = board.get_piece_bitboard(Self::PIECE) & my_pieces;
        let pinned = board.get_pinned_bitboard();
        let checkers = board.get_checkers_bitboard();

        let check_mask = match status {
            CheckStatus::InCheck => get_between(checkers.to_square(), king_square) ^ checkers,
            CheckStatus::NotInCheck => !BitBoard(0), // full bitboard
        };

        for square in (pieces & !pinned).get_squares() {
            let moves = Self::pseudo_legals(square, color, combined, mask) & check_mask;
            if !moves.is_empty() {
                movelist.push(BitBoardMove::new(square, moves, false));
            }
        }

        if status == CheckStatus::InCheck {
            for square in (pieces & pinned).get_squares() {
                let moves = Self::pseudo_legals(square, color, combined, mask)
                    & get_line(square, king_square);
                if !moves.is_empty() {
                    movelist.push(BitBoardMove::new(square, moves, false));
                }
            }
        }
    }
}

pub struct RookMoves;
impl PieceMoves for RookMoves {
    fn pseudo_legals(sq: Square, _: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        get_rook_moves(sq, combined) & mask
    }
}
impl AsPiece for RookMoves {
    const PIECE: Piece = Piece::Rook;
}

pub struct BishopMoves;
impl PieceMoves for BishopMoves {
    fn pseudo_legals(sq: Square, _: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        get_bishop_moves(sq, combined) & mask
    }
}
impl AsPiece for BishopMoves {
    const PIECE: Piece = Piece::Bishop;
}

pub struct QueenMoves;
impl PieceMoves for QueenMoves {
    fn pseudo_legals(sq: Square, _: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        (get_rook_moves(sq, combined) ^ get_bishop_moves(sq, combined)) & mask
    }
}
impl AsPiece for QueenMoves {
    const PIECE: Piece = Piece::Queen;
}
