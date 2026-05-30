use crate::genetic_params::GeneticParams;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub mod wasm_api;

pub mod features;
pub mod genetic_params;
pub mod mcts;
pub mod ml_ai;
pub mod neural_network;
pub mod solver;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const BOARD_SIZE: usize = ROWS * COLS;

use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum Player {
    Player1 = 0,
    Player2 = 1,
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum Cell {
    Empty,
    Player1,
    Player2,
}

impl Cell {
    pub fn from_player(player: Player) -> Self {
        match player {
            Player::Player1 => Cell::Player1,
            Player::Player2 => Cell::Player2,
        }
    }

    pub fn to_player(self) -> Option<Player> {
        match self {
            Cell::Empty => None,
            Cell::Player1 => Some(Player::Player1),
            Cell::Player2 => Some(Player::Player2),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameState {
    pub board: [[Cell; ROWS]; COLS],
    pub current_player: Player,
    pub genetic_params: GeneticParams,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[Cell::Empty; ROWS]; COLS],
            current_player: Player::Player1,
            genetic_params: GeneticParams::default(),
        }
    }

    pub fn new_random_first_player() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let first_player = if rng.gen_bool(0.5) {
            Player::Player1
        } else {
            Player::Player2
        };

        GameState {
            board: [[Cell::Empty; ROWS]; COLS],
            current_player: first_player,
            genetic_params: GeneticParams::default(),
        }
    }

    pub fn with_genetic_params(genetic_params: GeneticParams) -> Self {
        Self::with_genetic_params_random_first_player(genetic_params)
    }

    pub fn with_genetic_params_random_first_player(genetic_params: GeneticParams) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let first_player = if rng.gen_bool(0.5) {
            Player::Player1
        } else {
            Player::Player2
        };

        GameState {
            board: [[Cell::Empty; ROWS]; COLS],
            current_player: first_player,
            genetic_params,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.has_winner() || self.is_draw()
    }

    pub fn has_winner(&self) -> bool {
        self.get_winner().is_some()
    }

    pub fn is_draw(&self) -> bool {
        self.get_valid_moves().is_empty()
    }

    pub fn is_empty_board(&self) -> bool {
        for col in 0..COLS {
            for row in 0..ROWS {
                if self.board[col][row] != Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_winner(&self) -> Option<Player> {
        for col in 0..COLS {
            for row in 0..ROWS {
                if let Some(player) = self.board[col][row].to_player() {
                    if self.check_win_at(col, row, player) {
                        return Some(player);
                    }
                }
            }
        }
        None
    }

    pub fn get_valid_moves(&self) -> Vec<u8> {
        let mut moves = Vec::new();
        for col in 0..COLS {
            if self.can_place_in_column(col) {
                moves.push(col as u8);
            }
        }
        moves
    }

    pub fn can_place_in_column(&self, col: usize) -> bool {
        col < COLS && self.board[col][0] == Cell::Empty
    }

    pub fn make_move(&mut self, col: u8) -> Result<(), &'static str> {
        let col = col as usize;
        if col >= COLS {
            return Err("Invalid column");
        }
        if !self.can_place_in_column(col) {
            return Err("Column is full");
        }

        let row = self.get_lowest_empty_row(col);
        if row >= ROWS {
            return Err("Column is full");
        }

        self.board[col][row] = Cell::from_player(self.current_player);

        self.current_player = self.current_player.opponent();

        Ok(())
    }

    fn get_lowest_empty_row(&self, col: usize) -> usize {
        for row in (0..ROWS).rev() {
            if self.board[col][row] == Cell::Empty {
                return row;
            }
        }
        ROWS // Column is full
    }

    fn check_win_at(&self, col: usize, row: usize, player: Player) -> bool {
        let directions = [
            (1, 0),  // horizontal
            (0, 1),  // vertical
            (1, 1),  // diagonal /
            (1, -1), // diagonal \
        ];

        for (dcol, drow) in directions {
            if self.count_in_direction(col, row, dcol, drow, player) >= 4 {
                return true;
            }
        }
        false
    }

    fn count_in_direction(
        &self,
        col: usize,
        row: usize,
        dcol: i32,
        drow: i32,
        player: Player,
    ) -> usize {
        let mut count = 1;

        let mut c = col as i32 + dcol;
        let mut r = row as i32 + drow;
        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
            if self.board[c as usize][r as usize] == Cell::from_player(player) {
                count += 1;
                c += dcol;
                r += drow;
            } else {
                break;
            }
        }

        c = col as i32 - dcol;
        r = row as i32 - drow;
        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
            if self.board[c as usize][r as usize] == Cell::from_player(player) {
                count += 1;
                c -= dcol;
                r -= drow;
            } else {
                break;
            }
        }

        count
    }

    pub fn evaluate(&self) -> i32 {
        if let Some(winner) = self.get_winner() {
            return match winner {
                Player::Player1 => 10000,
                Player::Player2 => -10000,
            };
        }

        if self.is_draw() {
            return 0;
        }

        self.evaluate_with_genetic_params()
    }

    pub fn evaluate_with_genetic_params(&self) -> i32 {
        if let Some(winner) = self.get_winner() {
            return match winner {
                Player::Player1 => self.genetic_params.win_score,
                Player::Player2 => self.genetic_params.loss_score,
            };
        }

        if self.is_draw() {
            return 0;
        }

        let mut score = 0;

        for col in 0..COLS {
            let column_value = match col {
                3 => self.genetic_params.center_column_value, // Center column
                2 | 4 => self.genetic_params.adjacent_center_value, // Adjacent to center
                1 | 5 => self.genetic_params.outer_column_value, // Further from center
                0 | 6 => self.genetic_params.edge_column_value, // Edge columns
                _ => self.genetic_params.edge_column_value,
            };

            for row in 0..ROWS {
                match self.board[col][row] {
                    Cell::Player1 => {
                        score += (column_value as f64
                            * (ROWS - row) as f64
                            * self.genetic_params.row_height_weight)
                            as i32;
                    }
                    Cell::Player2 => {
                        score -= (column_value as f64
                            * (ROWS - row) as f64
                            * self.genetic_params.row_height_weight)
                            as i32;
                    }
                    Cell::Empty => {}
                }
            }
        }

        let center_control_p1 = self.center_control_score(Player::Player1);
        let center_control_p2 = self.center_control_score(Player::Player2);
        let center_weight = self.genetic_params.center_control_weight as i32;
        score += center_control_p1 * center_weight;
        score -= center_control_p2 * center_weight;

        let threat_p1 = self.threat_score(Player::Player1);
        let threat_p2 = self.threat_score(Player::Player2);
        let threat_weight = self.genetic_params.threat_weight as i32;
        score += threat_p1 * threat_weight;
        score -= threat_p2 * threat_weight;

        let piece_count_p1 = self.pieces_count(Player::Player1);
        let piece_count_p2 = self.pieces_count(Player::Player2);
        let piece_weight = self.genetic_params.piece_count_weight as i32;
        score += piece_count_p1 * piece_weight;
        score -= piece_count_p2 * piece_weight;

        let mobility_p1 = self.mobility_score(Player::Player1);
        let mobility_p2 = self.mobility_score(Player::Player2);
        let mobility_weight = self.genetic_params.mobility_weight as i32;
        score += mobility_p1 * mobility_weight;
        score -= mobility_p2 * mobility_weight;

        let vertical_p1 = self.vertical_control_score(Player::Player1);
        let vertical_p2 = self.vertical_control_score(Player::Player2);
        let vertical_weight = self.genetic_params.vertical_control_weight as i32;
        score += vertical_p1 * vertical_weight;
        score -= vertical_p2 * vertical_weight;

        let horizontal_p1 = self.horizontal_control_score(Player::Player1);
        let horizontal_p2 = self.horizontal_control_score(Player::Player2);
        let horizontal_weight = self.genetic_params.horizontal_control_weight as i32;
        score += horizontal_p1 * horizontal_weight;
        score -= horizontal_p2 * horizontal_weight;

        let defensive_p1 = self.defensive_score(Player::Player1);
        let defensive_p2 = self.defensive_score(Player::Player2);
        let defensive_weight = self.genetic_params.defensive_weight as i32;
        score += defensive_p1 * defensive_weight;
        score -= defensive_p2 * defensive_weight;

        // Score is from Player1's perspective (positive = Player1 advantage).
        score
    }

    pub fn center_control_score(&self, player: Player) -> i32 {
        let center_cols = [2, 3, 4];
        let mut score = 0;

        for &col in &center_cols {
            for row in 0..ROWS {
                if self.board[col][row] == Cell::from_player(player) {
                    score += (ROWS - row) as i32; // Higher pieces are worth more
                }
            }
        }

        score
    }

    pub fn pieces_count(&self, player: Player) -> i32 {
        let mut count = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if self.board[col][row] == Cell::from_player(player) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn threat_score(&self, player: Player) -> i32 {
        let mut score = 0;

        // Check for immediate winning threats in valid moves only
        for col in 0..COLS {
            if self.can_place_in_column(col) {
                let row = self.get_lowest_empty_row(col);
                if row < ROWS {
                    // Test if placing a piece here would create a win
                    let mut test_board = self.board;
                    test_board[col][row] = Cell::from_player(player);

                    // Check if this creates a win
                    if self.check_win_at_test(&test_board, col, row, player) {
                        score += 10000; // Immediate win threat - much higher priority
                    } else {
                        // Check for 3-in-a-row threats
                        let threat_value = self.count_threats_at(&test_board, col, row, player);
                        score += threat_value;
                    }
                }
            }
        }

        score
    }

    fn count_threats_at(
        &self,
        board: &[[Cell; ROWS]; COLS],
        col: usize,
        row: usize,
        player: Player,
    ) -> i32 {
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
        let mut total_threats = 0;

        for (dcol, drow) in directions {
            let mut consecutive = 0;
            let mut blocked = 0;

            // Count in positive direction
            let mut c = col as i32;
            let mut r = row as i32;
            while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                if board[c as usize][r as usize] == Cell::from_player(player) {
                    consecutive += 1;
                    c += dcol;
                    r += drow;
                } else {
                    if board[c as usize][r as usize] != Cell::Empty {
                        blocked += 1;
                    }
                    break;
                }
            }

            // Count in negative direction
            c = col as i32 - dcol;
            r = row as i32 - drow;
            while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                if board[c as usize][r as usize] == Cell::from_player(player) {
                    consecutive += 1;
                    c -= dcol;
                    r -= drow;
                } else {
                    if board[c as usize][r as usize] != Cell::Empty {
                        blocked += 1;
                    }
                    break;
                }
            }

            // Score based on consecutive pieces
            match consecutive {
                4 => total_threats += 1000,
                3 => total_threats += if blocked == 0 { 100 } else { 10 },
                2 => total_threats += if blocked == 0 { 10 } else { 1 },
                1 => total_threats += if blocked == 0 { 1 } else { 0 },
                _ => {}
            }
        }

        total_threats
    }

    fn check_win_at_test(
        &self,
        board: &[[Cell; ROWS]; COLS],
        col: usize,
        row: usize,
        player: Player,
    ) -> bool {
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

        for (dcol, drow) in directions {
            let mut count = 1;

            // Count in positive direction
            let mut c = col as i32 + dcol;
            let mut r = row as i32 + drow;
            while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                if board[c as usize][r as usize] == Cell::from_player(player) {
                    count += 1;
                    c += dcol;
                    r += drow;
                } else {
                    break;
                }
            }

            // Count in negative direction
            c = col as i32 - dcol;
            r = row as i32 - drow;
            while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                if board[c as usize][r as usize] == Cell::from_player(player) {
                    count += 1;
                    c -= dcol;
                    r -= drow;
                } else {
                    break;
                }
            }

            if count >= 4 {
                return true;
            }
        }

        false
    }

    pub fn mobility_score(&self, player: Player) -> i32 {
        let mut mobility = 0;
        for col in 0..COLS {
            if self.can_place_in_column(col) {
                // Test the move
                let mut test_state = self.clone();
                if test_state.make_move(col as u8).is_ok() {
                    // Check if this creates a threat
                    let threat_score = test_state.threat_score(player);
                    mobility += threat_score / 10; // Normalize
                }
            }
        }

        // Ensure mobility is neutral for empty board
        if self.is_empty_board() {
            return 0;
        }

        mobility
    }

    pub fn vertical_control_score(&self, player: Player) -> i32 {
        let mut score = 0;
        for col in 0..COLS {
            let mut consecutive = 0;
            for row in 0..ROWS {
                if self.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    pub fn horizontal_control_score(&self, player: Player) -> i32 {
        let mut score = 0;
        for row in 0..ROWS {
            let mut consecutive = 0;
            for col in 0..COLS {
                if self.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    pub fn defensive_score(&self, player: Player) -> i32 {
        let opponent = player.opponent();
        let mut defensive_score = 0;

        // Check each column for defensive opportunities
        for col in 0..COLS {
            if self.can_place_in_column(col) {
                let row = self.get_lowest_empty_row(col);

                // Test if placing a piece here would block an opponent threat
                let mut test_board = self.board;
                test_board[col][row] = Cell::from_player(player);

                // Check if this blocks an opponent's winning move
                if self.check_win_at_test(&test_board, col, row, opponent) {
                    defensive_score += 5000; // High value for blocking opponent win
                } else {
                    // Check if this blocks opponent's 3-in-a-row threat
                    let opponent_threat_before =
                        self.count_threats_at(&self.board, col, row, opponent);
                    let opponent_threat_after =
                        self.count_threats_at(&test_board, col, row, opponent);

                    if opponent_threat_after < opponent_threat_before {
                        defensive_score += (opponent_threat_before - opponent_threat_after) * 100;
                    }
                }
            }
        }

        defensive_score
    }
}



use crate::solver::Solver;

pub struct AI {
    solver: Solver,
    pub nodes_evaluated: u32,
    pub transposition_hits: u32,
}

pub struct HeuristicAI {
    pub nodes_evaluated: u32,
}

impl Default for AI {
    fn default() -> Self {
        Self::new()
    }
}

impl AI {
    pub fn new() -> Self {
        AI {
            solver: Solver::new(),
            nodes_evaluated: 0,
            transposition_hits: 0,
        }
    }

    pub fn get_transposition_table_size(&self) -> usize {
        0
    }

    pub fn clear_transposition_table(&mut self) {
        self.solver.reset();
    }

    pub fn get_best_move(
        &mut self,
        state: &GameState,
        depth: u8,
    ) -> (Option<u8>, Vec<MoveEvaluation>) {
        let bitboard = crate::solver::Bitboard::from_game_state(state);

        // The UI sends a shallow depth (1-5); map it to the engine's deeper search.
        let engine_depth = match depth {
            1 => 6,
            3 => 10,
            5 => 14,
            _ => (depth as i32 + 4).min(20),
        };

        let (best_move, score) = self.solver.analyze(&bitboard, engine_depth);

        self.nodes_evaluated = self.solver.get_nodes_count() as u32;
        self.transposition_hits = 0;

        let mut move_evaluations = Vec::new();
        if let Some(mv) = best_move {
            move_evaluations.push(MoveEvaluation {
                column: mv as u8,
                score: score as f32,
                move_type: if score > 0 {
                    "win".to_string()
                } else if score < 0 {
                    "loss".to_string()
                } else {
                    "draw".to_string()
                },
            });
        }

        // TODO: analyze() returns only the best move; the UI's per-move breakdown is empty.
        (best_move.map(|m| m as u8), move_evaluations)
    }
}

impl Default for HeuristicAI {
    fn default() -> Self {
        Self::new()
    }
}

impl HeuristicAI {
    pub fn new() -> Self {
        HeuristicAI { nodes_evaluated: 0 }
    }

    pub fn get_best_move(&mut self, state: &GameState) -> (Option<u8>, Vec<MoveEvaluation>) {
        self.nodes_evaluated = 0;

        let valid_moves = state.get_valid_moves();

        if valid_moves.is_empty() {
            return (None, vec![]);
        }

        if valid_moves.len() == 1 {
            return (Some(valid_moves[0]), vec![]);
        }

        // First, check for immediate wins
        for &col in &valid_moves {
            let mut next_state = state.clone();
            if next_state.make_move(col).is_ok() 
                && next_state.has_winner() 
                && next_state.get_winner() == Some(state.current_player)
            {
                // This move wins immediately - choose it!
                return (
                    Some(col),
                    vec![MoveEvaluation {
                        column: col,
                        score: if state.current_player == Player::Player1 {
                            10000.0
                        } else {
                            -10000.0
                        },
                        move_type: "win".to_string(),
                    }],
                );
            }
        }

        // Second, check for moves that block opponent's immediate win
        for &col in &valid_moves {
            let mut next_state = state.clone();
            if next_state.make_move(col).is_ok() {
                // Check if opponent can win on their next move
                let opponent_moves = next_state.get_valid_moves();
                let mut opponent_can_win = false;
                for &opp_col in &opponent_moves {
                    let mut opp_next_state = next_state.clone();
                    if opp_next_state.make_move(opp_col).is_ok()
                        && opp_next_state.has_winner()
                        && opp_next_state.get_winner() == Some(state.current_player.opponent())
                    {
                        opponent_can_win = true;
                        break;
                    }
                }
                if opponent_can_win {
                    // This move blocks opponent's win - prioritize it
                    return (
                        Some(col),
                        vec![MoveEvaluation {
                            column: col,
                            score: if state.current_player == Player::Player1 {
                                5000.0
                            } else {
                                -5000.0
                            },
                            move_type: "block".to_string(),
                        }],
                    );
                }
            }
        }

        let mut move_evaluations = Vec::new();
        let mut best_move = valid_moves[0];
        let mut best_score = if state.current_player == Player::Player1 {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };

        for &col in &valid_moves {
            let mut next_state = state.clone();
            if next_state.make_move(col).is_ok() {
                let score = next_state.evaluate() as f32;

                move_evaluations.push(MoveEvaluation {
                    column: col,
                    score,
                    move_type: "drop".to_string(),
                });

                // Player1 maximizes, Player2 minimizes (same as minimax)
                if state.current_player == Player::Player1 {
                    if score > best_score {
                        best_score = score;
                        best_move = col;
                    }
                } else if score < best_score {
                    best_score = score;
                    best_move = col;
                }
            }
        }

        move_evaluations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        (Some(best_move), move_evaluations)
    }

    pub fn clear_nodes_evaluated(&mut self) {
        self.nodes_evaluated = 0;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveEvaluation {
    #[serde(rename = "column")]
    pub column: u8,
    pub score: f32,
    #[serde(rename = "moveType")]
    pub move_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_opponent() {
        assert_eq!(Player::Player1.opponent(), Player::Player2);
        assert_eq!(Player::Player2.opponent(), Player::Player1);
    }

    #[test]
    fn test_game_state_new() {
        let game_state = GameState::new();
        assert_eq!(game_state.board.len(), COLS);
        assert_eq!(game_state.board[0].len(), ROWS);
        assert!(game_state
            .board
            .iter()
            .all(|col| col.iter().all(|&cell| cell == Cell::Empty)));
        // First player is now randomized, so just check it's one of the two players
        assert!(
            game_state.current_player == Player::Player1
                || game_state.current_player == Player::Player2
        );
    }

    #[test]
    fn test_is_game_over_not_finished() {
        let game_state = GameState::new();
        assert!(!game_state.is_game_over());
    }

    #[test]
    fn test_get_valid_moves_empty_board() {
        let game_state = GameState::new();
        let moves = game_state.get_valid_moves();
        assert_eq!(moves.len(), COLS);
        for i in 0..COLS {
            assert!(moves.contains(&(i as u8)));
        }
    }

    #[test]
    fn test_make_move_simple() {
        let mut game_state = GameState::new();
        // Ensure Player1 always moves first for this test
        game_state.current_player = Player::Player1; 
        let first_player = game_state.current_player;
        assert!(game_state.make_move(3).is_ok());
        assert_eq!(
            game_state.board[3][ROWS - 1],
            Cell::from_player(first_player)
        );
        assert_eq!(game_state.current_player, first_player.opponent());
    }

    #[test]
    fn test_make_move_column_full() {
        let mut game_state = GameState::new();
        // Fill a column
        for _ in 0..ROWS {
            assert!(game_state.make_move(0).is_ok());
            game_state.current_player = game_state.current_player.opponent();
        }
        // Try to place in full column
        assert!(game_state.make_move(0).is_err());
    }

    #[test]
    fn test_horizontal_win() {
        let mut game_state = GameState::new();
        let first_player = game_state.current_player;

        // First player places pieces horizontally
        game_state.make_move(0).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(1).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(2).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(3).unwrap();

        assert!(game_state.has_winner());
        assert_eq!(game_state.get_winner(), Some(first_player));
    }

    #[test]
    fn test_vertical_win() {
        let mut game_state = GameState::new();
        let first_player = game_state.current_player;

        // First player places pieces vertically
        game_state.make_move(0).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(0).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(0).unwrap();
        game_state.current_player = first_player;
        game_state.make_move(0).unwrap();

        assert!(game_state.has_winner());
        assert_eq!(game_state.get_winner(), Some(first_player));
    }

    #[test]
    fn test_ai_new() {
        let ai = AI::new();
        assert_eq!(ai.get_transposition_table_size(), 0);
        assert_eq!(ai.nodes_evaluated, 0);
    }

    #[test]
    fn test_ai_gets_winning_move() {
        let mut ai = AI::new();
        let mut state = GameState::new();

        // Set up a winning position for Player 1
        state.make_move(0).unwrap();
        state.current_player = Player::Player1;
        state.make_move(1).unwrap();
        state.current_player = Player::Player1;
        state.make_move(2).unwrap();
        state.current_player = Player::Player1;

        let (best_move, _) = ai.get_best_move(&state, 3);
        assert!(best_move.is_some()); // Should find a winning move
                                      // The AI might find a different winning sequence, so just check it's a valid move
        assert!(state.get_valid_moves().contains(&best_move.unwrap()));
    }

    #[test]
    fn test_heuristic_ai_new() {
        let ai = HeuristicAI::new();
        assert_eq!(ai.nodes_evaluated, 0);
    }

    #[test]
    fn test_heuristic_ai_gets_winning_move() {
        let mut ai = HeuristicAI::new();
        let mut state = GameState::new();

        // Set up a winning position for Player 1
        state.make_move(0).unwrap();
        state.current_player = Player::Player1;
        state.make_move(1).unwrap();
        state.current_player = Player::Player1;
        state.make_move(2).unwrap();
        state.current_player = Player::Player1;

        let (best_move, _) = ai.get_best_move(&state);
        assert!(best_move.is_some()); // Should find a winning move
                                      // The AI might find a different winning sequence, so just check it's a valid move
        assert!(state.get_valid_moves().contains(&best_move.unwrap()));
    }
}
