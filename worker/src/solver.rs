use crate::{Player, GameState, Cell, ROWS, COLS};
use std::collections::HashMap;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bitboard {
    player_board: u64, // Bitmask of current player's pieces
    mask: u64,         // Bitmask of all pieces
    moves_count: u8,   // Number of moves played
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Bitboard {
    pub fn new() -> Self {
        Bitboard {
            player_board: 0,
            mask: 0,
            moves_count: 0,
        }
    }

    pub fn can_play(&self, col: usize) -> bool {
        (self.mask & top_mask(col)) == 0
    }

    pub fn play(&mut self, col: usize) {
        self.player_board ^= self.mask;
        self.mask |= self.mask + bottom_mask(col);
        self.player_board ^= self.mask;
        // The standard XOR trick leaves player_board as "Just Moved".
        // We want it to represent "Next Player to Move".
        self.player_board ^= self.mask;
        self.moves_count += 1;
    }

    // Check if the opponent has won (connected 4)
    pub fn is_win(&self) -> bool {
        // self.player_board is current player.
        // We want to check if the PREVIOUS player won.
        // previous player pieces = self.player_board ^ self.mask
        let opponent_board = self.player_board ^ self.mask;
        Self::alignment(opponent_board)
    }

    pub fn alignment(pos: u64) -> bool {
        // Horizontal
        let mut m = pos & (pos >> 7);
        if m & (m >> 14) != 0 { return true; }
        
        // Diagonal \
        m = pos & (pos >> 6);
        if m & (m >> 12) != 0 { return true; }

        // Diagonal /
        m = pos & (pos >> 8);
        if m & (m >> 16) != 0 { return true; }

        // Vertical
        m = pos & (pos >> 1);
        if m & (m >> 2) != 0 { return true; }

        false
    }
    
    pub fn key(&self) -> u64 {
        self.player_board + self.mask
    }

    pub fn from_game_state(state: &GameState) -> Self {
        // Map GameState (row 5 = bottom) to bit index col * 7 + (5 - row).
        let mut p1: u64 = 0;
        let mut p2: u64 = 0;

        for col in 0..COLS {
            for row in 0..ROWS {
                let bit_index = col * 7 + (5 - row);
                match state.board[col][row] {
                    Cell::Player1 => p1 |= 1 << bit_index,
                    Cell::Player2 => p2 |= 1 << bit_index,
                    Cell::Empty => {}
                }
            }
        }

        let mask = p1 | p2;
        let current_position = if state.current_player == Player::Player1 {
            p1
        } else {
            p2
        };

        Bitboard {
            player_board: current_position,
            mask,
            moves_count: mask.count_ones() as u8,
        }
    }
}

// Helper for bitmasks
const fn top_mask(col: usize) -> u64 {
    1 << (HEIGHT - 1 + col * (HEIGHT + 1))
}

const fn bottom_mask(col: usize) -> u64 {
    1 << (col * (HEIGHT + 1))
}



pub struct Solver {
    transposition_table: HashMap<u64, (i8, u8)>, // Key -> (Score, Depth)
    nodes: u64,
    column_order: [usize; WIDTH],
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            transposition_table: HashMap::with_capacity(1_000_000), // Pre-allocate some space
            nodes: 0,
            // Search center columns first for better pruning
            column_order: [3, 2, 4, 1, 5, 0, 6],
        }
    }

    pub fn reset(&mut self) {
        self.transposition_table.clear();
        self.nodes = 0;
    }

    pub fn get_nodes_count(&self) -> u64 {
        self.nodes
    }

    pub fn tt_size(&self) -> usize {
        self.transposition_table.len()
    }

    // Returns (best_move_column, expected_score). depth is recursive depth remaining.
    pub fn analyze(&mut self, position: &Bitboard, depth: i32) -> (Option<usize>, i32) {
        self.nodes = 0;

        let mut best_score = -10000;
        let mut best_move = None;

        let search_order = self.column_order;
        for &col in &search_order {
            if position.can_play(col) {
                let mut next_pos = *position;
                next_pos.play(col);

                if next_pos.is_win() {
                    return (
                        Some(col),
                        (WIDTH * HEIGHT + 1 - position.moves_count as usize) as i32 / 2,
                    );
                }

                let score = -self.negamax(&next_pos, depth - 1, -10000, -best_score);

                if score > best_score {
                    best_score = score;
                    best_move = Some(col);
                }
            }
        }

        (best_move, best_score)
    }

    pub fn analyze_all(&mut self, position: &Bitboard, depth: i32) -> Vec<(usize, i32)> {
        self.nodes = 0;
        let mut evaluations = Vec::new();

        for col in 0..WIDTH {
            if position.can_play(col) {
                let mut next_pos = *position;
                next_pos.play(col);

                if next_pos.is_win() {
                    let score = (WIDTH * HEIGHT + 1 - next_pos.moves_count as usize) as i32 / 2;
                    evaluations.push((col, score));
                    continue;
                }

                let score = -self.negamax(&next_pos, depth - 1, -10000, 10000);
                evaluations.push((col, score));
            }
        }

        evaluations
    }

    fn negamax(&mut self, position: &Bitboard, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
        self.nodes += 1;

        if position.moves_count >= (WIDTH * HEIGHT) as u8 {
            return 0; // Draw
        }

        if depth == 0 {
            return self.heuristic_score(position);
        }

        let key = position.key();
        if let Some(&(val, cached_depth)) = self.transposition_table.get(&key) {
            if cached_depth >= depth as u8 {
                return val as i32;
            }
        }

        // Upper bound on score given remaining moves; tightens the window.
        let max_possible = (WIDTH * HEIGHT - position.moves_count as usize + 1) as i32 / 2;
        if beta > max_possible {
            beta = max_possible;
            if alpha >= beta { return beta; }
        }

        let search_order = self.column_order;
        for &col in &search_order {
            if position.can_play(col) {
                let mut next_pos = *position;
                next_pos.play(col); // flips the player to move

                if next_pos.is_win() {
                    // The move we just played wins, so score it positively for us.
                    let score = (WIDTH * HEIGHT + 1 - next_pos.moves_count as usize) as i32 / 2;
                    return score;
                }

                let score = -self.negamax(&next_pos, depth - 1, -beta, -alpha);

                if score >= beta {
                    self.transposition_table.insert(key, (score as i8, depth as u8));
                    return score;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }

        self.transposition_table.insert(key, (alpha as i8, depth as u8));

        alpha
    }

    // Heuristic for non-terminal leaves; deep search makes a flat score adequate.
    fn heuristic_score(&self, _position: &Bitboard) -> i32 {
        0
    }
}
