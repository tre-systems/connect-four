use connect_four_ai_core::{
    features::GameFeatures, ml_ai::MLAI, solver::{Bitboard, Solver},
    GameState, COLS, ROWS
};
use ndarray::Array1;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    println!("🚀 Starting ML AI Supervised Training Pipeline - Phase 2 (Solver Parity)");
    let start_time = Instant::now();

    // 1. Initialize Model
    let mut ml_ai = MLAI::new();
    println!("✅ Model initialized with architecture [256, 128, 64]");

    // 2. Generate Dataset
    const NUM_RAW_SAMPLES: usize = 10000; // Will be 20,000 with symmetry (~20 min total)
    const SOLVER_DEPTH: i32 = 12; // Balance of quality vs speed
    println!("📊 Generating {} raw samples ({} total with symmetry) using Bitboard Solver (Depth {})...", 
        NUM_RAW_SAMPLES, NUM_RAW_SAMPLES * 2, SOLVER_DEPTH);
    
    use std::sync::atomic::{AtomicUsize, Ordering};
    let progress_counter = AtomicUsize::new(0);
    let progress_interval = NUM_RAW_SAMPLES / 20; // Log every 5%
    
    let dataset: Vec<(Vec<f32>, f32, Vec<f32>)> = (0..NUM_RAW_SAMPLES)
        .into_par_iter()
        .flat_map(|_| {
            let mut solver = Solver::new();
            let mut state = GameState::new_random_first_player();
            
            // Randomize board state more aggressively for diversity
            let num_random_moves = rand::random::<usize>() % 25;
            for _ in 0..num_random_moves {
                if state.is_game_over() { break; }
                let moves = state.get_valid_moves();
                if moves.is_empty() { break; }
                let mv = moves[rand::random::<usize>() % moves.len()];
                let _ = state.make_move(mv);
            }

            if state.is_game_over() || state.get_valid_moves().is_empty() { 
                state = GameState::new_random_first_player();
            }

            let bitboard = Bitboard::from_game_state(&state);
            
            // Teacher Labels
            let core_evals = solver.analyze_all(&bitboard, SOLVER_DEPTH);
            
            // Progress logging
            let count = progress_counter.fetch_add(1, Ordering::Relaxed);
            if count % progress_interval == 0 {
                let pct = (count * 100) / NUM_RAW_SAMPLES;
                eprintln!("   📈 Progress: {}% ({}/{} samples)", pct, count, NUM_RAW_SAMPLES);
            }
            
            // --- Original State ---
            let f_orig = GameFeatures::from_game_state(&state).to_array().to_vec();
            let (v_orig, p_orig) = process_labels(&core_evals);

            // --- Mirrored State ---
            let mirrored_state = mirror_state(&state);
            let f_mirr = GameFeatures::from_game_state(&mirrored_state).to_array().to_vec();
            let p_mirr = mirror_policy(&p_orig);
            
            vec![(f_orig, v_orig, p_orig), (f_mirr, v_orig, p_mirr)]
        })
        .collect();

    println!("✅ Dataset generation complete ({} samples). Duration: {:?}", dataset.len(), start_time.elapsed());

    // 3. Training Loop
    println!("🧠 Starting Phase 2 Training (100 Epochs with LR Decay)...");
    let initial_lr = 0.001;
    let epochs = 100;
    let total_samples = dataset.len();
    
    for epoch in 1..=epochs {
        // LR Schedule: Decay at 50% and 80% of training
        let current_lr = if epoch > 80 {
            initial_lr * 0.01
        } else if epoch > 50 {
            initial_lr * 0.1
        } else {
            initial_lr
        };

        let mut total_value_loss = 0.0;
        let mut total_policy_loss = 0.0;
        
        for (features, value_label, policy_label) in &dataset {
            let input = Array1::from_vec(features.clone());
            
            // Train Value Network
            let v_target = Array1::from_vec(vec![*value_label]);
            total_value_loss += ml_ai.value_network.train_step(&input, &v_target, current_lr);
            
            // Train Policy Network
            let p_target = Array1::from_vec(policy_label.clone());
            total_policy_loss += ml_ai.policy_network.train_step(&input, &p_target, current_lr);
        }
        
        if epoch % 10 == 0 || epoch == 1 {
            println!("Epoch {:3}/{}: LR: {:.6}, Value Loss: {:.5}, Policy Loss: {:.5}", 
                epoch, epochs, current_lr, total_value_loss / total_samples as f32, total_policy_loss / total_samples as f32);
        }
    }

    // 4. Save Optimized Weights
    save_model(&ml_ai, total_samples, epochs);
    println!("🎉 Phase 2 Training Complete! Total Time: {:?}", start_time.elapsed());
}

fn process_labels(evals: &Vec<(usize, i32)>) -> (f32, Vec<f32>) {
    let mut best_score = -100.0;
    let mut policy_scores = vec![-100.0; 7];
    
    for &(col, score) in evals {
        let norm_score = (score as f32).max(-15.0).min(15.0) / 20.0; // Slightly different normalization
        if norm_score > best_score {
            best_score = norm_score;
        }
        policy_scores[col] = (score as f32).max(-20.0).min(20.0);
    }
    
    let max_p = policy_scores.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let exp_sum: f32 = policy_scores.iter().filter(|&&s| s > -50.0).map(|s| (s - max_p).exp()).sum();
    let p_label: Vec<f32> = policy_scores.iter().map(|&s| {
        if s < -50.0 { 0.0 } else { (s - max_p).exp() / exp_sum }
    }).collect();

    (best_score, p_label)
}

fn mirror_state(state: &GameState) -> GameState {
    let mut mirrored = state.clone();
    for col in 0..COLS {
        for row in 0..ROWS {
            mirrored.board[col][row] = state.board[COLS - 1 - col][row];
        }
    }
    mirrored
}

fn mirror_policy(policy: &[f32]) -> Vec<f32> {
    let mut mirrored = vec![0.0; 7];
    for i in 0..7 {
        mirrored[i] = policy[6 - i];
    }
    mirrored
}

fn save_model(ml_ai: &MLAI, samples: usize, epochs: usize) {
    let mut weights_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    weights_path.push("../../public/ml/data/weights/ml_ai_weights_best.json");
    
    if let Some(parent) = weights_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let value_weights = ml_ai.value_network.save_weights();
    let policy_weights = ml_ai.policy_network.save_weights();

    let json = serde_json::json!({
        "metadata": {
            "training_date": chrono::Local::now().to_rfc3339(),
            "phase": 2,
            "samples": samples,
            "epochs": epochs,
            "architecture": [256, 128, 64],
            "teacher": "BitboardSolver",
            "teacher_depth": 18
        },
        "value_network": { "weights": value_weights },
        "policy_network": { "weights": policy_weights }
    });

    let _ = fs::write(&weights_path, serde_json::to_string_pretty(&json).unwrap());
    println!("💾 Saved Phase 2 weights to: {:?}", weights_path);
}
