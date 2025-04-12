use crate::engine::{
    attacks::generated::pawn::{ISOLATED_PAWN_LOOKUP, PASSED_PAWN_LOOKUP},
    game::{self, Game},
    move_generation::mv_gen::get_occupancy,
    shared::{
        helper_func::{bit_pos_utility::get_bit_file, bitboard::BitboardTrait},
        structures::{color::*, piece::*},
    },
};

const DOUBLE_PAWN_WT: isize = -30;
const BLOCKED_PAWN_WT: isize = -30;
const ISOLATED_PAWN_WT: isize = -50;
const MOBILITY_WT: isize = 10;
const ROOK_OPEN_FILE_WT: isize = 30;
const PASSED_PAWN_WT: isize = 50;
const GAME_PHASE_INCREMENT: [isize; 14] = [0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 4, 4];

#[rustfmt::skip]
const PAWN_EVAL:[[isize; 64]; 2] = [
    [
      0,   0,   0,   0,   0,   0,  0,   0,
     98, 134,  61,  95,  68, 126, 34, -11,
     -6,   7,  26,  31,  65,  56, 25, -20,
    -14,  13,   6,  21,  23,  12, 17, -23,
    -27,  -2,  -5,  12,  17,   6, 10, -25,
    -26,  -4,  -4, -10,   3,   3, 33, -12,
    -35,  -1, -20, -23, -15,  24, 38, -22,
      0,   0,   0,   0,   0,   0,  0,   0,
    ],
    [
    0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 147, 132, 165, 187,
     94, 100,  85,  67,  56,  53,  82,  84,
     32,  24,  13,   5,  -2,   4,  17,  17,
     13,   9,  -3,  -7,  -7,  -8,   3,  -1,
      4,   7,  -6,   1,   0,  -5,  -1,  -8,
     13,   8,   8,  10,  13,   0,   2,  -7,
      0,   0,   0,   0,   0,   0,   0,   0,
    ]
];

#[rustfmt::skip]
const KNIGHT_EVAL:[[isize; 64]; 2] = [
    [
        -167, -89, -34, -49,  61, -97, -15, -107,
         -73, -41,  72,  36,  23,  62,   7,  -17,
         -47,  60,  37,  65,  84, 129,  73,   44,
          -9,  17,  19,  53,  37,  69,  18,   22,
         -13,   4,  16,  13,  28,  19,  21,   -8,
         -23,  -9,  12,  10,  19,  17,  25,  -16,
         -29, -53, -12,  -3,  -1,  18, -14,  -19,
        -105, -21, -58, -33, -17, -28, -19,  -23,
    ],
    [
        -58, -38, -13, -28, -31, -27, -63, -99,
        -25,  -8, -25,  -2,  -9, -25, -24, -52,
        -24, -20,  10,   9,  -1,  -9, -19, -41,
        -17,   3,  22,  22,  22,  11,   8, -18,
        -18,  -6,  16,  25,  16,  17,   4, -18,
        -23,  -3,  -1,  15,  10,  -3, -20, -22,
        -42, -20, -10,  -5,  -2, -20, -23, -44,
        -29, -51, -23, -15, -22, -18, -50, -64,
    ]
];

#[rustfmt::skip]
const BISHOP_EVAL:[[isize; 64]; 2] = [
    [
        -29,   4, -82, -37, -25, -42,   7,  -8,
        -26,  16, -18, -13,  30,  59,  18, -47,
        -16,  37,  43,  40,  35,  50,  37,  -2,
        -4,   5,  19,  50,  37,  37,   7,  -2,
        -6,  13,  13,  26,  34,  12,  10,   4,
        0,  15,  15,  15,  14,  27,  18,  10,
        4,  15,  16,   0,   7,  21,  33,   1,
        -33,  -3, -14, -21, -13, -12, -39, -21,
    ],
    [
        -14, -21, -11,  -8, -7,  -9, -17, -24,
        -8,  -4,   7, -12, -3, -13,  -4, -14,
        2,  -8,   0,  -1, -2,   6,   0,   4,
        -3,   9,  12,   9, 14,  10,   3,   2,
        -6,   3,  13,  19,  7,  10,  -3,  -9,
        -12,  -3,   8,  10, 13,   3,  -7, -15,
        -14, -18,  -7,  -1,  4,  -9, -15, -27,
        -23,  -9, -23,  -5, -9, -16,  -5, -17, 
    ]
];

#[rustfmt::skip]
const ROOK_EVAL:[[isize; 64]; 2] = [
    [
     32,  42,  32,  51, 63,  9,  31,  43,
     27,  32,  58,  62, 80, 67,  26,  44,
     -5,  19,  26,  36, 17, 45,  61,  16,
    -24, -11,   7,  26, 24, 35,  -8, -20,
    -36, -26, -12,  -1,  9, -7,   6, -23,
    -45, -25, -16, -17,  3,  0,  -5, -33,
    -44, -16, -20,  -9, -1, 11,  -6, -71,
    -19, -13,   1,  17, 16,  7, -37, -26,
    ],
    [
   13, 10, 18, 15, 12,  12,   8,   5,
    11, 13, 13, 11, -3,   3,   8,   3,
     7,  7,  7,  5,  4,  -3,  -5,  -3,
     4,  3, 13,  1,  2,   1,  -1,   2,
     3,  5,  8,  4, -5,  -6,  -8, -11,
    -4,  0, -5, -1, -7, -12,  -8, -16,
    -6, -6,  0,  2, -9,  -9, -11,  -3,
    -9,  2,  3, -1, -5, -13,   4, -20,
    ]
];

#[rustfmt::skip]
const QUEEN_EVAL:[[isize; 64]; 2] = [
    [
    -28,   0,  29,  12,  59,  44,  43,  45,
    -24, -39,  -5,   1, -16,  57,  28,  54,
    -13, -17,   7,   8,  29,  56,  47,  57,
    -27, -27, -16, -16,  -1,  17,  -2,   1,
     -9, -26,  -9, -10,  -2,  -4,   3,  -3,
    -14,   2, -11,  -2,  -5,   2,  14,   5,
    -35,  -8,  11,   2,   8,  15,  -3,   1,
     -1, -18,  -9,  10, -15, -25, -31, -50,
    ],
    [
     -9,  22,  22,  27,  27,  19,  10,  20,
    -17,  20,  32,  41,  58,  25,  30,   0,
    -20,   6,   9,  49,  47,  35,  19,   9,
      3,  22,  24,  45,  57,  40,  57,  36,
    -18,  28,  19,  47,  31,  34,  39,  23,
    -16, -27,  15,   6,   9,  17,  10,   5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43,  -5, -32, -20, -41,
    ]
];

#[rustfmt::skip]
const KING_EVAL:[[isize; 64]; 2] = [
    [
    -65,  23,  16, -15, -56, -34,   2,  13,
     29,  -1, -20,  -7,  -8,  -4, -38, -29,
     -9,  24,   2, -16, -20,   6,  22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49,  -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
      1,   7,  -8, -64, -43, -16,   9,   8,
    -15,  36,  12, -54,   8, -28,  24,  14,
    ],
    [
     -74, -35, -18, -18, -11,  15,   4, -17,
     -12,  17,  14,  17,  17,  38,  23,  11,
      10,  17,  23,  15,  20,  45,  44,  13,
      -8,  22,  24,  27,  26,  33,  26,   3,
     -18,  -4,  21,  24,  27,  23,   9, -11,
     -19,  -3,  11,  21,  23,  16,   7,  -9,
     -27, -11,   4,  13,  14,   4,  -5, -17,
     -53, -34, -21, -11, -28, -14, -24, -43
    ]
 ];

const EVAL_TABLE: [[[isize; 64]; 2]; 6] =
    [PAWN_EVAL, KNIGHT_EVAL, KING_EVAL, BISHOP_EVAL, ROOK_EVAL, QUEEN_EVAL];

#[rustfmt::skip]
const OPP_SQ:[usize;64] = [
    0,  1,  2,  3,  4,  5,  6,  7,
    8,  9,  10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 61, 62, 63,
];

#[rustfmt::skip]
const OK_SQ:[usize;64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
    8,  9,  10, 11, 12, 13, 14, 15,
    0,  1,  2,  3,  4,  5,  6,  7,
];

pub trait Evaluation {
    fn evaluate_pos(&self) -> isize;
    // fn material_sq(&self) -> isize;
    // fn piece_eval(piece: &Piece, sq: usize) -> isize;
    // fn pawn_eval(piece: &Piece, sq: usize, pawns: (u64, u64)) -> isize;
    // fn material_balance(&self) -> isize;
}

impl Evaluation for Game {
    #[inline(always)]
    fn evaluate_pos(&self) -> isize {
        let mut game_phase = 0;
        let mut middle_game = [0, 0];
        let mut end_game = [0, 0];

        /* evaluate each piece */
        for piece in &CLR_PIECES {
            let mut bb = self.bitboard[piece.idx()];
            while bb != 0 {
                let sq = bb.pop_lsb();
                middle_game[piece.color().idx()] +=
                    EVAL_TABLE[((piece.kind() / 2) - 1).idx()][0][sq];
                end_game[piece.color().idx()] += EVAL_TABLE[((piece.kind() / 2) - 1).idx()][1][sq];

                middle_game[piece.color().idx()] += piece.weight(0);
                end_game[piece.color().idx()] += piece.weight(1);

                game_phase += GAME_PHASE_INCREMENT[piece.idx()];
            }
        }

        let mg_score = (middle_game[WHITE.idx()] - middle_game[BLACK.idx()]) * self.color.sign();
        let eg_score = (end_game[WHITE.idx()] - end_game[BLACK.idx()]) * self.color.sign();
        let mg_phase = game_phase.min(24);
        let eg_phase = 24 - mg_phase;
        return (mg_score * mg_phase + eg_score * eg_phase) / 24;

        // (Self::material_balance(self) + Self::material_sq(self)) * self.color.sign()
    }

    // #[inline(always)]
    // fn material_sq(&self) -> isize {
    //     let mut score: isize = 0;
    //     for piece in &CLR_PIECES {
    //         let mut bb = self.bitboard[piece.idx()];
    //         while bb != 0 {
    //             let sq = bb.pop_lsb();
    //             if piece.is_white() {
    //                 score += Self::piece_eval(piece, OK_SQ[sq]);
    //                 score += Self::pawn_eval(
    //                     piece,
    //                     sq,
    //                     (self.bitboard[WHITE_PAWN.idx()], self.bitboard[BLACK_PAWN.idx()]),
    //                 )
    //             } else {
    //                 score -= Self::piece_eval(piece, OPP_SQ[sq]);
    //                 score -= Self::pawn_eval(
    //                     piece,
    //                     sq,
    //                     (self.bitboard[BLACK_PAWN.idx()], self.bitboard[WHITE_PAWN.idx()]),
    //                 )
    //             }
    //         }
    //     }

    //     score
    // }

    // #[inline(always)]
    // fn piece_eval(piece: &Piece, sq: usize) -> isize {
    //     match piece.kind() {
    //         PAWN => PAWN_EVAL[sq],
    //         KNIGHT => KNIGHT_EVAL[sq],
    //         BISHOP => BISHOP_EVAL[sq],
    //         ROOK => ROOK_EVAL[sq],
    //         QUEEN => QUEEN_EVAL[sq],
    //         KING => KING_MG_EVAL[sq],
    //         _ => panic!(" Not the right type, Something is wrong"),
    //     }
    // }

    // #[inline(always)]
    // fn material_balance(&self) -> isize {
    //     let mut score = 0;
    //     for piece in &PIECES {
    //         score += piece.weight()
    //             * (self.bitboard[(piece + WHITE).idx()].count() as isize
    //                 - self.bitboard[(piece + BLACK).idx()].count() as isize)
    //     }
    //     score
    // }

    // #[inline(always)]
    // fn pawn_eval(piece: &Piece, sq: usize, (own_pawns, enemy_pawns): (u64, u64)) -> isize {
    //     let mut score: isize = 0;
    //     if !piece.is_pawn() {
    //         return 0;
    //     }

    //     if PASSED_PAWN_LOOKUP[piece.color().idx()][sq] & enemy_pawns == 0 {
    //         let file = get_bit_file(sq) as usize;
    //         score += PASSED_PAWN_WT[piece.color().idx()][file];
    //     }

    //     if ISOLATED_PAWN_LOOKUP[sq] & own_pawns == 0 {
    //         score += ISOLATED_PAWN_WT;
    //     }

    //     // TODO: DOUBLE PAWNS
    //     // TODO: BLOCKED PAWNS

    //     return score;
    // }
}

// NOTE: For Each Peace
// 1. How much are on the board of that type (Material on the board)
// 2. How much is the square they are sitting on valuable (md, eg)
// 3. Unique parameters that give advantage (Rook -> Open Files, Rook -> Connectivity Pawn -> Passed Pawn, etc...)
// 4. Mobility
// 5.

// fn evaluation(game: &Game) -> isize {
//     let mut eval = 0;
//     // NOTE: WHITE_PAWN
//     eval += (game.bitboard[WHITE_PAWN.idx()].count() as isize) * WHITE_PAWN.weight();
//     eval += (game.bitboard[BLACK_PAWN.idx()].count() as isize) * BLACK_PAWN.weight();
//     eval += (game.bitboard[WHITE_KNIGHT.idx()].count() as isize) * WHITE_KNIGHT.weight();
//     eval += (game.bitboard[BLACK_KNIGHT.idx()].count() as isize) * BLACK_KNIGHT.weight();
//     eval += (game.bitboard[WHITE_BISHOP.idx()].count() as isize) * WHITE_BISHOP.weight();
//     eval += (game.bitboard[BLACK_BISHOP.idx()].count() as isize) * BLACK_BISHOP.weight();
//     eval += (game.bitboard[WHITE_ROOK.idx()].count() as isize) * WHITE_ROOK.weight();
//     eval += (game.bitboard[BLACK_ROOK.idx()].count() as isize) * BLACK_ROOK.weight();
//     eval += (game.bitboard[WHITE_QUEEN.idx()].count() as isize) * WHITE_QUEEN.weight();
//     eval += (game.bitboard[BLACK_QUEEN.idx()].count() as isize) * BLACK_QUEEN.weight();
//     eval += (game.bitboard[WHITE_KING.idx()].count() as isize) * WHITE_KING.weight();
//     eval += (game.bitboard[BLACK_KING.idx()].count() as isize) * BLACK_KING.weight();

//     let mut bb = game.bitboard[WHITE_PAWN.idx()];
//     while bb != 0 {
//         let sq = bb.pop_lsb();

//     }

//     return eval;
// }

// fn piece_eval();

// fn pawn_eval();
// fn knight_eval();
// fn bishop_eval();
// fn rook_eval();
// fn queen_eval();
// fn king_eval();
// fn
