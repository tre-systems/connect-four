# Game Guide

This document provides a comprehensive guide to Connect Four, including historical context, rules, strategy, and AI opponents.

## Historical Context

Connect Four is a classic strategy game invented by Howard Wexler and Ned Strongin in 1974. It was first sold by Milton Bradley (now Hasbro) and quickly became a popular two-player game. The game is known for its simple rules but deep strategic gameplay, making it accessible to players of all ages while still offering significant tactical depth.

The game is a perfect example of a "solved" game - in 1988, computer scientists James D. Allen and Victor Allis independently proved that with perfect play, the first player can always force a win. However, achieving perfect play is extremely difficult for humans, making the game still highly engaging and competitive.

## Game Overview

Connect Four is a strategic two-player game where players take turns dropping colored discs into a vertically suspended grid. The goal is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs.

### Board Layout

- **7 columns** × **6 rows** grid
- **42 total positions** for discs
- **Gravity effect**: discs fall to the lowest available position in each column
- **No diagonal movement**: discs can only be placed in columns

## Game Rules

### Setup

- Empty 7×6 grid
- Two players with different colored discs (typically red and yellow)
- Players alternate turns

### Gameplay

1. **Choose a column**: Player selects any of the 7 columns
2. **Drop a disc**: Disc falls to the lowest empty position in that column
3. **Check for win**: Look for 4-in-a-row (horizontal, vertical, or diagonal)
4. **Continue**: If no win, next player's turn

### Winning Conditions

- **Horizontal**: 4 discs in a row horizontally
- **Vertical**: 4 discs in a row vertically
- **Diagonal**: 4 discs in a row diagonally (both directions)
- **First to connect**: Player who creates the first 4-in-a-row wins

### Win Animation

When a player wins, the game features an **elegant animation sequence**:

1. **Winning Pieces Highlight**: The 4 connected pieces glow with a strong, pulsing colored effect
2. **Scale Animation**: Winning pieces pulse with a subtle scale animation for excitement
3. **Victory Celebration**: Simple text popup appears briefly
4. **Sound Effects**: Triumphant audio sequence plays with ascending notes
5. **Winner Modal**: The final winner modal appears with additional celebration

The winning pieces now have a clean, professional glow effect that clearly highlights the winning line without any distracting overlays or misaligned rings.

### Draw Condition

- **Full board**: If all 42 positions are filled without a winner
- **No more moves**: Game ends in a draw

## Strategy Guide

### Opening Strategy

**Control the center**:

- The center column (column 4) is most valuable
- It offers the most opportunities for creating multiple threats

**Create multiple threats**:

- Set up positions where you can win in multiple ways
- Force your opponent to block one threat while you create another

**Block opponent threats**:

- Always check if your opponent can win on their next move
- Prioritize blocking over creating your own threats when necessary

### Mid-Game Strategy

**Build from the bottom**:

- Start building your connections from the bottom rows
- This creates more stable positions and reduces opponent's blocking options

**Use the edges**:

- The edges can be used to create diagonal wins
- Don't underestimate edge positions

**Create forcing moves**:

- Set up positions where your opponent must block
- Use these to gain tempo and control

### End-Game Strategy

**Look ahead**:

- Calculate several moves ahead
- Consider all possible opponent responses

**Sacrifice for position**:

- Sometimes giving up a potential win can lead to a better position
- Think about the overall board state, not just immediate wins

**Use zugzwang**:

- Create positions where any move your opponent makes helps you
- This is a key concept in Connect Four strategy

## Advanced Concepts

### Threat Sequences

**Double threats**: Creating two simultaneous winning threats
**Triple threats**: Creating three simultaneous winning threats (very rare)
**Forced wins**: Sequences that lead to inevitable victory

### Board Evaluation

**Control of key squares**: Certain positions are more valuable than others
**Mobility**: Having more available moves than your opponent
**Tempo**: Making moves that force your opponent to respond defensively

## AI Opponents

The game features sophisticated AI opponents with evolved genetic parameters:

### Classic AI (Bitboard Solver)

- Uses a highly optimized bitboard-based Negamax algorithm
- **100% win rate** for Bitboard-Solver (Depth 6) in tactical testing
- **10.3ms/move** average response time (Release mode)
- Available in various levels from "Easy" (Depth 1) to "Expert" (Depth 6+)
- Provides a rock-solid, mathematically sound challenge

### ML AI (Deep Neural Network + MCTS)

- Uses a 4x128 ResNet-lite neural network trained via supervision from the bitboard teacher
- Integrates **Monte Carlo Tree Search (MCTS)** with 4000 simulations per move
- **Restored performance**: Plays strategically sound Connect Four without suicidal errors
- **~2-4s/move** "Thinking time" for deep tactical analysis (varies by hardware)
- Provides a more creative and sophisticated opponent compared to pure minimax

### AI Performance Rankings

1. **Bitboard-Solver (Expert)**: 100% average win rate (Master level)
2. **ML-MCTS (AlphaZero)**: High tactical strength (Challenger level)
3. **Bitboard-Solver (Intermediate)**: Balanced speed and strength
4. **Bitboard-Solver (Beginner)**: Foundational strategic play
5. **Random**: Baseline for testing only

## AI vs AI Mode

Watch two AI opponents battle it out in an automated match! This mode allows you to observe how different AI strategies play against each other.

### How to Access

- Click the **"Watch AI vs AI"** button on the AI selection screen
- The game will automatically start with both players controlled by AI
- Sit back and watch the strategic battle unfold

### Features

- **Fully automated gameplay**: Both players are AI-controlled
- **Real-time visualization**: Watch each move being calculated and executed
- **Strategic insights**: Observe how different AI approaches handle the same positions
- **No human interaction required**: Perfect for learning and entertainment
- **Educational value**: See how AI evaluates positions and makes decisions

### Use Cases

- **Learning strategy**: Watch how strong AI plays to improve your own game
- **AI comparison**: See how different AI types perform against each other
- **Entertainment**: Enjoy watching high-level strategic play
- **Analysis**: Study opening moves and mid-game tactics
- **Testing**: Observe AI behavior and performance in real games

## Historical Significance

Connect Four represents an important milestone in game theory and artificial intelligence:

- **Perfect information game**: All information is visible to both players
- **Deterministic**: No element of chance
- **Finite game**: Always ends in a finite number of moves
- **Solved game**: Optimal strategy is known (first player can force win)

## Learning Resources

For more information about Connect Four:

- [Wikipedia: Connect Four](https://en.wikipedia.org/wiki/Connect_Four) - Comprehensive overview of the game's history and strategy
- [Connect Four Strategy Guide](https://www.mathsisfun.com/games/connect4.html) - Interactive strategy guide
- [Solving Connect Four](http://blog.gamesolver.org/solving-connect-four/01-introduction/) - Technical analysis of game solving

## Conclusion

Connect Four is a perfect example of how simple rules can create complex and engaging gameplay. While the game has been solved by computers, it remains a challenging and enjoyable game for human players. The combination of accessibility and strategic depth makes it an excellent choice for players of all skill levels.

The dual AI system in this implementation provides both reliable play (Classic AI) and creative, learning-based play (ML AI), offering different challenges and learning opportunities for players.
