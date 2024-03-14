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
    // let mut atm = moves::mask_rook_attacks(0);
    // for i in 0..4095 {
        // let occ = moves::set_occupancy(i, count_bits(atm) as usize, atm);
        // print_bitboard(occ);
    // }

    for r in 0..8 {
        for f in 0..8 {
            let sq = r * 8 + f;
            print!("{:?}, ", count_bits(moves::mask_rook_attacks(sq)));
        }
        println!();


    }

}
