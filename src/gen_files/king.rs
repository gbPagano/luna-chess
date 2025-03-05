use std::io::Write;

use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::file::File;
use crate::square::Square;

static mut KING_MOVES: [BitBoard; 64] = [BitBoard(0); 64];
static mut KINGSIDE_CASTLE_SQUARES: [BitBoard; 2] = [BitBoard(0); 2];
static mut QUEENSIDE_CASTLE_SQUARES: [BitBoard; 2] = [BitBoard(0); 2];

fn gen_kingside_castle_squares() {
    for color in [Color::White, Color::Black] {
        unsafe {
            KINGSIDE_CASTLE_SQUARES[color.to_index()] =
                BitBoard::set(color.starting_rank(), File::F)
                    ^ BitBoard::set(color.starting_rank(), File::G);
        }
    }
}

fn gen_queenside_castle_squares() {
    for color in [Color::White, Color::Black] {
        unsafe {
            QUEENSIDE_CASTLE_SQUARES[color.to_index()] =
                BitBoard::set(color.starting_rank(), File::B)
                    ^ BitBoard::set(color.starting_rank(), File::C)
                    ^ BitBoard::set(color.starting_rank(), File::D);
        }
    }
}

pub fn gen_king_moves() {
    for square in Square::all_squares() {
        unsafe {
            KING_MOVES[square.to_index()] = Square::all_squares()
                .filter(|dest| {
                    let src_rank = square.get_rank().to_index() as i8;
                    let src_file = square.get_file().to_index() as i8;
                    let dest_rank = dest.get_rank().to_index() as i8;
                    let dest_file = dest.get_file().to_index() as i8;

                    ((src_rank - dest_rank).abs() == 1 || (src_rank - dest_rank).abs() == 0)
                        && ((src_file - dest_file).abs() == 1 || (src_file - dest_file).abs() == 0)
                        && square != *dest
                })
                .fold(BitBoard(0), |b, s| b | BitBoard::from_square(s));
        }
    }

    gen_kingside_castle_squares();
    gen_queenside_castle_squares();
}

fn castle_squares() -> BitBoard {
    BitBoard::from_square("c1".parse().unwrap())
        ^ BitBoard::from_square("c8".parse().unwrap())
        ^ BitBoard::from_square("e1".parse().unwrap())
        ^ BitBoard::from_square("e8".parse().unwrap())
        ^ BitBoard::from_square("g1".parse().unwrap())
        ^ BitBoard::from_square("g8".parse().unwrap())
}

pub fn write_king_moves(f: &mut std::fs::File) -> std::io::Result<()> {
    writeln!(f, "const KING_MOVES: [BitBoard; 64] = [")?;
    for i in 0..64 {
        unsafe {
            writeln!(f, "    BitBoard({}),", KING_MOVES[i].0)?;
        }
    }
    writeln!(f, "];")?;

    writeln!(f, "pub const KINGSIDE_CASTLE_SQUARES: [BitBoard; 2] = [")?;
    unsafe {
        writeln!(
            f,
            " BitBoard({}), BitBoard({})];",
            KINGSIDE_CASTLE_SQUARES[0].0, KINGSIDE_CASTLE_SQUARES[1].0
        )?;
    };

    writeln!(f, "pub const QUEENSIDE_CASTLE_SQUARES: [BitBoard; 2] = [")?;
    unsafe {
        writeln!(
            f,
            " BitBoard({}), BitBoard({})];",
            QUEENSIDE_CASTLE_SQUARES[0].0, QUEENSIDE_CASTLE_SQUARES[1].0
        )?;
    };

    writeln!(
        f,
        "const CASTLE_SQUARES: BitBoard = BitBoard({});",
        castle_squares().0
    )?;

    Ok(())
}
