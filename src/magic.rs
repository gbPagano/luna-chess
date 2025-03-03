use crate::bitboard::BitBoard;
use crate::square::Square;

include!(concat!(env!("OUT_DIR"), "/magic_file.rs"));

pub fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic: Magic = *MAGIC_NUMBERS
            .get_unchecked(0) // rook index
            .get_unchecked(square.to_index());
        *MOVES.get_unchecked(
            (magic.offset as usize)
                + ((magic.magic_number * (blockers & magic.mask)) >> magic.rightshift).0 as usize,
        ) & get_rook_rays(square)
    }
}

pub fn get_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic: Magic = *MAGIC_NUMBERS
            .get_unchecked(1) // bishop index
            .get_unchecked(square.to_index());
        *MOVES.get_unchecked(
            (magic.offset as usize)
                + ((magic.magic_number * (blockers & magic.mask)) >> magic.rightshift).0 as usize,
        ) & get_bishop_rays(square)
    }
}

pub fn get_bishop_rays(square: Square) -> BitBoard {
    unsafe { *BISHOP_RAYS.get_unchecked(square.to_index()) }
}

pub fn get_rook_rays(square: Square) -> BitBoard {
    unsafe { *ROOK_RAYS.get_unchecked(square.to_index()) }
}

pub fn get_line(sq_1: Square, sq_2: Square) -> BitBoard {
    unsafe {
        *LINES
            .get_unchecked(sq_1.to_index())
            .get_unchecked(sq_2.to_index())
    }
}

pub fn get_between(sq_1: Square, sq_2: Square) -> BitBoard {
    unsafe {
        *BETWEEN
            .get_unchecked(sq_1.to_index())
            .get_unchecked(sq_2.to_index())
    }
}
