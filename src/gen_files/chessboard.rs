use std::fs;
use std::io::Write;

use crate::bitboard::BitBoard;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

static mut RANKS: [BitBoard; 8] = [BitBoard(0); 8];
static mut FILES: [BitBoard; 8] = [BitBoard(0); 8];
static mut ADJACENT_FILES: [BitBoard; 8] = [BitBoard(0); 8];
static mut EDGES: BitBoard = BitBoard(0);

pub fn gen_chessboard_utils() {
    unsafe {
        EDGES = Square::all_squares()
            .filter(|x| {
                x.get_rank() == Rank::First
                    || x.get_rank() == Rank::Eighth
                    || x.get_file() == File::A
                    || x.get_file() == File::H
            })
            .fold(BitBoard(0), |v, s| v | BitBoard::from_square(s));
        for i in 0..8 {
            RANKS[i] = Square::all_squares()
                .filter(|x| x.get_rank().to_index() == i)
                .fold(BitBoard(0), |v, s| v | BitBoard::from_square(s));
            FILES[i] = Square::all_squares()
                .filter(|x| x.get_file().to_index() == i)
                .fold(BitBoard(0), |v, s| v | BitBoard::from_square(s));
            ADJACENT_FILES[i] = Square::all_squares()
                .filter(|y| {
                    ((y.get_file().to_index() as i8) == (i as i8) - 1)
                        || ((y.get_file().to_index() as i8) == (i as i8) + 1)
                })
                .fold(BitBoard(0), |v, s| v | BitBoard::from_square(s));
        }
    }
}

pub fn write_chessboard_utils(f: &mut fs::File) -> std::io::Result<()> {
    unsafe {
        writeln!(f, "const FILES: [BitBoard; 8] = [")?;
        for i in 0..8 {
            writeln!(f, "    BitBoard({}),", FILES[i].0)?;
        }
        writeln!(f, "];\n")?;
        writeln!(f, "const ADJACENT_FILES: [BitBoard; 8] = [")?;
        for i in 0..8 {
            writeln!(f, "    BitBoard({}),", ADJACENT_FILES[i].0)?;
        }
        writeln!(f, "];\n")?;
        writeln!(f, "const RANKS: [BitBoard; 8] = [")?;
        for i in 0..8 {
            writeln!(f, "    BitBoard({}),", RANKS[i].0)?;
        }
        writeln!(f, "];")?;
        writeln!(f, "pub const EDGES: BitBoard = BitBoard({});", EDGES.0)?;
    }
    Ok(())
}
