use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};
use rayon::prelude::*;
use std::time::Instant;

fn optimize_cpu_usage() {
    if cfg!(target_os = "macos") {
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);
        let optimal_threads = (num_cores as f64 * 0.8) as usize;
        rayon::ThreadPoolBuilder::new()
            .num_threads(optimal_threads)
            .stack_size(8 * 1024 * 1024)
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });
        println!(
            "🍎 Apple Silicon detected: Using {} threads ({} cores available)",
            optimal_threads, num_cores
        );
    } else {
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

#[derive(Debug)]
struct GameResult {
    evolved_wins: bool,
    evolved_time: u64,
    default_time: u64,
    moves_played: u32,
}

fn play_single_game(
    evolved_params: &GeneticParams,
    default_params: &GeneticParams,
    _game_num: usize,
) -> GameResult {
    let mut game_state = GameState::new();
    let mut moves_played = 0;
    let max_moves = 42; // Maximum moves in Connect Four (6x7 board)
    let mut evolved_time = 0;
    let mut default_time = 0;

    // Randomly decide which player uses evolved parameters
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let evolved_is_player2 = rng.gen_bool(0.5);

    while !game_state.is_game_over() && moves_played < max_moves {
        let current_player = game_state.current_player;
        let is_evolved_turn = if evolved_is_player2 {
            current_player == Player::Player2
        } else {
            current_player == Player::Player1
        };

        // Use different parameters based on whose turn it is
        let test_params = if is_evolved_turn {
            evolved_params.clone()
        } else {
            default_params.clone()
        };

        // Create a new game state with the test parameters
        let mut test_state = GameState::with_genetic_params(test_params);
        test_state.board = game_state.board.clone();
        test_state.current_player = game_state.current_player;

        let mut ai = AI::new();
        let start_time = Instant::now();
        let (best_move, _) = ai.get_best_move(&test_state, 3);
        let end_time = Instant::now();
        let move_time = end_time.duration_since(start_time).as_millis() as u64;

        if is_evolved_turn {
            evolved_time += move_time;
        } else {
            default_time += move_time;
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
    let evolved_wins = if let Some(winner) = game_state.get_winner() {
        if evolved_is_player2 {
            winner == Player::Player2
        } else {
            winner == Player::Player1
        }
    } else {
        // Game ended in draw - use neutral evaluation approach
        let evolved_eval = game_state.evaluate();
        
        // For draws, we need to be more conservative
        // Only count as evolved win if there's a clear advantage
        if evolved_is_player2 {
            evolved_eval < -100 // Significant advantage for Player2
        } else {
            evolved_eval > 100 // Significant advantage for Player1
        }
    };

    GameResult {
        evolved_wins,
        evolved_time,
        default_time,
        moves_played,
    }
}

#[test]
fn test_genetic_params_comparison() {
    println!("🧬 Genetic Parameters Comparison Test");
    println!("{}", "=".repeat(50));

    // Optimize CPU usage
    println!("🚀 Optimizing CPU usage for maximum performance...");
    optimize_cpu_usage();

    // Load evolved parameters
    let evolved_params =
        match GeneticParams::load_from_file("../../ml/data/genetic_params/evolved.json") {
            Ok(params) => params,
            Err(e) => {
                eprintln!("Failed to load evolved parameters: {}", e);
                return;
            }
        };

    let default_params = GeneticParams::default();
    let num_games = std::env::var("NUM_GAMES")
        .unwrap_or_else(|_| "100".to_string())
        .parse::<usize>()
        .unwrap_or(100);

    println!(
        "📊 Comparing evolved vs default parameters over {} games",
        num_games
    );
    println!("{}", "-".repeat(50));

    let start_time = Instant::now();

    // Play games in parallel
    let results: Vec<GameResult> = (0..num_games)
        .into_par_iter()
        .map(|game_num| play_single_game(&evolved_params, &default_params, game_num))
        .collect();

    let total_time = start_time.elapsed();

    // Analyze results
    let evolved_wins = results.iter().filter(|r| r.evolved_wins).count();
    let default_wins = num_games - evolved_wins;
    let evolved_win_rate = (evolved_wins as f64 / num_games as f64) * 100.0;
    let default_win_rate = (default_wins as f64 / num_games as f64) * 100.0;

    let total_evolved_time: u64 = results.iter().map(|r| r.evolved_time).sum();
    let total_default_time: u64 = results.iter().map(|r| r.default_time).sum();
    let avg_evolved_time = total_evolved_time as f64 / num_games as f64;
    let avg_default_time = total_default_time as f64 / num_games as f64;

    let total_moves: u32 = results.iter().map(|r| r.moves_played).sum();
    let avg_moves = total_moves as f64 / num_games as f64;

    println!("\n📈 Results:");
    println!("{}", "=".repeat(30));
    println!(
        "Evolved params wins: {} ({:.1}%)",
        evolved_wins, evolved_win_rate
    );
    println!(
        "Default params wins: {} ({:.1}%)",
        default_wins, default_win_rate
    );
    println!("Average moves per game: {:.1}", avg_moves);
    println!("Evolved params avg time: {:.1}ms", avg_evolved_time);
    println!("Default params avg time: {:.1}ms", avg_default_time);
    println!("Total test time: {:.2}s", total_time.as_secs_f64());

    println!("\n🎯 Analysis:");
    println!("{}", "=".repeat(20));

    if evolved_win_rate > default_win_rate + 5.0 {
        println!("✅ Evolved parameters show significant improvement!");
    } else if evolved_win_rate > default_win_rate {
        println!("✅ Evolved parameters show slight improvement");
    } else if evolved_win_rate < default_win_rate - 5.0 {
        println!("❌ Default parameters perform significantly better");
    } else {
        println!("🤝 Both parameter sets perform similarly");
    }

    let time_ratio = avg_evolved_time / avg_default_time;
    if time_ratio < 0.8 {
        println!("⚡ Evolved parameters are faster");
    } else if time_ratio > 1.2 {
        println!("🐌 Evolved parameters are slower");
    } else {
        println!("⚖️  Both parameter sets have similar performance");
    }

    println!("\n💡 Recommendations:");
    println!("{}", "=".repeat(20));

    if evolved_win_rate > 55.0 {
        println!("🎉 Evolved parameters are ready for production use!");
    } else if evolved_win_rate > 50.0 {
        println!("✅ Evolved parameters show promise, consider further optimization");
    } else {
        println!("🔧 Evolved parameters need improvement, review evolution process");
    }

    if avg_moves < 30.0 {
        println!("⚠️  Games are ending early - check win detection logic");
    }

    if total_time.as_secs() > 60 {
        println!("⏱️  Test took a long time - consider reducing number of games");
    }

    // Statistical significance
    let margin_of_error =
        1.96 * ((evolved_win_rate * (100.0 - evolved_win_rate)) / num_games as f64).sqrt();
    println!("\n📊 Statistical Analysis:");
    println!("Margin of error: ±{:.1}%", margin_of_error);

    if (evolved_win_rate - default_win_rate).abs() > margin_of_error {
        println!("✅ Difference is statistically significant");
    } else {
        println!("⚠️  Difference is not statistically significant");
    }
}
