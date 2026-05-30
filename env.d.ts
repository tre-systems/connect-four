/// <reference types="@cloudflare/workers-types" />

interface CloudflareEnv {
  DB: D1Database;
  ASSETS: Fetcher;
}

declare module '/wasm/connect_four_ai_core.js' {
  export class ConnectFourAI {
    free(): void;
    constructor();
    get_best_move(board_state: any, depth: number): any;
    get_heuristic_move(board_state: any): any;
    get_ml_move(board_state: any): any;
    evaluate_position(board_state: any): number;
    evaluate_position_ml(board_state: any): number;
    get_valid_moves(board_state: any): any;
    make_move(board_state: any, column: number): any;
    is_game_over(board_state: any): boolean;
    get_winner(board_state: any): any;
    create_new_game(): any;
    create_game_with_params(params: any): any;
    clear_transposition_table(): void;
    get_transposition_table_size(): number;
    load_ml_weights(value_weights: any, policy_weights: any): void;
  }

  export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

  export interface InitOutput {
    readonly memory: WebAssembly.Memory;
  }

  export type SyncInitInput = BufferSource | WebAssembly.Module;

  export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

  export default function __wbg_init(
    module_or_path?:
      | { module_or_path: InitInput | Promise<InitInput> }
      | InitInput
      | Promise<InitInput>,
  ): Promise<InitOutput>;
}
