use luna_chess::board::Board;
use std::str::FromStr;

fn main() {
    //let board = Board::default();
    
    let board = Board::from_str("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ").unwrap();

    println!("{}", board);
    println!("");
    println!("{:?}", board);
}
