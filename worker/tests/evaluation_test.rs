use connect_four_ai_core::GameState;

#[test]
fn test_evaluation_perspective() {
    println!("Testing evaluation function perspective...");

    // Test empty board
    let mut game = GameState::new();
    println!("Empty board evaluation: {}", game.evaluate());
    println!("Current player: {:?}", game.current_player);

    // Test after Player1 makes a move
    game.make_move(3).unwrap();
    println!("After Player1 move to column 3: {}", game.evaluate());
    println!("Current player: {:?}", game.current_player);

    // Test after Player2 makes a move
    game.make_move(3).unwrap();
    println!("After Player2 move to column 3: {}", game.evaluate());
    println!("Current player: {:?}", game.current_player);

    // Test a winning position for Player1
    let mut winning_game = GameState::new();
    winning_game.make_move(0).unwrap(); // P1
    winning_game.make_move(1).unwrap(); // P2
    winning_game.make_move(0).unwrap(); // P1
    winning_game.make_move(1).unwrap(); // P2
    winning_game.make_move(0).unwrap(); // P1
    winning_game.make_move(1).unwrap(); // P2
    winning_game.make_move(0).unwrap(); // P1 wins

    println!("Winning position for Player1: {}", winning_game.evaluate());
    println!("Game over: {}", winning_game.is_game_over());
    println!("Winner: {:?}", winning_game.get_winner());
}
