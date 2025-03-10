use super::fen::fen::FenTrait;
use super::search::transposition_table::TTTable;
use super::shared::helper_func::bitboard::*;
use super::shared::helper_func::const_utility::*;
use super::shared::structures::color::*;
use super::shared::structures::internal_move::PositionIrr;
use super::shared::structures::internal_move::PositionRev;
use super::shared::structures::piece::Piece;
use crate::engine::shared::structures::castling_struct::CastlingRights;

// TODO: Add More Constants, Max position moves, Max Depth
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub squares: [Option<Piece>; 64],
    pub occupancy: [Bitboard; 2],
    pub bitboard: [Bitboard; 14],

    pub key: u64,
    pub color: Color,
    pub castling: CastlingRights,
    pub ep: Option<u8>,
    pub half_move: u8,
    pub full_move: u16,

    pub pos_rev: Vec<PositionRev>,
    pub pos_irr: Vec<PositionIrr>,

    pub tt: TTTable,
    pub s_history: [[u64; 64]; 14], // FIXME: Rename This and check for better takes implementation because it takes a lot of memory
    pub s_killers: [[u64; 2]; 64], // FIXME: Rename This and check for better takes implementation because it takes a lot of memory
    pub ply: usize,
}

impl Game {
    pub fn initialize() -> Game {
        Game::read_fen(FEN_START)
    }

    pub fn create_board() -> Self {
        Self {
            squares: [None; 64],
            occupancy: [0 as Bitboard; 2],
            bitboard: [0 as Bitboard; 14],
            color: WHITE,
            castling: CastlingRights::NONE,
            ep: None,
            half_move: 0,
            full_move: 1,
            key: 0,

            pos_rev: Vec::with_capacity(1024),
            pos_irr: Vec::with_capacity(1024),
            tt: TTTable::init(),
            s_history: [[0u64; 64]; 14],
            s_killers: [[0u64; 2]; 64],
            ply: 0,
        }
    }

    pub fn reset_board(&mut self) {
        self.squares = [None; 64];
        self.occupancy = [0 as Bitboard; 2];
        self.bitboard = [0 as Bitboard; 14];
        self.color = WHITE;
        self.castling = CastlingRights::NONE;
        self.ep = None;
        self.half_move = 0;
        self.full_move = 1;
        self.pos_rev = Vec::with_capacity(1024);
        self.pos_irr = Vec::with_capacity(1024);
        self.tt = TTTable::init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_board() {
        let mut game = Game::initialize();
        game.reset_board();

        assert_eq!(game.squares, [None; 64]);
        assert_eq!(game.bitboard, [0; 14]);
        assert_eq!(game.occupancy, [0; 2]);
        assert_eq!(game.color, WHITE);
        assert_eq!(game.castling, CastlingRights::NONE);
        assert_eq!(game.ep, None);
        assert_eq!(game.half_move, 0);
        assert_eq!(game.full_move, 1);
        assert_eq!(game.pos_rev.len(), 0);
        assert_eq!(game.pos_irr.len(), 0);
    }
}
