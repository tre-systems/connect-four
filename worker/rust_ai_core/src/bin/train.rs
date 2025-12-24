use connect_four_ai_core::{
    features::GameFeatures, ml_ai::MLAI, solver::{Bitboard, Solver},
    GameState, COLS, ROWS
};
use rand::seq::SliceRandom;
use ndarray::{Array1, Array2};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::RefCell;

thread_local! {
    static THREAD_SOLVER: RefCell<Solver> = RefCell::new(Solver::new());
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TrainingSample {
    features: Vec<f32>,
    value: f32,
    policy: Vec<f32>,
}

fn main() {
    println!("🚀 Starting ML AI Supervised Training Pipeline - Phase 3 (Enhanced)");
    let start_time = Instant::now();

    // 1. Initialize Model
    let mut ml_ai = MLAI::new();
    println!("✅ Model initialized with 4x128 ResNet-lite architecture");

    // 2. Load or Generate Dataset
    const NUM_RAW_SAMPLES: usize = 250000; // Will be 500,000 with symmetry
    const SOLVER_DEPTH: i32 = 12; 
    
    let dataset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../public/ml/data/training/dataset_p4.json");
    let mut dataset: Vec<(Vec<f32>, f32, Vec<f32>)>;

    if dataset_path.exists() {
        println!("📂 Loading existing dataset from: {:?}", dataset_path);
        let data = fs::read_to_string(&dataset_path).expect("Failed to read dataset file");
        let samples: Vec<TrainingSample> = serde_json::from_str(&data).expect("Failed to parse dataset");
        dataset = samples.into_iter().map(|s| (s.features, s.value, s.policy)).collect();
        println!("✅ Loaded {} samples from disk", dataset.len());
    } else {
        // Curriculum: distribute across game phases
        const EARLY_GAME_RATIO: f32 = 0.40; // 0-8 moves
        const MID_GAME_RATIO: f32 = 0.40;   // 9-20 moves
        const LATE_GAME_RATIO: f32 = 0.20;  // 21+ moves
        
        println!("📊 Generating {} raw samples ({} total with symmetry)", NUM_RAW_SAMPLES, NUM_RAW_SAMPLES * 2);
        println!("   🎯 Solver Depth: {}", SOLVER_DEPTH);
        println!("   📚 Curriculum: {:.0}% early, {:.0}% mid, {:.0}% late game", 
            EARLY_GAME_RATIO * 100.0, MID_GAME_RATIO * 100.0, LATE_GAME_RATIO * 100.0);
        
        let progress_counter = AtomicUsize::new(0);
        let progress_interval = NUM_RAW_SAMPLES / 1000; // Log every 0.1%
        let gen_start = Instant::now();
        
        // Print initial progress immediately
        eprintln!("   📈 Progress:   0% (    0/{} samples) | Starting...", NUM_RAW_SAMPLES);
        
        dataset = (0..NUM_RAW_SAMPLES)
            .into_par_iter()
        .flat_map(|sample_idx| {
            THREAD_SOLVER.with(|s| {
                let mut solver = s.borrow_mut();
                let mut state = GameState::new_random_first_player();
                
                // Curriculum Learning: determine game phase for this sample
                let phase_selector = (sample_idx as f32) / (NUM_RAW_SAMPLES as f32);
                let num_random_moves = if phase_selector < EARLY_GAME_RATIO {
                    rand::random::<usize>() % 9
                } else if phase_selector < EARLY_GAME_RATIO + MID_GAME_RATIO {
                    9 + rand::random::<usize>() % 12
                } else {
                    21 + rand::random::<usize>() % 10
                };
                
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
                
                let count = progress_counter.fetch_add(1, Ordering::Relaxed);
                
                // Periodically reset solver if cache grows too large
                if solver.tt_size() > 500_000 {
                    solver.reset();
                }

                let core_evals = solver.analyze_all(&bitboard, SOLVER_DEPTH);
                if count % progress_interval == 0 && count > 0 {
                    let pct = (count as f32 * 100.0) / NUM_RAW_SAMPLES as f32;
                    let elapsed = gen_start.elapsed().as_secs_f32();
                    let rate = count as f32 / elapsed;
                    let remaining = (NUM_RAW_SAMPLES - count) as f32 / rate;
                    eprintln!("   📈 Progress: {:5.1}% ({:6}/{} samples) | {:.1} samples/sec | ETA: {:.0}s", 
                        pct, count, NUM_RAW_SAMPLES, rate, remaining);
                }
                
                let f_orig = GameFeatures::from_game_state(&state).to_array().to_vec();
                let (v_orig, p_orig) = process_labels(&core_evals);

                let mirrored_state = mirror_state(&state);
                let f_mirr = GameFeatures::from_game_state(&mirrored_state).to_array().to_vec();
                let p_mirr = mirror_policy(&p_orig);
                
                vec![(f_orig, v_orig, p_orig), (f_mirr, v_orig, p_mirr)]
            })
        })
    .collect();

        let gen_duration = gen_start.elapsed();
        println!("✅ Dataset generation complete ({} samples). Duration: {:?}", dataset.len(), gen_duration);

        // Save dataset for future runs
        println!("💾 Saving dataset to: {:?}", dataset_path);
        let samples_to_save: Vec<TrainingSample> = dataset.iter().map(|(f, v, p)| TrainingSample {
            features: f.clone(),
            value: *v,
            policy: p.clone(),
        }).collect();
        if let Some(parent) = dataset_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string(&samples_to_save).expect("Failed to serialize dataset");
        fs::write(&dataset_path, json).expect("Failed to write dataset to disk");
        println!("✅ Dataset saved successfully");
    }



    // 3. Training Loop with LR Warmup and Decay
    const EPOCHS: usize = 50;
    const WARMUP_EPOCHS: usize = 10;
    const BASE_LR: f32 = 0.001;
    const WARMUP_LR: f32 = 0.0001;
    
    println!("🧠 Starting Phase 3 Training ({} Epochs with Warmup + Decay)...", EPOCHS);
    println!("   📈 LR Schedule: Warmup {:.4} → Base {:.3} → Decay", WARMUP_LR, BASE_LR);
    
    let train_start = Instant::now();
    let total_samples = dataset.len();
    dataset.shuffle(&mut rand::thread_rng());
    println!("🎲 Dataset shuffled ({} samples)", total_samples);
    
    for epoch in 1..=EPOCHS {
        // LR Schedule: Warmup → Base → Decay at 60% and 85%
        let current_lr = if epoch <= WARMUP_EPOCHS {
            // Linear warmup from WARMUP_LR to BASE_LR
            WARMUP_LR + (BASE_LR - WARMUP_LR) * (epoch as f32 / WARMUP_EPOCHS as f32)
        } else if epoch > (EPOCHS * 85 / 100) {
            BASE_LR * 0.01  // Final decay
        } else if epoch > (EPOCHS * 60 / 100) {
            BASE_LR * 0.1   // First decay
        } else {
            BASE_LR
        };

        let mut total_value_loss = 0.0;
        let mut total_policy_loss = 0.0;
        
        const BATCH_SIZE: usize = 128;
        
        for batch in dataset.chunks(BATCH_SIZE) {
            // Process batch in parallel to compute gradients
            let batch_results: Vec<((f32, Vec<(Array2<f32>, Array1<f32>)>), (f32, Vec<(Array2<f32>, Array1<f32>)>))> = batch
                .into_par_iter()
                .map(|(features, value_label, policy_label)| {
                    let input = Array1::from_vec(features.clone());
                    
                    // Value Network Gradients
                    let v_target = Array1::from_vec(vec![*value_label]);
                    let v_res = ml_ai.value_network.compute_gradients(&input, &v_target);
                    
                    // Policy Network Gradients
                    let p_target = Array1::from_vec(policy_label.clone());
                    let p_res = ml_ai.policy_network.compute_gradients(&input, &p_target);
                    
                    (v_res, p_res)
                })
                .collect();

            // Accumulate Batch Value Gradients
            let mut v_batch_grads: Option<Vec<(Array2<f32>, Array1<f32>)>> = None;
            for ((v_loss, v_grads), _) in &batch_results {
                total_value_loss += v_loss;
                if let Some(ref mut acc) = v_batch_grads {
                    for i in 0..v_grads.len() {
                        acc[i].0 += &v_grads[i].0;
                        acc[i].1 += &v_grads[i].1;
                    }
                } else {
                    v_batch_grads = Some(v_grads.clone());
                }
            }

            // Accumulate Batch Policy Gradients
            let mut p_batch_grads: Option<Vec<(Array2<f32>, Array1<f32>)>> = None;
            for (_, (p_loss, p_grads)) in &batch_results {
                total_policy_loss += p_loss;
                if let Some(ref mut acc) = p_batch_grads {
                    for i in 0..p_grads.len() {
                        acc[i].0 += &p_grads[i].0;
                        acc[i].1 += &p_grads[i].1;
                    }
                } else {
                    p_batch_grads = Some(p_grads.clone());
                }
            }

            // Apply Average Gradients to Networks
            if let Some(mut v_grads) = v_batch_grads {
                for i in 0..v_grads.len() {
                    v_grads[i].0 /= batch.len() as f32;
                    v_grads[i].1 /= batch.len() as f32;
                }
                ml_ai.value_network.apply_gradients(&v_grads, current_lr);
            }

            if let Some(mut p_grads) = p_batch_grads {
                for i in 0..p_grads.len() {
                    p_grads[i].0 /= batch.len() as f32;
                    p_grads[i].1 /= batch.len() as f32;
                }
                ml_ai.policy_network.apply_gradients(&p_grads, current_lr);
            }
        }
        
        // Log every 5 epochs, plus first and last
        if epoch % 5 == 0 || epoch == 1 || epoch == EPOCHS {
            let elapsed = train_start.elapsed().as_secs_f32();
            let eta = elapsed / (epoch as f32) * ((EPOCHS - epoch) as f32);
            println!("Epoch {:3}/{}: LR: {:.6}, V_Loss: {:.5}, P_Loss: {:.5} | ETA: {:.0}s", 
                epoch, EPOCHS, current_lr, 
                total_value_loss / total_samples as f32, 
                total_policy_loss / total_samples as f32,
                eta);
        }
    }

    // 4. Save Optimized Weights
    save_model(&ml_ai, total_samples, EPOCHS);
    println!("🎉 Phase 3 Training Complete! Total Time: {:?}", start_time.elapsed());
}

fn process_labels(evals: &Vec<(usize, i32)>) -> (f32, Vec<f32>) {
    let mut best_score = -100.0;
    let mut policy_scores = vec![-100.0; 7];
    
    for &(col, score) in evals {
        // Normalize score to [-1, 1] based on max board capacity (21 moves per player)
        let norm_score = (score as f32).max(-21.0).min(21.0) / 21.0;
        if norm_score > best_score {
            best_score = norm_score;
        }
        policy_scores[col] = (score as f32).max(-30.0).min(30.0);
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
            "phase": 3,
            "samples": samples,
            "epochs": epochs,
            "architecture": [256, 128, 64],
            "teacher": "BitboardSolver",
            "teacher_depth": 18,
            "curriculum": "40% early, 40% mid, 20% late game",
            "lr_schedule": "warmup + decay"
        },
        "value_network": { "weights": value_weights },
        "policy_network": { "weights": policy_weights }
    });

    let _ = fs::write(&weights_path, serde_json::to_string_pretty(&json).unwrap());
    println!("💾 Saved Phase 3 weights to: {:?}", weights_path);
}
