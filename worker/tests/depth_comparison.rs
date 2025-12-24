use connect_four_ai_core::{GameState, AI};
use std::time::Instant;

#[test]
fn test_depth_comparison() {
    println!("=== MM Depth Comparison Test ===");

    let mut game = GameState::new();
    let mut ai_depth4 = AI::new();
    let mut ai_depth6 = AI::new();

    // Test a simple position
    println!("Empty board:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    // Make a move and see what happens
    game.make_move(3).unwrap();
    println!("\nAfter Player1 moves to column 3:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    // Test both depths
    println!("\n=== Testing Depth 4 vs Depth 6 ===");
    let valid_moves = game.get_valid_moves();
    println!("Valid moves: {:?}", valid_moves);

    let start_time = Instant::now();
    let (best_move_4, evaluations_4) = ai_depth4.get_best_move(&game, 4);
    let time_4 = start_time.elapsed();

    let start_time = Instant::now();
    let (best_move_6, evaluations_6) = ai_depth6.get_best_move(&game, 6);
    let time_6 = start_time.elapsed();

    println!(
        "Depth 4: Best move: {:?}, Nodes: {}, Time: {:?}",
        best_move_4, ai_depth4.nodes_evaluated, time_4
    );
    println!(
        "Depth 6: Best move: {:?}, Nodes: {}, Time: {:?}",
        best_move_6, ai_depth6.nodes_evaluated, time_6
    );

    println!("\nDepth 4 evaluations:");
    for eval in &evaluations_4 {
        println!("  Column {}: Score {:.2}", eval.column, eval.score);
    }

    println!("\nDepth 6 evaluations:");
    for eval in &evaluations_6 {
        println!("  Column {}: Score {:.2}", eval.column, eval.score);
    }

    // Test if they agree
    if best_move_4 == best_move_6 {
        println!("\n✅ Both depths agree on best move: {:?}", best_move_4);
    } else {
        println!("\n❌ Depths disagree:");
        println!("  Depth 4 chooses: {:?}", best_move_4);
        println!("  Depth 6 chooses: {:?}", best_move_6);
    }

    // Test a few more positions
    println!("\n=== Testing Multiple Positions ===");
    let mut test_game = GameState::new();

    for i in 0..3 {
        let current_player = test_game.current_player;
        let valid_moves = test_game.get_valid_moves();

        println!(
            "\nPosition {}: Player {:?}, Valid: {:?}",
            i, current_player, valid_moves
        );

        let (move_4, _) = ai_depth4.get_best_move(&test_game, 4);
        let (move_6, _) = ai_depth6.get_best_move(&test_game, 6);

        println!("  Depth 4 chooses: {:?}", move_4);
        println!("  Depth 6 chooses: {:?}", move_6);

        if move_4 == move_6 {
            println!("  ✅ Agree");
        } else {
            println!("  ❌ Disagree");
        }

        // Make the move (use depth 4's choice for consistency)
        if let Some(move_col) = move_4 {
            if test_game.make_move(move_col).is_ok() {
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
