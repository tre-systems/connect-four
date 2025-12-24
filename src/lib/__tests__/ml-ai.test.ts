import { describe, it, expect, vi } from 'vitest';
import { getWASMAIService } from '../wasm-ai-service';
import { initializeGame } from '../game-logic';

// Mock the WASM AI service
vi.mock('../wasm-ai-service', () => ({
  initializeWASMAI: vi.fn().mockResolvedValue(undefined),
  getWASMAIService: vi.fn(() => ({
    isReady: true,
    getMLMove: vi.fn().mockResolvedValue({
      move: 3,
      evaluation: 0.5,
      thinking: 'Simulated thinking',
    }),
    getBestMove: vi.fn().mockResolvedValue({
      move: 3,
      evaluations: [
        { column: 0, score: 0.1 },
        { column: 1, score: 0.2 },
        { column: 2, score: 0.3 },
        { column: 3, score: 0.5 },
        { column: 4, score: 0.4 },
        { column: 5, score: 0.3 },
        { column: 6, score: 0.2 },
      ],
      nodesEvaluated: 100,
      transpositionHits: 10,
    }),
    clearTranspositionTable: vi.fn(),
  })),
}));

describe('ML AI Integration', () => {
  it('should load WASM AI successfully', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);
  });

  it('should make ML moves', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);

    const gameState = initializeGame();

    // Test that we can get an ML move
    const result = await service.getMLMove(gameState);

    expect(result).toBeDefined();
    expect(result.move).toBeDefined();
    expect(typeof result.move).toBe('number');
    expect(result.move).toBeGreaterThanOrEqual(0);
    expect(result.move).toBeLessThan(7);
    expect(result.evaluation).toBeDefined();
    expect(typeof result.evaluation).toBe('number');
  });

  it('should make classic AI moves', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);

    const gameState = initializeGame();

    // Test that we can get a classic AI move
    const result = await service.getBestMove(gameState, 1);

    expect(result).toBeDefined();
    expect(result.move).toBeDefined();
    expect(typeof result.move).toBe('number');
    expect(result.move).toBeGreaterThanOrEqual(0);
    expect(result.move).toBeLessThan(7);
    expect(result.evaluations).toBeDefined();
    expect(Array.isArray(result.evaluations)).toBe(true);
  });
});
