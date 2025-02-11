use luna_chess::board::Board;
use std::str::FromStr;

use luna_chess::color::Color;
use luna_chess::gen_moves::rays::*;
use luna_chess::gen_moves::magics::*;
use luna_chess::pieces::Piece;
use luna_chess::square::Square;
use luna_chess::bitboard::BitBoard;
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
    let (blockers, attack_masks) = gen_magic_attack_map(square,Piece::Rook);
    
    for (b, a) in blockers.iter().zip(attack_masks) {
        let mut b_board = Board::new();
        b_board.xor(*b, Piece::Pawn, Color::Black);
        
        let mut a_board = Board::new();
        a_board.xor(a, Piece::Rook, Color::White);
        a_board.xor(BitBoard::from_square(square), Piece::Rook, Color::White);
        
        println!("{:?}", b_board);
        println!("{:?}", a_board);
        println!("-----------");

    }

    
    Ok(())
}
