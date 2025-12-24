use connect_four_ai_core::{genetic_params::GeneticParams, ml_ai::MLAI, GameState};
use serde_json;

#[test]
fn test_ml_ai_threat_detection() {
    println!("🧪 Testing ML AI threat detection...");

    // Load evolved genetic parameters
    let evolved_params = GeneticParams::load_from_file("../../ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default());

    let mut ml_ai = MLAI::new();

    // Try to load trained weights
    if let Ok(weights_data) =
        std::fs::read_to_string("../../ml/data/weights/simple_model_enhanced.json")
    {
        if let Ok(weights) = serde_json::from_str::<serde_json::Value>(&weights_data) {
            if let (Some(value_network), Some(policy_network)) =
                (weights.get("value_network"), weights.get("policy_network"))
            {
                // Extract weights from the network structure
                let value_weights = extract_weights_from_network(value_network);
                let policy_weights = extract_weights_from_network(policy_network);

                if !value_weights.is_empty() && !policy_weights.is_empty() {
                    ml_ai.load_weights(&value_weights, &policy_weights);
                    println!("✅ Loaded trained ML weights");
                }
            }
        }
    }

    // Helper function to extract weights from network structure
    fn extract_weights_from_network(network: &serde_json::Value) -> Vec<f32> {
        let mut weights = Vec::new();
        if let Some(layers) = network.as_object() {
            for (layer_name, layer_data) in layers {
                if layer_name.contains("weight") {
                    if let Some(weight_array) = layer_data.as_array() {
                        for row in weight_array {
                            if let Some(row_array) = row.as_array() {
                                for weight in row_array {
                                    if let Some(weight_value) = weight.as_f64() {
                                        weights.push(weight_value as f32);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        weights
    }

    // Test case 1: ML AI should block opponent's immediate win
    println!("\n🔍 Test 1: ML AI should block opponent's immediate win");

    // Create a game state where Yellow can win on the next move
    let mut game_state = GameState::with_genetic_params(evolved_params.clone());

    // Set up the board to match the scenario from the logs
    // Red plays first, then Yellow, then Red, then Yellow can win
    game_state.make_move(0).unwrap(); // Red in column 0
    game_state.make_move(4).unwrap(); // Yellow in column 4
    game_state.make_move(0).unwrap(); // Red in column 0
    game_state.make_move(4).unwrap(); // Yellow in column 4
    game_state.make_move(0).unwrap(); // Red in column 0
    game_state.make_move(4).unwrap(); // Yellow in column 4
    game_state.make_move(1).unwrap(); // Red in column 1
    game_state.make_move(4).unwrap(); // Yellow in column 4 - now Yellow can win in column 4

    println!("Board state:");
    print_board(&game_state);

    // Now it's Red's turn (ML AI) - it should block column 4
    let response = ml_ai.get_best_move(&game_state);
    println!("ML AI chose column: {:?}", response.r#move);
    println!("ML AI thinking: {}", response.thinking);

    // Check if ML AI chose to block column 4
    if response.r#move == Some(4) {
        println!("✅ ML AI correctly blocked the immediate threat!");
    } else {
        println!("❌ ML AI failed to block the immediate threat!");
        println!("   Expected: column 4");
        println!("   Got: column {:?}", response.r#move);
    }

    // Test case 2: ML AI should make a winning move when available
    println!("\n🔍 Test 2: ML AI should make a winning move when available");

    let mut game_state2 = GameState::with_genetic_params(evolved_params.clone());

    // Set up a board where Red can win immediately
    game_state2.make_move(0).unwrap(); // Red in column 0
    game_state2.make_move(1).unwrap(); // Yellow in column 1
    game_state2.make_move(0).unwrap(); // Red in column 0
    game_state2.make_move(1).unwrap(); // Yellow in column 1
    game_state2.make_move(0).unwrap(); // Red in column 0
    game_state2.make_move(1).unwrap(); // Yellow in column 1

    println!("Board state:");
    print_board(&game_state2);

    // Now it's Red's turn (ML AI) - it should win in column 0
    let response2 = ml_ai.get_best_move(&game_state2);
    println!("ML AI chose column: {:?}", response2.r#move);
    println!("ML AI thinking: {}", response2.thinking);

    // Check if ML AI chose to win in column 0
    if response2.r#move == Some(0) {
        println!("✅ ML AI correctly made the winning move!");
    } else {
        println!("❌ ML AI failed to make the winning move!");
        println!("   Expected: column 0");
        println!("   Got: column {:?}", response2.r#move);
    }
}

fn print_board(state: &GameState) {
    for row in 0..6 {
        for col in 0..7 {
            match state.board[col][row] {
                connect_four_ai_core::Cell::Empty => print!("⚫"),
                connect_four_ai_core::Cell::Player1 => print!("🔴"),
                connect_four_ai_core::Cell::Player2 => print!("🟡"),
            }
        }
        println!();
    }
}
