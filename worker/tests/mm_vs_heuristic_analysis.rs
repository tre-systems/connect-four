use connect_four_ai_core::{GameState, HeuristicAI, AI};
use std::time::Instant;

#[test]
fn test_mm_vs_heuristic_analysis() {
    println!("🔍 MM vs Heuristic Analysis");
    println!("{}", "=".repeat(50));

    // Test early game positions where strategic concepts matter
    test_early_game_positions();

    // Test evaluation function differences
    test_evaluation_differences();

    // Test depth vs evaluation quality
    test_depth_vs_evaluation();
}

fn test_early_game_positions() {
    println!("\n🎯 Test 1: Early Game Positions");
    println!("{}", "-".repeat(30));

    // Test position 1: Empty board
    println!("\n📊 Position 1: Empty Board");
    let mut game = GameState::new();
    test_position(&game, "Empty Board");

    // Test position 2: After first move
    println!("\n📊 Position 2: After First Move");
    game.make_move(3).unwrap();
    test_position(&game, "After First Move");

    // Test position 3: After a few moves
    println!("\n📊 Position 3: After Several Moves");
    game.make_move(3).unwrap();
    game.make_move(2).unwrap();
    game.make_move(4).unwrap();
    test_position(&game, "After Several Moves");

    // Test position 4: More complex early position
    println!("\n📊 Position 4: Complex Early Position");
    game.make_move(1).unwrap();
    game.make_move(5).unwrap();
    game.make_move(0).unwrap();
    game.make_move(6).unwrap();
    test_position(&game, "Complex Early Position");
}

fn test_evaluation_differences() {
    println!("\n🎯 Test 2: Evaluation Function Differences");
    println!("{}", "-".repeat(30));

    let mut game = GameState::new();

    // Create a position where evaluation differences are apparent
    let moves = [3, 3, 2, 4, 1, 5, 0, 6];

    for &col in &moves {
        game.make_move(col).unwrap();
    }

    println!("Position after {} moves:", moves.len());
    print_board(&game);
    println!("Evaluation: {}", game.evaluate());

    // Test what each AI sees
    let mut heuristic_ai = HeuristicAI::new();
    let mut mm_ai = AI::new();

    let (heuristic_move, heuristic_evals) = heuristic_ai.get_best_move(&game);
    let (mm_move, mm_evals) = mm_ai.get_best_move(&game, 3);

    println!("\nHeuristic AI evaluations:");
    for eval in &heuristic_evals {
        println!("  Column {}: Score {:.2}", eval.column, eval.score);
    }

    println!("\nMM-Depth3 evaluations:");
    for eval in &mm_evals {
        println!("  Column {}: Score {:.2}", eval.column, eval.score);
    }

    println!("\nChoices:");
    println!("  Heuristic: {:?}", heuristic_move);
    println!("  MM-Depth3: {:?}", mm_move);

    // This demonstrates that evaluation function quality is crucial
    // A good heuristic can make better decisions than deeper search with poor evaluation
}

fn test_depth_vs_evaluation() {
    println!("\n🎯 Test 3: Search Depth vs Evaluation Quality");
    println!("{}", "-".repeat(30));

    let mut game = GameState::new();

    // Create a position where evaluation quality matters more than search depth
    let moves = [3, 3, 2, 4, 1, 5, 0, 6, 3, 2, 4, 1, 5, 0, 6];

    for &col in &moves {
        game.make_move(col).unwrap();
    }

    println!("Complex position after {} moves:", moves.len());
    print_board(&game);
    println!("Evaluation: {}", game.evaluate());

    // Test different depths
    let mut ai = AI::new();
    let mut heuristic_ai = HeuristicAI::new();

    let (heuristic_move, _) = heuristic_ai.get_best_move(&game);

    let mut depth_results = Vec::new();
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let start = Instant::now();
        let (move_option, _) = ai.get_best_move(&game, depth);
        let time = start.elapsed();

        depth_results.push((depth, move_option, ai.nodes_evaluated, time));
    }

    println!("\nDepth vs Performance Analysis:");
    println!("Depth | Move | Nodes | Time");
    println!("------|------|-------|------");
    for (depth, move_option, nodes, time) in depth_results {
        println!(
            "{:5} | {:4?} | {:5} | {:?}",
            depth, move_option, nodes, time
        );
    }

    println!("\nHeuristic choice: {:?}", heuristic_move);

    // Key insight: Beyond a certain depth, the returns diminish
    // A good evaluation function can be more valuable than deeper search
}

fn test_position(game: &GameState, description: &str) {
    print_board(game);
    println!("{} - Evaluation: {}", description, game.evaluate());

    let mut heuristic_ai = HeuristicAI::new();
    let mut mm_ai = AI::new();

    let (heuristic_move, _) = heuristic_ai.get_best_move(game);
    let (mm_move, _) = mm_ai.get_best_move(game, 3);

    println!(
        "Heuristic: {:?}, MM-Depth3: {:?}",
        heuristic_move, mm_move
    );

    // Show evaluation differences if they exist
    if heuristic_move != mm_move {
        println!("⚠️  DIFFERENCE DETECTED: Heuristics and MM disagree!");
        println!("   This demonstrates how heuristics can capture strategic concepts");
        println!("   that tactical search might miss.");
    }
}

fn print_board(game_state: &GameState) {
    for row in (0..6).rev() {
        print!("|");
        for col in 0..7 {
            match game_state.board[col][row] {
                connect_four_ai_core::Cell::Empty => print!(" "),
                connect_four_ai_core::Cell::Player1 => print!("X"),
                connect_four_ai_core::Cell::Player2 => print!("O"),
            }
            print!("|");
        }
        println!();
    }
    println!(" 0 1 2 3 4 5 6");
}
