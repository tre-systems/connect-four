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
# Value Network: 100 -> 256 -> 128 -> 64 -> 1
# Policy Network: 100 -> 256 -> 128 -> 64 -> 7
```

**Key Features:**

- **Input**: 100 strategic features (Board occupancy + Positional advantages)
- **Hidden layers**: 256 → 128 → 64 neurons (ReLU activation)
- **Output**: Value (Tanh, -1 to 1) and Policy (Softmax, 7 move probabilities)
- **Search**: MCTS (Monte Carlo Tree Search) with 800 simulations per move
- **Performance**: High tactical strength through supervised training
- **Size**: ~328KB per network

### Training Data (Supervised Teacher)

The model is trained using a **Supervised Learning** pipeline where the **Bitboard Solver** acts as a teacher:

- **Teacher Evaluation**: Bitboard Solver (12+ plies) provides "perfect" value and policy labels.
- **Dataset**: 10,000+ board positions generated through randomized self-play.
- **Labels**: Value (normalized win probability) and Policy (target probabilities based on solver scores).

### Training Status

✅ **Supervised Training Complete (Dec 2024)**:

- **Model**: 50 epochs, 10,000 teacher samples
- **Architecture**: [256, 128, 64]
- **Verification**: Verified via `test_ai_matrix` against Solver teacher.

## Available WASM AI Infrastructure

The codebase contains a robust Rust/WASM AI system:

- **Classic AI**: Bitboard Solver with Negamax and alpha-beta pruning ✅
- **ML AI**: Deep neural networks with MCTS (AlphaZero style) ✅
- **Genetic AI**: Parameter-optimized evaluation functions ✅
- **Training Utility**: Supervised training script (`train.rs`) for rapid model improvement ✅

### Recent Fix: MCTS & Perspective Correction

**Issue Fixed (Dec 2024)**: The ML AI exhibited "suicidal" play due to a sign error in the MCTS backpropagation and a perspectival mismatch (NN output was always from Player 1's perspective).

**Solution**:

- Corrected MCTS update logic in `mcts.rs` to properly accumulate values.
- Updated `ml_ai.rs` to negate NN evaluations when it's Player 2's turn.
- Switched to a deeper [256, 128, 64] architecture for better tactical representation.

## AI Performance Comparison

Based on Dec 2024 testing:

1. **ML-MCTS (AlphaZero)**: Restored to >70% win rate against previous baselines.
2. **Bitboard-Solver**: Perfect tactical player for limited depths.

## Recommendations

- **Production**: Use **ML AI (MCTS)** for the most "human-like" but strong tactical play.
- **Solver**: Use for analytical verification of positions.
- **Testing**: Use Random AI for baseline.

## Future Improvements

1. **Curriculum Learning**: Progressively increase solver depth during training.
2. **Larger Dataset**: Scale to 50k+ positions for even higher accuracy.
3. **Self-Play Overlap**: Fine-tune the supervised model through reinforcement learning.

## Troubleshooting

### Model Loading Issues

If the ML AI fails to load:

1. Check that `ml_ai_weights_best.json` exists in `public/ml/data/weights/`
2. Verify the architecture in `neural_network.rs` matches the weights.
3. Check browser console for WASM errors.

## Conclusion

The shift to a supervised training pipeline using the Bitboard Solver as a teacher has successfully restored the ML AI's performance, providing a competitive and Tactically sound opponent that combines the strengths of MCTS with accurate state evaluations.
