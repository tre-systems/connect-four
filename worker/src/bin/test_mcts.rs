use connect_four_ai_core::{mcts::MCTS, ml_ai::MLAI, GameState};

fn main() {
    println!("🧪 Testing MCTS performance...");

    let ai = MLAI::new();
    let game_state = GameState::new();

    println!("🔄 Running MCTS with 10 simulations...");
    let start = std::time::Instant::now();

    let mut mcts = MCTS::new(1.0, 10);
    let value_fn = |state: &GameState| ai.evaluate_position(state);
    let policy_fn = |_state: &GameState| {
        // Simple random policy for testing
        vec![1.0 / 7.0; 7]
    };

    let (best_move, move_probs) = mcts.search(game_state, &value_fn, &policy_fn, 0.0, false);

    let duration = start.elapsed();
    println!("✅ MCTS completed in {:.3} seconds", duration.as_secs_f64());
    println!("🎯 Best move: {best_move}");
    println!("📊 Move probabilities: {move_probs:?}");

    if duration.as_secs_f64() > 1.0 {
        println!("⚠️  MCTS is very slow!");
    } else {
        println!("✅ MCTS performance is reasonable");
    }
}
