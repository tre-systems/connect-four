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
    
    // Returns a bitmask of all valid moves (top empty cell of each column)
    pub fn possible_moves(&self) -> u64 {
        // valid moves are where the top bit map is 0
        // We take (mask + bottom_mask) & board_mask
        // But simpler: checking each column top bit.
        
        // However, for the solver, we often just iterate columns 0..6
        0 
    }

    pub fn key(&self) -> u64 {
        self.player_board + self.mask
    }

    pub fn from_game_state(state: &GameState) -> Self {

        // The Bitboard structure expects moves to be played in order to build valid masks.
        // However, we can reconstruct it from the board array.
        // Bitboard logic:
        // mask: 1 if piece present, 0 otherwise
        // player_board: 1 if CURRENT player's piece ?? No.
        // The bitboard design often uses:
        // 'position': bitmask of current player pieces
        // 'mask': bitmask of ALL pieces
        //
        // But in our 'play' method:
        // self.player_board ^= self.mask;
        // self.mask |= self.mask + bottom_mask(col);
        //
        // This implies 'player_board' tracks the player who JUST moved?
        // Let's trace:
        // Start: p=0, m=0.
        // Play(col): p = 0^0 = 0. m = 0 | (0 + bottom) = bottom.
        // Result: p=0 (empty), m=bottom.
        //
        // Wait, standard implementation:
        // position: current player
        // mask: all pieces
        // make_move(col):
        //   position ^= mask
        //   mask |= mask + bottom_mask(col)
        //
        // This means 'position' accumulates the "accumulated XOR sum" of masks?
        // Actually this is the Fhourstones benchmark implementation trick.
        // 'position' stores the stones of the player whose turn it is -> NO.
        // 'position' stores the stones of the player who just moved ??
        
        // Let's stick to a simpler, explicit definition to avoid confusion.
        // P1 stones, P2 stones.
        // But the Solver uses the optimized one.
        // Let's rely on reconstructing by "playing" the moves if history is available?
        // GameState doesn't verify history deeply.
        // Let's reconstruct by scanning column by column from bottom to top.
        
        // We need to know whose turn it is.
        // If state.current_player == Player1, we want 'player_board' to represent Player1?
        // The implementation:
        // Start: p=0, m=0. (Player1 to move).
        // If we want to set up arbitrary board:
        // We can't easily use the XOR trick without replaying moves.
        // BUT, valid Connect 4 boards are built from bottom up.
        

        
        // Revised plan: Use explicit p1/p2 bitboards for safety and ease of setting up provided state.
        // Then implementing bitwise ops is still fast.
        
        // Let's assume we rewrite Bitboard explicitly.
        
        let mut p1: u64 = 0;
        let mut p2: u64 = 0;
        
        // 5 is bottom (index 5 in array), 0 is top.
        // In bitboard: usually bit 0 is bottom-left.
        // col 0: 0..5. col 1: 7..12.
        
         for col in 0..COLS {
            for row in 0..ROWS {
                // GameState: row 5 is bottom.
                // Bitboard: let's map GameState row 5 -> bit 0 (of that column).
                // So bit_index = col * 7 + (5 - row).
                
                let cell = state.board[col][row];
                match cell {
                    Cell::Player1 => {
                        let bit_index = col * 7 + (5 - row);
                        p1 |= 1 << bit_index;
                    }
                    Cell::Player2 => {
                        let bit_index = col * 7 + (5 - row);
                        p2 |= 1 << bit_index;
                    }
                    Cell::Empty => {}
                }
            }
        }
        
        // Construct the solver state
        // If state.current_player == Player1, we need (pos=p1, mask=p1|p2).
        // If Player2, we need (pos=p2, mask=p1|p2).
        // Wait, the XOR trick requires consistent history.
        // IF we use "position = current player pieces", "mask = all pieces".
        // Play(col):
        //   new_mask = mask | (mask + bottom(col))
        //   new_position = (position ^ mask) ^ new_mask ?? No.
        //   
        //   Correct update for (pos, mask) where pos is current player:
        //   new_pos = pos ^ mask; // became opponent's pieces (which was 'mask ^ pos' before) ??
        //   new_mask = mask | (mask + bottom);
        //   new_pos = new_pos ^ new_mask; // now it's opponent (who just moved) pieces? 
        
        // Let's switch to proper "position" and "mask" where position = current player.
        // And make_move logic:
        // make_move(col) {
        //    position ^= mask;
        //    mask |= mask + bottom_mask(col);
        //    position ^= mask;
        // }
        // This is the Fhourstones algorithm.
        
        let mask = p1 | p2;
        let current_position = if state.current_player == Player::Player1 { p1 } else { p2 };
        
        Bitboard {
            player_board: current_position,
            mask,
            moves_count: (mask.count_ones()) as u8,
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

    // Main entry point for searching
    pub fn solve(&mut self, _position: &Bitboard, _weak: bool) -> i32 {
        self.nodes = 0;
        // Optimization: check for immediate winning moves first?
        // Actually, iterative deepening is better if we want to return quickly,
        // but for a pure solver, we might just call negamax with max depth.
        // Given 6x7 is "small" for a strong solver, we can try to solve deep.
        
        // Use a standard Negamax
        // Score is:
        // > 0 if winning
        // < 0 if losing
        // 0 if draw
        // The magnitude is (MAX_SCORE - moves) to prefer faster wins.
        
        let _min = -(WIDTH as i32 * HEIGHT as i32) / 2;
        let _max = (WIDTH as i32 * HEIGHT as i32) / 2;
        
        // Iterative deepening only if needed, but for now fixed deep search
        // effectively solving the position given the depth remaining?
        // Actually, since we want to play, we usually solve with limited depth if not end-game.
        // But this is a "Solver", so let's try to search as deep as possible moves allow.
        
        // For a playing engine, we want to return a Move, not just a Score.
        // So we need a root function.
        0
    }
    
    // Returns (best_move_column, expected_score)
    // depth: recursive depth remaining
    pub fn analyze(&mut self, position: &Bitboard, depth: i32) -> (Option<usize>, i32) {
         self.nodes = 0;
         
         // Start with a small search window
         let mut best_score = -10000;
         let mut best_move = None;
         
         let search_order = self.column_order;
         for &col in &search_order {
             if position.can_play(col) {
                 let mut next_pos = *position;
                 next_pos.play(col);
                 
                 // If this move wins immediately, take it
                 if next_pos.is_win() {
                     return (Some(col), (WIDTH*HEIGHT + 1 - (position.moves_count as usize)) as i32 / 2);
                 }
                 
                 // Otherwise search
                 // We pass -beta, -alpha.
                 // alpha was best_score
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

        // Check for overflow win is NOT needed here because we check .is_win() after making a move
        // But if we entered here, safe to assume previous move didn't win?
        // Actually, the standard pattern is check win, then recurse.
        // Or make move, check win, if win return score, else recurse.
        // Here we are in the "else recurse" part.
        
        if depth == 0 {
             // Heuristic evaluation if we hit depth limit
             // This is crucial for a playing engine (not a full solver)
             return self.heuristic_score(position);
        }

        // Transposition Table Lookup
        let key = position.key();
        if let Some(&(val, cached_depth)) = self.transposition_table.get(&key) {
             if cached_depth >= depth as u8 {
                 return val as i32;
             }
        }

        // Move ordering: columns 3, 2, 4, 1, 5, 0, 6
        // Already defined in self.column_order
        
        // Limit max score by remaining moves: optimization
        // max possible score is determined by how many empty cells left
        let max_possible = (WIDTH * HEIGHT - position.moves_count as usize + 1) as i32 / 2;
        if beta > max_possible {
            beta = max_possible;
            if alpha >= beta { return beta; }
        }

        let search_order = self.column_order;
        for &col in &search_order {
            if position.can_play(col) {
                let mut next_pos = *position;
                next_pos.play(col); // This flips player

                if next_pos.is_win() {
                    // Current player (who just played) won
                    // Meaning the PREVIOUS player (us, at start of this function) lost?
                    // No. 'next_pos' has 'player_board' as the NEXT player.
                    // 'next_pos.is_win()' checks if the person who JUST PLAYED has 4 aligned.
                    // If so, that move was a winning move for US.
                    // So we return a positive score.
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
        
        // Cache exact result
        self.transposition_table.insert(key, (alpha as i8, depth as u8));

        alpha
    }
    
    // Simple heuristic for non-terminal nodes
    fn heuristic_score(&self, _position: &Bitboard) -> i32 {
         // Count 3-in-a-rows, etc.
         // But wait, 'player_board' is the current player to move.
         // 'mask ^ player_board' is the opponent.
         
         // Basic heuristic: random score or simple alignment count?
         // For "Superhuman" we should just search deep (12-14 plies).
         // Connect 4 is solvable. 
         // But let's add a basic positional evaluation.
         0
    }
}
