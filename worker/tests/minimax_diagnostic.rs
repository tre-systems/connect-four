use connect_four_ai_core::{genetic_params::GeneticParams, GameState, AI};
use std::time::Instant;

fn get_evolved_params() -> GeneticParams {
    GeneticParams::load_from_file("ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default())
}

#[test]
fn test_minimax_diagnostic() {
    println!("🔍 Minimax Diagnostic Test");
    println!("{}", "=".repeat(40));

    let evolved_params = get_evolved_params();
    println!("📋 Using evolved genetic parameters");

    let mut game_state = GameState::with_genetic_params(evolved_params);
    let mut ai = AI::new();
    let mut total_nodes = 0;
    let mut total_time = 0;
    let mut moves_analyzed = 0;

    println!("Starting diagnostic game...");
    println!("{}", "-".repeat(30));

    while !game_state.is_game_over() && moves_analyzed < 42 {
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            println!("No valid moves available - game is a draw");
            break;
        }

        let start_time = Instant::now();
        let (best_move, move_evaluations) = ai.get_best_move(&game_state, 3);
        let end_time = Instant::now();

        let move_time = end_time.duration_since(start_time).as_millis();
        total_time += move_time;
        total_nodes += ai.nodes_evaluated as u64;
        moves_analyzed += 1;

        println!(
            "Move {}: Player {:?}, Valid moves: {:?}",
            moves_analyzed, game_state.current_player, valid_moves
        );

        println!(
            "  Best move: {:?}, Nodes: {}, Time: {}ms, Cache hits: {}",
            best_move, ai.nodes_evaluated, move_time, ai.transposition_hits
        );

        if let Some(column) = best_move {
            if let Err(e) = game_state.make_move(column) {
                println!("  Error making move: {}", e);
                break;
            }
        } else {
            println!("  No valid move found");
            break;
        }

        for eval in &move_evaluations[..move_evaluations.len().min(3)] {
            println!(
                "    Column {}: Score {:.2}, Type: {}",
                eval.column, eval.score, eval.move_type
            );
        }

        println!();
    }

    println!("{}", "=".repeat(50));
    println!("Diagnostic Results:");
    println!("  Total moves analyzed: {}", moves_analyzed);
    println!("  Total nodes evaluated: {}", total_nodes);
    println!("  Total time: {}ms", total_time);
    println!(
        "  Average nodes per move: {:.1}",
        total_nodes as f64 / moves_analyzed as f64
    );
    println!(
        "  Average time per move: {:.1}ms",
        total_time as f64 / moves_analyzed as f64
    );
    println!(
        "  Nodes per second: {:.0}",
        (total_nodes as f64 / total_time as f64) * 1000.0
    );

    // Check final game state
    if let Some(winner) = game_state.get_winner() {
        println!("  Winner: Player {:?}", winner);
    } else {
        println!("  Game ended in draw");
    }

    println!(
        "  Game status: {:?}",
        if game_state.is_game_over() {
            "Finished"
        } else {
            "In Progress"
        }
    );

    // Performance analysis
    println!("\n📊 Performance Analysis:");
    println!("{}", "=".repeat(25));

    let avg_nodes = total_nodes as f64 / moves_analyzed as f64;
    let avg_time = total_time as f64 / moves_analyzed as f64;
    let nodes_per_sec = (total_nodes as f64 / total_time as f64) * 1000.0;

    if avg_nodes < 1000.0 {
        println!("✅ Excellent search efficiency");
    } else if avg_nodes < 10000.0 {
        println!("✅ Good search efficiency");
    } else if avg_nodes < 100000.0 {
        println!("⚠️  Moderate search efficiency");
    } else {
        println!("❌ Poor search efficiency - consider reducing depth");
    }

    if avg_time < 10.0 {
        println!("🚀 Excellent move speed");
    } else if avg_time < 100.0 {
        println!("⚡ Good move speed");
    } else if avg_time < 1000.0 {
        println!("⚠️  Moderate move speed");
    } else {
        println!("🐌 Slow move speed - consider optimization");
    }

    if nodes_per_sec > 1000000.0 {
        println!("🎯 Excellent search speed");
    } else if nodes_per_sec > 100000.0 {
        println!("🎯 Good search speed");
    } else if nodes_per_sec > 10000.0 {
        println!("⚠️  Moderate search speed");
    } else {
        println!("❌ Poor search speed - check algorithm efficiency");
    }

    // Recommendations
    println!("\n💡 Recommendations:");
    println!("{}", "=".repeat(20));

    if avg_nodes > 50000.0 || avg_time > 500.0 {
        println!("🔧 Consider reducing search depth for better performance");
    }

    if u64::from(ai.transposition_hits) < total_nodes / 10 {
        println!("🔧 Transposition table usage could be improved");
    }

    if moves_analyzed < 20 {
        println!("🔧 Game ended early - check move generation logic");
    }

    if total_time > 5000 {
        println!("🔧 Consider implementing move ordering for better pruning");
    }
}
