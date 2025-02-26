use std::fs::File;
use std::io::Write;

use super::attacks::gen_magic_attack_map;
use super::rays::*;
use crate::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::square::Square;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub fn magic_mask(square: Square, piece: Piece) -> BitBoard {
    get_rays(square, piece)
        & !Square::all_squares()
            .filter(|other| match piece {
                Piece::Bishop => other.is_edge(),
                Piece::Rook => {
                    (square.get_rank() == other.get_rank() && (other.get_file().is_edge()))
                        || (square.get_file() == other.get_file() && (other.get_rank().is_edge()))
                }
                _ => panic!("Magic only for Rooks and Bishops"),
            })
            .fold(BitBoard(0), |b, s| b | BitBoard::from_square(s))
}

#[derive(Copy, Clone)]
struct Magic {
    magic_number: BitBoard,
    mask: BitBoard,
    offset: u32,
    rightshift: u8,
}

static mut MAGIC_NUMBERS: [[Magic; 64]; 2] = [[Magic {
    magic_number: BitBoard(0),
    mask: BitBoard(0),
    offset: 0,
    rightshift: 0,
}; 64]; 2]; // for rooks and bishops

const NUM_MOVES: usize = 64 * (1<<12) /* Rook Moves */ +
                         64 * (1<<9) /* Bishop Moves */;
static mut GEN_MOVES_SIZE: usize = 0;
static mut MOVES: [BitBoard; NUM_MOVES] = [BitBoard(0); NUM_MOVES];
static mut MOVE_RAYS: [BitBoard; NUM_MOVES] = [BitBoard(0); NUM_MOVES];

fn generate_magic(square: Square, piece: Piece, curr_offset: usize) -> usize {
    let (blockers, attacks) = gen_magic_attack_map(square, piece);
    let mask = magic_mask(square, piece);

    let mut new_offset = curr_offset;

    for i in 0..curr_offset {
        let mut found = true;
        for j in 0..attacks.len() {
            unsafe {
                if MOVE_RAYS[i + j] & get_rays(square, piece) != BitBoard(0) {
                    found = false;
                    break;
                }
            }
        }
        if found {
            new_offset = i;
            break;
        }
    }

    let mut magic = Magic {
        magic_number: BitBoard(0),
        mask,
        offset: new_offset as u32,
        rightshift: (blockers.len().leading_zeros() + 1) as u8,
    };

    // TODO: tranform this into unittest
    debug_assert_eq!(blockers.len().count_ones(), 1);
    debug_assert_eq!(blockers.len(), attacks.len());

    debug_assert_eq!(blockers.iter().fold(BitBoard(0), |b, n| b | *n), mask);
    debug_assert_eq!(
        attacks.iter().fold(BitBoard(0), |b, n| b | *n),
        get_rays(square, piece)
    );

    let mut rng = SmallRng::from_os_rng();

    let mut done = false;
    while !done {
        let magic_number = BitBoard::random(&mut rng);

        //if (mask * magic_number).0.count_ones() < 6 {
        //    continue;
        //}
        done = true;

        let mut new_attacks = vec![BitBoard(0); blockers.len()];
        for (i, &blocker) in blockers.iter().enumerate() {
            let j = ((magic_number * blocker) >> magic.rightshift).0 as usize;
            if new_attacks[j] == BitBoard(0) || new_attacks[j] == attacks[i] {
                new_attacks[j] = new_attacks[i];
            } else {
                done = false;
                break;
            }
        }

        if done {
            magic.magic_number = magic_number;
        }
    }

    unsafe {
        MAGIC_NUMBERS[if piece == Piece::Rook { 0 } else { 1 }][square.to_index()] = magic;

        for (i, &blocker) in blockers.iter().enumerate() {
            let j = ((magic.magic_number * blocker) >> magic.rightshift).0 as usize;
            MOVES[magic.offset as usize + j] |= attacks[i];
            MOVE_RAYS[magic.offset as usize + j] |= get_rays(square, piece);
        }
    }
    let next_offset = if new_offset + attacks.len() < curr_offset {
        curr_offset
    } else {
        new_offset + attacks.len()
    };

    next_offset
}

pub fn gen_all_magic() {
    let mut offset = 0;
    for piece in [Piece::Rook, Piece::Bishop].iter() {
        for square in Square::all_squares() {
            offset = generate_magic(square, *piece, offset);
        }
    }
    unsafe {
        GEN_MOVES_SIZE = offset;
    }
    dbg!(offset);
}

pub fn write_magics(f: &mut File) {
    let magic_struct = format!(
        r#"#[derive(Copy, Clone)]
struct Magic {{
    magic_number: BitBoard,
    mask: BitBoard,
    offset: u32,
    rightshift: u8
}}
"#
    );
    writeln!(f, "{}", magic_struct).unwrap();

    writeln!(f, "const MAGIC_NUMBERS: [[Magic; 64]; 2] = [[").unwrap();
    for i in 0..2 {
        for j in 0..64 {
            unsafe {
                writeln!(f, "    Magic {{ magic_number: BitBoard({}), mask: BitBoard({}), offset: {}, rightshift: {} }},",
                    MAGIC_NUMBERS[i][j].magic_number.0,
                    MAGIC_NUMBERS[i][j].mask.0,
                    MAGIC_NUMBERS[i][j].offset,
                    MAGIC_NUMBERS[i][j].rightshift).unwrap();
            }
        }
        if i != 1 {
            writeln!(f, "], [").unwrap();
        }
    }
    writeln!(f, "]];").unwrap();
    
    unsafe {
        writeln!(f, "const MOVES: [BitBoard; {}] = [", GEN_MOVES_SIZE).unwrap();
        for i in 0..GEN_MOVES_SIZE {
            writeln!(f, "    BitBoard({}),", MOVES[i].0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

#[test]
fn name() {
    gen_rook_rays();
    gen_bishop_rays();

    //find_magic("c3".parse().unwrap(), Piece::Rook, 0);

    gen_all_magic();

    assert!(false);
}
