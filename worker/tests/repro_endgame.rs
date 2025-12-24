#[cfg(test)]
mod tests {
    use connect_four_ai_core::{
        ml_ai::MLAI,
        GameState,
        Player,
        Cell,
    };
    use std::fs;
    use std::path::PathBuf;
    use serde_json;

    fn parse_board(board_str: &str) -> GameState {
        let mut board = [[Cell::Empty; 6]; 7];
        let lines: Vec<&str> = board_str.trim().lines().collect();
        
        // Input is Top to Bottom (Row 5 to Row 0)
        // Check dimensions
        assert_eq!(lines.len(), 6, "Expected 6 rows");
        
        for (row_idx, line) in lines.iter().enumerate() {
            // Visual string is like "🔴⚫🔴🟡⚫🔴🟡"
            // We need to parse chars.
            // ⚫ = Empty, 🔴 = Red (P1), 🟡 = Yellow (P2)
            // But chars might be multi-byte.
            // Let's use a simple char iterator.
            let chars: Vec<char> = line.chars().collect();
            // Filter out non-circle chars if any spacing?
            // The log shows tight packing.
            // Let's print chars to be sure.
            
            let mut col = 0;
            // Iterate chars, identifying cells
            for c in chars {
                let cell = match c {
                    '🔴' => Some(Cell::Player1),
                    '🟡' => Some(Cell::Player2),
                    '⚫' => Some(Cell::Empty),
                    _ => None, // Ignore spaces or other chars
                };
                
                if let Some(cell_val) = cell {
                     // Row index in array: 0 is TOP (Visual Row 0).
                     // input row_idx 0 is TOP.
                     let rank = row_idx;
                     board[col][rank] = cell_val;
                     col += 1;
                }
            }
        }
        
        // Determine current player
        let red_count: usize = board.iter().flat_map(|col| col.iter()).filter(|&&c| c == Cell::Player1).count();
        let yellow_count: usize = board.iter().flat_map(|col| col.iter()).filter(|&&c| c == Cell::Player2).count();
        
        let current_player = if red_count > yellow_count { Player::Player2 } else { Player::Player1 };
        
        GameState {
            board,
            current_player,
            genetic_params: connect_four_ai_core::genetic_params::GeneticParams::default(),
        }
    }

    #[test]
    fn repro_browser_loss() {
        // Board state from browser logs BEFORE Yellow's move at Col 1.
        // Red had just played.
        
        // Log visual:
        // board-logic.ts:32 🔴⚫🔴🟡⚫🔴🟡
        // board-logic.ts:32 🟡⚫🟡🔴⚫🔴🔴
        // board-logic.ts:32 🔴⚫🔴🟡⚫🟡🟡
        // board-logic.ts:32 🟡⚫🟡🔴⚫🔴🔴
        // board-logic.ts:32 🟡⚫🔴🔴⚫🟡🟡
        // board-logic.ts:32 🟡⚫🔴🟡🟡🟡🔴
        
        // Corrected Board String based on stack analysis
        // Col 4 MUST have pieces at Rows 0, 1, 2 for the win to happen at Row 3.
        let board_str = "
🔴⚫🔴🟡⚫🔴🟡
🟡⚫🟡🔴⚫🔴🔴
🔴⚫🔴🟡⚫🟡🟡
🟡⚫🟡🔴⚫🔴🔴
🟡⚫🔴🔴🔴🟡🟡
🟡⚫🔴🟡🟡🟡🔴
";
        let mut state = parse_board(board_str);
        
        println!("Reconstructed State: {:?}", state);
        
        // Force Yellow turn (Player2) to ensure we test the specific move
        state.current_player = Player::Player2;
        println!("Forced Current Player: {:?}", state.current_player); 
        
        // Initialize ML AI
        let mut ml_ai = MLAI::new();
        ml_ai.mcts_simulations = 4000;
        
        // Load weights
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../public/ml/data/weights/ml_ai_weights_best.json");
        let content = fs::read_to_string(&path).expect("Failed to read weights");
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let v_w: Vec<f32> = json["value_network"]["weights"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
        let p_w: Vec<f32> = json["policy_network"]["weights"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
        ml_ai.load_weights(&v_w, &p_w);
        
        // Get Move with Diagnostics
        let response = ml_ai.get_best_move(&state);
        
        println!("🤖 ML Response: {}", response.thinking);
        for eval in response.diagnostics.move_evaluations {
            println!("  Col {}: Score {:.4} ({})", eval.column, eval.score, eval.move_type);
        }
        
        // Analyze Win Threat
        // The user said Red played Col 4 next and WON.
        // Let's verify if playing Col 4 for RED (after our move) is a win.
        // We are Yellow. We should BLOCK Col 4 if it's a threat.
        // Or maybe Col 4 is a win for Yellow?
        
        // Check if Col 4 is a valid move for Yellow?
        if state.can_place_in_column(4) {
             println!("Col 4 is valid for Yellow.");
             // What happens if Yellow plays Col 4?
             let mut test_state = state.clone();
             test_state.make_move(4).unwrap();
             if let Some(w) = test_state.get_winner() {
                 println!("🎉 Playing Col 4 WINS for Yellow! (Winner: {:?})", w);
             }
        }
        
        // Check if Red playing Col 4 wins (if Yellow plays elsewhere, e.g. Col 1)
        println!("Testing Red threat at Col 4...");
        let mut test_state = state.clone();
        test_state.make_move(1).unwrap(); // Yellow plays Col 1 (the mistake)
        
        if test_state.can_place_in_column(4) {
            test_state.make_move(4).unwrap(); // Red plays Col 4
             if let Some(w) = test_state.get_winner() {
                 println!("💀 Red playing Col 4 WINS! (Winner: {:?})", w);
                 if w == Player::Player1 {
                     println!("CRITICAL: Yellow missed a block at Col 4.");
                 }
             } else {
                 println!("Red playing Col 4 did not win immediately.");
             }
        }
    }
}
