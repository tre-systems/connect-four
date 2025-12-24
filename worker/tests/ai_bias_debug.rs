use connect_four_ai_core::{GameState, Player, AI};

#[test]
fn test_ai_bias_debug() {
    println!("=== AI Bias Debug Test ===");

    // Test 1: Simple game with AI vs AI
    println!("\n=== Test 1: AI vs AI Game ===");
    let mut game = GameState::new();
    let mut ai = AI::new();

    println!("Starting game - Player1 goes first");
    println!("Initial board evaluation: {}", game.evaluate());

    let mut move_count = 0;
    while !game.is_game_over() && move_count < 42 {
        let current_player = game.current_player;
        println!("\nMove {}: {:?}'s turn", move_count + 1, current_player);

        // Get AI move
        let (best_move, evaluations) = ai.get_best_move(&game, 3);

        if let Some(column) = best_move {
            println!(
                "AI chooses column {} (score: {:.1})",
                column, evaluations[0].score
            );

            // Show all move evaluations
            println!("All move evaluations:");
            for eval in &evaluations[..std::cmp::min(3, evaluations.len())] {
                println!("  Column {}: {:.1}", eval.column, eval.score);
            }

            game.make_move(column).unwrap();
            println!("Board evaluation after move: {}", game.evaluate());
        } else {
            println!("No valid moves available");
            break;
        }

        move_count += 1;
    }

    if let Some(winner) = game.get_winner() {
        println!("\n🎯 Game Over: {:?} wins!", winner);
    } else {
        println!("\n🤝 Game Over: Draw");
    }

    // Test 2: Check if there's a bias in the evaluation function
    println!("\n=== Test 2: Evaluation Function Bias ===");
    let mut test_game = GameState::new();

    // Test empty board
    println!("Empty board evaluation: {}", test_game.evaluate());

    // Test after Player1 moves to center
    test_game.make_move(3).unwrap();
    println!("After Player1 center move: {}", test_game.evaluate());

    // Test after Player2 moves to center
    test_game.make_move(3).unwrap();
    println!("After Player2 center move: {}", test_game.evaluate());

    // Test after Player1 moves to edge
    test_game.make_move(0).unwrap();
    println!("After Player1 edge move: {}", test_game.evaluate());

    // Test after Player2 moves to edge
    test_game.make_move(0).unwrap();
    println!("After Player2 edge move: {}", test_game.evaluate());

    // Test 3: Check if there's a bias in move selection
    println!("\n=== Test 3: Move Selection Bias ===");
    let mut bias_test = GameState::new();

    // Player1's first move options
    println!("Player1's first move options:");
    let (p1_move, p1_evals) = ai.get_best_move(&bias_test, 3);
    for eval in &p1_evals[..std::cmp::min(4, p1_evals.len())] {
        println!("  Column {}: {:.1}", eval.column, eval.score);
    }

    // Make Player1's move
    if let Some(column) = p1_move {
        bias_test.make_move(column).unwrap();
        println!("Player1 chose column {}", column);

        // Player2's move options
        println!("Player2's move options:");
        let (_p2_move, p2_evals) = ai.get_best_move(&bias_test, 3);
        for eval in &p2_evals[..std::cmp::min(4, p2_evals.len())] {
            println!("  Column {}: {:.1}", eval.column, eval.score);
        }
    }

    // Test 4: Check if the issue is with depth 3
    println!("\n=== Test 4: Depth Comparison ===");
    let depth_test = GameState::new();

    for depth in [1, 2, 3, 4] {
        let mut test_ai = AI::new();
        let (move_choice, evals) = test_ai.get_best_move(&depth_test, depth);
        if let Some(col) = move_choice {
            println!(
                "Depth {}: Column {} (score: {:.1})",
                depth, col, evals[0].score
            );
        }
    }

    // Test 5: Check opponent threat detection
    println!("\n=== Test 5: Opponent Threat Detection ===");
    let mut threat_test = GameState::new();

    // Player1 moves to center
    threat_test.make_move(3).unwrap();
    println!("After Player1 moves to center:");
    println!("Board evaluation: {}", threat_test.evaluate());
    println!(
        "Player1 threat score: {}",
        threat_test.threat_score(Player::Player1)
    );
    println!(
        "Player2 threat score: {}",
        threat_test.threat_score(Player::Player2)
    );

    // Check what happens if Player2 moves to the same column
    let mut test_move = threat_test.clone();
    test_move.make_move(3).unwrap();
    println!("If Player2 moves to same column:");
    println!("Board evaluation: {}", test_move.evaluate());
    println!(
        "Player1 threat score: {}",
        test_move.threat_score(Player::Player1)
    );
    println!(
        "Player2 threat score: {}",
        test_move.threat_score(Player::Player2)
    );

    // Check if Player1 can win on next move
    if test_move.threat_score(Player::Player1) >= 10000 {
        println!("⚠️  WARNING: Player1 has a winning threat!");
    }
}
