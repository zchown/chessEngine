use strum_macros::EnumIter;
use crate::{
    board::bitboard::{Bitboard},
};
// default starting position as fen
pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const COLORS: usize = 2;
}

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const MAX_MOVES: u8 = 100;

pub type Square = usize;
pub type Piece = usize;

pub const EMPTY: u64 = 0;

pub struct Castling;
impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
    pub const ALL: u8 = 15;

}

pub enum Pieces {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    NONE,
}

impl Pieces {
    pub fn as_string(&self) -> &'static str {
        match self {
            Pieces::KING => "King",
            Pieces::QUEEN => "Queen",
            Pieces::ROOK => "Rook",
            Pieces::BISHOP => "Bishop",
            Pieces::KNIGHT => "Knight",
            Pieces::PAWN => "Pawn",
            Pieces::NONE => "_",
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Pieces::KING => 'K',
            Pieces::QUEEN => 'Q',
            Pieces::ROOK => 'R',
            Pieces::BISHOP => 'B',
            Pieces::KNIGHT => 'N',
            Pieces::PAWN => 'P',
            Pieces::NONE => '_',
        }
    }
}


pub type Color = usize;

pub enum Colors {
    WHITE,
    BLACK,
}

impl Colors {
    pub fn as_string(&self) -> &'static str {
        match self {
            Colors::WHITE => "White",
            Colors::BLACK => "Black",
        }
    }
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

#[derive(Debug, EnumIter)]
pub enum ALL_SQUARES {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl ALL_SQUARES {
    pub fn as_string(&self) -> &'static str {
        match self {
            ALL_SQUARES::A1 => "a1",
            ALL_SQUARES::B1 => "b1",
            ALL_SQUARES::C1 => "c1",
            ALL_SQUARES::D1 => "d1",
            ALL_SQUARES::E1 => "e1",
            ALL_SQUARES::F1 => "f1",
            ALL_SQUARES::G1 => "g1",
            ALL_SQUARES::H1 => "h1",
            ALL_SQUARES::A2 => "a2",
            ALL_SQUARES::B2 => "b2",
            ALL_SQUARES::C2 => "c2",
            ALL_SQUARES::D2 => "d2",
            ALL_SQUARES::E2 => "e2",
            ALL_SQUARES::F2 => "f2",
            ALL_SQUARES::G2 => "g2",
            ALL_SQUARES::H2 => "h2",
            ALL_SQUARES::A3 => "a3",
            ALL_SQUARES::B3 => "b3",
            ALL_SQUARES::C3 => "c3",
            ALL_SQUARES::D3 => "d3",
            ALL_SQUARES::E3 => "e3",
            ALL_SQUARES::F3 => "f3",
            ALL_SQUARES::G3 => "g3",
            ALL_SQUARES::H3 => "h3",
            ALL_SQUARES::A4 => "a4",
            ALL_SQUARES::B4 => "b4",
            ALL_SQUARES::C4 => "c4",
            ALL_SQUARES::D4 => "d4",
            ALL_SQUARES::E4 => "e4",
            ALL_SQUARES::F4 => "f4",
            ALL_SQUARES::G4 => "g4",
            ALL_SQUARES::H4 => "h4",
            ALL_SQUARES::A5 => "a5",
            ALL_SQUARES::B5 => "b5",
            ALL_SQUARES::C5 => "c5",
            ALL_SQUARES::D5 => "d5",
            ALL_SQUARES::E5 => "e5",
            ALL_SQUARES::F5 => "f5",
            ALL_SQUARES::G5 => "g5",
            ALL_SQUARES::H5 => "h5",
            ALL_SQUARES::A6 => "a6",
            ALL_SQUARES::B6 => "b6",
            ALL_SQUARES::C6 => "c6",
            ALL_SQUARES::D6 => "d6",
            ALL_SQUARES::E6 => "e6",
            ALL_SQUARES::F6 => "f6",
            ALL_SQUARES::G6 => "g6",
            ALL_SQUARES::H6 => "h6",
            ALL_SQUARES::A7 => "a7",
            ALL_SQUARES::B7 => "b7",
            ALL_SQUARES::C7 => "c7",
            ALL_SQUARES::D7 => "d7",
            ALL_SQUARES::E7 => "e7",
            ALL_SQUARES::F7 => "f7",
            ALL_SQUARES::G7 => "g7",
            ALL_SQUARES::H7 => "h7",
            ALL_SQUARES::A8 => "a8",
            ALL_SQUARES::B8 => "b8",
            ALL_SQUARES::C8 => "c8",
            ALL_SQUARES::D8 => "d8",
            ALL_SQUARES::E8 => "e8",
            ALL_SQUARES::F8 => "f8",
            ALL_SQUARES::G8 => "g8",
            ALL_SQUARES::H8 => "h8",
        }
    }
}

