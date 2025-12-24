# AI Matrix Test Results

_Last updated: 29/07/2025, 07:27:13_

## Matrix Table

**Test Configuration:**
Total games played: 144
Duration: 2.51 seconds
Games per second: 57.4

| AI Type                   | Random | Heuristic | MM-Depth1 | MM-Depth2 | MM-Depth3 | MM-Depth4 | MM-Depth5 | Bitboard-Solver (Depth 6) | ML-MCTS (AlphaZero) |
| ------------------------- | ------ | --------- | --------- | --------- | --------- | --------- | --------- | ------------------------- | ------------------- |
| Random                    | -      | 0.0       | 25.0      | 50.0      | 25.0      | 25.0      | 25.0      | 25.0                      | 25.0                |
| Heuristic                 | 100.0  | -         | 75.0      | 0.0       | 75.0      | 100.0     | 25.0      | 50.0                      | 50.0                |
| MM-Depth1                 | 75.0   | 25.0      | -         | 50.0      | 50.0      | 100.0     | 50.0      | 50.0                      | 0.0                 |
| MM-Depth2                 | 50.0   | 100.0     | 50.0      | -         | 50.0      | 50.0      | 25.0      | 50.0                      | 50.0                |
| MM-Depth3                 | 75.0   | 25.0      | 50.0      | 50.0      | -         | 0.0       | 50.0      | 25.0                      | 25.0                |
| MM-Depth4                 | 75.0   | 0.0       | 0.0       | 50.0      | 100.0     | -         | 50.0      | 0.0                       | 0.0                 |
| MM-Depth5                 | 75.0   | 75.0      | 50.0      | 75.0      | 50.0      | 50.0      | -         | 100.0                     | 25.0                |
| Bitboard-Solver (Depth 6) | 75.0   | 50.0      | 50.0      | 50.0      | 75.0      | 100.0     | 0.0       | -                         | 50.0                |
| ML-MCTS (AlphaZero)       | 75.0   | 50.0      | 100.0     | 50.0      | 75.0      | 100.0     | 75.0      | 50.0                      | -                   |

## Performance Summary

1. ML-MCTS (AlphaZero): 71.9% average win rate
2. MM-Depth5: 62.5% average win rate
3. Heuristic: 59.4% average win rate
4. Bitboard-Solver (Depth 6): 56.2% average win rate
5. MM-Depth2: 53.1% average win rate
6. MM-Depth1: 50.0% average win rate
7. MM-Depth3: 37.5% average win rate
8. MM-Depth4: 34.4% average win rate
9. Random: 25.0% average win rate

## Speed Analysis

| AI                        | ms/move | Speed     |
| ------------------------- | ------- | --------- |
| Random                    | 0.0     | Very Fast |
| MM-Depth1                 | 0.0     | Very Fast |
| Heuristic                 | 0.0     | Very Fast |
| ML-MCTS (AlphaZero)       | 0.4     | Very Fast |
| MM-Depth2                 | 0.6     | Very Fast |
| MM-Depth3                 | 4.0     | Fast      |
| MM-Depth4                 | 26.8    | Moderate  |
| MM-Depth5                 | 82.1    | Slow      |
| Bitboard-Solver (Depth 6) | 226.2   | Slow      |

## Recommendations

- ML-MCTS (AlphaZero) shows excellent performance (71.9% avg win rate) and is ready for production
- Random is very fast (0.0ms/move) and suitable for real-time play
- Use MM-Depth3 for best performance/speed balance
- Use Random AI for baseline testing
- Use Heuristic AI for educational purposes

## Recent Fixes

### Genetic Parameters Issue (July 2025)

**Problem**: The app was using incomplete fallback genetic parameters when the evolved.json file failed to load, causing inconsistent AI performance.

**Solution**: Updated the WASM AI service to use complete default genetic parameters that match the Rust `GeneticParams::default()` implementation.

**Impact**:

- All AI types now use consistent genetic parameters
- Matrix test results are now reliable and reproducible
- Both Classic AI and ML AI are properly configured

**Files Changed**:

- `src/lib/wasm-ai-service.ts`: Fixed genetic parameters fallback
