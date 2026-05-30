use crate::{GameState, COLS};
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
    pub r#move: Option<u8>, // The move that led to this node
}

impl MCTSNode {
    pub fn new(state: GameState, parent: Option<usize>, prior_probability: f32, r#move: Option<u8>) -> Self {
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
            r#move,
        }
    }

    pub fn ucb_score(&self, exploration_constant: f32, parent_visits: u32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }

        // Standard AlphaZero UCB: Q + U
        // exploitation = average value for the player who moves TO this node (Parent).
        // Since self.total_value is stored from THIS node's perspective, 
        // we must negate it to get the parent's value.
        let exploitation = -(self.total_value / self.visits as f32);
        
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
    pub terminal_nodes_found: u32,
}

impl MCTS {
    pub fn new(exploration_constant: f32, num_simulations: usize) -> Self {
        Self {
            nodes: Vec::new(),
            exploration_constant,
            num_simulations,
            terminal_nodes_found: 0,
        }
    }

    pub fn search(
        &mut self,
        root_state: GameState,
        value_fn: &dyn Fn(&GameState) -> f32,
        policy_fn: &dyn Fn(&GameState) -> Vec<f32>,
        temperature: f32,
        add_noise: bool,
    ) -> (u8, Vec<f32>) {
        self.terminal_nodes_found = 0;
        // Create root node
        let root_idx = self.add_node(root_state, None, 1.0, None);

        // Run simulations
        for i in 0..self.num_simulations {
            self.simulate(root_idx, value_fn, policy_fn);
            
            // Add Dirichlet Noise to root priors for the first simulation (AlphaZero style)
            if i == 0 && add_noise {
                self.add_root_noise(root_idx);
            }
        }

        // Get move probabilities
        let root_node = &self.nodes[root_idx];
        let mut move_probs = vec![0.0; COLS];
        let mut total_visits = 0;

        for &child_idx in &root_node.children {
            let child = &self.nodes[child_idx];
            let move_idx = child.r#move.unwrap_or(0);
            move_probs[move_idx as usize] = child.visits as f32;
            total_visits += child.visits;
        }

        if total_visits > 0 {
            if temperature > 0.1 {
                // Stochastic selection based on visit counts raised to 1/temp
                for prob in &mut move_probs {
                    *prob = prob.powf(1.0 / temperature);
                }
                let new_total: f32 = move_probs.iter().sum();
                for prob in &mut move_probs {
                    *prob /= new_total;
                }
            } else {
                // Deterministic selection: only the best move gets probability
                let max_visits = move_probs.iter().cloned().fold(0.0, f32::max);
                for prob in &mut move_probs {
                    if *prob == max_visits && max_visits > 0.0 {
                        *prob = 1.0;
                    } else {
                        *prob = 0.0;
                    }
                }
                // Handle ties (rare but possible)
                let sum: f32 = move_probs.iter().sum();
                if sum > 1.0 {
                    for prob in &mut move_probs {
                        *prob /= sum;
                    }
                }
            }
        }

        // Select move based on the transformed probabilities
        let mut rng = rand::thread_rng();
        let r: f32 = rand::Rng::gen(&mut rng);
        let mut cumulative = 0.0;
        let mut best_move = 0;

        for (i, &prob) in move_probs.iter().enumerate() {
            cumulative += prob;
            if r <= cumulative {
                best_move = i as u8;
                break;
            }
            if i == move_probs.len() - 1 {
                best_move = i as u8;
            }
        }

        (best_move, move_probs)
    }

    fn add_root_noise(&mut self, root_idx: usize) {
        let n = self.nodes[root_idx].children.len();
        if n == 0 { return; }

        let children_indices: Vec<usize> = self.nodes[root_idx].children.clone();
        
        // Simple Dirichlet-like noise (alpha=0.3 for Connect Four branching factor)
        const EPSILON: f32 = 0.25;
        
        let mut rng = rand::thread_rng();
        let mut noise = vec![0.0; n];
        let mut sum = 0.0;
        for val in noise.iter_mut().take(n) {
            let r: f32 = rand::Rng::gen(&mut rng); 
            *val = r;
            sum += r;
        }

        for i in 0..n {
            let child_idx = children_indices[i];
            let child = &mut self.nodes[child_idx];
            child.prior_probability = (1.0 - EPSILON) * child.prior_probability + EPSILON * (noise[i] / sum);
        }
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

        let value = {
            let node = &self.nodes[node_idx];
            if node.is_terminal {
                self.get_terminal_value(&node.state)
            } else if !node.is_fully_expanded() {
                // Expand node
                let new_child_idx = self.expand_node(node_idx, policy_fn);
                
                let child = &self.nodes[new_child_idx];
                
                // AlphaZero style with Terminal Certainty:
                // Use the value network prediction directly EXCEPT for terminal states.
                // This prevents MCTS from missing 1-move wins/losses due to neural network bias.
                let relative_value = if child.is_terminal {
                    self.terminal_nodes_found += 1;
                    self.get_terminal_value(&child.state)
                } else {
                    value_fn(&child.state)
                };
                
                // Update the new child node immediately
                self.nodes[new_child_idx].visits = 1;
                self.nodes[new_child_idx].total_value = relative_value;
                
                // Return NEGATED value to parent (Parent's perspective)
                -relative_value
            } else {
                // Select child using UCB
                let parent_visits = self.nodes[node_idx].visits;
                let children = self.nodes[node_idx].children.clone();

                // total_value is stored relative to each node's own player, so standard UCB applies.
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

                // Negate: child value is relative to the child's player, we want the current player's.
                -self.simulate_with_depth(best_child_idx, value_fn, policy_fn, depth + 1)
            }
        };
        
        // Update current node
        self.nodes[node_idx].visits += 1;
        self.nodes[node_idx].total_value += value;
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
                .filter_map(|&child_idx| self.nodes[child_idx].r#move)
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
            let child_idx = self.add_node(new_state, Some(node_idx), prior_prob, Some(unexpanded_move));
            self.nodes[node_idx].children.push(child_idx);
            child_idx
        } else {
            node_idx // Fallback to current node
        }
    }

    fn get_terminal_value(&self, state: &GameState) -> f32 {
        if let Some(winner) = state.get_winner() {
            if winner == state.current_player {
                1.0
            } else {
                -1.0
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
        r#move: Option<u8>,
    ) -> usize {
        let node = MCTSNode::new(state, parent, prior_probability, r#move);
        self.nodes.push(node);
        self.nodes.len() - 1
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts_node_creation() {
        let state = GameState::new();
        let node = MCTSNode::new(state, None, 1.0, None);

        assert_eq!(node.visits, 0);
        assert_eq!(node.total_value, 0.0);
        assert_eq!(node.prior_probability, 1.0);
        assert!(!node.is_terminal);
        assert_eq!(node.valid_moves.len(), 7);
    }

    #[test]
    fn test_mcts_ucb_score() {
        let state = GameState::new();
        let mut node = MCTSNode::new(state, None, 1.0, None);

        // Test unvisited node
        assert_eq!(node.ucb_score(1.0, 10), f32::INFINITY);

        // Test visited node
        node.visits = 5;
        node.total_value = -3.0; // Negative total_value means good for parent
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

        let (best_move, move_probs) = mcts.search(state, &value_fn, &policy_fn, 0.0, false);

        assert!(usize::from(best_move) < COLS);
        assert_eq!(move_probs.len(), COLS as usize);
        assert!((move_probs.iter().sum::<f32>() - 1.0).abs() < 0.001);
    }
}
