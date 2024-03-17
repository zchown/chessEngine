use crate::{
    board::{Board},
    board::bitboard::{Bitboard, get_bit, get_lsb, count_bits, print_bitboard, pop_bit},
    defs::{Castling, NrOf, Piece, Square, EMPTY, Colors, Color, Pieces, BB_SQUARES},
    sorcery::{mask_bishop_attacks, mask_rook_attacks, BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS, set_occupancy, bishop_attacks, rook_attacks, },
    magics::{BISHOP_MAGICS, ROOK_MAGICS},
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
    pub bishops: [[Bitboard; 512]; NrOf::SQUARES],
    pub rooks: [[Bitboard; 4096]; NrOf::SQUARES],
    bishop_masks: [Bitboard; 64],
    rook_masks: [Bitboard; 64],
}

impl AttackTable {
    pub fn new() -> AttackTable {
        let mut a = AttackTable {
            pawns: [[0; NrOf::SQUARES]; NrOf::COLORS],
            knights: [0; NrOf::SQUARES],
            kings: [0; NrOf::SQUARES],
            bishops: [[0; 512]; NrOf::SQUARES],
            rooks: [[0; 4096]; NrOf::SQUARES],
            bishop_masks: [0; 64],
            rook_masks: [0; 64],

        };
        a.init_pawns();
        a.init_knights();
        a.init_kings();
        a.init_sliders();
        a
    }

    fn init_pawns(&mut self) {
        let mut pawns: [[Bitboard; NrOf::SQUARES]; NrOf::COLORS] = [[0; NrOf::SQUARES]; NrOf::COLORS];
        for sq in 0..NrOf::SQUARES {
            self.pawns[Colors::WHITE as Color][sq] = pawn_attacks(Colors::WHITE as Color, sq);
            self.pawns[Colors::BLACK as Color ][sq] = pawn_attacks(Colors::BLACK as Color, sq);
        }
    }

    fn init_knights(&mut self) {
        for sq in 0..NrOf::SQUARES {
            self.knights[sq] = knight_attacks(sq);
        }
    }

    fn init_kings(&mut self) {
        for sq in 0..NrOf::SQUARES {
            self.kings[sq] = king_attacks(sq);
        }
    }

    fn init_sliders(&mut self) {
        for sq in 0..NrOf::SQUARES {
            self.bishop_masks[sq] = mask_bishop_attacks(sq);
            self.rook_masks[sq] = mask_rook_attacks(sq);

            let attack_mask_bishop: Bitboard = self.bishop_masks[sq];
            let attack_mask_rook: Bitboard = self.rook_masks[sq];

            let relevant_bits_bishop: i32 = BISHOP_RELEVANT_BITS[sq];
            let relevant_bits_rook: i32 = ROOK_RELEVANT_BITS[sq];

            let occupancy_index_bishop: u64 = 1 << relevant_bits_bishop;
            let occupancy_index_rook: u64 = 1 << relevant_bits_rook;

            for i in 0..occupancy_index_bishop {
                let occ = set_occupancy(i, relevant_bits_bishop as usize, self.bishop_masks[sq]);
                let magic_index = occ * BISHOP_MAGICS[sq] >> (64 - relevant_bits_bishop);
                self.bishops[sq][magic_index as usize] = bishop_attacks(sq, occ);

            }

            for i in 0..occupancy_index_rook {
                let occ = set_occupancy(i, relevant_bits_rook as usize, self.rook_masks[sq]);
                let magic_index = occ * ROOK_MAGICS[sq] >> (64 - relevant_bits_rook);
                self.rooks[sq][magic_index as usize] = rook_attacks(sq, occ);
            }
        }
    }

    #[inline(always)]
    pub fn get_bishop_attacks(&self, sq: Square, occ: Bitboard) -> Bitboard {
        let mut occ = occ;
        occ &= self.bishop_masks[sq];
        occ *= BISHOP_MAGICS[sq];
        occ >>= 64 - BISHOP_RELEVANT_BITS[sq];
        self.bishops[sq][occ as usize]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, sq: Square, occ: Bitboard) -> Bitboard {
        let mut occ = occ;
        occ &= self.rook_masks[sq];
        occ *= ROOK_MAGICS[sq];
        occ >>= 64 - ROOK_RELEVANT_BITS[sq];
        self.rooks[sq][occ as usize]
    }
}


fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    let b = BB_SQUARES[sq];
    let mut attacks: Bitboard = 0;
    if color == Colors::WHITE as Color {
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

fn knight_attacks(sq: Square) -> Bitboard {
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

fn king_attacks(sq: Square) -> Bitboard {
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

