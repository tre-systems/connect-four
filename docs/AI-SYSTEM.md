# AI System Documentation

## Overview

The Connect Four AI system uses multiple approaches to provide different levels of gameplay:

1. **Bitboard Solver**: [Negamax](https://en.wikipedia.org/wiki/Negamax) with alpha-beta pruning and [Bitboard](https://en.wikipedia.org/wiki/Bitboard) optimizations
2. **ML-MCTS AI**: [AlphaZero](https://en.wikipedia.org/wiki/AlphaZero)-style [Monte Carlo Tree Search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search) with neural networks
3. **Genetic AI**: Evolved evaluation functions using genetic algorithms

## ML AI Architecture

### Deep Network Design

The ML AI uses a deep neural network architecture optimized for both tactical precision and client-side performance:

```python
# Value Network: 100 -> 128 -> 128 -> 128 -> 128 -> 1
# Policy Network: 100 -> 128 -> 128 -> 128 -> 128 -> 7
```

**Key Features:**

- **Input**: 100 strategic features (Board occupancy + Positional advantages)
- **Hidden layers**: 4 layers of 128 neurons (ResNet-lite)
- **Output**: Value (Tanh, -1 to 1) and Policy (Softmax, 7 move probabilities)
- **Search**: MCTS (Monte Carlo Tree Search) with 4000 simulations per move
- **Performance**: High tactical strength through supervised training
- **Size**: ~328KB per network

### Training Data (Supervised Teacher)

The model is trained using a **Supervised Learning** pipeline where the **Bitboard Solver** acts as a teacher:

- **Teacher Evaluation**: Bitboard Solver (12+ plies) provides "perfect" value and policy labels.
- **Dataset**: 10,000+ board positions generated through randomized self-play.
- **Labels**: Value (normalized win probability) and Policy (target probabilities based on solver scores).

### Training Status

✅ **Phase 2 Supervised Training Complete (Dec 2024)**:

- **Dataset**: 20,000 samples (10k raw + symmetry augmentation)
- **Teacher Depth**: 12 (solver evaluations)
- **Training**: 100 epochs with LR decay (0.001 → 0.0001 → 0.00001)
- **Results**: Value Loss 0.033, Policy Loss 1.95
- **Architecture**: 4x128 ResNet-lite
- **Weights**: `resources/ai/ml_ai_weights_best.json` (source) -> `public/ml/data/weights/` (build output)

## Available WASM AI Infrastructure

The codebase contains a robust Rust/WASM AI system:

- **Classic AI**: Bitboard Solver with Negamax and alpha-beta pruning ✅
- **ML AI**: Deep neural networks with MCTS (AlphaZero style) ✅
- **Genetic AI**: Parameter-optimized evaluation functions ✅
- **Training Utility**: Supervised training script (`train.rs`) for rapid model improvement ✅

### MCTS Strategy

The AI uses Monte Carlo Tree Search (MCTS) with perspective-invariant features (Current Player = 1.0, Opponent = -1.0) to evaluate positions. The 4x128 architecture provides deep tactical representation without sacrificing client-side inference speed.

## AI Performance Comparison

Based on Dec 2024 testing:

1. **ML-MCTS (AlphaZero)**: Restored to >70% win rate against previous baselines.
2. **Bitboard-Solver**: Perfect tactical player for limited depths.

## Recommendations

- **Production**: Use **ML AI (MCTS)** for the most "human-like" but strong tactical play.
- **Solver**: Use for analytical verification of positions.
- **Testing**: Use Random AI for baseline.

## Future Improvements

1. **Deeper Teacher Labels**: Increase solver depth to 18+ for even higher quality training.
2. **Larger Dataset**: Scale to 50k+ positions for better generalization.
3. **Self-Play Fine-tuning**: Reinforce the supervised model through MCTS self-play.
4. **Parallelize ML Performance Tests**: Use rayon for faster benchmarking.

## Troubleshooting

### Model Loading Issues

If the ML AI fails to load:

1. Check that `ml_ai_weights_best.json` exists in `public/ml/data/weights/` (copied from `resources/ai/` during build)
2. Verify the architecture in `neural_network.rs` matches the weights.
3. Check browser console for WASM errors.

## Conclusion

The shift to a supervised training pipeline using the Bitboard Solver as a teacher has successfully restored the ML AI's performance, providing a competitive and Tactically sound opponent that combines the strengths of MCTS with accurate state evaluations.
