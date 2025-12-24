use connect_four_ai_core::{GameState, Player};

#[test]
fn test_vertical_win_debug() {
    println!("=== Vertical Win Detection Debug ===");

    // Create a board where Player1 has 3 pieces in column 3
    let mut game = GameState::new();

    // Player1 moves to center 3 times
    game.make_move(3).unwrap(); // Player1: row 5, col 3
    game.make_move(0).unwrap(); // Player2: row 5, col 0 (different column)
    game.make_move(3).unwrap(); // Player1: row 4, col 3
    game.make_move(0).unwrap(); // Player2: row 4, col 0
    game.make_move(3).unwrap(); // Player1: row 3, col 3

    println!("Board after Player1 has 3 pieces in column 3:");
    print_board(&game);

    // Check if Player1 can win by placing in column 3
    let col = 3;
    let mut test_game = game.clone();
    test_game.make_move(col as u8).unwrap();
    
    println!("Testing if Player1 can win by placing in column 3:");
    if test_game.has_winner() {
        println!("WIN for {:?}!", test_game.get_winner().unwrap());
    } else {
        println!("No win");
    }

    // Check threat score
    let threat_score = game.threat_score(Player::Player1);
    println!("Player1 threat score: {}", threat_score);

    // Now test what happens if Player2 moves to column 3
    println!("\n=== Testing Player2 moving to column 3 ===");
    let mut test_game = game.clone();
    test_game.make_move(3).unwrap(); // Player2 moves to column 3

    println!("Board after Player2 moves to column 3:");
    print_board(&test_game);

    // Check if Player1 can now win
    let p1_threat = test_game.threat_score(Player::Player1);
    let p2_threat = test_game.threat_score(Player::Player2);
    println!("Player1 threat score: {}", p1_threat);
    println!("Player2 threat score: {}", p2_threat);

    // Test each possible move for Player1
    println!("\n=== Testing Player1's next move options ===");
    for col in 0..7 {
        if test_game.can_place_in_column(col) {
            let mut test_move = test_game.clone();
            test_move.make_move(col as u8).unwrap();

            if test_move.has_winner() {
                println!(
                    "Column {}: WIN for {:?}!",
                    col,
                    test_move.get_winner().unwrap()
                );
            } else {
                let eval = test_move.evaluate();
                println!("Column {}: evaluation {}", col, eval);
            }
        }
    }
}

fn print_board(game: &GameState) {
    for row in 0..6 {
        for col in 0..7 {
            match game.board[col][row] {
                connect_four_ai_core::Cell::Empty => print!(" ."),
                connect_four_ai_core::Cell::Player1 => print!(" X"),
                connect_four_ai_core::Cell::Player2 => print!(" O"),
            }
        }
        println!();
    }
    println!(" 0 1 2 3 4 5 6");
}
