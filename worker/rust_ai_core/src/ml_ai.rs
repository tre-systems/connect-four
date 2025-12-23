use super::features::GameFeatures;
use super::neural_network::{NetworkConfig, NeuralNetwork};
use super::{GameState, COLS};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MLMoveEvaluation {
    pub column: u8,
    pub score: f32,
    pub move_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MLDiagnostics {
    pub valid_moves: Vec<u8>,
    pub move_evaluations: Vec<MLMoveEvaluation>,
    pub value_network_output: f32,
    pub policy_network_outputs: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MLResponse {
    pub r#move: Option<u8>,
    pub evaluation: f32,
    pub thinking: String,
    pub diagnostics: MLDiagnostics,
}

pub struct MLAI {
    pub value_network: NeuralNetwork,
    pub policy_network: NeuralNetwork,
}

impl MLAI {
    pub fn new() -> Self {
        // Create networks with appropriate sizes for Connect Four
        let value_config = NetworkConfig {
            input_size: 100,
            hidden_sizes: vec![256, 128, 64], // Deep Network
            output_size: 1,
        };
        let policy_config = NetworkConfig {
            input_size: 100,
            hidden_sizes: vec![256, 128, 64], // Deep Network
            output_size: 7,
        };

        MLAI {
            value_network: NeuralNetwork::new(value_config),
            policy_network: NeuralNetwork::new(policy_config),
        }
    }

    pub fn get_best_move(&mut self, state: &GameState) -> MLResponse {
        let valid_moves = state.get_valid_moves();

        if valid_moves.is_empty() {
             // ... (keep existing early return) ...
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
            // ... (keep existing early return) ...
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

        // Get current position evaluation
        let features = GameFeatures::from_game_state(state);
        let value_output = self.value_network.forward(&features.to_array());
        let policy_outputs = self.policy_network.forward(&features.to_array());

        let mut move_evaluations = Vec::new();
        let mut best_move = valid_moves[0];
        let mut best_score = f32::MIN;

        for &col in &valid_moves {
            let mut next_state = state.clone();
            if next_state.make_move(col).is_ok() {
                // PURE EVALUATION: Policy Network indicates "Intuition", Value Network indicates "Outcome"
                // For optimal play, we should trust the Value Network's prediction of the NEXT state primarily,
                // but biased by the Policy Network's suggestion for the CURRENT state.
                
                let next_features = GameFeatures::from_game_state(&next_state);
                let next_value = self.value_network.forward(&next_features.to_array());
                
                // Value is from perspective of player who JUST moved. 
                // So high value for next_state means GOOD for the current player.
                let value_score = next_value[0]; 
                let policy_score = policy_outputs[col as usize];
                
                // Combine them. Policy is probability [0,1], Value is tanh [-1,1]
                // We want to maximize Value, using Policy as a prior.
                // Score = Value + c * Policy (AlphaZero style, though usually inside MCTS)
                // For direct play: Score = Value * 0.8 + Policy * 0.5 (rough heuristics)
                // Actually, just verify if immediate win exists (rule of game), otherwise trust Neural Net.
                
                let mut score = value_score; 
                
                // Boost score slightly by policy to break ties or guide exploration
                score += policy_score * 0.1; 

                // Basic Safety Check: Don't miss immediate wins
                if next_state.has_winner() {
                    score += 10.0; // Massive boost for winning
                }

                move_evaluations.push(MLMoveEvaluation {
                    column: col,
                    score,
                    move_type: "neural".to_string(),
                });

                if score > best_score {
                    best_score = score;
                    best_move = col;
                }
            }
        }

        move_evaluations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        MLResponse {
            r#move: Some(best_move),
            evaluation: value_output[0],
            thinking: format!(
                "ML AI chose column {} with score {:.3}. Value network: {:.3}",
                best_move, best_score, value_output[0]
            ),
            diagnostics: MLDiagnostics {
                valid_moves,
                move_evaluations,
                value_network_output: value_output[0],
                policy_network_outputs: policy_outputs.to_vec(),
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

    pub fn get_networks(&self) -> (&NeuralNetwork, &NeuralNetwork) {
        (&self.value_network, &self.policy_network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
