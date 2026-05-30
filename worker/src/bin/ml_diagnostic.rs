use connect_four_ai_core::{ml_ai::MLAI, GameState, Player, COLS, ROWS};
use connect_four_ai_core::features::GameFeatures;
use std::fs;

fn main() {
    println!("🔍 ML AI Diagnostic Tool");
    println!("========================");

    let mut ml_ai = MLAI::new();

    // Load weights
    let weights_path = "../../public/ml/data/weights/ml_ai_weights_best.json";
    if let Ok(weights_data) = fs::read_to_string(weights_path) {
        if let Ok(weights) = serde_json::from_str::<serde_json::Value>(&weights_data) {
            if let (Some(v_obj), Some(p_obj)) = (weights.get("value_network"), weights.get("policy_network")) {
                let v_weights_val = v_obj.get("weights").unwrap();
                let p_weights_val = p_obj.get("weights").unwrap();
                let value_weights: Vec<f32> = v_weights_val.as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
                let policy_weights: Vec<f32> = p_weights_val.as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
                ml_ai.load_weights(&value_weights, &policy_weights);
                println!("✅ Successfully loaded weights");
            }
        }
    } else {
        println!("❌ Failed to load weights from {weights_path}");
        return;
    }

    // Diagnostic Case 1: Empty board
    let state = GameState::new();
    inspect_state(&mut ml_ai, &state, "Empty Board");

    // Diagnostic Case 2: Vertical threat (P1 has 3 in a row in col 3)
    let mut state2 = GameState::new();
    state2.current_player = Player::Player1;
    state2.make_move(3).unwrap(); // P1
    state2.make_move(0).unwrap(); // P2 (waste move)
    state2.make_move(3).unwrap(); // P1
    state2.make_move(0).unwrap(); // P2
    state2.make_move(3).unwrap(); // P1
    state2.make_move(0).unwrap(); // P2
    inspect_state(&mut ml_ai, &state2, "Vertical Win Threat (Col 3)");

    // Diagnostic Case 3: Blocking move (P2 has 3 in a row, P1 must block)
    let mut state3 = GameState::new();
    state3.current_player = Player::Player1;
    state3.make_move(0).unwrap(); // P1 (waste)
    state3.make_move(3).unwrap(); // P2
    state3.make_move(0).unwrap(); // P1
    state3.make_move(3).unwrap(); // P2
    state3.make_move(0).unwrap(); // P1
    state3.make_move(3).unwrap(); // P2
    inspect_state(&mut ml_ai, &state3, "Blocking Move Required (Col 3)");
}

fn inspect_state(ai: &mut MLAI, state: &GameState, label: &str) {
    println!("\n--- {label} ---");
    print_board(state);

    let features = GameFeatures::from_game_state(state);
    let value = ai.value_network.forward(&features.to_array())[0];
    let policy = ai.policy_network.forward(&features.to_array());

    println!("Value Network Output: {:.4} (Relative to {:?})", value, state.current_player);
    println!("Policy Probabilities:");
    for i in 0..7 {
        println!("  Col {}: {:.4}", i, policy[i]);
    }

    let response = ai.get_best_move(state);
    println!("MCTS Decision: Col {} (Eval: {:.4})", response.r#move.unwrap_or(99), response.evaluation);
    println!("Thinking: {}", response.thinking);
}

fn print_board(state: &GameState) {
    for row in 0..ROWS {
        for col in 0..COLS {
            match state.board[col][row] {
                connect_four_ai_core::Cell::Empty => print!(". "),
                connect_four_ai_core::Cell::Player1 => print!("X "),
                connect_four_ai_core::Cell::Player2 => print!("O "),
            }
        }
        println!();
    }
}
