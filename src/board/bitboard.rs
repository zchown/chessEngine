use crate::defs::{NrOf, Square};

pub type Bitboard = u64;

pub fn print_bitboard(bb: Bitboard) {
    println!();
    for r in 0..NrOf::RANKS {
        for file in 0..NrOf::FILES {
            let rank = 7 - r;
            let sq: Square = rank * 8 + file;

            if file == 0 {
                print!("{} ", 8 - r);
            }

            print!("{} ", if get_bit(bb, sq) { "X" } else { "_" });
        }
        println!();
    }
    println!("  a b c d e f g h");
    println!("{:?}", bb);
}

pub fn count_bits(bb: Bitboard) -> u32 {
    bb.count_ones()
}

pub fn get_lsb(bb: Bitboard) -> Square {
    bb.trailing_zeros() as Square
}

pub fn get_bit(bb: Bitboard, sq: Square) -> bool {
    bb & (1 << sq) != 0
}

pub fn set_bit(bb: &mut Bitboard, sq: Square) {
    *bb |= 1 << sq;
}

pub fn clear_bit(bb: &mut Bitboard, sq: Square) {
    *bb &= !(1 << sq);
}

pub fn pop_bit(bb: &mut Bitboard, sq: Square) {
    if get_bit(*bb, sq) {
        *bb ^= 1 << sq
    };
}

pub fn bitboard_to_array(bb: Bitboard) -> [bool; 64] {
    let mut array = [false; 64];
    for i in 0..64 {
        array[i] = get_bit(bb, i);
    }
    array
}

