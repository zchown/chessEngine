use crate::{
    board::defs::{Colors, Color, Pieces, Squares, BB_SQUARES, SQUARE_NAMES},
    board::{Board},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    defs::{Castling, NrOf, Piece, Square, EMPTY},
};

pub const BISHOP_RELEVANT_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6, 
];

pub const ROOK_RELEVANT_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12, 
];

// sorcerer creates magic
pub struct Sorcerer {
    pub state: u32,
}

impl Sorcerer {
    pub fn new() -> Sorcerer {
        Sorcerer {
            state: 1804289383,
        }
    }

    pub fn get_random_number(&mut self) -> u32{
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

pub fn mask_bishop_attacks(sq: Square) -> Bitboard {
    let mut attacks: Bitboard = 0;

    let target_rank = sq / 8;
    let target_file = sq % 8;

    for &(rank_dir, file_dir) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
        calc_attacks(rank_dir, file_dir, target_rank, target_file, &mut attacks);
    }
    attacks
}

pub fn mask_rook_attacks(sq: Square) -> Bitboard {
    let mut attacks: Bitboard = 0;

    let target_rank = sq / 8;
    let target_file = sq % 8;

    for &(rank_dir, file_dir) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        calc_attacks(rank_dir, file_dir, target_rank, target_file, &mut attacks);
    }
    attacks
}

pub fn calc_attacks(rank_dir: isize, file_dir: isize, target_rank: usize, target_file: usize, attacks: &mut Bitboard) {
    let mut rank = (target_rank as isize) + rank_dir;
    let mut file = (target_file as isize) + file_dir;
    while ((1..=6).contains(&rank) || rank_dir == 0) && ((1..=6).contains(&file) || file_dir == 0) {
        *attacks |= 1 << ((rank as usize) * 8 + (file as usize));
        rank += rank_dir;
        file += file_dir;
    }
}

pub fn calc_attacks_with_blocks(rank_dir: isize, file_dir: isize, target_rank: usize, target_file: usize, attacks: &mut Bitboard, blocks: Bitboard) {
    let mut rank = (target_rank as isize) + rank_dir;
    let mut file = (target_file as isize) + file_dir;
    while (0..=7).contains(&rank) && (0..=7).contains(&file) {
        let sq = (rank as usize) * 8 + (file as usize);
        *attacks |= 1 << sq;
        if get_bit(blocks, sq) {
            break;
        }
        rank += rank_dir;
        file += file_dir;
    }
}

pub fn bishop_attacks(sq: Square, blocks: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = 0;

    let target_rank = sq / 8;
    let target_file = sq % 8;

    for &(rank_dir, file_dir) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
        calc_attacks_with_blocks(rank_dir, file_dir, target_rank, target_file, &mut attacks, blocks);
    }

    attacks
}

pub fn rook_attacks(sq: Square, blocks: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = 0;

    let target_rank = sq / 8;
    let target_file = sq % 8;

    for &(rank_dir, file_dir) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        calc_attacks_with_blocks(rank_dir, file_dir, target_rank, target_file, &mut attacks, blocks);
    }

    attacks
}

pub fn set_occupancy(index: Bitboard, bits_in_mask: usize, attack_mask: Bitboard) -> Bitboard {
    let mut occ: Bitboard = 0;
    let mut atm = attack_mask.clone();

    for i in 0..bits_in_mask {
        let sq = get_lsb(atm);
        pop_bit(&mut atm, sq);

        if (index & (1 << i)) != 0 {
            occ |= 1 << sq;
        }
    }
    occ
}

