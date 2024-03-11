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
