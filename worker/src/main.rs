use connect_four_ai_core::{GameState, AI};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <get_move|evaluate> <input_file> [--depth N]",
            args[0]
        );
        process::exit(1);
    }

    let mut depth = 3u8;
    let mut i = 3;
    while i < args.len() {
        if args[i] == "--depth" && i + 1 < args.len() {
            depth = args[i + 1].parse().unwrap_or(3);
            i += 2;
        } else {
            i += 1;
        }
    }

    let command = &args[1];
    let input_file = &args[2];

    let input = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            process::exit(1);
        }
    };

    let game_state: GameState = match serde_json::from_str(&input) {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Invalid game state JSON: {}", e);
            process::exit(1);
        }
    };

    match command.as_str() {
        "get_move" => {
            let mut ai = AI::new();
            let (best_move, move_evaluations) = ai.get_best_move(&game_state, depth);
            let evaluation = game_state.evaluate();
            let response = serde_json::json!({
                "move": best_move,
                "evaluation": evaluation,
                "moveEvaluations": move_evaluations
            });
            println!("{}", serde_json::to_string(&response).unwrap());
        }
        "evaluate" => {
            let evaluation = game_state.evaluate();
            println!("{}", evaluation);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}
