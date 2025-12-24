use connect_four_ai_core::{features::GameFeatures, ml_ai::MLAI, GameState};

fn main() {
    println!("🧪 Testing neural network evaluation speed...");

    let ai = MLAI::new();
    let game_state = GameState::new();
    let features = GameFeatures::from_game_state(&game_state);
    let _features_array = features.to_array();

    println!("🔄 Running 1000 neural network evaluations...");
    let start = std::time::Instant::now();

    for i in 0..1000 {
        if i % 100 == 0 {
            println!("  Progress: {}/1000", i);
        }
        let _value = ai.evaluate_position(&game_state);
    }

    let duration = start.elapsed();
    println!(
        "✅ Completed 1000 evaluations in {:.2} seconds",
        duration.as_secs_f64()
    );
    println!(
        "📊 Average time per evaluation: {:.3} ms",
        duration.as_millis() as f64 / 1000.0
    );

    if duration.as_secs_f64() > 10.0 {
        println!("⚠️  Neural network evaluation is very slow!");
    } else {
        println!("✅ Neural network evaluation speed is reasonable");
    }
}
