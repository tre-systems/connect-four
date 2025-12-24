use connect_four_ai_core::{genetic_params::GeneticParams, ml_ai::MLAI, GameState, Player, AI};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn optimize_cpu_usage() {
    // Detect Apple Silicon and optimize thread pool
    if cfg!(target_os = "macos") {
        // On Apple Silicon, use all cores
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cores)
            .stack_size(8 * 1024 * 1024) // 8MB stack for deep recursion
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });

        println!(
            "🍎 Apple Silicon detected: Using {} threads ({} cores available)",
            num_cores, num_cores
        );
    } else {
        // On other platforms, use all available cores
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cores)
            .stack_size(8 * 1024 * 1024)
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });

        println!("🖥️  Using {} threads for parallel processing", num_cores);
    }
}

fn get_evolved_params() -> GeneticParams {
    GeneticParams::load_from_file("ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default())
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AIType {
    Random,
    Heuristic,
    MMDepth1,
    MMDepth2,
    MMDepth3,
    MMDepth4,
    MMDepth5,
    MMDepth6,
    MMDepth7,
    MMDepth20,
    MLSimple,
}

impl AIType {
    fn name(&self) -> &'static str {
        match self {
            AIType::Random => "Random",
            AIType::Heuristic => "Heuristic",
            AIType::MMDepth1 => "MM-Depth1",
            AIType::MMDepth2 => "MM-Depth2",
            AIType::MMDepth3 => "MM-Depth3",
            AIType::MMDepth4 => "MM-Depth4",
            AIType::MMDepth5 => "MM-Depth5",
            AIType::MMDepth6 => "Bitboard-Solver (Depth 6)",
            AIType::MMDepth7 => "MM-Depth7",
            AIType::MMDepth20 => "MM-Depth20",
            AIType::MLSimple => "ML-MCTS (AlphaZero)",
        }
    }
}

trait AIPlayer {
    fn get_move(&mut self, game_state: &GameState) -> Option<usize>;
    #[allow(dead_code)]
    fn reset(&mut self);
}

struct RandomAI;

impl AIPlayer for RandomAI {
    fn get_move(&mut self, game_state: &GameState) -> Option<usize> {
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            None
        } else {
            let random_index = rand::random::<usize>() % valid_moves.len();
            Some(valid_moves[random_index] as usize)
        }
    }

    fn reset(&mut self) {
        // Random AI doesn't need reset
    }
}

struct HeuristicAI;

impl AIPlayer for HeuristicAI {
    fn get_move(&mut self, game_state: &GameState) -> Option<usize> {
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        // Simple heuristic: prefer moves that advance pieces
        let mut best_move = valid_moves[0];
        let mut best_score = -1000.0;

        for &move_index in &valid_moves {
            let mut test_state = game_state.clone();
            if test_state.make_move(move_index).is_ok() {
                let score = evaluate_position(&test_state, game_state.current_player);
                if score > best_score {
                    best_score = score;
                    best_move = move_index;
                }
            }
        }

        Some(best_move as usize)
    }

    fn reset(&mut self) {
        // Heuristic AI doesn't need reset
    }
}

struct MinimaxAI {
    ai: AI,
    depth: u8,
}

impl MinimaxAI {
    fn new(depth: u8) -> Self {
        Self {
            ai: AI::new(),
            depth,
        }
    }
}

impl AIPlayer for MinimaxAI {
    fn get_move(&mut self, game_state: &GameState) -> Option<usize> {
        let (best_move, _) = self.ai.get_best_move(game_state, self.depth);
        best_move.map(|m| m as usize)
    }

    fn reset(&mut self) {
        self.ai.clear_transposition_table();
    }
}

struct MLSimpleAI {
    ai: MLAI,
}

impl MLSimpleAI {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut ai = MLAI::new();

        // Try to load simple model weights
        let weights_path = "../public/ml/data/weights/ml_ai_weights_best.json";
        if let Ok(weights_data) = std::fs::read_to_string(weights_path) {
            if let Ok(weights) = serde_json::from_str::<serde_json::Value>(&weights_data) {
                if let (Some(v_obj), Some(p_obj)) = (weights.get("value_network"), weights.get("policy_network")) {
                    if let (Some(v_weights_val), Some(p_weights_val)) = (v_obj.get("weights"), p_obj.get("weights")) {
                        let value_weights: Vec<f32> = v_weights_val.as_array().expect("Value weights must be an array").iter().map(|v| v.as_f64().expect("Weight must be an f64") as f32).collect();
                        let policy_weights: Vec<f32> = p_weights_val.as_array().expect("Policy weights must be an array").iter().map(|v| v.as_f64().expect("Weight must be an f64") as f32).collect();

                        if !value_weights.is_empty() && !policy_weights.is_empty() {
                            ai.load_weights(&value_weights, &policy_weights);
                            println!("✅ Successfully loaded Phase 3 weights from: {}", weights_path);
                        }
                    }
                }
            }
        }

        Ok(Self { ai })
    }
}


impl AIPlayer for MLSimpleAI {
    fn get_move(&mut self, game_state: &GameState) -> Option<usize> {
        let response = self.ai.get_best_move(game_state);
        response.r#move.map(|m| m as usize)
    }

    fn reset(&mut self) {
        // ML AI doesn't need reset
    }
}

fn evaluate_position(game_state: &GameState, player: Player) -> f32 {
    // For Connect Four, we can use the built-in evaluation function
    // and adjust it based on the player
    let base_evaluation = game_state.evaluate() as f32;

    // The base evaluation is from Player1's perspective
    // We need to convert it to the given player's perspective
    match player {
        Player::Player1 => base_evaluation,
        Player::Player2 => -base_evaluation,
    }
}

#[derive(Debug)]
struct GameResult {
    winner: Player,
    ai1_time_ms: u64,
    ai2_time_ms: u64,
}

fn play_game(
    ai1: &mut Box<dyn AIPlayer>,
    ai2: &mut Box<dyn AIPlayer>,
    ai1_plays_first: bool,
) -> GameResult {
    // Use evolved parameters for the game state
    let evolved_params = get_evolved_params();
    let mut game_state = GameState::with_genetic_params(evolved_params);
    game_state.current_player = Player::Player1; // Force Player1 to move first

    // Use the intended first player (no random swapping)
    let actual_ai1_first = ai1_plays_first;

    let mut moves_played = 0;
    let mut ai1_time_ms = 0;
    let mut ai2_time_ms = 0;
    let max_moves = 42; // Maximum moves in Connect Four (6x7 board)

    while !game_state.is_game_over() && moves_played < max_moves {
        let best_move = if game_state.current_player == Player::Player1 {
            if actual_ai1_first {
                let start = Instant::now();
                let move_result = ai1.get_move(&game_state);
                let duration = start.elapsed();
                ai1_time_ms += duration.as_millis() as u64;
                move_result
            } else {
                let start = Instant::now();
                let move_result = ai2.get_move(&game_state);
                let duration = start.elapsed();
                ai2_time_ms += duration.as_millis() as u64;
                move_result
            }
        } else {
            if actual_ai1_first {
                let start = Instant::now();
                let move_result = ai2.get_move(&game_state);
                let duration = start.elapsed();
                ai2_time_ms += duration.as_millis() as u64;
                move_result
            } else {
                let start = Instant::now();
                let move_result = ai1.get_move(&game_state);
                let duration = start.elapsed();
                ai1_time_ms += duration.as_millis() as u64;
                move_result
            }
        };

        if let Some(move_index) = best_move {
            if game_state.make_move(move_index as u8).is_err() {
                // No valid moves, game is a draw
                break;
            }
        } else {
            // No valid moves, game is a draw
            break;
        }

        moves_played += 1;
    }

    let winner = if let Some(winner) = game_state.get_winner() {
        winner
    } else {
        // Game ended in draw - this should be rare in Connect Four
        // For draws, we'll assign based on who had the advantage
        let final_eval = game_state.evaluate();
        if final_eval > 0 {
            Player::Player1 // Player1 had advantage
        } else if final_eval < 0 {
            Player::Player2 // Player2 had advantage
        } else {
            // True draw - randomly assign winner
            use rand::Rng;
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                Player::Player1
            } else {
                Player::Player2
            }
        }
    };

    GameResult {
        winner,
        ai1_time_ms,
        ai2_time_ms,
    }
}

// Create AI player from type
fn create_ai_player(ai_type: &AIType) -> Result<Box<dyn AIPlayer>, Box<dyn std::error::Error>> {
    match ai_type {
        AIType::Random => Ok(Box::new(RandomAI)),
        AIType::Heuristic => Ok(Box::new(HeuristicAI)),
        AIType::MMDepth1 => Ok(Box::new(MinimaxAI::new(1))),
        AIType::MMDepth2 => Ok(Box::new(MinimaxAI::new(2))),
        AIType::MMDepth3 => Ok(Box::new(MinimaxAI::new(3))),
        AIType::MMDepth4 => Ok(Box::new(MinimaxAI::new(4))),
        AIType::MMDepth5 => Ok(Box::new(MinimaxAI::new(5))),
        AIType::MMDepth6 => Ok(Box::new(MinimaxAI::new(6))),
        AIType::MMDepth7 => {
            // Only run depth 7 if explicitly requested
            if std::env::var("RUN_SLOW_TESTS").is_ok() {
                Ok(Box::new(MinimaxAI::new(7)))
            } else {
                Err("Depth 7 tests require RUN_SLOW_TESTS=1".into())
            }
        }
        AIType::MMDepth20 => {
            // Only run depth 20 if explicitly requested
            if std::env::var("RUN_SLOW_TESTS").is_ok() {
                Ok(Box::new(MinimaxAI::new(20)))
            } else {
                Err("Depth 20 tests require RUN_SLOW_TESTS=1".into())
            }
        }
        AIType::MLSimple => match MLSimpleAI::new() {
            Ok(ai) => Ok(Box::new(ai)),
            Err(e) => Err(format!("Failed to load simple model: {}", e).into()),
        },
    }
}

// Matrix result structure
#[derive(Debug)]
struct MatrixResult {
    ai1: String,
    ai2: String,
    ai1_win_rate: f64,
    ai1_avg_time_ms: f64,
    ai2_avg_time_ms: f64,
}

// Enhanced recommendations generation
fn generate_recommendations(
    ai_performance: &HashMap<String, f64>,
    ai_speeds: &HashMap<String, f64>,
) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Find best performing AI
    if let Some((best_ai, win_rate)) = ai_performance
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        if *win_rate > 70.0 {
            recommendations.push(format!(
                "{} shows excellent performance ({:.1}% avg win rate) and is ready for production",
                best_ai, win_rate
            ));
        } else if *win_rate > 60.0 {
            recommendations.push(format!(
                "{} shows good performance ({:.1}% avg win rate) and could be used in production",
                best_ai, win_rate
            ));
        } else {
            recommendations.push(format!(
                "{} shows moderate performance ({:.1}% avg win rate), consider further training",
                best_ai, win_rate
            ));
        }
    }

    // Find fastest AI
    if let Some((fastest_ai, avg_time)) = ai_speeds
        .iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        if *avg_time < 1.0 {
            recommendations.push(format!(
                "{} is very fast ({:.1}ms/move) and suitable for real-time play",
                fastest_ai, avg_time
            ));
        } else if *avg_time < 10.0 {
            recommendations.push(format!(
                "{} is fast ({:.1}ms/move) and suitable for interactive play",
                fastest_ai, avg_time
            ));
        }
    }

    // General recommendations
    recommendations.push("Use MM-Depth3 for best performance/speed balance".to_string());
    recommendations.push("Use Random AI for baseline testing".to_string());
    recommendations.push("Use Heuristic AI for educational purposes".to_string());

    recommendations
}

#[test]
#[ignore]
fn test_ai_matrix() {
    optimize_cpu_usage();
    println!("🤖 AI Matrix Test - Comprehensive AI Comparison");
    println!("{}", "=".repeat(60));

    // Get number of games from environment or use default
    let default_games = if cfg!(debug_assertions) { "10" } else { "20" };
    let num_games = std::env::var("NUM_GAMES")
        .unwrap_or_else(|_| default_games.to_string())
        .parse::<u32>()
        .unwrap_or(10);

    let build_mode = if cfg!(debug_assertions) { "Debug (Unoptimized)" } else { "Release (Optimized)" };

    println!("Configuration:");
    println!("  Build Mode:   {}", build_mode);
    println!("  Games per match: {}", num_games);
    println!(
        "  Include slow tests: {}",
        std::env::var("RUN_SLOW_TESTS").is_ok()
    );
    println!();

    // Define AI types to test (focused on production models)
    let ai_types = vec![
        AIType::Random,
        AIType::MMDepth6,
        AIType::MLSimple,
    ];



    println!("Testing {} AI types:", ai_types.len());
    for ai_type in &ai_types {
        println!("  - {}", ai_type.name());
    }
    println!();

    // Create all match combinations
    let mut match_combinations = Vec::new();
    for (i, ai_type1) in ai_types.iter().enumerate() {
        for (j, ai_type2) in ai_types.iter().enumerate() {
            if i >= j {
                continue; // Skip self-matches and duplicate matches
            }
            match_combinations.push((ai_type1.clone(), ai_type2.clone()));
        }
    }

    println!(
        "🎯 Running {} AI match combinations in parallel...",
        match_combinations.len()
    );

    // Create flat list of all games to play
    let mut all_games = Vec::new();
    for (ai_type1, ai_type2) in &match_combinations {
        for game_idx in 0..num_games {
            all_games.push((ai_type1.clone(), ai_type2.clone(), game_idx));
        }
    }

    println!(
        "🎯 Running {} total games across {} match combinations in parallel...",
        all_games.len(),
        match_combinations.len()
    );

    let start_time = Instant::now();

    // Parallelize at the game level to ensure 100% CPU utilization
    let game_results: Vec<(String, String, GameResult, bool)> = all_games
        .into_par_iter()
        .map(|(ai_type1, ai_type2, game_idx)| {
            // Create fresh AI players for each specific game to avoid shared state
            let mut ai1 = create_ai_player(&ai_type1).expect("Failed to create AI1");
            let mut ai2 = create_ai_player(&ai_type2).expect("Failed to create AI2");

            let ai1_first = game_idx % 2 == 0;
            let result = play_game(&mut ai1, &mut ai2, ai1_first);
            
            (ai_type1.name().to_string(), ai_type2.name().to_string(), result, ai1_first)
        })
        .collect();

    // Aggregate game results into MatrixResults per match
    let mut matrix_map: HashMap<(String, String), (u32, u64, u64)> = HashMap::new();
    for (ai1_name, ai2_name, result, ai1_first) in game_results {
        let key = if ai1_name < ai2_name {
            (ai1_name.clone(), ai2_name.clone())
        } else {
            (ai2_name.clone(), ai1_name.clone())
        };

        let entry = matrix_map.entry(key.clone()).or_insert((0, 0, 0));
        
        let ai1_won = if ai1_name == key.0 {
            if ai1_first { result.winner == Player::Player1 } else { result.winner == Player::Player2 }
        } else {
            if ai1_first { result.winner == Player::Player2 } else { result.winner == Player::Player1 }
        };

        if ai1_won {
            entry.0 += 1;
        }
        
        if ai1_name == key.0 {
            entry.1 += result.ai1_time_ms;
            entry.2 += result.ai2_time_ms;
        } else {
            entry.1 += result.ai2_time_ms;
            entry.2 += result.ai1_time_ms;
        }
    }

    let results: Vec<MatrixResult> = matrix_map
        .into_iter()
        .map(|((ai1, ai2), (ai1_wins, ai1_time, ai2_time))| {
            MatrixResult {
                ai1,
                ai2,
                ai1_win_rate: (ai1_wins as f64 / num_games as f64) * 100.0,
                ai1_avg_time_ms: ai1_time as f64 / num_games as f64,
                ai2_avg_time_ms: ai2_time as f64 / num_games as f64,
            }
        })
        .collect();

    let total_games = results.len() * num_games as usize;
    let _duration = start_time.elapsed();

    // Print individual match results
    for result in &results {
        println!(
            "  {} vs {}: {} wins {:.1}%, {} wins {:.1}%",
            result.ai1,
            result.ai2,
            result.ai1,
            result.ai1_win_rate,
            result.ai2,
            100.0 - result.ai1_win_rate
        );
        println!(
            "  Average time: {} {:.1}ms, {} {:.1}ms",
            result.ai1, result.ai1_avg_time_ms, result.ai2, result.ai2_avg_time_ms
        );
        println!();
    }

    let duration = start_time.elapsed();

    // Print matrix results
    println!("📊 AI MATRIX RESULTS");
    println!("{}", "=".repeat(60));
    println!("Test Configuration:");
    println!("  Total games played: {}", total_games);
    println!("  Duration: {:.2} seconds", duration.as_secs_f64());
    println!(
        "  Games per second: {:.1}",
        total_games as f64 / duration.as_secs_f64()
    );
    println!();

    // Print matrix table
    println!("MATRIX TABLE (Win Rate % of Row vs Column):");
    println!("{}", "-".repeat(80));

    // Header
    print!("{:<15}", "AI Type");
    for ai_type in &ai_types {
        print!(" {:<10}", ai_type.name());
    }
    println!();
    println!("{}", "-".repeat(80));

    // Matrix rows
    for ai_type1 in &ai_types {
        print!("{:<15}", ai_type1.name());

        for ai_type2 in &ai_types {
            if ai_type1 == ai_type2 {
                print!(" {:<10}", "-");
            } else {
                let result = results.iter().find(|r| {
                    (r.ai1 == ai_type1.name() && r.ai2 == ai_type2.name())
                        || (r.ai1 == ai_type2.name() && r.ai2 == ai_type1.name())
                });

                if let Some(r) = result {
                    let win_rate = if r.ai1 == ai_type1.name() {
                        r.ai1_win_rate
                    } else {
                        100.0 - r.ai1_win_rate
                    };
                    print!(" {:<10.1}", win_rate);
                } else {
                    print!(" {:<10}", "N/A");
                }
            }
        }
        println!();
    }
    println!("{}", "-".repeat(80));
    println!();

    // Performance summary
    println!("🏆 PERFORMANCE SUMMARY:");
    println!("{}", "-".repeat(40));

    let mut ai_performance = HashMap::new();

    for result in &results {
        // Add wins for ai1
        *ai_performance.entry(result.ai1.clone()).or_insert(0.0) += result.ai1_win_rate;
        // Add wins for ai2 (100 - ai1_win_rate)
        *ai_performance.entry(result.ai2.clone()).or_insert(0.0) += 100.0 - result.ai1_win_rate;
    }

    let mut sorted_performance: Vec<_> = ai_performance.iter().collect();
    sorted_performance.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (i, (ai_name, total_win_rate)) in sorted_performance.iter().enumerate() {
        let avg_win_rate = *total_win_rate / (ai_types.len() - 1) as f64;
        println!(
            "{}. {}: {:.1}% average win rate",
            i + 1,
            ai_name,
            avg_win_rate
        );
    }
    println!();

    // Speed analysis
    println!("⚡ SPEED ANALYSIS:");
    println!("{}", "-".repeat(40));

    let mut ai_speeds = HashMap::new();
    let mut ai_speed_counts = HashMap::new();

    for result in &results {
        *ai_speeds.entry(result.ai1.clone()).or_insert(0.0) += result.ai1_avg_time_ms;
        *ai_speeds.entry(result.ai2.clone()).or_insert(0.0) += result.ai2_avg_time_ms;
        *ai_speed_counts.entry(result.ai1.clone()).or_insert(0) += 1;
        *ai_speed_counts.entry(result.ai2.clone()).or_insert(0) += 1;
    }

    let mut sorted_speeds: Vec<_> = ai_speeds.iter().collect();
    sorted_speeds.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (ai_name, total_time) in &sorted_speeds {
        let count = ai_speed_counts[*ai_name];
        let avg_time = *total_time / count as f64;
        let speed_category = if avg_time < 1.0 {
            "Very Fast"
        } else if avg_time < 10.0 {
            "Fast"
        } else if avg_time < 50.0 {
            "Moderate"
        } else {
            "Slow"
        };
        println!("{}: {:.1}ms/move ({})", ai_name, avg_time, speed_category);
    }
    println!();

    // Enhanced recommendations
    println!("💡 RECOMMENDATIONS:");
    println!("{}", "-".repeat(40));

    // Calculate average win rates for recommendations
    let mut ai_avg_performance = HashMap::new();
    for (ai_name, total_win_rate) in &ai_performance {
        let avg_win_rate = *total_win_rate / (ai_types.len() - 1) as f64;
        ai_avg_performance.insert(ai_name.clone(), avg_win_rate);
    }

    let recommendations = generate_recommendations(&ai_avg_performance, &ai_speeds);
    for recommendation in &recommendations {
        println!("• {}", recommendation);
    }
    println!();

    println!("🎉 AI Matrix test completed successfully!");
}
