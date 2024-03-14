mod board;
mod defs;
mod moves;
mod sorcery;

use crate::{
    board::defs::{Colors, BB_SQUARES, Squares},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    moves::{AttackTable, NOT_EIGHTH_RANK, NOT_FIRST_RANK},
    sorcery::{Sorcerer},
};


fn main() {

}
