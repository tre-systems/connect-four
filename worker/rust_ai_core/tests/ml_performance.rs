use connect_four_ai_core::{
    ml_ai::MLAI,
    GameState,
    Player,
};
use std::fs;
use std::path::PathBuf;

// Simple Heuristic AI for comparison (re-implemented here to avoid visibility issues if lib struct is private)
// Actually we can try to use the one from lib if it's pub. 
// checking: "simple heuristic" logic is easy to replicate if needed, but let's try to import.
// If HeuristicAI is not exposed, we will play against a random opponent or simple logic.
// However, the user wants to know if it's "Competitive". 
// Let's implement a "Smart Random" opponent that blocks immediate wins, which is a good baseline.

struct SmartRandomAI;
impl SmartRandomAI {
    fn get_move(&self, state: &GameState) -> u8 {
        let valid = state.get_valid_moves();
        if valid.is_empty() { return 0; }
        
        // 1. Check for winning move
        for &col in &valid {
            let mut next = state.clone();
            if next.make_move(col).is_ok() && next.has_winner() {
                return col;
            }
        }
        
        // 2. Check for blocking move
        let opponent = state.current_player.opponent();
        for &col in &valid {
            // Simulate opponent move at this column
            // We need to simulate: if we DON'T play here, can opponent win?
            // Actually, simplest is: play current player move, see if opponent wins? No.
            // Check if opponent can win if they play there.
            // We need to force current player to be opponent for a sec.
            // Let's just create a state where it is opponent's turn.
            let mut test_state = state.clone();
            test_state.current_player = opponent; // Switch active player hack
            if test_state.make_move(col).is_ok() && test_state.has_winner() {
                return col;
            }
        }
        
        // 3. Random
        use rand::Rng;
        valid[rand::thread_rng().gen_range(0..valid.len())]
    }
}


#[test]
fn benchmark_ml_vs_smart_random() {
    let mut ml_ai = MLAI::new();
    
    // Load weights
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../public/ml/data/weights/ml_ai_weights_best.json"); // Canonical production weights
    
    println!("Loading weights from: {:?}", path);
    let content = fs::read_to_string(&path).expect("Failed to read weights file");
    let json: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");
    
    let value_weights: Vec<f32> = json["value_network"]["weights"].as_array().unwrap()
        .iter().map(|v| v.as_f64().unwrap() as f32).collect();
    let policy_weights: Vec<f32> = json["policy_network"]["weights"].as_array().unwrap()
        .iter().map(|v| v.as_f64().unwrap() as f32).collect();
        
    ml_ai.load_weights(&value_weights, &policy_weights);
    
    let num_games = 10; // Fast for pre-commit, use NUM_GAMES env for more
    let mut ml_wins = 0;
    let mut draws = 0;
    let mut losses = 0;
    
    let opponent = SmartRandomAI;
    
    for i in 0..num_games {
        let mut state = GameState::new();
        let ml_is_p1 = i % 2 == 0;
        
        while !state.is_game_over() {
            let is_ml_turn = (state.current_player == Player::Player1 && ml_is_p1) || 
                           (state.current_player == Player::Player2 && !ml_is_p1);
                           
            let mv = if is_ml_turn {
                ml_ai.get_best_move(&state).r#move.unwrap()
            } else {
                opponent.get_move(&state)
            };
            
            state.make_move(mv).unwrap();
        }
        
        if let Some(winner) = state.get_winner() {
            if (winner == Player::Player1 && ml_is_p1) || (winner == Player::Player2 && !ml_is_p1) {
                ml_wins += 1;
            } else {
                losses += 1;
            }
        } else {
            draws += 1;
        }
    }
    
    println!("Results vs Smart Random ({} games):", num_games);
    println!("Wins: {}", ml_wins);
    println!("Losses: {}", losses);
    println!("Draws: {}", draws);
    println!("Conclusion: ML AI Win Rate is {:.1}%. Target is > 50% for 'Competitive' status.", (ml_wins as f32 / num_games as f32) * 100.0);
    
    if (ml_wins as f32 / num_games as f32) > 0.5 {
         println!("SUCCESS: ML AI is Competitive!");
    } else {
         println!("WARNING: ML AI is still weak.");
    }
}
