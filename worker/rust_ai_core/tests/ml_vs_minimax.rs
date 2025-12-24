use connect_four_ai_core::{genetic_params::GeneticParams, ml_ai::MLAI, GameState, Player, AI};
use std::time::Instant;
use serde_json;

fn get_evolved_params() -> GeneticParams {
    GeneticParams::load_from_file("ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default())
}

#[test]
fn test_ml_vs_minimax_ai() {
    println!("🤖 ML vs Minimax AI Test");
    println!("{}", "=".repeat(40));

    let evolved_params = get_evolved_params();
    println!("📋 Using evolved genetic parameters");

    let mut ml_ai = MLAI::new();
    
    // Try to load trained weights
    if let Ok(weights_data) = std::fs::read_to_string("../../ml/data/weights/simple_model_enhanced.json") {
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
                    println!("   Value weights: {} values", value_weights.len());
                    println!("   Policy weights: {} values", policy_weights.len());
                } else {
                    println!("❌ Failed to extract weights from model");
                    println!("   Value weights: {} values", value_weights.len());
                    println!("   Policy weights: {} values", policy_weights.len());
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
    let num_games = std::env::var("NUM_GAMES")
        .unwrap_or_else(|_| "20".to_string())
        .parse::<usize>()
        .unwrap_or(20);

    println!("🎮 Playing {} games...", num_games);

    let mut ml_wins = 0;
    let mut mm_wins = 0;
    let mut total_moves = 0;
    let mut ml_total_time = 0;
    let mut mm_total_time = 0;

    for game_num in 0..num_games {
        let mut game_state = GameState::with_genetic_params(evolved_params.clone());
        let mut moves_played = 0;
        let max_moves = 42; // Maximum moves in Connect Four (6x7 board)
        let mut ml_time = 0;
        let mut mm_time = 0;

        while !game_state.is_game_over() && moves_played < max_moves {
            let start_time = Instant::now();
            let best_move = if game_state.current_player == Player::Player1 {
                // ML AI uses the same evolved genetic parameters as the game state
                let response = ml_ai.get_best_move(&game_state);
                response.r#move
            } else {
                let mut mm_ai = AI::new();
                let (move_option, _) = mm_ai.get_best_move(&game_state, 3);
                move_option
            };
            let end_time = Instant::now();
            let move_time = end_time.duration_since(start_time).as_millis() as u64;

            if game_state.current_player == Player::Player1 {
                ml_time += move_time;
            } else {
                mm_time += move_time;
            }

            if let Some(column) = best_move {
                if game_state.make_move(column).is_err() {
                    // No valid moves, game is a draw
                    break;
                }
            } else {
                // No valid moves, game is a draw
                break;
            }

            moves_played += 1;
        }

        // Determine winner
        if let Some(winner) = game_state.get_winner() {
            if winner == Player::Player1 {
                ml_wins += 1;
            } else {
                mm_wins += 1;
            }
        } else {
            // Game ended in draw, evaluate final position
            let final_eval = game_state.evaluate();
            if final_eval > 0 {
                mm_wins += 1; // MM (Player2) wins
            } else {
                ml_wins += 1; // ML (Player1) wins
            }
        }

        total_moves += moves_played;
        ml_total_time += ml_time;
        mm_total_time += mm_time;

        if (game_num + 1) % 10 == 0 {
            println!("Completed {} games...", game_num + 1);
        }
    }

    let ml_win_rate = (ml_wins as f64 / num_games as f64) * 100.0;
    let mm_win_rate = (mm_wins as f64 / num_games as f64) * 100.0;
    let avg_moves = total_moves as f64 / num_games as f64;
    let ml_avg_time = ml_total_time as f64 / num_games as f64;
    let mm_avg_time = mm_total_time as f64 / num_games as f64;

    println!("\n📊 Results:");
    println!("{}", "=".repeat(30));
    println!("ML AI wins: {} ({:.1}%)", ml_wins, ml_win_rate);
    println!("MM AI wins: {} ({:.1}%)", mm_wins, mm_win_rate);
    println!("Average moves per game: {:.1}", avg_moves);
    println!("ML AI avg time per game: {:.1}ms", ml_avg_time);
    println!("MM AI avg time per game: {:.1}ms", mm_avg_time);

    println!("\n🎯 Performance Analysis:");
    println!("{}", "=".repeat(25));

    if ml_win_rate > mm_win_rate + 5.0 {
        println!("✅ ML AI shows significant advantage!");
    } else if ml_win_rate > mm_win_rate {
        println!("✅ ML AI shows slight advantage");
    } else if ml_win_rate < mm_win_rate - 5.0 {
        println!("❌ MM AI (with evolved params) shows significant advantage");
    } else {
        println!("🤝 AI performance is roughly equal");
    }

    println!("\n⚡ Speed Analysis:");
    println!("{}", "=".repeat(20));
    if ml_avg_time < mm_avg_time * 0.5 {
        println!("🚀 ML AI is significantly faster");
    } else if ml_avg_time < mm_avg_time {
        println!("⚡ ML AI is faster");
    } else if ml_avg_time > mm_avg_time * 2.0 {
        println!("🐌 ML AI is significantly slower");
    } else {
        println!("⚖️  AI speeds are comparable");
    }

    println!("\n📈 Recommendations:");
    println!("{}", "=".repeat(20));
    if ml_win_rate > 55.0 && ml_avg_time < mm_avg_time {
        println!("🎉 ML AI is ready for production use!");
    } else if ml_win_rate > 50.0 {
        println!("✅ ML AI shows promise, consider further training");
    } else {
        println!("🔧 ML AI needs improvement, review training data and parameters");
    }
}
