mod board;
mod defs;
mod moves;
use moves::{AttackTable, NOT_EIGHTH_RANK, NOT_FIRST_RANK};

use crate::{
    board::defs::{Colors, BB_SQUARES, Squares},
};


fn main() {
    let at = moves::AttackTable::new();
    // board::print_bitboard(NOT_FIRST_RANK);
    // board::print_bitboard(NOT_EIGHTH_RANK);
    // for i in 0..64 {
        // board::print_bitboard(at.kings[i]);
    // }
    for i in 0..64 {
        board::print_bitboard(AttackTable::mask_rook_attacks(i));
    }

}
