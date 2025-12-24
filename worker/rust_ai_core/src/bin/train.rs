use connect_four_ai_core::{
    features::GameFeatures, ml_ai::MLAI, solver::{Bitboard, Solver},
    GameState, Player, COLS
};
use ndarray::Array1;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    println!("🚀 Starting ML AI Supervised Training Pipeline");
    let start_time = Instant::now();

    // 1. Initialize Model
    let mut ml_ai = MLAI::new();
    println!("✅ Model initialized with architecture [256, 128, 64]");

    // 2. Generate Dataset
    const NUM_SAMPLES: usize = 10000;
    println!("📊 Generating {} training samples using Bitboard Solver teacher...", NUM_SAMPLES);
    
    let dataset: Vec<(Vec<f32>, f32, Vec<f32>)> = (0..NUM_SAMPLES)
        .into_par_iter()
        .map(|_| {
            let mut solver = Solver::new();
            let mut state = GameState::new_random_first_player();
            
            // Randomize board state a bit by playing some moves
            let num_random_moves = rand::random::<usize>() % 15;
            for _ in 0..num_random_moves {
                if state.is_game_over() { break; }
                let moves = state.get_valid_moves();
                if moves.is_empty() { break; }
                let mv = moves[rand::random::<usize>() % moves.len()];
                let _ = state.make_move(mv);
            }

            if state.is_game_over() { 
                // If game ended, try again with fresh state
                state = GameState::new_random_first_player();
            }

            let features = GameFeatures::from_game_state(&state).to_array().to_vec();
            let bitboard = Bitboard::from_game_state(&state);
            
            // Get teacher labels from Solver (depth 12 for high quality)
            let evaluations = solver.analyze_all(&bitboard, 12);
            
            // Value Label: Best score found
            let mut best_score = -100.0;
            let mut policy_scores = vec![-100.0; 7];
            
            for (col, score) in evaluations {
                let norm_score = (score as f32).max(-10.0).min(10.0) / 10.0;
                if norm_score > best_score {
                    best_score = norm_score;
                }
                policy_scores[col] = (score as f32).max(-10.0).min(10.0);
            }
            
            // Policy Label: Softmax over scores
            let max_p = policy_scores.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let exp_sum: f32 = policy_scores.iter().filter(|&&s| s > -50.0).map(|s| (s - max_p).exp()).sum();
            let policy_label: Vec<f32> = policy_scores.iter().map(|&s| {
                if s < -50.0 { 0.0 } else { (s - max_p).exp() / exp_sum }
            }).collect();

            (features, best_score, policy_label)
        })
        .collect();

    println!("✅ Dataset generation complete. Duration: {:?}", start_time.elapsed());

    // 3. Training Loop
    println!("🧠 Starting Supervised Training...");
    let learning_rate = 0.001;
    let epochs = 50;
    
    for epoch in 1..=epochs {
        let mut total_value_loss = 0.0;
        let mut total_policy_loss = 0.0;
        
        for (features, value_label, policy_label) in &dataset {
            let input = Array1::from_vec(features.clone());
            
            // Train Value Network
            let v_target = Array1::from_vec(vec![*value_label]);
            total_value_loss += ml_ai.value_network.train_step(&input, &v_target, learning_rate);
            
            // Train Policy Network
            let p_target = Array1::from_vec(policy_label.clone());
            total_policy_loss += ml_ai.policy_network.train_step(&input, &p_target, learning_rate);
        }
        
        if epoch % 5 == 0 || epoch == 1 {
            println!("Epoch {:2}/{}: Value Loss: {:.4}, Policy Loss: {:.4}", 
                epoch, epochs, total_value_loss / NUM_SAMPLES as f32, total_policy_loss / NUM_SAMPLES as f32);
        }
    }

    // 4. Save Weights
    let mut weights_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    weights_path.push("../../public/ml/data/weights/ml_ai_weights_best.json");
    
    // Create directory if it doesn't exist
    if let Some(parent) = weights_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let value_weights = ml_ai.value_network.save_weights();
    let policy_weights = ml_ai.policy_network.save_weights();

    let json = serde_json::json!({
        "metadata": {
            "training_date": chrono::Local::now().to_rfc3339(),
            "samples": NUM_SAMPLES,
            "epochs": epochs,
            "architecture": [256, 128, 64]
        },
        "value_network": { "weights": value_weights },
        "policy_network": { "weights": policy_weights }
    });

    match fs::write(&weights_path, serde_json::to_string_pretty(&json).unwrap()) {
        Ok(_) => println!("💾 Saved optimized weights to: {:?}", weights_path),
        Err(e) => eprintln!("❌ Failed to save weights: {}", e),
    }

    println!("🎉 Training Pipeline Finished! Total Time: {:?}", start_time.elapsed());
}
