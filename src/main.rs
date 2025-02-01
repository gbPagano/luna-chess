use std::str::FromStr;

struct Board {
    white_pieces: [u64; 6],
    black_pieces: [u64; 6],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            white_pieces: [0; 6],
            black_pieces: [0; 6],
        }
    }
}
impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for c in s.chars() {}
        todo!()
    }
}

fn main() {
    println!("Hello, world! ♞");

    //let pw = Piece::Pawn(Color::White);
    //let pb = Piece::Pawn(Color::Black);

    let b = Board::default();

    //let new_b = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    //println!("{pw} {pb}");
    //dbg!(pw as u32);
    //dbg!(pb as u32);
    let a = 1usize;
    dbg!(a - 1);
    dbg!(a.wrapping_sub(2) & 7);
}
