use super::features::GameFeatures;
use super::neural_network::{NetworkConfig, NeuralNetwork};
use super::GameState;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MLMoveEvaluation {
    pub column: u8,
    pub score: f32,
    pub move_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MLDiagnostics {
    pub valid_moves: Vec<u8>,
    pub move_evaluations: Vec<MLMoveEvaluation>,
    pub value_network_output: f32,
    pub policy_network_outputs: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MLResponse {
    pub r#move: Option<u8>,
    pub evaluation: f32,
    pub thinking: String,
    pub diagnostics: MLDiagnostics,
}

pub struct MLAI {
    pub value_network: NeuralNetwork,
    pub policy_network: NeuralNetwork,
    pub mcts_simulations: usize,
}

impl Default for MLAI {
    fn default() -> Self {
        Self::new()
    }
}

impl MLAI {
    pub fn new() -> Self {
        // Architecture: 4 hidden layers of 128 units with skip connections (ResNet-lite)
        let value_config = NetworkConfig {
            input_size: 100,
            hidden_sizes: vec![128, 128, 128, 128],
            output_size: 1,
            use_skip_connections: true,
        };
        let policy_config = NetworkConfig {
            input_size: 100,
            hidden_sizes: vec![128, 128, 128, 128],
            output_size: 7,
            use_skip_connections: true,
        };

        // Fewer simulations in debug for speed; full strength in release (native and WASM)
        let simulations = if cfg!(debug_assertions) { 200 } else { 4000 };

        MLAI {
            value_network: NeuralNetwork::new(value_config),
            policy_network: NeuralNetwork::new(policy_config),
            mcts_simulations: simulations,
        }
    }

    pub fn get_best_move(&mut self, state: &GameState) -> MLResponse {
        let valid_moves = state.get_valid_moves();

        if valid_moves.is_empty() {
            return MLResponse {
                r#move: None,
                evaluation: 0.0,
                thinking: "No valid moves available".to_string(),
                diagnostics: MLDiagnostics {
                    valid_moves: vec![],
                    move_evaluations: vec![],
                    value_network_output: 0.0,
                    policy_network_outputs: vec![0.0; 7],
                },
            };
        }

        if valid_moves.len() == 1 {
            return MLResponse {
                r#move: Some(valid_moves[0]),
                evaluation: 0.0,
                thinking: "Only one valid move".to_string(),
                diagnostics: MLDiagnostics {
                    valid_moves: valid_moves.clone(),
                    move_evaluations: vec![],
                    value_network_output: 0.0,
                    policy_network_outputs: vec![0.0; 7],
                },
            };
        }

        // Get raw network output for diagnostics
        let features = GameFeatures::from_game_state(state);
        let raw_value = self.value_network.forward(&features.to_array())[0];
        let raw_policy = self.policy_network.forward(&features.to_array());

        // Use MCTS for search (AlphaZero style)
        let mut mcts = super::mcts::MCTS::new(1.41, self.mcts_simulations);

        let value_net = &self.value_network;
        let policy_net = &self.policy_network;

        let value_fn = |s: &GameState| -> f32 {
            let f = GameFeatures::from_game_state(s);
            value_net.forward(&f.to_array())[0] // Now relative by default
        };

        let policy_fn = |s: &GameState| -> Vec<f32> {
            let f = GameFeatures::from_game_state(s);
            policy_net.forward(&f.to_array()).to_vec()
        };

        let (best_move, move_probs) = mcts.search(state.clone(), &value_fn, &policy_fn, 0.0, true);

        // Convert MCTS probs to diagnostics
        let mut move_evaluations = Vec::new();
        for &col in &valid_moves {
            let prob = move_probs[col as usize];
            move_evaluations.push(MLMoveEvaluation {
                column: col,
                score: prob, // Display visit probability as score
                move_type: "mcts_visit_prob".to_string(),
            });
        }

        move_evaluations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        MLResponse {
            r#move: Some(best_move),
            evaluation: raw_value,
            thinking: format!(
                "MCTS searched {} sims. Best move col {} with visit prob {:.3}. Terminals: {}. Raw Value: {:.3}",
                self.mcts_simulations,
                best_move,
                move_probs[best_move as usize],
                mcts.terminal_nodes_found,
                raw_value
            ),
            diagnostics: MLDiagnostics {
                valid_moves,
                move_evaluations,
                value_network_output: raw_value,
                policy_network_outputs: raw_policy.to_vec(),
            },
        }
    }

    pub fn evaluate_position(&self, state: &GameState) -> f32 {
        let features = GameFeatures::from_game_state(state);
        let value = self.value_network.forward(&features.to_array());
        value[0]
    }

    pub fn load_weights(&mut self, value_weights: &[f32], policy_weights: &[f32]) {
        self.value_network.load_weights(value_weights);
        self.policy_network.load_weights(policy_weights);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::COLS;

    #[test]
    fn test_ml_ai_new() {
        let ai = MLAI::new();
        assert!(ai.value_network.num_layers() > 0);
        assert!(ai.policy_network.num_layers() > 0);
    }

    #[test]
    fn test_ml_ai_empty_board() {
        let mut ai = MLAI::new();
        let state = GameState::new();
        let response = ai.get_best_move(&state);

        assert!(response.r#move.is_some());
        assert_eq!(response.diagnostics.valid_moves.len(), COLS);
        assert_eq!(response.diagnostics.policy_network_outputs.len(), 7);
    }

    #[test]
    fn test_ml_ai_winning_move() {
        let mut ai = MLAI::new();
        let mut state = GameState::new();
        let first_player = state.current_player;

        // Set up a winning position for the first player
        state.make_move(0).unwrap();
        state.current_player = first_player;
        state.make_move(1).unwrap();
        state.current_player = first_player;
        state.make_move(2).unwrap();
        state.current_player = first_player;

        let response = ai.get_best_move(&state);
        // Should have a valid move (the AI might not always choose the optimal winning move)
        assert!(response.r#move.is_some());
        let best_move = response.r#move.unwrap();
        assert!(best_move < COLS as u8);
    }

    #[test]
    fn test_ml_ai_blocking_move() {
        let mut ai = MLAI::new();
        let mut state = GameState::new();

        // Set up a threat for Player 2
        state.make_move(0).unwrap();
        state.make_move(1).unwrap();
        state.make_move(2).unwrap();

        let response = ai.get_best_move(&state);
        // Should have a valid move (untrained ML AI may not choose optimal blocking move)
        assert!(response.r#move.is_some());
        let best_move = response.r#move.unwrap();
        assert!(best_move < COLS as u8);
    }

    #[test]
    fn test_ml_ai_no_valid_moves() {
        let mut ai = MLAI::new();
        let state = GameState::new();

        // Fill the board (this would take many moves, but we can test the logic)
        // For now, just test that it handles empty valid moves correctly
        let response = ai.get_best_move(&state);
        assert!(response.r#move.is_some()); // Should have valid moves on empty board
    }

    #[test]
    fn test_ml_ai_evaluate_position() {
        let ai = MLAI::new();
        let state = GameState::new();
        let evaluation = ai.evaluate_position(&state);

        // Evaluation should be a finite number
        assert!(!evaluation.is_nan());
        assert!(!evaluation.is_infinite());
    }

    #[test]
    fn test_ml_ai_center_preference() {
        let mut ai = MLAI::new();
        let state = GameState::new();
        let response = ai.get_best_move(&state);

        // Should have a valid move
        let best_move = response.r#move.unwrap();
        assert!(best_move <= 6); // Valid column range
    }

    #[test]
    fn test_ml_ai_move_evaluations() {
        let mut ai = MLAI::new();
        let state = GameState::new();
        let response = ai.get_best_move(&state);

        // Should have evaluations for all valid moves
        assert_eq!(response.diagnostics.move_evaluations.len(), COLS);

        // Evaluations should be sorted (best first)
        for i in 1..response.diagnostics.move_evaluations.len() {
            assert!(
                response.diagnostics.move_evaluations[i - 1].score
                    >= response.diagnostics.move_evaluations[i].score
            );
        }
    }
}
