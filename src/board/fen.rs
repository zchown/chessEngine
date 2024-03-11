use super::defs::{Colors, Pieces, Squares, EMPTY, SQUARE_NAMES};

use crate::{
    board::{Bitboard, Board},
    defs::{Castling, Square, FEN_START_POSITION, MAX_MOVES},
};

use if_chain::if_chain;
use std::fmt::{self, Display};
use std::ops::RangeInclusive;

const FEN_NR_OF_PARTS: usize = 6;
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const EP_SQUARES_WHITE: RangeInclusive<Square> = Squares::A3..=Squares::H3;
const EP_SQUARES_BLACK: RangeInclusive<Square> = Squares::A6..=Squares::H6;
const WHITE_OR_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';

#[derive(Debug)]
pub enum FenError {
    IncorrectLength,
    Part1,
    Part2,
    Part3,
    Part4,
    Part5,
    Part6,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in FEN string: Must be 6 parts",
            Self::Part1 => "Error in FEN Part 1: Pieces or squares",
            Self::Part2 => "Error in FEN Part 2: Colors",
            Self::Part3 => "Error in FEN Part 3: Castling rights",
            Self::Part4 => "Error in FEN Part 4: En passant field",
            Self::Part5 => "Error in FEN Part 5: Half-move clock",
            Self::Part6 => "Error in FEN Part 6: Full-move number",
        };
        write!(f, "{error}")
    }
}

pub type FenResult = Result<(), FenError>;
pub type SplitResult = Result<Vec<String>, FenError>;
type FenPartParser = fn(board: &mut Board, part: &str) -> FenResult;

impl Board {
    pub fn parse_fen(&mut self, fen_string: Option<&str>) -> FenResult {
        let parts = split_fen_string(fen_string)?;
        let parsers = create_part_parsers();

        let mut temp = self.clone();
        temp.reset();

        let mut i: usize = 0;
        while i < FEN_NR_OF_PARTS {
            let parser = parsers[i];
            let part = &parts[i];
            parser(&mut temp, &part);
            i += 1;
        }

        temp.init();
        *self = temp;

        Ok(())
    }
}

fn split_fen_string(fen_string: Option<&str>) -> SplitResult {
    const SHORT_FEN_LENGTH: usize = 4;

    let mut fen_string: Vec<String> = match fen_string {
        Some(fen) => fen,
        None => FEN_START_POSITION,
    }
    .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
    .split(SPACE)
    .map(String::from)
    .collect();

    if fen_string.len() == SHORT_FEN_LENGTH {
        fen_string.append(&mut vec![String::from("0"), String::from("1")]);
    }

    if fen_string.len() != FEN_NR_OF_PARTS {
        return Err(FenError::IncorrectLength);
    }

    Ok(fen_string)
}

fn create_part_parsers() -> [FenPartParser; FEN_NR_OF_PARTS] {
    [
        pieces,
        color,
        castling,
        en_passant,
        half_move_clock,
        full_move_number,
    ]
}

fn pieces(board: &mut Board, part: &str) -> FenResult {
    let mut rank: u8 = 7;
    let mut file: u8 = 0;

    for c in part.chars() {
        let square = ((rank * 8) + file) as usize;
        match c {
            'k' => {
                board.pieces[Pieces::KING] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'q' => {
                board.pieces[Pieces::QUEEN] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'r' => {
                board.pieces[Pieces::ROOK] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'b' => {
                board.pieces[Pieces::BISHOP] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'n' => {
                board.pieces[Pieces::KNIGHT] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'p' => {
                board.pieces[Pieces::PAWN] |= 1 << square;
                board.color[Colors::BLACK] |= 1 << square;
            }
            'K' => {
                board.pieces[Pieces::KING] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            'Q' => {
                board.pieces[Pieces::QUEEN] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            'R' => {
                board.pieces[Pieces::ROOK] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            'B' => {
                board.pieces[Pieces::BISHOP] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            'N' => {
                board.pieces[Pieces::KNIGHT] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            'P' => {
                board.pieces[Pieces::PAWN] |= 1 << square;
                board.color[Colors::WHITE] |= 1 << square;
            }
            '1'..='8' => {
                if let Some(n) = c.to_digit(10) {
                    file += n as u8;
                }
            }
            SPLITTER => {
                if file != 8 {
                    return Err(FenError::Part1);
                }
                rank -= 1;
                file = 0;
            }
            _ => return Err(FenError::Part1),
        }

        if LIST_OF_PIECES.contains(c) {
            file += 1;
        }
    }
    Ok(())
}

fn color(board: &mut Board, part: &str) -> FenResult {
    if part.len() == 1 {
        if part == "w" {
            board.turn = true;
            return Ok(());
        } else if part == "b" {
            board.turn = false;
            return Ok(());
        }
    }
    Err(FenError::Part2)
}

fn castling(board: &mut Board, part: &str) -> FenResult {
    if (1..=4).contains(&part.len()) {
        for c in part.chars() {
            match c {
                'K' => board.castling |= Castling::WK,
                'Q' => board.castling |= Castling::WQ,
                'k' => board.castling |= Castling::BK,
                'q' => board.castling |= Castling::BQ,
                '-' => (),
                _ => return Err(FenError::Part3),
            }
        }
        return Ok(());
    }
    Err(FenError::Part3)
}

//CHECK: does this work?
fn en_passant(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if part.len() == 1;
        if let Some(x) = part.chars().next();
        if x == DASH;
        then {
            return Ok(());
        }
    }

    if part.len() == 2 {
        let square = SQUARE_NAMES.iter().position(|&s| s == part);
        match square {
            Some(sq) if EP_SQUARES_WHITE.contains(&sq) || EP_SQUARES_BLACK.contains(&sq) => {
                board.en_passant = Some(sq as u8);
                return Ok(());
            }
            _ => return Err(FenError::Part4),
        };
    }
    Err(FenError::Part4)
}

fn half_move_clock(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if (1..=3).contains(&part.len());
        if let Ok(n) = part.parse::<u8>();
        if n <= MAX_MOVES;
        then {
            board.half_move = n;
            return Ok(());
        }
    }
    Err(FenError::Part5)
}

fn full_move_number(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if (1..=3).contains(&part.len());
        if let Ok(n) = part.parse::<u8>();
        if n <= MAX_MOVES;
        then {
            board.full_move = n;
            return Ok(());
        }
    }
    Err(FenError::Part6)
}
