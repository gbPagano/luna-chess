use std::fs::File;
use std::io::Write;

use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::file::ALL_FILES;
use crate::rank::Rank;
use crate::square::Square;

static mut PAWN_MOVES: [[BitBoard; 64]; 2] = [[BitBoard(0); 64]; 2];
static mut PAWN_ATTACKS: [[BitBoard; 64]; 2] = [[BitBoard(0); 64]; 2];

pub fn gen_pawn_moves() {
    for color in [Color::White, Color::Black] {
        for square in Square::all_squares() {
            unsafe {
                if square.get_rank() == color.starting_rank().forward(color) {
                    PAWN_MOVES[color.to_index()][square.to_index()] =
                        BitBoard::from_square(square.forward(color).unwrap())
                            ^ BitBoard::from_square(
                                square.forward(color).unwrap().forward(color).unwrap(),
                            );
                } else {
                    match square.forward(color) {
                        None => PAWN_MOVES[color.to_index()][square.to_index()] = BitBoard(0),
                        Some(x) => {
                            PAWN_MOVES[color.to_index()][square.to_index()] =
                                BitBoard::from_square(x)
                        }
                    };
                }
            }
        }
    }
}

pub fn gen_pawn_attacks() {
    for color in [Color::White, Color::Black] {
        for square in Square::all_squares() {
            unsafe {
                PAWN_ATTACKS[color.to_index()][square.to_index()] = BitBoard(0);

                if let Some(f) = square.forward(color) {
                    if let Some(fl) = f.left() {
                        PAWN_ATTACKS[color.to_index()][square.to_index()] ^=
                            BitBoard::from_square(fl)
                    };
                    if let Some(fr) = f.right() {
                        PAWN_ATTACKS[color.to_index()][square.to_index()] ^=
                            BitBoard::from_square(fr)
                    };
                };
            }
        }
    }
}

fn source_double_moves() -> BitBoard {
    let mut result = BitBoard(0);
    for rank in [Rank::Second, Rank::Seventh] {
        for file in ALL_FILES.iter() {
            result ^= BitBoard::set(rank, *file);
        }
    }
    result
}

fn dest_double_moves() -> BitBoard {
    let mut result = BitBoard(0);
    for rank in [Rank::Fourth, Rank::Fifth] {
        for file in ALL_FILES.iter() {
            result ^= BitBoard::set(rank, *file);
        }
    }
    result
}

pub fn write_pawn_moves(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "const PAWN_MOVES: [[BitBoard; 64]; 2] = [[")?;
    for i in 0..2 {
        for j in 0..64 {
            unsafe {
                writeln!(f, "    BitBoard({}),", PAWN_MOVES[i][j].0)?;
            };
        }
        if i != 1 {
            writeln!(f, "  ], [")?;
        }
    }
    writeln!(f, "]];")?;

    Ok(())
}

pub fn write_pawn_attacks(f: &mut File) -> std::io::Result<()> {
    writeln!(f, "const PAWN_ATTACKS: [[BitBoard; 64]; 2] = [[")?;
    for i in 0..2 {
        for j in 0..64 {
            unsafe {
                writeln!(f, "    BitBoard({}),", PAWN_ATTACKS[i][j].0)?;
            };
        }
        if i != 1 {
            writeln!(f, "  ], [")?;
        }
    }
    writeln!(f, "]];")?;

    writeln!(
        f,
        "const PAWN_SOURCE_DOUBLE_MOVES: BitBoard = BitBoard({0});",
        source_double_moves().0
    )?;

    writeln!(
        f,
        "const PAWN_DEST_DOUBLE_MOVES: BitBoard = BitBoard({0});",
        dest_double_moves().0
    )?;

    Ok(())
}
