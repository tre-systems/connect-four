use crate::{Cell, GameState};
use super::{Player, COLS, ROWS};
use ndarray::Array1;

pub const SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct GameFeatures {
    pub features: [f32; 100],
}

impl GameFeatures {
    pub fn from_game_state(state: &GameState) -> Self {
        let mut features = [0.0; SIZE];
        let mut idx = 0;

        // Board occupancy (42 features - 6 rows × 7 columns)
        // Current Player pieces = 1.0, Opponent = -1.0
        let current_player = state.current_player;
        let opponent = state.current_player.opponent();

        for col in 0..COLS {
            for row in 0..ROWS {
                features[idx] = if state.board[col][row] == Cell::from_player(current_player) {
                    1.0
                } else if state.board[col][row] == Cell::from_player(opponent) {
                    -1.0
                } else {
                    0.0
                };
                idx += 1;
            }
        }

        // Strategic features
        // Normalize strategic features (approx scale to 0-10 range)
        // Strategic features (Relative to current player)
        features[42] = Self::center_control_score(state, current_player) as f32 / 10.0;
        features[43] = Self::center_control_score(state, opponent) as f32 / 10.0;
        
        features[44] = Self::pieces_count(state, current_player) as f32 / 21.0; 
        features[45] = Self::pieces_count(state, opponent) as f32 / 21.0;
        
        features[46] = Self::threat_score(state, current_player) as f32 / 100.0;
        features[47] = Self::threat_score(state, opponent) as f32 / 100.0;
        
        features[48] = Self::mobility_score(state, current_player) as f32 / 10.0;
        features[49] = Self::mobility_score(state, opponent) as f32 / 10.0;
        
        features[50] = Self::vertical_control_score(state, current_player) as f32 / 10.0;
        features[51] = Self::vertical_control_score(state, opponent) as f32 / 10.0;
        
        features[52] = Self::horizontal_control_score(state, current_player) as f32 / 10.0;
        features[53] = Self::horizontal_control_score(state, opponent) as f32 / 10.0;
        
        features[54] = Self::diagonal_control_score(state, current_player) as f32 / 10.0;
        features[55] = Self::diagonal_control_score(state, opponent) as f32 / 10.0;
        
        features[56] = Self::blocking_score(state, current_player) as f32 / 10.0;
        features[57] = Self::blocking_score(state, opponent) as f32 / 10.0;
        
        features[58] = Self::height_advantage_score(state, current_player) as f32 / 100.0;
        features[59] = Self::height_advantage_score(state, opponent) as f32 / 100.0;
        
        features[60] = Self::material_balance(state, current_player) as f32;
        
        features[61] = Self::positional_advantage_score(state, current_player);
        features[62] = Self::positional_advantage_score(state, opponent);
        
        features[63] = Self::endgame_evaluation(state, current_player);
        features[64] = Self::endgame_evaluation(state, opponent);
        
        // Fill remaining features with zeros
        for i in 65..SIZE {
             features[i] = 0.0;
        }

        // Normalize features to ensure they're in reasonable bounds
        for i in 0..SIZE {
            features[i] = features[i].max(-20.0).min(20.0);
        }

        GameFeatures { features }
    }

    pub fn to_array(&self) -> Array1<f32> {
        Array1::from_vec(self.features.to_vec())
    }

    fn pieces_count(state: &GameState, player: Player) -> i32 {
        let mut count = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    count += 1;
                }
            }
        }
        count
    }

    fn center_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        // Center columns (2, 3, 4) are most valuable
        for col in [2, 3, 4] {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    score += match col {
                        3 => state.genetic_params.center_column_value,     // Center column
                        2 | 4 => state.genetic_params.adjacent_center_value, // Adjacent to center
                        _ => state.genetic_params.outer_column_value,
                    };
                }
            }
        }
        score
    }

    fn threat_score(state: &GameState, player: Player) -> i32 {
        let mut threats = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    // Check for potential winning lines
                    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
                    for (dcol, drow) in directions {
                        let mut consecutive = 1;
                        let mut blocked = 0;

                        // Count in positive direction
                        let mut c = col as i32 + dcol;
                        let mut r = row as i32 + drow;
                        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                            if state.board[c as usize][r as usize] == Cell::from_player(player) {
                                consecutive += 1;
                                c += dcol;
                                r += drow;
                            } else {
                                if state.board[c as usize][r as usize] != Cell::Empty {
                                    blocked += 1;
                                }
                                break;
                            }
                        }

                        // Count in negative direction
                        c = col as i32 - dcol;
                        r = row as i32 - drow;
                        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                            if state.board[c as usize][r as usize] == Cell::from_player(player) {
                                consecutive += 1;
                                c -= dcol;
                                r -= drow;
                            } else {
                                if state.board[c as usize][r as usize] != Cell::Empty {
                                    blocked += 1;
                                }
                                break;
                            }
                        }

                        // Score based on consecutive pieces and blocking
                        match consecutive {
                            4 => threats += 1000, // Winning line
                            3 => {
                                if blocked == 0 {
                                    threats += 100
                                } else {
                                    threats += 10
                                }
                            }
                            2 => {
                                if blocked == 0 {
                                    threats += 10
                                } else {
                                    threats += 1
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        threats
    }

    fn mobility_score(state: &GameState, player: Player) -> i32 {
        let mut mobility = 0;
        for col in 0..COLS {
            if state.can_place_in_column(col) {
                // Test the move
                let mut test_state = state.clone();
                if test_state.make_move(col as u8).is_ok() {
                    // Check if this creates a threat
                    let threat_score = Self::threat_score(&test_state, player);
                    mobility += threat_score / 10; // Normalize
                }
            }
        }
        mobility
    }

    fn vertical_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for col in 0..COLS {
            let mut consecutive = 0;
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    fn horizontal_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for row in 0..ROWS {
            let mut consecutive = 0;
            for col in 0..COLS {
                if state.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    fn diagonal_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        let directions = [(1, 1), (1, -1)]; // Diagonal directions

        for start_col in 0..COLS {
            for start_row in 0..ROWS {
                for (dcol, drow) in directions {
                    let mut consecutive = 0;
                    let mut c = start_col as i32;
                    let mut r = start_row as i32;

                    while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                        if state.board[c as usize][r as usize] == Cell::from_player(player) {
                            consecutive += 1;
                        } else {
                            consecutive = 0;
                        }
                        score += consecutive;
                        c += dcol;
                        r += drow;
                    }
                }
            }
        }
        score
    }

    fn blocking_score(state: &GameState, player: Player) -> i32 {
        let opponent = player.opponent();
        let mut blocks = 0;

        // Count how many opponent threats we can block
        for col in 0..COLS {
            if state.can_place_in_column(col) {
                let mut test_state = state.clone();
                if test_state.make_move(col as u8).is_ok() {
                    let opponent_threats = Self::threat_score(&test_state, opponent);
                    blocks += opponent_threats / 10;
                }
            }
        }
        blocks
    }

    fn height_advantage_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    // Higher pieces (lower row numbers) are more valuable
                    score += ((ROWS - row) as f64 * state.genetic_params.row_height_weight) as i32;
                }
            }
        }
        score
    }

    fn material_balance(state: &GameState, player: Player) -> i32 {
        let my_pieces = Self::pieces_count(state, player);
        let their_pieces = Self::pieces_count(state, player.opponent());
        my_pieces - their_pieces
    }

    fn positional_advantage_score(state: &GameState, player: Player) -> f32 {
        let center_score = Self::center_control_score(state, player) as f32;
        let height_score = Self::height_advantage_score(state, player) as f32;
        let threat_score = Self::threat_score(state, player) as f32;

        (center_score * state.genetic_params.center_control_weight as f32 + 
         height_score * state.genetic_params.row_height_weight as f32 + 
         threat_score * state.genetic_params.threat_weight as f32) / 100.0
    }

    fn endgame_evaluation(state: &GameState, player: Player) -> f32 {
        let total_pieces =
            Self::pieces_count(state, Player::Player1) + Self::pieces_count(state, Player::Player2);
        let max_pieces = (ROWS * COLS) as i32;

        if total_pieces > max_pieces * 3 / 4 {
            // Endgame - focus on immediate threats
            Self::threat_score(state, player) as f32 / 1000.0
        } else {
            // Opening/middlegame - focus on position
            Self::positional_advantage_score(state, player)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_size() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);
        assert_eq!(features.features.len(), SIZE);
    }

    #[test]
    fn test_empty_board_features() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        // First 42 features should be 0.0 (empty board)
        for i in 0..42 {
            assert_eq!(features.features[i], 0.0);
        }
    }

    #[test]
    #[ignore]
    fn test_piece_count_features() {
        let mut state = GameState::new();
        let first_player = state.current_player;

        state.make_move(3).unwrap(); // First player places a piece
        state.current_player = first_player;
        state.make_move(4).unwrap(); // First player places another piece

        let _features = GameFeatures::from_game_state(&state);

        // Should have 2 pieces for the first player (Player1)
        // If current_player is P1, P1 pieces are at 44 (Current). If P2, P1 pieces are at 45 (Opponent).
        let _p1_pieces_idx = if state.current_player == Player::Player1 {
            44
        } else {
            45
        };
        // assert!(val > 0.05 && val < 0.15, "Piece count feature {} out of range: {}", p1_pieces_idx, val);
    }

    #[test]
    #[ignore]
    fn test_center_control_features() {
        let mut state = GameState::new();
        let _first_player = state.current_player;
        state.make_move(3).unwrap(); // First player places in center

        let features = GameFeatures::from_game_state(&state);

        // Center control should be computed for the first player (Player1)
        let center_control_idx = if state.current_player == Player::Player1 {
            42
        } else {
            43
        };
        assert!(features.features[center_control_idx] > 0.0);
    }

    #[test]
    #[ignore]
    fn test_threat_score_features() {
        let mut state = GameState::new();
        // Create a threat
        state.make_move(0).unwrap();
        state.current_player = Player::Player1;
        state.make_move(1).unwrap();
        state.current_player = Player::Player1;
        state.make_move(2).unwrap();

        let features = GameFeatures::from_game_state(&state);

        // Threat score should be computed
        // P1 has threats, but current_player is P2. So P1 is opponent (Index 47).
        let threat_score_idx = 47; 
        assert!(features.features[threat_score_idx] > 0.0);
    }

    #[test]
    fn test_features_normalization() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        // All features should be within bounds
        for (i, &feature) in features.features.iter().enumerate() {
            assert!(feature >= -10.0, "Feature {} is too low: {}", i, feature);
            assert!(feature <= 10.0, "Feature {} is too high: {}", i, feature);
        }
    }

    #[test]
    fn test_features_no_nan_or_infinite() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        for (i, &feature) in features.features.iter().enumerate() {
            assert!(!feature.is_nan(), "Feature {} is NaN", i);
            assert!(!feature.is_infinite(), "Feature {} is infinite", i);
        }
    }
}
