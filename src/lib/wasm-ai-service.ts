import { GameState } from './schemas';
import type { WasmBestMoveResponse, WasmHeuristicResponse, WasmMLResponse } from './bindings';
import { DEFAULT_GENETIC_PARAMS } from './constants';

// The main-thread instance only drives the classic/heuristic engines. The ML
// engine (get_ml_move / load_ml_weights) lives in the Web Worker — see ai.worker.ts.
interface WASMAIInstance {
  get_best_move: (state: unknown, depth: number) => WasmBestMoveResponse;
  get_heuristic_move: (state: unknown) => WasmHeuristicResponse;
  evaluate_position: (state: unknown) => number;
  clear_transposition_table: () => void;
  get_transposition_table_size: () => number;
}

interface WASMModule {
  default: () => Promise<unknown>;
  ConnectFourAI: new () => WASMAIInstance;
}

class WASMAIService {
  private ai: WASMAIInstance | null = null;
  private isLoaded = false;
  private loadPromise: Promise<void> | null = null;

  private mlWorker: Worker | null = null;
  private mlReqId = 0;
  private mlPending = new Map<
    number,
    { resolve: (r: WasmMLResponse) => void; reject: (e: Error) => void }
  >();

  async initialize(): Promise<void> {
    if (this.loadPromise) {
      return this.loadPromise;
    }

    this.loadPromise = this._initialize();
    return this.loadPromise;
  }

  private async _initialize(): Promise<void> {
    if (typeof window === 'undefined') {
      console.log('🔄 Skipping WASM AI initialization in non-browser environment');
      return;
    }

    try {
      const wasmModulePath = '/wasm/connect_four_ai_core.js';

      const wasmModule = (await import(/* webpackIgnore: true */ wasmModulePath)) as WASMModule;

      await wasmModule.default();
      this.ai = new wasmModule.ConnectFourAI();
      this.isLoaded = true;
      console.log('✅ WASM AI loaded successfully');
    } catch (error) {
      console.error('❌ Failed to load WASM AI:', error);
      console.error('❌ Error details:', error instanceof Error ? error.stack : error);

      if (error instanceof TypeError && error.message.includes('Failed to fetch')) {
        console.error(
          '❌ This might be a network issue - check if the WASM files are being served correctly',
        );
      }

      throw new Error(`Failed to load WASM AI: ${error}`);
    }
  }

  private async convertGameStateToWASM(gameState: GameState): Promise<unknown> {
    const board = gameState.board.map(col => col.map(cell => cell ?? 'empty'));

    const geneticParams = await this.loadGeneticParams();

    return {
      board,
      current_player: gameState.currentPlayer,
      genetic_params: geneticParams,
    };
  }

  private async loadGeneticParams(): Promise<Record<string, string | number | string[]>> {
    try {
      const response = await fetch('/ml/data/genetic_params/evolved.json');
      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      console.warn('Failed to load evolved genetic parameters, using defaults:', error);
    }
    return DEFAULT_GENETIC_PARAMS;
  }

  async getBestMove(gameState: GameState, depth: number = 1): Promise<WasmBestMoveResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      return this.ai.get_best_move(wasmState, depth);
    } catch (error) {
      console.error('WASM AI error:', error);
      throw new Error(`WASM AI failed: ${error}`);
    }
  }

  async getHeuristicMove(gameState: GameState): Promise<WasmHeuristicResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      return this.ai.get_heuristic_move(wasmState);
    } catch (error) {
      throw new Error(`WASM heuristic AI failed: ${error}`);
    }
  }

  private getMLWorker(): Worker {
    if (!this.mlWorker) {
      this.mlWorker = new Worker(new URL('./ai.worker.ts', import.meta.url), { type: 'module' });
      this.mlWorker.addEventListener('message', (event: MessageEvent) => {
        const { id, response, error } = event.data as {
          id: number;
          response?: WasmMLResponse;
          error?: string;
        };
        const pending = this.mlPending.get(id);
        if (!pending) return;
        this.mlPending.delete(id);
        if (error) pending.reject(new Error(error));
        else pending.resolve(response as WasmMLResponse);
      });
      this.mlWorker.addEventListener('error', (event: ErrorEvent) => {
        const err = new Error(`ML worker error: ${event.message}`);
        this.mlPending.forEach(pending => pending.reject(err));
        this.mlPending.clear();
      });
    }
    return this.mlWorker;
  }

  // The ML MCTS search is slow (~seconds), so it runs in a Web Worker to keep the
  // main thread responsive. The worker owns its own WASM instance + weights.
  async getMLMove(gameState: GameState): Promise<WasmMLResponse> {
    const wasmState = await this.convertGameStateToWASM(gameState);
    const worker = this.getMLWorker();
    const id = ++this.mlReqId;
    return new Promise<WasmMLResponse>((resolve, reject) => {
      this.mlPending.set(id, { resolve, reject });
      worker.postMessage({ id, state: wasmState });
    });
  }

  async evaluatePosition(gameState: GameState): Promise<number> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      return this.ai.evaluate_position(wasmState);
    } catch (error) {
      throw new Error(`WASM position evaluation failed: ${error}`);
    }
  }

  get isReady(): boolean {
    return this.isLoaded;
  }

  clearTranspositionTable(): void {
    if (this.isLoaded && this.ai) {
      this.ai.clear_transposition_table();
    }
  }

  getTranspositionTableSize(): number {
    if (this.isLoaded && this.ai) {
      return this.ai.get_transposition_table_size();
    }
    return 0;
  }
}

let wasmAIInstance: WASMAIService | null = null;

export function getWASMAIService(): WASMAIService {
  if (!wasmAIInstance) {
    wasmAIInstance = new WASMAIService();
  }
  return wasmAIInstance;
}

export function resetWASMAIService(): void {
  wasmAIInstance = null;
}

// Loads the WASM module on the main thread for the classic/heuristic engines (and
// the classic fallback). The ML engine runs in a Web Worker that loads its own WASM
// + weights on demand (see ai.worker.ts), so the main thread never fetches the ML
// weights — they would otherwise be loaded into a network it never queries.
export async function initializeWASMAI(): Promise<void> {
  await getWASMAIService().initialize();
}

export default WASMAIService;
