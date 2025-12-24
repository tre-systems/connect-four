import { GameState, Player } from '../schemas';
import { getWASMAIService } from '../wasm-ai-service';
import { getValidMoves, printBoard } from './board-logic';

export function otherPlayer(player: Player): Player {
  return player === 'player1' ? 'player2' : 'player1';
}

export async function makeAIMove(
  gameState: GameState,
  aiType: 'classic' | 'ml' = 'classic',
): Promise<number> {
  const wasmAI = getWASMAIService();

  if (!wasmAI.isReady) {
    throw new Error('WASM AI not loaded. Please refresh the page and try again.');
  }

  try {
    console.log(`\n🤖 ${aiType.toUpperCase()} AI thinking...`);
    printBoard(gameState.board, 'Current board before AI move');

    // Clear transposition table to ensure fresh calculations
    wasmAI.clearTranspositionTable();

    let response;
    switch (aiType) {
      case 'classic':
        response = await wasmAI.getBestMove(gameState, 5);
        break;
      case 'ml':
        const mlResponse = await wasmAI.getMLMove(gameState);
        if (mlResponse.thinking) {
          console.log(`🧠 ML AI Thinking: ${mlResponse.thinking}`);
        }
        response = {
          move: mlResponse.move,
          evaluations: [] as any[],
          nodesEvaluated: 0,
          transpositionHits: 0,
        };
        break;
      default:
        response = await wasmAI.getBestMove(gameState, 1);
    }

    // Check if we got a valid move (0-6 for columns)
    if (
      response.move !== null &&
      response.move !== undefined &&
      response.move >= 0 &&
      response.move < 7
    ) {
      const moveTime =
        response.nodesEvaluated > 0
          ? `(${response.nodesEvaluated} nodes, ${response.transpositionHits || 0} cache hits)`
          : '';
      const chosenCol = response.move;
      let bestType = '';
      let bestReason = '';
      let scoreTable = '\nAI Evaluation Table:';
      if (response.evaluations && response.evaluations.length > 0) {
        // Find the best score and type
        let maxScore = -Infinity;
        response.evaluations.forEach((e: { score: number; column: number; moveType: string }) => {
          if (typeof e.score === 'number' && e.score > maxScore) {
            maxScore = e.score;
          }
        });
        bestType = response.evaluations.find((e: any) => e.column === chosenCol)?.moveType || '';
        bestReason = bestType ? ` (${bestType})` : '';
        // Print table
        scoreTable += '\n-------------------------------------------';
        scoreTable += '\n Col |   Score   |   Type';
        scoreTable += '\n-------------------------------------------';
        response.evaluations.forEach((e: any) => {
          const highlight = e.column === chosenCol ? ' <==' : '';
          const moveType = e.moveType || 'normal';
          scoreTable += `\n  ${e.column}  | ${String(e.score).padStart(8)} | ${moveType.padEnd(8)}${highlight}`;
        });
        scoreTable += '\n-------------------------------------------';
      }
      console.log(`🤖 AI chose column ${chosenCol} ${moveTime}${bestReason}`);
      if (scoreTable) console.log(scoreTable);
      return chosenCol;
    }

    console.error('WASM AI returned invalid move:', response.move);

    // Fallback to classic AI if ML AI fails
    console.log('🤖 Falling back to classic AI...');
    try {
      const fallbackResponse = await wasmAI.getBestMove(gameState, 3);
      if (
        fallbackResponse.move !== null &&
        fallbackResponse.move !== undefined &&
        fallbackResponse.move >= 0 &&
        fallbackResponse.move < 7
      ) {
        console.log(`🤖 Classic AI fallback chose column ${fallbackResponse.move}`);
        return fallbackResponse.move;
      }
    } catch (fallbackError) {
      console.error('Classic AI fallback also failed:', fallbackError);
    }

    // Last resort: pick first valid move
    const validMoves = getValidMoves(gameState.board);
    if (validMoves.length > 0) {
      const randomMove = validMoves[Math.floor(Math.random() * validMoves.length)];
      console.log(`🤖 Random fallback chose column ${randomMove}`);
      return randomMove;
    }
  } catch (error) {
    console.error('WASM AI failed:', error);

    // Try classic AI as fallback
    try {
      console.log('🤖 Trying classic AI as fallback...');
      const fallbackResponse = await wasmAI.getBestMove(gameState, 3);
      if (
        fallbackResponse.move !== null &&
        fallbackResponse.move !== undefined &&
        fallbackResponse.move >= 0 &&
        fallbackResponse.move < 7
      ) {
        console.log(`🤖 Classic AI fallback chose column ${fallbackResponse.move}`);
        return fallbackResponse.move;
      }
    } catch (fallbackError) {
      console.error('Classic AI fallback failed:', fallbackError);
    }

    // Last resort: pick first valid move
    const validMoves = getValidMoves(gameState.board);
    if (validMoves.length > 0) {
      const randomMove = validMoves[Math.floor(Math.random() * validMoves.length)];
      console.log(`🤖 Random fallback chose column ${randomMove}`);
      return randomMove;
    }

    throw new Error(`AI calculation failed: ${error}`);
  }

  throw new Error('No valid move found');
}
