use luna_chess::board::Board;

fn main() {
    let board = Board::default();

    println!("{}", board);
    println!("");
    println!("{:?}", board);
}
