use luna_chess::board::Board;
use std::str::FromStr;

use luna_chess::bitboard::BitBoard;
use luna_chess::color::Color;
use luna_chess::gen_files::attacks::*;
use luna_chess::gen_files::rays::*;
use luna_chess::pieces::Piece;
use luna_chess::square::Square;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //let board = Board::default();

    let board =
        Board::from_str("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ").unwrap();

    println!("{}", board);
    println!("");
    println!("{:?}", board);

    gen_rook_rays();
    gen_bishop_rays();

    let square: Square = "c3".parse()?;
    let piece = Piece::Rook;
    let (blockers, attack_masks) = gen_magic_attack_map(square, piece);

    for (b, a) in blockers.iter().zip(attack_masks) {
        let mut b_board = Board::new();
        b_board.xor(Piece::Pawn, *b, Color::Black);

        let mut a_board = Board::new();
        a_board.xor(piece, a, Color::White);
        a_board.xor(piece, BitBoard::from_square(square), Color::White);

        println!("{:?}", b_board);
        println!("{:?}", a_board);
        println!("-----------");
    }

    let square: Square = "a8".parse()?;
    dbg!(square);

    let square: Square = "e8".parse()?;
    dbg!(square);

    let square: Square = "h8".parse()?;
    dbg!(square);

    Ok(())
}
