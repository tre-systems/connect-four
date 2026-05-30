import { describe, it, expect } from 'vitest';
import type { GameState } from '../types';

interface WASMGameState {
  board: string[][];
  current_player: string;
  genetic_params: {
    center_control_weight: number;
    piece_count_weight: number;
    threat_weight: number;
    mobility_weight: number;
    vertical_control_weight: number;
    horizontal_control_weight: number;
  };
}

describe('WASM AI Service - Player Value Conversion', () => {
  const convertGameStateToWASM = (gameState: GameState): WASMGameState => {
    const board = gameState.board.map(col => col.map(cell => cell ?? 'empty'));

    return {
      board,
      current_player: gameState.currentPlayer,
      genetic_params: {
        center_control_weight: 1.0,
        piece_count_weight: 0.5,
        threat_weight: 2.0,
        mobility_weight: 0.8,
        vertical_control_weight: 1.2,
        horizontal_control_weight: 1.0,
      },
    };
  };

  it('should convert player values correctly for WASM', () => {
    const gameState: GameState = {
      board: [
        [null, null, null, null, null, 'player1'],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
      ],
      currentPlayer: 'player2',
      gameStatus: 'playing',
      winner: null,
      history: [],
      winningLine: null,
    };

    const wasmState = convertGameStateToWASM(gameState);

    expect(wasmState.board[0][5]).toBe('player1');
    expect(wasmState.current_player).toBe('player2');
  });

  it('should handle empty cells correctly', () => {
    const gameState: GameState = {
      board: [
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
      ],
      currentPlayer: 'player1',
      gameStatus: 'playing',
      winner: null,
      history: [],
      winningLine: null,
    };

    const wasmState = convertGameStateToWASM(gameState);

    expect(wasmState.board[0][0]).toBe('empty');
    expect(wasmState.current_player).toBe('player1');
  });

  it('should convert both player types correctly', () => {
    const gameState: GameState = {
      board: [
        [null, null, null, null, null, 'player1'],
        [null, null, null, null, null, 'player2'],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
        [null, null, null, null, null, null],
      ],
      currentPlayer: 'player1',
      gameStatus: 'playing',
      winner: null,
      history: [],
      winningLine: null,
    };

    const wasmState = convertGameStateToWASM(gameState);

    expect(wasmState.board[0][5]).toBe('player1');
    expect(wasmState.board[1][5]).toBe('player2');
    expect(wasmState.current_player).toBe('player1');
  });
});
