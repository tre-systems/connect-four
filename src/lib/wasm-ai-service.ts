import { GameState } from './schemas';
import type { WasmBestMoveResponse, WasmHeuristicResponse, WasmMLResponse } from './bindings';

interface WASMAIInstance {
  get_best_move: (state: unknown, depth: number) => WasmBestMoveResponse;
  get_heuristic_move: (state: unknown) => WasmHeuristicResponse;
  get_ml_move: (state: unknown) => WasmMLResponse;
  evaluate_position: (state: unknown) => number;
  evaluate_position_ml: (state: unknown) => number;
  load_ml_weights: (value_weights: unknown, policy_weights: unknown) => void;
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

  // ... (initialize and _initialize methods same as before)
  async initialize(): Promise<void> {
    if (this.loadPromise) {
      return this.loadPromise;
    }

    this.loadPromise = this._initialize();
    return this.loadPromise;
  }

  private async _initialize(): Promise<void> {
    // Only load WASM in browser environment
    if (typeof window === 'undefined') {
      console.log('🔄 Skipping WASM AI initialization in non-browser environment');
      return;
    }

    try {
      // Load WASM module using runtime import
      console.log('🔄 Loading WASM module...');

      // Use a runtime import that won't be resolved at build time
      const wasmModulePath = '/wasm/connect_four_ai_core.js';
      console.log('🔄 Attempting to load WASM module from:', wasmModulePath);

      const wasmModule = (await import(/* webpackIgnore: true */ wasmModulePath)) as WASMModule;

      console.log('🔄 WASM module imported, initializing...');
      await wasmModule.default();
      console.log('🔄 WASM module initialized, creating AI instance...');
      this.ai = new wasmModule.ConnectFourAI();
      this.isLoaded = true;
      console.log('✅ WASM AI loaded successfully');
    } catch (error) {
      console.error('❌ Failed to load WASM AI:', error);
      console.error('❌ Error details:', error instanceof Error ? error.stack : error);

      // Try to provide more specific error information
      if (error instanceof TypeError && error.message.includes('Failed to fetch')) {
        console.error(
          '❌ This might be a network issue - check if the WASM files are being served correctly'
        );
      }

      throw new Error(`Failed to load WASM AI: ${error}`);
    }
  }

  private async convertGameStateToWASM(gameState: GameState): Promise<unknown> {
    const board = gameState.board.map(col =>
      col.map(cell => {
        if (cell === null) return 'empty';
        return cell;
      })
    );

    // Load genetic parameters from evolved.json
    const geneticParams = await this.loadGeneticParams();

    return {
      board,
      current_player: gameState.currentPlayer,
      genetic_params: geneticParams,
    };
  }

  private async loadGeneticParams(): Promise<Record<string, string | number | string[]>> {
    try {
      // Try to load from the evolved.json file
      const response = await fetch('/ml/data/genetic_params/evolved.json');
      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      console.warn('Failed to load evolved genetic parameters, using defaults:', error);
    }

    // Fallback to default parameters (matching Rust GeneticParams::default())
    return {
      id: 'default-fallback',
      parent_ids: [],
      generation: 0,
      win_score: 10000,
      loss_score: -10000,
      center_column_value: 165,
      adjacent_center_value: 97,
      outer_column_value: 17,
      edge_column_value: 6,
      row_height_weight: 1.798,
      center_control_weight: 2.022,
      piece_count_weight: 0.965,
      threat_weight: 1.588,
      mobility_weight: 1.453,
      vertical_control_weight: 2.862,
      horizontal_control_weight: 1.344,
      defensive_weight: 1.372,
      horizontal_connection_weight: 1.344, // Added missing fields if needed or just keep generic map
    };
  }

  async getBestMove(gameState: GameState, depth: number = 1): Promise<WasmBestMoveResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      const result = this.ai.get_best_move(wasmState, depth);

      console.log('WASM AI: Result:', result);
      return result;
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
      const result = this.ai.get_heuristic_move(wasmState);

      return result;
    } catch (error) {
      throw new Error(`WASM heuristic AI failed: ${error}`);
    }
  }

  async getMLMove(gameState: GameState): Promise<WasmMLResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      console.log('🔍 ML AI: Converting game state to WASM...');
      const wasmState = await this.convertGameStateToWASM(gameState);
      console.log('🔍 ML AI: Calling WASM get_ml_move...');
      const result = this.ai.get_ml_move(wasmState);
      console.log('🔍 ML AI: Result:', result);
      return result;
    } catch (error) {
      console.error('🔍 ML AI: Error details:', error);
      throw new Error(`WASM ML AI failed: ${error}`);
    }
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

  async loadMLWeights(valueWeights: number[], policyWeights: number[]): Promise<void> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      this.ai.load_ml_weights(valueWeights, policyWeights);
      console.log('✅ ML weights loaded successfully');
    } catch (error) {
      throw new Error(`Failed to load ML weights: ${error}`);
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

// Singleton instance
let wasmAIInstance: WASMAIService | null = null;

export function getWASMAIService(): WASMAIService {
  if (!wasmAIInstance) {
    wasmAIInstance = new WASMAIService();
  }
  return wasmAIInstance;
}

// For testing purposes
export function resetWASMAIService(): void {
  wasmAIInstance = null;
}

export async function initializeWASMAI(): Promise<void> {
  const service = getWASMAIService();
  await service.initialize();

  // Try to load ML weights (trained Connect Four model)
  try {
    console.log('🔍 Loading ML weights from /ml/data/weights/ml_ai_weights_best.json...');
    const weightsResponse = await fetch('/ml/data/weights/ml_ai_weights_best.json');
    console.log('🔍 Weights response status:', weightsResponse.status, weightsResponse.ok);

    let weightsLoaded = false;

    if (weightsResponse.ok) {
      const model = (await weightsResponse.json()) as {
        value_network?: { weights: number[] };
        policy_network?: { weights: number[] };
      };
      console.log('🔍 Model structure:', Object.keys(model));
      console.log('🔍 Value network exists:', !!model.value_network);
      console.log('🔍 Policy network exists:', !!model.policy_network);
      console.log('🔍 Value weights length:', model.value_network?.weights?.length);
      console.log('🔍 Policy weights length:', model.policy_network?.weights?.length);

      if (model.value_network?.weights && model.policy_network?.weights) {
        await service.loadMLWeights(model.value_network.weights, model.policy_network.weights);
        console.log('✅ ML weights loaded successfully (trained Connect Four model)');
        weightsLoaded = true;
      } else {
        console.warn('Model format not recognized - missing weights arrays');
      }
    }

    if (!weightsLoaded) {
      console.log('🔍 Trying simple model fallback...');
      // Fallback to simple model
      try {
        const simpleResponse = await fetch('/ml/data/weights/ml_ai_weights_simple.json');
        if (simpleResponse.ok) {
          const simpleModel = (await simpleResponse.json()) as {
            value_network?: { weights: number[] };
            policy_network?: { weights: number[] };
          };

          if (simpleModel.value_network?.weights && simpleModel.policy_network?.weights) {
            await service.loadMLWeights(
              simpleModel.value_network.weights,
              simpleModel.policy_network.weights
            );
            console.log('✅ ML weights loaded successfully (simple model fallback)');
            weightsLoaded = true;
          } else {
            console.warn('Simple model format not recognized - missing weights arrays');
          }
        } else {
          console.error(
            'Failed to fetch simple ML weights:',
            simpleResponse.status,
            simpleResponse.statusText
          );
        }
      } catch (fallbackError) {
        console.error('Could not load simple ML weights:', fallbackError);
      }
    }

    if (!weightsLoaded) {
      console.warn('⚠️ No ML weights loaded - ML AI will use random weights');
    }
  } catch (error) {
    console.error('Could not load ML weights:', error);
  }
}

export default WASMAIService;
