use std::fs::File;
use std::io::Write;

use crate::bitboard::BitBoard;
use crate::square::Square;

static mut KNIGHT_MOVES: [BitBoard; 64] = [BitBoard(0); 64];

pub fn gen_knight_moves() {
    for square in Square::all_squares() {
        unsafe {
            KNIGHT_MOVES[square.to_index()] = Square::all_squares()
                .filter(|dest| {
                    let src_rank = square.get_rank().to_index() as i8;
                    let src_file = square.get_file().to_index() as i8;
                    let dest_rank = dest.get_rank().to_index() as i8;
                    let dest_file = dest.get_file().to_index() as i8;

                    ((src_rank - dest_rank).abs() == 2 && (src_file - dest_file).abs() == 1)
                        || ((src_rank - dest_rank).abs() == 1 && (src_file - dest_file).abs() == 2)
                })
                .fold(BitBoard(0), |b, s| b | BitBoard::from_square(s));
        }
    }
}

pub fn write_knight_moves(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "const KNIGHT_MOVES: [BitBoard; 64] = [")?;
    for i in 0..64 {
        unsafe {
            writeln!(f, "    BitBoard({}),", KNIGHT_MOVES[i].0)?;
        }
    }
    writeln!(f, "];")?;
    Ok(())
}
