use connect_four_ai_core::{GameState, Player, AI};

#[test]
fn test_simple_game() {
    println!("=== Simple Game Test ===");

    let mut game = GameState::new();
    let mut ai = AI::new();

    // Play a simple game: MM-Depth6 vs Random
    println!("Playing MM-Depth6 vs Random...");

    for move_num in 1..=10 {
        let current_player = game.current_player;
        let valid_moves = game.get_valid_moves();

        println!(
            "\nMove {}: Player {:?}, Valid: {:?}",
            move_num, current_player, valid_moves
        );

        let best_move = if current_player == Player::Player1 {
            // MM AI (Player1)
            let (move_result, evaluations) = ai.get_best_move(&game, 6);
            println!("  MM-Depth6 chooses: {:?}", move_result);
            println!("  Evaluations:");
            for eval in &evaluations[..evaluations.len().min(3)] {
                println!("    Column {}: Score {:.2}", eval.column, eval.score);
            }
            move_result
        } else {
            // Random AI (Player2)
            let random_move = valid_moves[0]; // Always choose first valid move for simplicity
            println!("  Random chooses: {:?}", Some(random_move));
            Some(random_move)
        };

        if let Some(move_col) = best_move {
            if game.make_move(move_col).is_ok() {
                println!("  Made move to column {}", move_col);

                // Check if game is over
                if game.is_game_over() {
                    if let Some(winner) = game.get_winner() {
                        println!("  Game over! Winner: {:?}", winner);
                    } else {
                        println!("  Game over! Draw");
                    }
                    break;
                }
            } else {
                println!("  Failed to make move to column {}", move_col);
                break;
            }
        } else {
            println!("  No valid move found");
            break;
        }
    }

    println!("\nFinal game state:");
    println!("  Winner: {:?}", game.get_winner());
    println!("  Game over: {}", game.is_game_over());
    println!("  Final evaluation: {}", game.evaluate());
}
