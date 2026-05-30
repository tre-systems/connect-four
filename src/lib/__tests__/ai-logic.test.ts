import { describe, it, expect } from 'vitest';
import { immediateTacticalMove } from '../logic/ai-logic';
import { createEmptyBoard } from '../logic/board-logic';
import type { GameState } from '../types';

const base: Omit<GameState, 'board'> = {
  currentPlayer: 'player1',
  gameStatus: 'playing',
  winner: null,
  history: [],
  winningLine: null,
};

describe('immediateTacticalMove (ML tactical safety net)', () => {
  it('takes an immediate winning move', () => {
    const board = createEmptyBoard();
    board[0][5] = 'player1';
    board[1][5] = 'player1';
    board[2][5] = 'player1';
    expect(immediateTacticalMove({ ...base, board })).toBe(3);
  });

  it('blocks an immediate opponent win', () => {
    const board = createEmptyBoard();
    board[0][5] = 'player2';
    board[1][5] = 'player2';
    board[2][5] = 'player2';
    expect(immediateTacticalMove({ ...base, board })).toBe(3);
  });

  it('prefers winning over blocking when both are available', () => {
    const board = createEmptyBoard();
    // player1 can win at col 3 (row 5); player2 threatens a vertical win at col 6.
    board[0][5] = 'player1';
    board[1][5] = 'player1';
    board[2][5] = 'player1';
    board[6][5] = 'player2';
    board[6][4] = 'player2';
    board[6][3] = 'player2';
    expect(immediateTacticalMove({ ...base, board })).toBe(3);
  });

  it('returns null when there is no forced move', () => {
    const board = createEmptyBoard();
    board[3][5] = 'player1';
    expect(immediateTacticalMove({ ...base, board })).toBeNull();
  });
});
