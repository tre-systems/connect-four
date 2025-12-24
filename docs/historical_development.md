# Historical Development of Connect Four AI

This document serves as a historical record of the AI experiments and training methodologies developed for the Connect Four project. These components have been deprecated in favor of a streamlined production architecture but document the research process.

## 1. Genetic Parameter Evolution

We implemented a genetic algorithm to evolve the weights for the heuristic evaluation function.

### Methodology

- **Population**: 50-100 competing AI instances, each with a unique set of heuristic weights (e.g., specific weights for center control, mobility, threats).
- **Selection**: Tournament selection based on game outcomes against other population members.
- **Crossover & Mutation**: Winners bred new generations with random mutations applied to weights.
- **Outcome**: Produced `evolved.json`, a highly optimized set of static weights for the WASM heuristic evaluator.

## 2. Self-Play Reinforcement Learning (AlphaZero-style)

We developed a complete reinforcement learning pipeline inspired by AlphaZero to train neural networks from zero knowledge.

### Architecture

- **Dual-Head Network**: A shared body with two heads:
  - **Value Head**: Predicted win/loss probability (`tanh` output).
  - **Policy Head**: Predicted optimal move distribution (`softmax` output).
- **Features**:
  - **Attention Layers**: Multi-head attention to capture board patterns.
  - **Residual Connections**: Deep networks with skip connections to improve gradient flow.

### Training Pipeline (`train:self-play`)

1.  **Data Generation**:
    - Self-play games using MCTS (Monte Carlo Tree Search).
    - 800+ simulations per move.
    - Dirichlet noise added to root node for exploration.
2.  **Training**:
    - PyTorch backbone.
    - Simultaneous reduction of Value Loss (MSE) and Policy Loss (Cross-Entropy).
    - Curriculum learning with progressive difficulty.
3.  **Intensive Training**:
    - Configuration for 6+ hour training sessions using `caffeinate`.
    - Generated datasets of 5000+ games.

### Legacy Scripts (Removed)

- `ml/scripts/train_self_play.py`: Main PyTorch training loop.
- `worker/rust_ai_core/src/bin/train.rs`: Rust binary for orchestrating self-play.
- `worker/rust_ai_core/src/self_play.rs`: MCTS implementation for self-play data generation.

## 3. Heuristic Scoring Evolution

Explored multiple scoring strategies:

- **Center Control**: Prioritizing the center column (essential for Connect 4).
- **Mobility**: Incentivizing moves that maximize future options.
- **Threat Detection**: Heavy penalties for allowing opponent 3-in-a-row setups.

## 4. Artifacts Preserved

While the training infrastructure has been removed to reduce repository size and complexity, the following artifacts were preserved in `resources/ai/`:

- `evolved.json`: Best result from genetic evolution.
- `simple_model.json`: A baseline neural network.
- `ml_ai_weights_best.json`: The best performing weights from the self-play experiments (formerly "distilled").
