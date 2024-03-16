use crate::{
    board::defs::{Colors, Color, Pieces, Squares, BB_SQUARES, SQUARE_NAMES},
    board::{Board},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    defs::{Castling, NrOf, Piece, Square, EMPTY},
};

pub const BISHOP_RELEVANT_BITS: [i32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6, 
];

pub const ROOK_RELEVANT_BITS: [i32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12, 
];

// pub const rook_magic_numbers: [u64;

// sorcerer creates magic
pub struct Sorcerer {
    pub state: u32,
    pub bishop_magic_numbers: [u64; 64],
    pub rook_magic_numbers: [u64; 64],
}

impl Sorcerer {
    pub fn new() -> Sorcerer {
        Sorcerer {
            state: 1804289383,
            bishop_magic_numbers: [0; 64],
            rook_magic_numbers: [0; 64],
        }
    }

    pub fn get_random_u32_number(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
   
    pub fn get_random_u64_number(&mut self) -> u64 {
        let x = (self.get_random_u32_number() as u64) & 0xFFFF;
        let y = (self.get_random_u32_number() as u64) & 0xFFFF;
        let z = (self.get_random_u32_number() as u64) & 0xFFFF;
        let w = (self.get_random_u32_number() as u64) & 0xFFFF;
        x | (y << 16) | (z << 32) | (w << 48)
    }

    pub fn generate_magic_num(&mut self) -> u64 {
        self.get_random_u64_number() & self.get_random_u64_number() & self.get_random_u64_number()
    }

    pub fn find_magic_number(&mut self, sq: Square, relevant_bits: i32, bishop: bool) -> u64 {
        let mut occupancies: [Bitboard; 4096] = [0; 4096];
        let mut attacks: [Bitboard; 4096] = [0; 4096];

        let attack_mask = if bishop { mask_bishop_attacks(sq) } else { mask_rook_attacks(sq) };

        let occupancy_index: u64 = 1 << relevant_bits;

        for i in 0..occupancy_index {
            occupancies[i as usize] = set_occupancy(i, relevant_bits as usize, attack_mask);
            attacks[i as usize] = if bishop { bishop_attacks(sq, occupancies[i as usize]) } else { rook_attacks(sq, occupancies[i as usize]) };
        }

        for _ in 0..100000000 {
            let magic = self.generate_magic_num();

            // this will cause an overflow
            if count_bits((attack_mask * magic) & 0xFF00000000000000) < 6 {
                continue;
            }

            let mut fail = false;
            let mut used_attacks: [Bitboard; 4096] = [0; 4096];

            for i in 0..occupancy_index {
                let index = (occupancies[i as usize] * magic) >> (64 - relevant_bits);
                if used_attacks[index as usize] == 0 {
                    used_attacks[index as usize] = attacks[i as usize];
                } else if used_attacks[index as usize] != attacks[i as usize] {
                    fail = true;
                    break;
                }
            }
            if !fail {
                return magic;
            }
        }
        // hopefully this will never happen
        println!("Failed to find magic number for square {}", SQUARE_NAMES[sq]);
        0
    }

    pub fn init_magic_numbers(&mut self) {
        for sq in 0..NrOf::SQUARES {
            self.rook_magic_numbers[sq] = self.find_magic_number(sq, ROOK_RELEVANT_BITS[sq], false);
            println!("0x{:x},", self.rook_magic_numbers[sq]);
        }

        println!();

        for sq in 0..NrOf::SQUARES {
            self.bishop_magic_numbers[sq] = self.find_magic_number(sq, BISHOP_RELEVANT_BITS[sq], true);
            println!("0x{:x},", self.bishop_magic_numbers[sq]);
        }
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

