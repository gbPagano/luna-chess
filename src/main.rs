use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}
//impl fmt::Display for Piece {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            Piece::Pawn(Color::White) => write!(f, "♙"),
//            Piece::Knight(Color::White) => write!(f, "♘"),
//            Piece::Bishop(Color::White) => write!(f, "♗"),
//            Piece::Rook(Color::White) => write!(f, "♖"),
//            Piece::Queen(Color::White) => write!(f, "♕"),
//            Piece::King(Color::White) => write!(f, "♔"),
//            Piece::Pawn(Color::Black) => write!(f, "♟︎"),
//            Piece::Knight(Color::Black) => write!(f, "♞"),
//            Piece::Bishop(Color::Black) => write!(f, "♝"),
//            Piece::Rook(Color::Black) => write!(f, "♜"),
//            Piece::Queen(Color::Black) => write!(f, "♛"),
//            Piece::King(Color::Black) => write!(f, "♚"),
//        }
//    }
//}

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
        for c in s.chars() {

            
        }
        todo!()
    }
}


fn main() {
    println!("Hello, world! ♞");

    let pw = Piece::Pawn(Color::White);
    let pb = Piece::Pawn(Color::Black);

    let b = Board::default();

    //let new_b = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    //println!("{pw} {pb}");
    //dbg!(pw as u32);
    //dbg!(pb as u32);
    let a= 1usize;
    dbg!(a - 1);
    dbg!(a.wrapping_sub(2) & 7);
}
