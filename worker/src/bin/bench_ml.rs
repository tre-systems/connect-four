use connect_four_ai_core::ml_ai::MLAI;
use connect_four_ai_core::{GameState};
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut ml_ai = MLAI::new();
    ml_ai.mcts_simulations = 4000;

    // Load weights
    let mut weights_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    weights_path.push("../../public/ml/data/weights/ml_ai_weights_best.json");

    let content = fs::read_to_string(weights_path).expect("Failed to read weights file");
    let json: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");

    let value_weights: Vec<f32> = serde_json::from_value(json["value_network"]["weights"].clone()).unwrap();
    let policy_weights: Vec<f32> = serde_json::from_value(json["policy_network"]["weights"].clone()).unwrap();

    ml_ai.load_weights(&value_weights, &policy_weights);
    println!("✅ Weights loaded.");

    let mut state = GameState::new();

    // Move 1: Red (P1) plays Col 3
    println!("🔴 Red plays column 3");
    state.make_move(3).unwrap();

    // Move 2: Yellow (P2 / ML AI) plays
    println!("🚀 ML AI thinking for Yellow...");
    let res = ml_ai.get_best_move(&state);
    println!("💭 Thinking: {}", res.thinking);

    // In the browser, it chose Col 4. Let's see if our local one chooses Col 4.
    if let Some(mv) = res.r#move {
        state.make_move(mv).unwrap();
    }
}
