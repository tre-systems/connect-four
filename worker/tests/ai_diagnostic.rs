use connect_four_ai_core::{GameState, HeuristicAI, AI};

#[test]
fn test_ai_diagnostic() {
    println!("🔍 AI Diagnostic Test");
    println!("====================");

    // Test 1: Basic evaluation
    test_basic_evaluation();

    // Test 2: Simple game scenarios
    test_simple_scenarios();

    // Test 3: AI decision making
    test_ai_decisions();

    // Test 4: Minimax behavior
    test_minimax_behavior();
}

fn test_basic_evaluation() {
    println!("\n📊 Test 1: Basic Evaluation");
    println!("---------------------------");

    let game_state = GameState::new();
    let eval = game_state.evaluate();
    println!("Empty board evaluation: {}", eval);

    // Test with a piece in center
    let mut game_state = GameState::new();
    game_state.make_move(3).unwrap();
    let eval = game_state.evaluate();
    println!("Center piece (Player1) evaluation: {}", eval);

    // Test with opponent piece
    game_state.make_move(3).unwrap();
    let eval = game_state.evaluate();
    println!("Two center pieces evaluation: {}", eval);
}

fn test_simple_scenarios() {
    println!("\n🎯 Test 2: Simple Game Scenarios");
    println!("--------------------------------");

    // Test winning move detection
    test_winning_move_detection();

    // Test blocking move detection
    test_blocking_move_detection();
}

fn test_winning_move_detection() {
    println!("\n🏆 Testing Winning Move Detection");

    // Create a scenario where Player1 can win
    let mut game_state = GameState::new();

    // Build up a winning position
    game_state.make_move(0).unwrap(); // P1
    game_state.make_move(1).unwrap(); // P2
    game_state.make_move(0).unwrap(); // P1
    game_state.make_move(1).unwrap(); // P2
    game_state.make_move(0).unwrap(); // P1

    println!("Board state before winning move:");
    print_board(&game_state);

    let eval = game_state.evaluate();
    println!("Evaluation before winning move: {}", eval);

    // Test if AI can find the winning move
    let mut ai = AI::new();
    let (best_move, evaluations) = ai.get_best_move(&game_state, 3);

    println!("AI evaluations:");
    for eval in &evaluations {
        println!("  Column {}: score {:.2}", eval.column, eval.score);
    }

    println!("Best move: {:?}", best_move);
    println!("Expected: Some(0) - the winning move");
}

fn test_blocking_move_detection() {
    println!("\n🛡️ Testing Blocking Move Detection");

    // Create a scenario where Player2 can win, Player1 should block
    let mut game_state = GameState::new();

    // Build up a threatening position for Player2
    game_state.make_move(1).unwrap(); // P1
    game_state.make_move(0).unwrap(); // P2
    game_state.make_move(1).unwrap(); // P1
    game_state.make_move(0).unwrap(); // P2
    game_state.make_move(1).unwrap(); // P1
    game_state.make_move(0).unwrap(); // P2

    println!("Board state before blocking move:");
    print_board(&game_state);

    let eval = game_state.evaluate();
    println!("Evaluation before blocking move: {}", eval);

    // Test if AI can find the blocking move
    let mut ai = AI::new();
    let (best_move, evaluations) = ai.get_best_move(&game_state, 3);

    println!("AI evaluations:");
    for eval in &evaluations {
        println!("  Column {}: score {:.2}", eval.column, eval.score);
    }

    println!("Best move: {:?}", best_move);
    println!("Expected: Some(0) - the blocking move");
}

fn test_ai_decisions() {
    println!("\n🤖 Test 3: AI Decision Making");
    println!("-----------------------------");

    // Test Heuristic AI vs Regular AI
    let mut game_state = GameState::new();
    game_state.make_move(3).unwrap(); // P1 center

    let mut heuristic_ai = HeuristicAI::new();
    let mut regular_ai = AI::new();

    let (heuristic_move, _) = heuristic_ai.get_best_move(&game_state);
    let (regular_move, _) = regular_ai.get_best_move(&game_state, 1);

    println!("After P1 plays center:");
    println!("Heuristic AI move: {:?}", heuristic_move);
    println!("Regular AI move (depth 1): {:?}", regular_move);
    println!("Expected: Both should prefer center or adjacent columns");
}

fn test_minimax_behavior() {
    println!("\n🧠 Test 4: Minimax Behavior");
    println!("---------------------------");

    // Test depth progression
    let mut game_state = GameState::new();
    game_state.make_move(3).unwrap(); // P1 center

    let mut ai = AI::new();

    for depth in 1..=3 {
        let (best_move, evaluations) = ai.get_best_move(&game_state, depth);
        println!(
            "Depth {}: Best move {:?}, Evaluated {} nodes",
            depth, best_move, ai.nodes_evaluated
        );

        for eval in &evaluations {
            println!("  Column {}: score {:.2}", eval.column, eval.score);
        }
        println!();
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
