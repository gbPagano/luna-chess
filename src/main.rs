use luna_chess::board::Board;
use std::str::FromStr;

use luna_chess::bitboard::BitBoard;
use luna_chess::color::Color;
use luna_chess::movegen::MoveGen;
use luna_chess::pieces::Piece;
use luna_chess::square::Square;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut board: Board = "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".parse()?;
    println!("{:?}", board);

    for m in MoveGen::new_legal(&board) {
        println!("{:?}", m);
    }

    Ok(())
}
