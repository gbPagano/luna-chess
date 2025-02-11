use super::rays::*;
use crate::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::square::Square;

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

pub fn gen_blocker_combinations(mask: BitBoard) -> Vec<BitBoard> {
    let mut result = vec![];
    let squares = mask.get_squares();

    for i in 0..(1u64 << squares.len()) {
        let mut current = BitBoard(0);
        for j in 0..squares.len() {
            if (i & (1u64 << j)) == (1u64 << j) {
                current |= BitBoard::from_square(squares[j as usize]);
            }
        }
        result.push(current);
    }

    result
}

pub fn gen_magic_attack_map(square: Square, piece: Piece) -> (Vec<BitBoard>, Vec<BitBoard>) {
    let occupancy_mask = magic_mask(square, piece);
    let blockers_combinations = gen_blocker_combinations(occupancy_mask);
    let mut attack_map = Vec::new();

    let directions: Vec<fn(Square) -> Option<_>> = match piece {
        Piece::Rook => vec![|s| s.left(), |s| s.right(), |s| s.up(), |s| s.down()],
        Piece::Bishop => vec![
            |s| s.left().map_or(None, |s| s.up()),
            |s| s.right().map_or(None, |s| s.up()),
            |s| s.left().map_or(None, |s| s.down()),
            |s| s.right().map_or(None, |s| s.down()),
        ],
        _ => panic!("Magic only for Rooks and Bishops"),
    };

    for blockers in blockers_combinations.iter() {
        let mut attack_mask = BitBoard(0);
        for dir in directions.iter() {
            let mut next_square = dir(square);
            while let Some(curr_sq) = next_square {
                attack_mask ^= BitBoard::from_square(curr_sq);
                if (BitBoard::from_square(curr_sq) & *blockers) != BitBoard(0) {
                    break;
                }
                next_square = dir(curr_sq);
            }
        }
        attack_map.push(attack_mask);
    }

    (blockers_combinations, attack_map)
}
