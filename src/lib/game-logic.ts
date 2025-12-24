import { GameState, Player, Board, MoveRecord } from './schemas';
import {
  createEmptyBoard,
  printBoard,
  isDraw,
  checkWin,
  getValidMoves,
  WinningLine,
  checkDirection,
} from './logic/board-logic';
import { makeAIMove, otherPlayer } from './logic/ai-logic';

export {
  WinningLine,
  createEmptyBoard,
  printBoard,
  getValidMoves,
  isDraw,
  checkWin,
  checkDirection,
  makeAIMove,
  otherPlayer,
};

const COLS = 7;

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
