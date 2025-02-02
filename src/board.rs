use crate::bitboard::BitBoard;
use crate::castle_rights::CastleRights;
use crate::color::Color;
use crate::file::{File, ALL_FILES};
use crate::pieces::{Piece, ALL_PIECES};
use crate::rank::{Rank, ALL_RANKS};
use crate::square::Square;

use anyhow::{bail, Error};
use std::fmt;
use std::str::FromStr;

pub struct Board {
    pieces_bitboards: [BitBoard; 6],
    colors_bitboards: [BitBoard; 2],
    combined_bitboard: BitBoard,
    side_to_move: Color,
    en_passant: Option<Square>,
    castle_rights: CastleRights,
}
impl Board {
    fn new() -> Self {
        Self {
            pieces_bitboards: [BitBoard::new(0); 6],
            colors_bitboards: [BitBoard::new(0); 2],
            combined_bitboard: BitBoard::new(0),
            side_to_move: Color::White,
            en_passant: None,
            castle_rights: CastleRights::default(),
        }
    }

    fn place_piece(&mut self, square: Square, piece: Piece, color: Color) {
        let bitboard = BitBoard::from_square(square);
        self.xor(bitboard, piece, color);
    }

    fn xor(&mut self, bitboard: BitBoard, piece: Piece, color: Color) {
        self.pieces_bitboards[piece.to_index()] ^= bitboard;
        self.colors_bitboards[color.to_index()] ^= bitboard;
        self.combined_bitboard ^= bitboard;
    }

    fn set_side(&mut self, color: Color) {
        self.side_to_move = color;
    }

    fn set_castling_rights(&mut self, rights: CastleRights) {
        self.castle_rights = rights;
    }

    fn set_en_passant(&mut self, square: Option<Square>) -> Result<(), Error> {
        if let Some(s) = square {
            if ![Rank::Third, Rank::Sixth].contains(&s.get_rank())
                || s.get_rank() == Rank::Third && self.side_to_move == Color::White
                || s.get_rank() == Rank::Sixth && self.side_to_move == Color::Black
            {
                bail!("invalid");
            }
        }
        self.en_passant = square;
        Ok(())
    }

    fn get_piece(&self, square: Square) -> Option<Piece> {
        let bitboard = BitBoard::from_square(square);
        if (self.combined_bitboard & bitboard).is_empty() {
            return None;
        }

        for piece in ALL_PIECES {
            let piece_bitboard = self.pieces_bitboards[piece.to_index()];
            if !(piece_bitboard & bitboard).is_empty() {
                return Some(piece);
            }
        }

        None
    }

    fn get_color(&self, square: Square) -> Option<Color> {
        let bitboard = BitBoard::from_square(square);
        if !(self.colors_bitboards[Color::White.to_index()] & bitboard).is_empty() {
            Some(Color::White)
        } else if !(self.colors_bitboards[Color::Black.to_index()] & bitboard).is_empty() {
            Some(Color::Black)
        } else {
            None
        }
    }

    fn get_piece_and_color(&self, square: Square) -> Option<(Piece, Color)> {
        let piece = self.get_piece(square)?;
        let color = self.get_color(square)?;
        Some((piece, color))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = fen.split_whitespace().collect();
        if tokens.len() < 4 {
            bail!("invalid fen string");
        }

        let ranks = tokens[0].split('/').collect::<Vec<_>>();
        if ranks.len() != 8 {
            bail!("invalid fen string");
        }

        let mut board = Self::new();
        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = Rank::from_index(7 - rank_idx); // 8th rank first
            let mut file = File::from_index(0);
            for c in rank_str.chars() {
                match c {
                    '1'..='8' => {
                        let skip = c.to_digit(10).unwrap() as usize;
                        file = File::from_index(file.to_index() + skip);
                    }
                    _ => {
                        let color = if c.is_uppercase() {
                            Color::White
                        } else {
                            Color::Black
                        };

                        let piece = match c.to_ascii_lowercase() {
                            'k' => Piece::King,
                            'q' => Piece::Queen,
                            'r' => Piece::Rook,
                            'b' => Piece::Bishop,
                            'n' => Piece::Knight,
                            'p' => Piece::Pawn,
                            _ => bail!("invalid fen string"),
                        };

                        let square = Square::new(rank, file);
                        board.place_piece(square, piece, color);

                        file = file.right();
                    }
                }
            }
        }

        match tokens[1] {
            "w" => board.set_side(Color::White),
            "b" => board.set_side(Color::Black),
            _ => bail!("Turno inválido: {}", tokens[1]),
        }

        let rights = CastleRights::from_str(tokens[2])?;
        board.set_castling_rights(rights);

        board.set_en_passant(Square::from_str(tokens[3]).ok())?;

        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in ALL_RANKS.iter().rev() {
            let mut empty = 0;
            for file in ALL_FILES.iter() {
                let square = Square::new(*rank, *file);
                if let Some((piece, color)) = self.get_piece_and_color(square) {
                    if empty != 0 {
                        write!(f, "{}", empty)?;
                        empty = 0;
                    }
                    write!(f, "{}", piece.to_string(color))?;
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                write!(f, "{}", empty)?;
            }
            if *rank != Rank::First {
                write!(f, "/")?;
            }
        }
        write!(f, " ")?;

        if self.side_to_move == Color::White {
            write!(f, "w ")?;
        } else {
            write!(f, "b ")?;
        }

        write!(f, "{}", self.castle_rights)?;
        write!(f, " ")?;

        if let Some(square) = self.en_passant {
            write!(f, "{}", square)?;
        } else {
            write!(f, "-")?;
        }

        write!(f, " 0 1") // TODO
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in ALL_RANKS.iter().rev() {
            if *rank == Rank::Eighth {
                write!(f, "  ╭───┬───┬───┬───┬───┬───┬───┬───╮\n")?;
            } else {
                write!(f, "  ├───┼───┼───┼───┼───┼───┼───┼───┤\n")?;
            }
            write!(f, "{}", rank.to_index() + 1)?;

            for file in ALL_FILES.iter() {
                write!(f, " │ ")?;
                let square = Square::new(*rank, *file);
                if let Some((piece, color)) = self.get_piece_and_color(square) {
                    write!(f, "{}", piece.to_symbol(color))?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, " │\n")?;
        }
        write!(f, "  ╰───┴───┴───┴───┴───┴───┴───┴───╯\n")?;
        write!(f, "    A   B   C   D   E   F   G   H  ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initial_position() {
        let board = Board::default();
        let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board_fen = format!("{}", board);
        assert_eq!(board_fen, initial_fen);
    }
    
    #[test]
    fn test_board_from_str() {
        assert!(Board::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").is_ok());
        assert!(Board::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e3 0 1").is_err());
        assert!(Board::from_str("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").is_ok());
    }

    #[test]
    fn set_en_passant() {
        let mut board = Board::default();
        assert!(board.set_en_passant(Square::from_str("e6").ok()).is_ok());
        assert!(board.set_en_passant(Square::from_str("e3").ok()).is_err());
        board.set_side(Color::Black);
        assert!(board.set_en_passant(Square::from_str("e3").ok()).is_ok());
        assert!(board.set_en_passant(Square::from_str("e6").ok()).is_err());

        assert!(board.set_en_passant(Square::from_str("e4").ok()).is_err());
    }
}
