
#[cfg(test)]
mod tests {
    use connect_four_ai_core::{
        ml_ai::MLAI,
        solver::Solver,
        solver::Bitboard,
        GameState,
        Player,
        COLS,
    };
    use std::fs;
    use std::path::PathBuf;
    use serde_json;

    #[test]
    fn benchmark_ml_vs_solver() {
        let mut ml_ai = MLAI::new();
        let mut solver = Solver::new();
        
        // Load weights
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("ml/data/weights/ml_ai_weights_distilled.json");
        
        println!("Loading weights from: {:?}", path);
        let content = fs::read_to_string(&path).expect("Failed to read weights file");
        let json: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");
        
        let value_weights: Vec<f32> = json["value_network"]["weights"].as_array().unwrap()
            .iter().map(|v| v.as_f64().unwrap() as f32).collect();
        let policy_weights: Vec<f32> = json["policy_network"]["weights"].as_array().unwrap()
            .iter().map(|v| v.as_f64().unwrap() as f32).collect();
            
        ml_ai.load_weights(&value_weights, &policy_weights);
        
        let num_games = 20; // Run 20 games (Solver is slow)
        let mut ml_wins = 0;
        let mut draws = 0;
        let mut losses = 0;
        
        // ML AI plays as Player 1 (needs to be aggressive) and Player 2 (needs to be defensive)
        
        for i in 0..num_games {
            let mut state = GameState::new();
            let ml_is_p1 = i % 2 == 0;
            
            println!("Game {}/{} (ML is P{})", i + 1, num_games, if ml_is_p1 { 1 } else { 2 });
            
            while !state.is_game_over() {
                let is_ml_turn = (state.current_player == Player::Player1 && ml_is_p1) || 
                               (state.current_player == Player::Player2 && !ml_is_p1);
                               
                let mv = if is_ml_turn {
                    ml_ai.get_best_move(&state).r#move.unwrap()
                } else {
                    // Solver move
                    let bitboard = Bitboard::from_game_state(&state);
                    // Use depth 10 for speed, effectively strong
                    let (best_move, _) = solver.analyze(&bitboard, 10);
                    best_move.unwrap() as u8
                };
                
                state.make_move(mv).unwrap();
            }
            
            if let Some(winner) = state.get_winner() {
                if (winner == Player::Player1 && ml_is_p1) || (winner == Player::Player2 && !ml_is_p1) {
                    println!("Result: ML WIN");
                    ml_wins += 1;
                } else {
                    println!("Result: ML LOSS");
                    losses += 1;
                }
            } else {
                println!("Result: DRAW");
                draws += 1;
            }
        }
        
        println!("Results vs Solver ({} games):", num_games);
        println!("Wins: {}", ml_wins);
        println!("Losses: {}", losses);
        println!("Draws: {}", draws);
        println!("Win/Draw Rate: {:.1}%", ((ml_wins + draws) as f32 / num_games as f32) * 100.0);
    }
}
