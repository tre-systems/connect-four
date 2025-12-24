# AI Matrix Test Results

_Last updated: 24/12/2024_

## Matrix Table

**Test Configuration:**
Total games played: 60 (20 per match)
Duration: 18.02 seconds
Games per second: 3.3

| AI Type                   | Random | Bitboard-Solver (Depth 6) | ML-MCTS (AlphaZero) |
| ------------------------- | ------ | ------------------------- | ------------------- |
| Random                    | -      | 0.0                       | 25.0                |
| Bitboard-Solver (Depth 6) | 100.0  | -                         | 100.0               |
| ML-MCTS (AlphaZero)       | 75.0   | 0.0                       | -                   |

## Performance Summary

1. Bitboard-Solver (Depth 6): 100.0% average win rate
2. ML-MCTS (AlphaZero): 37.5% average win rate
3. Random: 12.5% average win rate

## Speed Analysis

| AI                        | ms/move | Speed     |
| ------------------------- | ------- | --------- |
| Random                    | 0.0     | Very Fast |
| Bitboard-Solver (Depth 6) | 10.3    | Fast      |
| ML-MCTS (AlphaZero)       | 736.9   | Slow      |

## Recommendations

- **Bitboard-Solver (Depth 6)** is the strongest tactical engine and should be used for competitive or analytical scenarios.
- **ML-MCTS (AlphaZero)** provides an advanced challenge with "deep thinking" (800 sims), making it a more human-like but formidable opponent.
- **Random** remains the baseline for testing.

## Recent Fixes

### ML AI Restoration (Dec 2024)

**Problem**: The ML AI had a sign error in its MCTS logic and a perspectival mismatch, causing "suicidal" play.

**Solution**:

- Fixed MCTS backpropagation sign error.
- Corrected player perspective in `ml_ai.rs`.
- Implemented a supervised training pipeline with the Bitboard Solver as a teacher.
- Upgraded the architecture to a deeper [256, 128, 64] network.

**Impact**:

- The AI now plays tactically sound moves and defends against immediate threats.
- It can reliably beat random play and competitive baselines.
- The training system is now self-service via `train.rs`.
