# Backlog

Known gaps and future work for Connect Four, ordered roughly by priority.

## Product / AI

- **Difficulty levels.** Connect Four is solved, so the AI plays near-perfectly — not fun for casual players. Expose difficulty as solver search depth (1 = easy … 8+ = unbeatable), and treat the ML AI as an optional "experimental" mode rather than a peer of Classic.
- **Revisit the genetic-params evolution.** The bitboard solver solves the game cheaply, so evolved heuristic weights are largely a solution in search of a problem. Keep the GA only if building it is itself a goal; otherwise it's a candidate to cut.

## Tech debt

- **Dependency refresh.** A few majors behind: `wrangler`, `next` (15 → 16), `eslint` (9 → 10), plus minors (`framer-motion`, `vitest`, `playwright`). Review with `npm run deps`, apply with `npm run deps:update`, then re-run `npm run check`.
- **Oversized Rust files vs the 200-line house rule** — `lib.rs`, `neural_network.rs`, `solver.rs`, `mcts.rs`, `genetic_params.rs`, `features.rs`, `wasm_api.rs`. `lib.rs` (the WASM boundary + glue) is the best split candidate.

## Pattern consistency

Deviations from the documented [architecture patterns](ARCHITECTURE.md#architecture-patterns):

- **`GameMode` has an unused `human-vs-human`** member.
- **`heuristic` engine isn't a first-class `AIType`** — it exists in Rust + the facade (`getHeuristicMove`) but the UI only selects `classic` / `ml`. Surface it or drop the path.
- **Typed boundary errors** — replace stringly-typed throws at the WASM edge with a discriminated `WasmAiError`, building on the `number | null` that `fallbackMove` already returns toward a `Result<column, reason>`.

## Nice to have

- **CI on pull requests.** Other projects (antenna, uwp) run a separate `ci.yml` for lint/type/test on PRs; this repo only has `deploy.yml` (push-to-main). Fine for the current solo workflow.
- **Stop hand-maintaining metrics in docs.** Prefer a coverage badge / generated summary over hardcoded numbers.
