mod board;
mod defs;
mod moves;
// use moves::{AttackTable, NOT_EIGHTH_RANK, NOT_FIRST_RANK};

use crate::{
    board::defs::{Colors, BB_SQUARES, Squares},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    moves::{AttackTable, NOT_EIGHTH_RANK, NOT_FIRST_RANK},
};


fn main() {
    let at = AttackTable::new();

    let atm = AttackTable::mask_rook_attacks(0, 0);
    let occ = at::set_occupancy(0, count_bits(atm));
}
