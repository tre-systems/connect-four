import { describe, it, expect, beforeEach, vi } from 'vitest';
import { useGameStore } from '../game-store';
import { initializeGame } from '../game-logic';

// Mock WASM AI service
vi.mock('../wasm-ai-service', () => ({
  initializeWASMAI: vi.fn().mockResolvedValue(undefined),
}));

describe('Game Store', () => {
  beforeEach(() => {
    useGameStore.setState({
      gameState: { ...initializeGame() },
      aiThinking: false,
      pendingMove: null,
      showWinnerModal: false,
      actions: useGameStore.getState().actions,
    });
  });

  describe('initialize', () => {
    it('should reset game state to initial state', () => {
      const store = useGameStore.getState();

      store.actions.initialize();

      const newState = useGameStore.getState();
      expect(newState.gameState.gameStatus).toBe('playing');
      expect(newState.aiThinking).toBe(false);
      expect(newState.pendingMove).toBe(null);
      expect(newState.showWinnerModal).toBe(false);
    });
  });

  describe('makeMove', () => {
    it('should set pending move when game is playing', () => {
      const store = useGameStore.getState();
      const initialPlayer = store.gameState.currentPlayer;

      store.actions.makeMove(3);

      const newState = useGameStore.getState();
      expect(newState.pendingMove).toEqual({
        column: 3,
        player: initialPlayer,
      });
    });

    it('should not set pending move when game is finished', () => {
      useGameStore.setState(state => {
        state.gameState.gameStatus = 'finished';
        state.gameState.winner = 'player1';
      });

      const store = useGameStore.getState();
      store.actions.makeMove(3);

      const newState = useGameStore.getState();
      expect(newState.pendingMove).toBe(null);
    });
  });

  describe('completeMove', () => {
    it('should complete pending move and update game state', () => {
      const store = useGameStore.getState();
      const initialPlayer = store.gameState.currentPlayer;

      store.actions.makeMove(3);
      store.actions.completeMove();

      const newState = useGameStore.getState();
      expect(newState.pendingMove).toBe(null);
      expect(newState.gameState.board[3][5]).toBe(initialPlayer);
      expect(newState.gameState.currentPlayer).not.toBe(initialPlayer);
    });

    it('should not complete move when no pending move exists', () => {
      const store = useGameStore.getState();
      const initialState = { ...store.gameState };

      store.actions.completeMove();

      const newState = useGameStore.getState();
      expect(newState.gameState).toEqual(initialState);
    });
  });

  describe('makeAIMove', () => {
    it('should set aiThinking to true when starting AI move', async () => {
      useGameStore.setState(state => {
        state.gameState.currentPlayer = 'player2';
      });

      const store = useGameStore.getState();
      const aiMovePromise = store.actions.makeAIMove();

      const thinkingState = useGameStore.getState();
      expect(thinkingState.aiThinking).toBe(true);

      await aiMovePromise;
    });

    it('should not start AI move when not player2 turn', async () => {
      useGameStore.setState(state => {
        state.gameState.currentPlayer = 'player1';
      });

      const store = useGameStore.getState();
      await store.actions.makeAIMove();

      const newState = useGameStore.getState();
      expect(newState.aiThinking).toBe(false);
    });

    it('should not start AI move when game is finished', async () => {
      useGameStore.setState(state => {
        state.gameState.currentPlayer = 'player2';
        state.gameState.gameStatus = 'finished';
        state.gameState.winner = 'player1';
      });

      const store = useGameStore.getState();
      await store.actions.makeAIMove();

      const newState = useGameStore.getState();
      expect(newState.aiThinking).toBe(false);
    });
  });

  describe('reset', () => {
    it('should reset game to initial state', () => {
      useGameStore.setState(state => {
        state.gameState.gameStatus = 'finished';
        state.gameState.winner = 'player1';
        state.aiThinking = true;
        state.pendingMove = { column: 3, player: 'player1' };
        state.showWinnerModal = true;
      });

      const store = useGameStore.getState();
      store.actions.reset();

      const newState = useGameStore.getState();
      expect(newState.gameState.gameStatus).toBe('not_started');
      expect(newState.gameState.winner).toBe(null);
      expect(newState.aiThinking).toBe(false);
      expect(newState.pendingMove).toBe(null);
      expect(newState.showWinnerModal).toBe(false);
    });
  });

  describe('showWinnerModal', () => {
    it('should set showWinnerModal to true', () => {
      const store = useGameStore.getState();

      store.actions.showWinnerModal();

      const newState = useGameStore.getState();
      expect(newState.showWinnerModal).toBe(true);
    });
  });

  describe('game state persistence', () => {
    it('should maintain game state structure', () => {
      const store = useGameStore.getState();

      expect(store.gameState).toHaveProperty('board');
      expect(store.gameState).toHaveProperty('currentPlayer');
      expect(store.gameState).toHaveProperty('gameStatus');
      expect(store.gameState).toHaveProperty('winner');
      expect(store.gameState).toHaveProperty('history');
      expect(store.gameState).toHaveProperty('winningLine');
    });

    it('should have valid initial board state', () => {
      const store = useGameStore.getState();

      expect(store.gameState.board).toHaveLength(7);
      store.gameState.board.forEach(column => {
        expect(column).toHaveLength(6);
        column.forEach(cell => {
          expect(cell).toBe(null);
        });
      });
    });
  });
});
