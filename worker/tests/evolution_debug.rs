use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

fn evaluate_params_vs_default(params: &GeneticParams, num_games: usize) -> f64 {
    let default_params = GeneticParams::default();
    let mut wins = 0;

    for _ in 0..num_games {
        let mut game_state = GameState::new();
        let mut moves_played = 0;
        let max_moves = 42;

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let evolved_is_player2 = rng.gen_bool(0.5);

        while !game_state.is_game_over() && moves_played < max_moves {
            let current_player = game_state.current_player;
            let is_evolved_turn = if evolved_is_player2 {
                current_player == Player::Player2
            } else {
                current_player == Player::Player1
            };

            let ai_params = if is_evolved_turn {
                params.clone()
            } else {
                default_params.clone()
            };

            let mut ai_state = GameState::with_genetic_params(ai_params);
            ai_state.board = game_state.board.clone();
            ai_state.current_player = game_state.current_player;

            let mut ai = AI::new();
            let (best_move, _) = ai.get_best_move(&ai_state, 5);

            if let Some(column) = best_move {
                game_state.make_move(column).ok();
            } else {
                break;
            }
            moves_played += 1;
        }

        if let Some(winner) = game_state.get_winner() {
            let evolved_won = if evolved_is_player2 {
                winner == Player::Player2
            } else {
                winner == Player::Player1
            };

            if evolved_won {
                wins += 1;
            }
        }
    }

    wins as f64 / num_games as f64
}

fn main() {
    println!("🔍 Testing fitness distribution with 200 games per evaluation");

    // Test 20 random parameter sets with 200 games each (matching evolution settings)
    let mut fitness_scores = Vec::new();
    let mut high_fitness_count = 0;
    let mut perfect_fitness_count = 0;

    for i in 0..20 {
        let random_params = GeneticParams::random();
        let fitness = evaluate_params_vs_default(&random_params, 200);
        fitness_scores.push(fitness);

        if fitness > 0.8 {
            high_fitness_count += 1;
            println!("Test {}: {:.3} ⚠️ HIGH FITNESS", i + 1, fitness);
        } else if fitness > 0.6 {
            println!("Test {}: {:.3} (good)", i + 1, fitness);
        } else if fitness < 0.2 {
            println!("Test {}: {:.3} (poor)", i + 1, fitness);
        } else {
            println!("Test {}: {:.3}", i + 1, fitness);
        }

        if fitness >= 1.0 {
            perfect_fitness_count += 1;
        }
    }

    let avg_fitness = fitness_scores.iter().sum::<f64>() / fitness_scores.len() as f64;
    let max_fitness = fitness_scores.iter().fold(0.0_f64, |a, &b| a.max(b));
    let min_fitness = fitness_scores.iter().fold(1.0_f64, |a, &b| a.min(b));

    println!("\n=== Summary ===");
    println!("Average fitness: {:.3}", avg_fitness);
    println!("Max fitness: {:.3}", max_fitness);
    println!("Min fitness: {:.3}", min_fitness);
    println!("High fitness (>0.8) count: {}/20", high_fitness_count);
    println!("Perfect fitness (1.0) count: {}/20", perfect_fitness_count);

    if max_fitness >= 1.0 {
        println!("⚠️  PERFECT FITNESS DETECTED! This explains the 1.000 in first generation.");
    } else {
        println!("✅ No perfect fitness scores - the increased sample size is working!");
    }
}
