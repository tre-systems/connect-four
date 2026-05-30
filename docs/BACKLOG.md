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

Deviations from the documented [architecture patterns](ARCHITECTURE.md#architecture-patterns) — mostly small, mechanical fixes:

- **Consolidate the AI/mode vocabulary.** `AITypeSchema` is `['classic','ml']`, but `ui-store` hard-codes `'heuristic'|'classic'|'ml'|'watch'` and `'heuristic'|'client'|'ml'`, and the DB `gameType` enum is `['classic','ml','watch','heuristic']`. Define one Zod enum (engine: `classic`/`ml`/`heuristic`) + reuse `GameMode`, and derive the rest. The heuristic engine already exists (`getHeuristicMove`) but isn't a first-class `AIType`.
- **Resolve `GameActionSchema`.** Defined and exported but unused — either adopt a `dispatch(action)` reducer that consumes it, or delete it.
- **`useShallow` for `useUIState`.** It returns a fresh object literal each render; wrap with `useShallow` (Zustand v5) to avoid needless re-renders.
- **Type the AI boundary.** Replace `(e: any)` in `ai-logic.ts` with the generated `MLMoveEvaluation` / `MoveEvaluationWasm` types.
- **DRY the fallback ladder.** The classic→random fallback is duplicated inside `ai-logic.makeAIMove`; fold it into one helper (a `Result<column, reason>` makes this clean).
- **Typed boundary errors.** Swap stringly-typed `throw new Error(\`…${error}\`)`at the WASM edge for a small discriminated`WasmAiError`.

## Nice to have

- **CI on pull requests.** Other projects (antenna, uwp) run a separate `ci.yml` for lint/type/test on PRs; this repo only has `deploy.yml` (push-to-main). Fine for the current solo, push-to-main workflow, but a lightweight PR check would help if collaborators appear.
- **Stop hand-maintaining metrics in docs.** Coverage %/test counts have repeatedly drifted. Prefer a coverage badge or generated summary over hardcoded numbers.
