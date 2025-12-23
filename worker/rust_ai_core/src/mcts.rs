use crate::{GameState, Player, COLS};
use rand::Rng;
use std::f32;

#[derive(Debug, Clone)]
pub struct MCTSNode {
    pub state: GameState,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub visits: u32,
    pub total_value: f32,
    pub prior_probability: f32,
    pub is_terminal: bool,
    pub valid_moves: Vec<u8>,
}

impl MCTSNode {
    pub fn new(state: GameState, parent: Option<usize>, prior_probability: f32) -> Self {
        let valid_moves = state.get_valid_moves();
        let is_terminal = state.is_game_over();

        Self {
            state,
            parent,
            children: Vec::new(),
            visits: 0,
            total_value: 0.0,
            prior_probability,
            is_terminal,
            valid_moves,
        }
    }

    pub fn ucb_score(&self, exploration_constant: f32, parent_visits: u32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration =
            exploration_constant * self.prior_probability * (parent_visits as f32).sqrt()
                / (1.0 + self.visits as f32);

        exploitation + exploration
    }

    pub fn is_fully_expanded(&self) -> bool {
        self.children.len() >= self.valid_moves.len()
    }
}

pub struct MCTS {
    pub nodes: Vec<MCTSNode>,
    pub exploration_constant: f32,
    pub num_simulations: usize,
}

impl MCTS {
    pub fn new(exploration_constant: f32, num_simulations: usize) -> Self {
        Self {
            nodes: Vec::new(),
            exploration_constant,
            num_simulations,
        }
    }

    pub fn search(
        &mut self,
        root_state: GameState,
        value_fn: &dyn Fn(&GameState) -> f32,
        policy_fn: &dyn Fn(&GameState) -> Vec<f32>,
    ) -> (u8, Vec<f32>) {
        // Create root node
        let root_idx = self.add_node(root_state, None, 1.0);

        // Run simulations
        for _ in 0..self.num_simulations {
            self.simulate(root_idx, value_fn, policy_fn);
        }

        // Get move probabilities
        let root_node = &self.nodes[root_idx];
        let mut move_probs = vec![0.0; COLS as usize];
        let mut total_visits = 0;

        for &child_idx in &root_node.children {
            let child = &self.nodes[child_idx];
            let move_idx = self.get_move_from_parent(root_idx, child_idx);
            move_probs[move_idx as usize] = child.visits as f32;
            total_visits += child.visits;
        }

        if total_visits > 0 {
            for prob in &mut move_probs {
                *prob /= total_visits as f32;
            }
        }

        // Select best move
        let best_move = if root_node.valid_moves.is_empty() {
            // No valid moves available
            0
        } else {
            root_node
                .valid_moves
                .iter()
                .max_by(|&&a, &&b| {
                    let a_visits = self.nodes[root_node
                        .children
                        .iter()
                        .position(|&c| self.get_move_from_parent(root_idx, c) == a)
                        .unwrap_or(0)]
                    .visits;
                    let b_visits = self.nodes[root_node
                        .children
                        .iter()
                        .position(|&c| self.get_move_from_parent(root_idx, c) == b)
                        .unwrap_or(0)]
                    .visits;
                    a_visits.cmp(&b_visits)
                })
                .copied()
                .unwrap_or(root_node.valid_moves[0]) // Fallback to first valid move
        };

        (best_move, move_probs)
    }

    fn simulate(
        &mut self,
        node_idx: usize,
        value_fn: &dyn Fn(&GameState) -> f32,
        policy_fn: &dyn Fn(&GameState) -> Vec<f32>,
    ) -> f32 {
        self.simulate_with_depth(node_idx, value_fn, policy_fn, 0)
    }

    fn simulate_with_depth(
        &mut self,
        node_idx: usize,
        value_fn: &dyn Fn(&GameState) -> f32,
        policy_fn: &dyn Fn(&GameState) -> Vec<f32>,
        depth: usize,
    ) -> f32 {
        const MAX_SIMULATION_DEPTH: usize = 100;

        if depth > MAX_SIMULATION_DEPTH {
            // Return a neutral value if we've gone too deep
            return 0.0;
        }

        {
            let node = &self.nodes[node_idx];
            if node.is_terminal {
                return self.get_terminal_value(&node.state);
            }
        }

        {
            let node = &self.nodes[node_idx];
            if !node.is_fully_expanded() {
                // Expand node
                let new_child_idx = self.expand_node(node_idx, policy_fn);
                
                // AlphaZero style: Use the value network prediction directly
                // No random rollouts!
                let value = value_fn(&self.nodes[new_child_idx].state);
                
                self.backpropagate(new_child_idx, value);
                return value;
            }
        }

        // Select child using UCB
        let parent_visits = self.nodes[node_idx].visits;
        let children = self.nodes[node_idx].children.clone();

        let best_child_idx = children
            .iter()
            .max_by(|&&a, &&b| {
                let a_score = self.nodes[a].ucb_score(self.exploration_constant, parent_visits);
                let b_score = self.nodes[b].ucb_score(self.exploration_constant, parent_visits);
                a_score
                    .partial_cmp(&b_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(node_idx);

        let value = self.simulate_with_depth(best_child_idx, value_fn, policy_fn, depth + 1);
        self.backpropagate(node_idx, value);
        value
    }

    fn expand_node(
        &mut self,
        node_idx: usize,
        policy_fn: &dyn Fn(&GameState) -> Vec<f32>,
    ) -> usize {
        let policy = {
            let node = &self.nodes[node_idx];
            policy_fn(&node.state)
        };

        // Find unexpanded move
        let expanded_moves: Vec<u8> = {
            let node = &self.nodes[node_idx];
            node.children
                .iter()
                .map(|&child_idx| self.get_move_from_parent(node_idx, child_idx))
                .collect()
        };

        let unexpanded_move = {
            let node = &self.nodes[node_idx];
            node.valid_moves
                .iter()
                .find(|&&mv| !expanded_moves.contains(&mv))
                .copied()
                .unwrap_or(0)
        };

        // Create new state
        let mut new_state = self.nodes[node_idx].state.clone();
        if new_state.make_move(unexpanded_move).is_ok() {
            let prior_prob = policy.get(unexpanded_move as usize).copied().unwrap_or(0.0);
            let child_idx = self.add_node(new_state, Some(node_idx), prior_prob);
            self.nodes[node_idx].children.push(child_idx);
            child_idx
        } else {
            node_idx // Fallback to current node
        }
    }



    fn backpropagate(&mut self, node_idx: usize, value: f32) {
        let mut current_idx = node_idx;

        while current_idx < self.nodes.len() {
            let parent_idx = self.nodes[current_idx].parent;
            self.nodes[current_idx].visits += 1;
            self.nodes[current_idx].total_value += value;

            if let Some(parent) = parent_idx {
                current_idx = parent;
            } else {
                // Reached root node
                break;
            }
        }
    }

    fn get_terminal_value(&self, state: &GameState) -> f32 {
        if let Some(winner) = state.get_winner() {
            match winner {
                Player::Player1 => 1.0,
                Player::Player2 => -1.0,
            }
        } else {
            0.0 // Draw
        }
    }

    fn add_node(
        &mut self,
        state: GameState,
        parent: Option<usize>,
        prior_probability: f32,
    ) -> usize {
        let node = MCTSNode::new(state, parent, prior_probability);
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn get_move_from_parent(&self, parent_idx: usize, child_idx: usize) -> u8 {
        let parent_state = &self.nodes[parent_idx].state;
        let child_state = &self.nodes[child_idx].state;

        // Find the move that was made
        for col in 0..COLS {
            let mut test_state = parent_state.clone();
            if test_state.make_move(col as u8).is_ok() && test_state.board == child_state.board {
                return col as u8;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts_node_creation() {
        let state = GameState::new();
        let node = MCTSNode::new(state, None, 1.0);

        assert_eq!(node.visits, 0);
        assert_eq!(node.total_value, 0.0);
        assert_eq!(node.prior_probability, 1.0);
        assert!(!node.is_terminal);
        assert_eq!(node.valid_moves.len(), 7);
    }

    #[test]
    fn test_mcts_ucb_score() {
        let state = GameState::new();
        let mut node = MCTSNode::new(state, None, 1.0);

        // Test unvisited node
        assert_eq!(node.ucb_score(1.0, 10), f32::INFINITY);

        // Test visited node
        node.visits = 5;
        node.total_value = 3.0;
        let score = node.ucb_score(1.0, 10);
        assert!(score.is_finite());
        assert!(score > 0.0);
    }

    #[test]
    fn test_mcts_search() {
        let state = GameState::new();
        let mut mcts = MCTS::new(1.0, 100);

        let value_fn = |_state: &GameState| 0.0;
        let policy_fn = |_state: &GameState| vec![1.0 / 7.0; 7];

        let (best_move, move_probs) = mcts.search(state, &value_fn, &policy_fn);

        assert!(usize::from(best_move) < COLS);
        assert_eq!(move_probs.len(), COLS as usize);
        assert!((move_probs.iter().sum::<f32>() - 1.0).abs() < 0.001);
    }
}
