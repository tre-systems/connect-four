use connect_four_ai_core::{GameState, HeuristicAI, Player};

fn evaluate_position_fixed(game_state: &GameState, player: Player) -> f32 {
    let base_evaluation = game_state.evaluate() as f32;
    match player {
        Player::Player1 => base_evaluation,
        Player::Player2 => -base_evaluation,
    }
}

#[test]
fn test_heuristic_debug() {
    println!("=== Heuristic AI Debug ===");

    let mut game = GameState::new();
    let mut heuristic_ai = HeuristicAI::new();

    // Test a simple position
    println!("Empty board:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    // Make a move and see what happens
    game.make_move(3).unwrap();
    println!("\nAfter Player1 moves to column 3:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    // Test Heuristic AI move
    println!("\n=== Testing Heuristic AI ===");
    let valid_moves = game.get_valid_moves();
    println!("Valid moves: {:?}", valid_moves);

    let (best_move, _) = heuristic_ai.get_best_move(&game);
    println!("Heuristic AI chooses: {:?}", best_move);

    // Test all possible moves and their evaluations
    println!("\n=== Testing evaluate_position function ===");
    for &col in &valid_moves {
        let mut next_state = game.clone();
        if next_state.make_move(col).is_ok() {
            let base_score = next_state.evaluate();
            let p1_score = evaluate_position_fixed(&next_state, Player::Player1);
            let p2_score = evaluate_position_fixed(&next_state, Player::Player2);
            let opponent_score = evaluate_position_fixed(&next_state, game.current_player.opponent());
            println!(
                "Column {}: Base={}, P1={}, P2={}, Opponent={}",
                col, base_score, p1_score, p2_score, opponent_score
            );
        }
    }

    // Test a few more moves to see the pattern
    println!("\n=== Testing Multiple Moves ===");
    let mut test_game = GameState::new();

    for i in 0..5 {
        let current_player = test_game.current_player;
        let valid_moves = test_game.get_valid_moves();
        let (best_move, _) = heuristic_ai.get_best_move(&test_game);

        println!(
            "Move {}: Player {:?}, Valid: {:?}, Heuristic chooses: {:?}",
            i, current_player, valid_moves, best_move
        );

        if let Some(move_col) = best_move {
            if test_game.make_move(move_col as u8).is_ok() {
                println!("  Made move to column {}", move_col);
            } else {
                println!("  Failed to make move to column {}", move_col);
                break;
            }
        } else {
            println!("  No valid move found");
            break;
        }
    }
}
