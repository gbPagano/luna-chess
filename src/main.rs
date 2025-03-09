use luna_chess::board::Board;
use luna_chess::movegen::MoveGen;
use std::time::Instant;

fn main() {
    let board = Board::default();
    let depth = 6;

    let start = Instant::now();
    let _ = MoveGen::perft_test(&board, depth);
    let duration = start.elapsed();
    println!("Perft {depth} in: {:?}", duration);
}
