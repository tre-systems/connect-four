use connect_four_ai_core::GameState;

#[test]
fn test_evaluation_debug() {
    println!("=== Evaluation Function Debug ===");

    let mut game = GameState::new();

    // Test empty board
    println!("Empty board evaluation: {}", game.evaluate());

    // Make some moves and see what happens
    game.make_move(3).unwrap(); // Player1 moves to center
    println!("After Player1 moves to column 3: {}", game.evaluate());

    game.make_move(3).unwrap(); // Player2 moves to center
    println!("After Player2 moves to column 3: {}", game.evaluate());

    game.make_move(0).unwrap(); // Player1 moves to column 0
    println!("After Player1 moves to column 0: {}", game.evaluate());

    game.make_move(0).unwrap(); // Player2 moves to column 0
    println!("After Player2 moves to column 0: {}", game.evaluate());

    // Test a position where the AI is getting 0.00 scores
    let mut test_game = GameState::new();
    test_game.make_move(3).unwrap(); // Player1: center
    test_game.make_move(3).unwrap(); // Player2: center
    test_game.make_move(0).unwrap(); // Player1: left
    test_game.make_move(0).unwrap(); // Player2: left

    println!("\nTest position evaluation: {}", test_game.evaluate());
    println!("Current player: {:?}", test_game.current_player);

    // Test all possible next moves
    let valid_moves = test_game.get_valid_moves();
    println!("Valid moves: {:?}", valid_moves);

    for &col in &valid_moves {
        let mut next_state = test_game.clone();
        if next_state.make_move(col).is_ok() {
            println!("Column {} evaluation: {}", col, next_state.evaluate());
        }
    }
}
