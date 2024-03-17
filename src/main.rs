mod board;
mod defs;
mod moves;
mod sorcery;
mod magics;

use defs::{Square, Colors, BB_SQUARES, ALL_SQUARES};

use crate::{
    board::bitboard::{Bitboard, set_bit, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    moves::{AttackTable, NOT_EIGHTH_RANK, NOT_FIRST_RANK},
    sorcery::{Sorcerer},
};


fn main() {
    let attack_table = AttackTable::new();
    let mut occ: Bitboard = 0;
    set_bit(&mut occ, ALL_SQUARES::C5 as usize);
    set_bit(&mut occ, ALL_SQUARES::F2 as usize);
    set_bit(&mut occ, ALL_SQUARES::G7 as usize);
    set_bit(&mut occ, ALL_SQUARES::H8 as usize);
    set_bit(&mut occ, ALL_SQUARES::B2 as usize);
    set_bit(&mut occ, ALL_SQUARES::G5 as usize);
    set_bit(&mut occ, ALL_SQUARES::E2 as usize);
    set_bit(&mut occ, ALL_SQUARES::E7 as usize);

    print_bitboard(occ);

    print_bitboard(attack_table.get_bishop_attacks(ALL_SQUARES::D4 as Square, occ));
    print_bitboard(attack_table.get_rook_attacks(ALL_SQUARES::E5 as Square, occ));

}
