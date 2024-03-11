use crate::board::Bitboard;
use crate::defs::{NrOf, Square};
// number of pieces, squares, files, ranks, castling permissions, and piece types
// create definitions for the board and pieces
pub struct Pieces;
impl Pieces {
    pub const KING: usize = 0;
    pub const QUEEN: usize = 1;
    pub const ROOK: usize = 2;
    pub const BISHOP: usize = 3;
    pub const KNIGHT: usize = 4;
    pub const PAWN: usize = 5;
    pub const NONE: usize = 6;
}

pub type Color = usize;
//create an enum for the colors
pub struct Colors;
impl Colors {
    pub const WHITE: Color = 0;
    pub const BLACK: Color = 1;
}

pub const PIECE_NAMES: [&str; NrOf::PIECE_TYPES + 1] =
    ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn", "_"];
pub const PIECE_CHARS: [char; NrOf::PIECE_TYPES + 1] = ['K', 'Q', 'R', 'B', 'N', 'P', '_'];

// all squares on the board
pub const SQUARE_NAMES: [&str; NrOf::SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

pub const EMPTY: u64 = 0;

pub struct Squares;
impl Squares {
    // White side squares that are important for castling
    pub const A1: Square = 0;
    pub const B1: Square = 1;
    pub const C1: Square = 2;
    pub const D1: Square = 3;
    pub const E1: Square = 4;
    pub const F1: Square = 5;
    pub const G1: Square = 6;
    pub const H1: Square = 7;
    pub const A2: Square = 8;
    pub const B2: Square = 9;
    pub const C2: Square = 10;
    pub const D2: Square = 11;
    pub const E2: Square = 12;
    pub const F2: Square = 13;
    pub const G2: Square = 14;
    pub const H2: Square = 15;

    // Black side squares that are important for castling
    pub const A8: Square = 56;
    pub const B8: Square = 57;
    pub const C8: Square = 58;
    pub const D8: Square = 59;
    pub const E8: Square = 60;
    pub const F8: Square = 61;
    pub const G8: Square = 62;
    pub const H8: Square = 63;

    // White EP-squares start/end
    pub const A3: Square = 16;
    pub const H3: Square = 23;

    // Black EP-squares start/end
    pub const A6: Square = 40;
    pub const H6: Square = 47;
}

const fn init_bb_squares() -> [Bitboard; NrOf::SQUARES] {
    let mut squares = [0; NrOf::SQUARES];
    let mut i = 0;
    while i < NrOf::SQUARES {
        squares[i] = 1u64 << i;
        i += 1;
    }
    squares
}

pub const BB_SQUARES: [Bitboard; NrOf::SQUARES] = init_bb_squares();

