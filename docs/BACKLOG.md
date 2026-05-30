# Backlog

Known gaps, tech debt, and future work for Connect Four, ordered roughly by priority. Replaces the old `TODO.md` and follows the same `docs/BACKLOG.md` convention used in the maintainer's other projects.

## Decisions needed

- **Database layer: wire it up or remove it.** The Drizzle schema (`src/lib/db/schema.ts`), the `getDb()` connector (`src/lib/db/index.ts`), the D1 binding (`wrangler.toml`), and the migration in `migrations/` all exist — but **no application code calls them** (only tests). `src/app/` has no API routes or server actions. Pick one:
  - **(a) Wire it up** — add a route handler / server action that writes a `games` row on completion, and call it from `game-store`. Gives you the analytics the docs describe.
  - **(b) Remove it** — drop the schema, connector, migration, the D1 binding, and the `better-sqlite3` / `drizzle-orm` / `drizzle-kit` deps. Shrinks the surface area and the dependency/security footprint.

  Until then it is maintained dead weight and the docs have to carry a "not wired in" caveat.

## Tech debt

- **Dependency refresh.** Several majors behind: `@opennextjs/cloudflare` 1.14 → 1.19, `wrangler` 4.56 → 4.95, `next` 15.1 → 16, `eslint` 9 → 10, plus many minors (`framer-motion`, `vitest`, `playwright`, `better-sqlite3`, …). Review with `npm run deps`, apply with `npm run deps:update`, then re-run `npm run check`.
- **Security audit.** `npm audit` reports vulnerabilities, almost all in build/dev tooling (`miniflare`/`ws`, `yaml`). The deployed app ships no server endpoints, so runtime exposure is low — but the high/critical advisories should be triaged. Several clear by bumping `wrangler`.
- **Lint warning.** `react-hooks/exhaustive-deps` in `src/hooks/useGameAnimations.ts:54` (missing `boardRef`). AGENTS.md says never ignore lint — fix it or add a justified disable.
- **Oversized files vs the 200-line house rule.** `worker/src/lib.rs` (1024), `neural_network.rs` (737), `solver.rs` (420), `mcts.rs` (390), `genetic_params.rs` (376), `features.rs` (371), `wasm_api.rs` (344); on the TS side `src/lib/visuals/background-effects.ts` (384) and `src/lib/game-store.ts` (239). `lib.rs` is the WASM boundary + glue and is the best split candidate.

## Pattern consistency

Deviations from the documented [architecture patterns](ARCHITECTURE.md#architecture-patterns).

**Done:** removed the dead `selectedMode` / `aiSourceP1` / `aiSourceP2` vocabulary from `ui-store`; components import `GameMode` instead of inline unions; removed the unused `GameActionSchema`; typed the `ai-logic` WASM boundary and de-duplicated its fallback ladder into a `fallbackMove` helper.

**Remaining (deep clean):**

- **DB `gameType` enum** still lists `watch` / `heuristic` — align it with the domain once the [DB wire-up-or-remove decision](#decisions-needed) is made.
- **Unused UI state in `ui-store`** — `showModelOverlay`, `diagnosticsPanelOpen`, `howToPlayOpen`, and the `useUIState` selector have no consumers; remove or wire them.
- **`GameMode` has an unused `human-vs-human`** member.
- **`heuristic` engine isn't a first-class `AIType`** — it exists in Rust + the facade (`getHeuristicMove`) but the UI only selects `classic` / `ml`. Decide whether to surface it or drop the path.
- **Typed boundary errors** — replace stringly-typed throws at the WASM edge with a discriminated `WasmAiError`, building on the `number | null` that `fallbackMove` already returns toward a `Result<column, reason>`.

## Nice to have

- **CI on pull requests.** Other projects (antenna, uwp) run a separate `ci.yml` for lint/type/test on PRs; this repo only has `deploy.yml` (push-to-main). Fine for the current solo, push-to-main workflow, but a lightweight PR check would help if collaborators appear.
- **Stop hand-maintaining metrics in docs.** Coverage %/test counts have repeatedly drifted. Prefer a coverage badge or generated summary over hardcoded numbers.
