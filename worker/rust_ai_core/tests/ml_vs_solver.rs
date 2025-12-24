
#[cfg(test)]
mod tests {
    use connect_four_ai_core::{
        ml_ai::MLAI,
        solver::Solver,
        solver::Bitboard,
        GameState,
        Player,
    };
    use std::fs;
    use std::path::PathBuf;
    use serde_json;

    #[test]
    #[ignore]
    fn benchmark_ml_vs_solver() {
        let mut ml_ai = MLAI::new();
        let mut solver = Solver::new();
        
        // Load weights
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../public/ml/data/weights/ml_ai_weights_best.json");
        
        println!("Loading weights from: {:?}", path);
        let content = fs::read_to_string(&path).expect("Failed to read weights file");
        let json: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");
        
        let value_weights: Vec<f32> = json["value_network"]["weights"].as_array().unwrap()
            .iter().map(|v| v.as_f64().unwrap() as f32).collect();
        let policy_weights: Vec<f32> = json["policy_network"]["weights"].as_array().unwrap()
            .iter().map(|v| v.as_f64().unwrap() as f32).collect();
            
        ml_ai.load_weights(&value_weights, &policy_weights);
        
        let num_games = 10; // Scaled for Depth 14 Audit
        let mut ml_p1_wins = 0;
        let mut ml_p1_draws = 0;
        let mut ml_p1_losses = 0;
        let mut ml_p2_wins = 0;
        let mut ml_p2_draws = 0;
        let mut ml_p2_losses = 0;
        
        for i in 0..num_games {
            let mut state = GameState::new();
            let ml_is_p1 = i % 2 == 0;
            
            println!("Game {}/{} (ML is P{})", i + 1, num_games, if ml_is_p1 { 1 } else { 2 });
            
            let mut moves_count = 0;
            while !state.is_game_over() {
                let is_ml_turn = (state.current_player == Player::Player1 && ml_is_p1) || 
                               (state.current_player == Player::Player2 && !ml_is_p1);
                               
                let mv = if is_ml_turn {
                    ml_ai.get_best_move(&state).r#move.unwrap()
                } else {
                    // Solver move
                    let bitboard = Bitboard::from_game_state(&state);
                    // Use depth 14 for a much more rigorous comparison
                    let (best_move, _) = solver.analyze(&bitboard, 14);
                    best_move.unwrap() as u8
                };
                
                state.make_move(mv).unwrap();
                moves_count += 1;
            }
            
            println!("Game finished in {} moves.", moves_count);
            
            if let Some(winner) = state.get_winner() {
                if (winner == Player::Player1 && ml_is_p1) || (winner == Player::Player2 && !ml_is_p1) {
                    println!("Result: ML WIN");
                    if ml_is_p1 { ml_p1_wins += 1; } else { ml_p2_wins += 1; }
                } else {
                    println!("Result: ML LOSS");
                    if ml_is_p1 { ml_p1_losses += 1; } else { ml_p2_losses += 1; }
                }
            } else {
                println!("Result: DRAW");
                if ml_is_p1 { ml_p1_draws += 1; } else { ml_p2_draws += 1; }
            }
        }
        
        println!("\n📊 CONFIDENCE AUDIT RESULTS ({} games):", num_games);
        println!("----------------------------------");
        println!("ML as Player 1 (Aggressive):");
        println!("  Wins:   {}", ml_p1_wins);
        println!("  Draws:  {}", ml_p1_draws);
        println!("  Losses: {}", ml_p1_losses);
        println!("\nML as Player 2 (Defensive):");
        println!("  Wins:   {}", ml_p2_wins);
        println!("  Draws:  {}", ml_p2_draws);
        println!("  Losses: {}", ml_p2_losses);
        
        let total_wins = ml_p1_wins + ml_p2_wins;
        let total_draws = ml_p1_draws + ml_p2_draws;
        let total_losses = ml_p1_losses + ml_p2_losses;
        
        println!("\nOVERALL TOTALS:");
        println!("  Total Wins:   {}", total_wins);
        println!("  Total Draws:  {}", total_draws);
        println!("  Total Losses: {}", total_losses);
        println!("  Win/Draw Rate: {:.1}%", ((total_wins + total_draws) as f32 / num_games as f32) * 100.0);
    }
}
