pub mod defs;
pub mod bitboard;
mod fen;

use std::fmt;

use self::{
    defs::{Colors, Pieces, BB_SQUARES, EMPTY, PIECE_CHARS},
    fen::FenResult,
    bitboard::{Bitboard, print_bitboard, get_bit, set_bit, clear_bit, pop_bit, count_bits, bitboard_to_array},

};

use crate::defs::{NrOf, Piece, Square, FEN_START_POSITION};

// create a struct to represent the board with bitboards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub pieces: [Bitboard; NrOf::PIECE_TYPES],
    pub color: [Bitboard; NrOf::COLORS],
    pub castling: u8,
    pub en_passant: Option<u8>,
    pub half_move: u8,
    pub turn: bool,
    pub full_move: u8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [EMPTY; NrOf::PIECE_TYPES],
            color: [EMPTY; NrOf::COLORS],
            castling: 0,
            en_passant: None,
            half_move: 0,
            turn: true,
            full_move: 0,
        }
    }

    pub fn print_bitboards(&self) {
        for i in 0..NrOf::PIECE_TYPES {
            print!("{}: ", i);
            print_bitboard(self.pieces[i]);
            println!();
        }
        for i in 0..NrOf::COLORS {
            print!("{}: ", i);
            print_bitboard(self.color[i]);
            println!();
        }
    }

    // get pieces for a specific color
    pub fn get_pieces(&self, piece: usize, color: usize) -> Bitboard {
        self.pieces[piece] & self.color[color]
    }

    pub fn get_all_pieces(&self) -> Bitboard {
        self.color[Colors::WHITE] | self.color[Colors::BLACK]
    }

    pub fn remove_piece(&mut self, piece: Piece, square: Square, color: usize) {
        self.pieces[piece] ^= BB_SQUARES[square];
        self.color[color] ^= BB_SQUARES[square];
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square, color: usize) {
        self.pieces[piece] |= BB_SQUARES[square];
        self.color[color] |= BB_SQUARES[square];
    }

    pub fn move_piece(&mut self, piece: Piece, from: Square, to: Square) {
        let color = if self.color[Colors::WHITE] & BB_SQUARES[from] != 0 {
            Colors::WHITE
        } else {
            Colors::BLACK
        };
        self.remove_piece(piece, from, color);
        self.add_piece(piece, to, color);
    }

    //TODO: implement init
    pub fn init(&mut self) {}

    pub fn reset(&mut self) {
        self.pieces = [EMPTY; NrOf::PIECE_TYPES];
        self.color = [EMPTY; NrOf::COLORS];
        self.castling = 0;
        self.en_passant = None;
        self.half_move = 0;
        self.turn = true;
        self.full_move = 0;
    }
}
