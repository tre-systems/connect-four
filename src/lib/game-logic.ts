import { GameState, Player, Board, MoveRecord } from './schemas';
import { getWASMAIService } from './wasm-ai-service';

const ROWS = 6;
const COLS = 7;

export interface WinningLine {
  positions: Array<{ column: number; row: number }>;
  direction: 'horizontal' | 'vertical' | 'diagonal';
}

function createEmptyBoard(): Board {
  return Array.from({ length: COLS }, () => Array.from({ length: ROWS }, () => null));
}

function printBoard(board: Board, moveInfo?: string) {
  const header = moveInfo ? `\n${moveInfo}` : '\nBoard:';
  console.log(header);

  // Print board from top to bottom (row 0 to 5)
  for (let row = 0; row < ROWS; row++) {
    let rowStr = '';
    for (let col = 0; col < COLS; col++) {
      const cell = board[col][row];
      if (cell === 'player1') {
        rowStr += '🔴';
      } else if (cell === 'player2') {
        rowStr += '🟡';
      } else {
        rowStr += '⚫'; // Black circle for empty cells
      }
    }
    console.log(rowStr);
  }
}

export function initializeGame(): GameState {
  const startingPlayer: Player = Math.random() < 0.5 ? 'player1' : 'player2';
  const gameState = {
    board: createEmptyBoard(),
    currentPlayer: startingPlayer,
    gameStatus: 'playing' as const,
    winner: null,
    history: [],
    winningLine: null,
  };

  console.log(`🎮 Game started - ${startingPlayer === 'player1' ? 'Red' : 'Yellow'} goes first`);
  printBoard(gameState.board);

  return gameState;
}

export function makeMove(gameState: GameState, column: number): GameState {
  if (gameState.gameStatus !== 'playing') return gameState;
  if (column < 0 || column >= COLS) return gameState;

  // Find the lowest empty row in the column (bottom is index 5, top is index 0)
  const col = gameState.board[column];
  const row = col.lastIndexOf(null);
  if (row === -1) return gameState; // Column full

  // Place the piece in the lowest empty row
  const newBoard: Board = gameState.board.map((c, i) =>
    i === column ? [...c.slice(0, row), gameState.currentPlayer, ...c.slice(row + 1)] : [...c]
  );

  const newHistory: MoveRecord[] = [
    ...gameState.history,
    { player: gameState.currentPlayer, column, row },
  ];

  // Check for win
  const winResult = checkWin(newBoard, column, row, gameState.currentPlayer);
  const winner = winResult ? gameState.currentPlayer : null;
  const isDrawn = !winner && isDraw(newBoard);

  const newGameState = {
    board: newBoard,
    currentPlayer:
      winner || isDrawn ? gameState.currentPlayer : otherPlayer(gameState.currentPlayer),
    gameStatus: winner ? 'finished' : isDrawn ? 'finished' : 'playing',
    winner: winner,
    history: newHistory,
    winningLine: winResult,
  } as GameState;

  // Log the move
  const playerName = gameState.currentPlayer === 'player1' ? 'Red' : 'Yellow';
  const moveInfo = `${playerName} placed in column ${column} (row ${row})`;

  if (winner) {
    console.log(`🏆 ${moveInfo} - ${playerName} WINS!`);
    printBoard(newBoard, moveInfo);
  } else if (isDrawn) {
    console.log(`🤝 ${moveInfo} - Game is a DRAW!`);
    printBoard(newBoard, moveInfo);
  } else {
    const nextPlayer = newGameState.currentPlayer === 'player1' ? 'Red' : 'Yellow';
    console.log(`${moveInfo} - ${nextPlayer}'s turn`);
    printBoard(newBoard, moveInfo);
  }

  return newGameState;
}

export function getValidMoves(board: Board): number[] {
  return board
    .map((col, index) => (col.some(cell => cell === null) ? index : -1))
    .filter(index => index !== -1);
}

export async function makeAIMove(
  gameState: GameState,
  aiType: 'classic' | 'ml' = 'classic'
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
        response = {
          move: mlResponse.move,
          evaluations: [] as any[], // Casting to any[] or properly MoveEvaluationWasm[] if imported
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
        bestType = response.evaluations.find(e => e.column === chosenCol)?.moveType || '';
        bestReason = bestType ? ` (${bestType})` : '';
        // Print table
        scoreTable += '\n-------------------------------------------';
        scoreTable += '\n Col |   Score   |   Type';
        scoreTable += '\n-------------------------------------------';
        response.evaluations.forEach(e => {
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

function otherPlayer(player: Player): Player {
  return player === 'player1' ? 'player2' : 'player1';
}

export function isDraw(board: Board): boolean {
  return board.every(col => col.every(cell => cell !== null));
}

export function checkWin(
  board: Board,
  col: number,
  row: number,
  player: Player
): WinningLine | null {
  // Check horizontal
  const horizontalLine = checkDirection(board, col, row, 1, 0, player);
  if (horizontalLine) return { positions: horizontalLine, direction: 'horizontal' };

  // Check vertical
  const verticalLine = checkDirection(board, col, row, 0, 1, player);
  if (verticalLine) return { positions: verticalLine, direction: 'vertical' };

  // Check diagonal /
  const diagonal1Line = checkDirection(board, col, row, 1, 1, player);
  if (diagonal1Line) return { positions: diagonal1Line, direction: 'diagonal' };

  // Check diagonal \
  const diagonal2Line = checkDirection(board, col, row, 1, -1, player);
  if (diagonal2Line) return { positions: diagonal2Line, direction: 'diagonal' };

  return null;
}

function checkDirection(
  board: Board,
  col: number,
  row: number,
  dCol: number,
  dRow: number,
  player: Player
): Array<{ column: number; row: number }> | null {
  const positions: Array<{ column: number; row: number }> = [];

  // Count in positive direction
  let count = 1;
  positions.push({ column: col, row });
  let c = col + dCol;
  let r = row + dRow;
  while (c >= 0 && c < COLS && r >= 0 && r < ROWS && board[c][r] === player) {
    count++;
    positions.push({ column: c, row: r });
    c += dCol;
    r += dRow;
  }

  // Count in negative direction
  c = col - dCol;
  r = row - dRow;
  while (c >= 0 && c < COLS && r >= 0 && r < ROWS && board[c][r] === player) {
    count++;
    positions.unshift({ column: c, row: r });
    c -= dCol;
    r -= dRow;
  }

  return count >= 4 ? positions : null;
}
