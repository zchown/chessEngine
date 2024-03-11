use crate::{
    board::defs::{Colors, Color, Pieces, Squares, BB_SQUARES, SQUARE_NAMES},
    board::{Board},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    defs::{Castling, NrOf, Piece, Square, EMPTY},
};

pub const NOT_A_FILE: Bitboard = 0xfefefefefefefefe;
pub const NOT_H_FILE: Bitboard = 0x7f7f7f7f7f7f7f7f;
pub const NOT_HG_FILE: Bitboard = 0x3f3f3f3f3f3f3f3f;
pub const NOT_AB_FILE: Bitboard = 0xfcfcfcfcfcfcfcfc;
pub const NOT_FIRST_RANK: Bitboard = 0xffffffffffffff00;
pub const NOT_EIGHTH_RANK: Bitboard = 0x00ffffffffffffff;
pub const DARK_SQUARES: Bitboard = 0xaa55aa55aa55aa55;

pub struct AttackTable{
    pub pawns: [[Bitboard; NrOf::SQUARES]; NrOf::COLORS],
    pub knights: [Bitboard; NrOf::SQUARES],
    pub kings: [Bitboard; NrOf::SQUARES],
}

impl AttackTable {
    pub fn new() -> AttackTable {
        let mut a = AttackTable {
            pawns: [[0; NrOf::SQUARES]; NrOf::COLORS],
            knights: [0; NrOf::SQUARES],
            kings: [0; NrOf::SQUARES],
        };
        a.init_pawns();
        a.init_knights();
        a.init_kings();
        a
    }

    fn init_pawns(&self) {
        let mut pawns: [[Bitboard; NrOf::SQUARES]; NrOf::COLORS] = [[0; NrOf::SQUARES]; NrOf::COLORS];
        for sq in 0..NrOf::SQUARES {
            self.pawns[Colors::WHITE][sq] = pawn_attacks(Colors::WHITE, sq);
            self.pawns[Colors::BLACK][sq] = pawn_attacks(Colors::BLACK, sq);
        }
    }
    pub fn init_knights(&self) {
        for sq in 0..NrOf::SQUARES {
            self.knights[sq] = knight_attacks(sq);
        }
    }

    pub fn init_kings(&self) {
        for sq in 0..NrOf::SQUARES {
            self.kings[sq] = king_attacks(sq);
        }
    }

}

pub fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    let b = BB_SQUARES[sq];
    let mut attacks: Bitboard = 0;
    if color == Colors::WHITE {
        if b & NOT_A_FILE != 0 {
            attacks |= b << 7; 
        }
        if b & NOT_H_FILE != 0 {
            attacks |= b << 9;
        }
    } else {
        if b & NOT_A_FILE != 0 {
            attacks |= b >> 9;
        }
        if b & NOT_H_FILE != 0 {
            attacks |= b >> 7;
        }
    }
    attacks
}

pub fn knight_attacks(sq: Square) -> Bitboard {
    let b = BB_SQUARES[sq];
    let mut attacks: Bitboard = 0;
    if b & NOT_H_FILE != 0 {
        attacks |= b << 17;
        attacks |= b >> 15;
    }
    if b & NOT_A_FILE != 0 {
        attacks |= b << 15;
        attacks |= b >> 17;
    }
    if b & NOT_HG_FILE != 0 {
        attacks |= b << 10;
        attacks |= b >> 6;
    }
    if b & NOT_AB_FILE != 0 {
        attacks |= b << 6;
        attacks |= b >> 10;
    }
    attacks
}

pub fn king_attacks(sq: Square) -> Bitboard {
    let b = BB_SQUARES[sq];
    let mut attacks: Bitboard = 0;
    if b & NOT_H_FILE != 0 {
        if b & NOT_EIGHTH_RANK != 0 {
            attacks |= b << 9;
        }
        if b & NOT_FIRST_RANK != 0 {
            attacks |= b >> 7;
        }
        attacks |= b << 1;
    }
    if b & NOT_A_FILE != 0 {
        if b & NOT_EIGHTH_RANK != 0 {
            attacks |= b << 7;
        }
        if b & NOT_FIRST_RANK != 0 {
            attacks |= b >> 9;
        }
        attacks |= b >> 1;
    }
    if b & NOT_EIGHTH_RANK != 0 {
        attacks |= b << 8;
    }
    if b & NOT_FIRST_RANK != 0 {
        attacks |= b >> 8;
    }
    attacks
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
    while (1..=6).contains(&rank) && (1..=6).contains(&file) {
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

pub fn set_occupancy(index: usize, bits_in_mask: usize, attack_mask: &mut Bitboard) -> Bitboard {
    let mut occ: Bitboard = 0;

    for i in 0..bits_in_mask {
        let sq = get_lsb(*attack_mask);
        pop_bit(attack_mask, sq);

        if (index & (1 << i)) != 0 {
            occ |= 1 << sq;
        }
    }

    occ
}

