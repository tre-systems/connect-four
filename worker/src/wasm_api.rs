#[cfg(feature = "wasm")]
use super::genetic_params::GeneticParams;
#[cfg(feature = "wasm")]
use super::{GameState, HeuristicAI, AI};
#[cfg(feature = "wasm")]
use super::ml_ai::MLAI;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsValue;
#[cfg(feature = "wasm")]
use serde_wasm_bindgen;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MoveEvaluationWasm {
    pub column: u8,
    pub score: f32,
    pub move_type: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct WasmBestMoveResponse {
    pub r#move: Option<u8>,
    pub evaluations: Vec<MoveEvaluationWasm>,
    pub nodes_evaluated: u32,
    pub transposition_hits: u32,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct WasmHeuristicResponse {
    pub r#move: Option<u8>,
    pub evaluations: Vec<MoveEvaluationWasm>,
    pub nodes_evaluated: u32,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct WasmMLResponse {
    pub r#move: Option<u8>,
    pub evaluation: f32,
    pub thinking: String,
    pub diagnostics: crate::ml_ai::MLDiagnostics,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct ConnectFourAI {
    ai: AI,
    heuristic_ai: HeuristicAI,
    ml_ai: MLAI,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ConnectFourAI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ConnectFourAI {
            ai: AI::new(),
            heuristic_ai: HeuristicAI::new(),
            ml_ai: MLAI::new(),
        }
    }
    pub fn get_best_move(&mut self, board_state: &JsValue, depth: u8) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (best_move, evaluations) = self.ai.get_best_move(&state, depth);

        let wasm_evaluations: Vec<MoveEvaluationWasm> = evaluations.into_iter().map(|e| MoveEvaluationWasm {
            column: e.column,
            score: e.score,
            move_type: e.move_type,
        }).collect();

        let response = WasmBestMoveResponse {
            r#move: best_move,
            evaluations: wasm_evaluations,
            nodes_evaluated: self.ai.nodes_evaluated,
            transposition_hits: self.ai.transposition_hits,
        };

        Ok(serde_wasm_bindgen::to_value(&response).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn get_heuristic_move(&mut self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (best_move, evaluations) = self.heuristic_ai.get_best_move(&state);

        let wasm_evaluations: Vec<MoveEvaluationWasm> = evaluations.into_iter().map(|e| MoveEvaluationWasm {
            column: e.column,
            score: e.score,
            move_type: e.move_type,
        }).collect();

        let response = WasmHeuristicResponse {
            r#move: best_move,
            evaluations: wasm_evaluations,
            nodes_evaluated: self.heuristic_ai.nodes_evaluated,
        };

        Ok(serde_wasm_bindgen::to_value(&response).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn get_ml_move(&mut self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if state.is_game_over() {
            return Err(JsValue::from_str("Game is already over"));
        }

        let valid_moves = state.get_valid_moves();
        if valid_moves.is_empty() {
            return Err(JsValue::from_str("No valid moves available"));
        }
        let ml_response = self.ml_ai.get_best_move(&state);

        #[cfg(feature = "wasm")]
        web_sys::console::log_1(&JsValue::from_str(&format!("🧠 ML AI: {}", ml_response.thinking)));

        let response = WasmMLResponse {
             r#move: ml_response.r#move,
             evaluation: ml_response.evaluation,
             thinking: ml_response.thinking,
             diagnostics: ml_response.diagnostics,
        };

        Ok(serde_wasm_bindgen::to_value(&response).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn evaluate_position(&self, board_state: &JsValue) -> Result<f32, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(state.evaluate() as f32)
    }

    pub fn evaluate_position_ml(&self, board_state: &JsValue) -> Result<f32, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(self.ml_ai.evaluate_position(&state))
    }

    pub fn get_valid_moves(&self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let moves = state.get_valid_moves();
        Ok(serde_wasm_bindgen::to_value(&moves).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn make_move(&self, board_state: &JsValue, column: u8) -> Result<JsValue, JsValue> {
        let mut state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        match state.make_move(column) {
            Ok(()) => {
                let result = serde_json::json!({
                    "success": true,
                    "new_state": state,
                });
                Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": e,
                });
                Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
        }
    }

    pub fn is_game_over(&self, board_state: &JsValue) -> Result<bool, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(state.is_game_over())
    }

    pub fn get_winner(&self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let winner = state.get_winner();
        Ok(serde_wasm_bindgen::to_value(&winner).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn create_new_game(&self) -> Result<JsValue, JsValue> {
        let state = GameState::new();
        Ok(serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn create_game_with_params(&self, params: &JsValue) -> Result<JsValue, JsValue> {
        let genetic_params: GeneticParams = serde_wasm_bindgen::from_value(params.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let state = GameState::with_genetic_params(genetic_params);
        Ok(serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn clear_transposition_table(&mut self) {
        self.ai.clear_transposition_table();
    }

    pub fn get_transposition_table_size(&self) -> usize {
        self.ai.get_transposition_table_size()
    }

    pub fn load_ml_weights(
        &mut self,
        value_weights: &JsValue,
        policy_weights: &JsValue,
    ) -> Result<(), JsValue> {
        let value_weights: Vec<f32> = serde_wasm_bindgen::from_value(value_weights.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let policy_weights: Vec<f32> = serde_wasm_bindgen::from_value(policy_weights.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        #[cfg(feature = "wasm")]
        web_sys::console::log_1(&JsValue::from_str(&format!(
            "📦 Loading ML Weights: Value={}, Policy={}", 
            value_weights.len(), 
            policy_weights.len()
        )));

        // Basic validation for 4x128 architecture
        // Value: 62593 (100*128 + 128 + 3*(128*128 + 128) + 128*1 + 1)
        // Policy: 63367 (100*128 + 128 + 3*(128*128 + 128) + 128*7 + 7)
        if value_weights.len() != 62593 || policy_weights.len() != 63367 {
             #[cfg(feature = "wasm")]
             web_sys::console::warn_1(&JsValue::from_str(&format!(
                "⚠️ ML Weight size mismatch! Expected V:62593 P:63367, Got V:{} P:{}", 
                value_weights.len(), 
                policy_weights.len()
            )));
        }

        self.ml_ai.load_weights(&value_weights, &policy_weights);
        
        #[cfg(feature = "wasm")]
        web_sys::console::log_1(&JsValue::from_str("✅ ML weights applied to networks"));
        
        Ok(())
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_ai_creation() {
        let ai = ConnectFourAI::new();
        assert_eq!(ai.get_transposition_table_size(), 0);
    }

    #[wasm_bindgen_test]
    fn test_new_game_creation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        assert!(!game_state.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_valid_moves_empty_board() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let valid_moves = ai.get_valid_moves(&game_state).unwrap();
        assert!(!valid_moves.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_make_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.make_move(&game_state, 3).unwrap();
        assert!(!result.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_game_over_detection() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let is_over = ai.is_game_over(&game_state).unwrap();
        assert!(!is_over); // Empty board should not be game over
    }

    #[wasm_bindgen_test]
    fn test_winner_detection() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let winner = ai.get_winner(&game_state).unwrap();
        assert!(!winner.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_position_evaluation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let evaluation = ai.evaluate_position(&game_state).unwrap();
        assert!(evaluation.is_finite());
    }

    #[wasm_bindgen_test]
    fn test_ml_position_evaluation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let evaluation = ai.evaluate_position_ml(&game_state).unwrap();
        assert!(evaluation.is_finite());
    }

    #[wasm_bindgen_test]
    fn test_heuristic_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.get_heuristic_move(&game_state).unwrap();
        assert!(!result.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_ml_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.get_ml_move(&game_state).unwrap();
        assert!(!result.is_undefined());
    }
}
