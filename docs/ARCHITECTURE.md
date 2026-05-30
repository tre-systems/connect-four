# Architecture Overview

This document details the architecture of the Connect Four project, focusing on its AI engine, frontend, deployment, and infrastructure.

## What Makes This Special?

This implementation stands out for several reasons:

- **Classic Game, Modern Tech**: Brings the timeless Connect 4 game to life with cutting-edge web technologies
- **Dual AI System**: Features both a classic minimax AI and a neural network AI, each with distinct playstyles
- **Browser-Native AI**: All AI runs locally in your browser via WebAssembly - no server calls needed
- **Offline-First**: Works completely offline once loaded, perfect for mobile or unreliable connections
- **Performance**: Rust-compiled AI provides desktop-level performance in the browser
- **Evolutionary Architecture**: Successfully migrated from hybrid client/server AI to pure client-side execution

## Core Principles

- **High Performance**: Rust and WebAssembly for AI
- **Offline Capability**: Fully playable without internet
- **Seamless UX**: Modern, responsive UI
- **Maintainability**: Clear separation of UI, logic, and AI

## System Architecture

### Frontend (`src/`)

- **UI Components**: `src/components/` (React 19, Tailwind, Framer Motion)
- **Game State**: `src/lib/game-store.ts` (Zustand + Immer, persisted to `localStorage`)
- **UI State**: `src/lib/ui-store.ts` (Zustand)
- **Game Logic**: `src/lib/game-logic.ts` and `src/lib/logic/` (pure functions)
- **AI Services**: `src/lib/wasm-ai-service.ts` (loads the WASM module; handles Classic and ML AI)
- **Persistence layer**: `src/lib/db/` (Drizzle schema + D1/SQLite connector — see "Database System" below)

### AI Engine

- **Classic AI**: Rust, minimax with alpha-beta pruning, compiled to WebAssembly
- **ML AI**: Rust, neural network, compiled to WebAssembly
- **Performance**: All AI runs locally in the browser (no server calls)
- **Architecture**: Pure client-side execution via Web Workers

### WASM Architecture Evolution

The project has evolved from a hybrid client/server architecture to a pure client-side implementation:

**Original Design (Early Development)**:

- AI computation could run on either client (WASM) or server (Cloudflare Worker)
- Server-side AI provided backup and potential performance benefits
- More complex deployment and infrastructure requirements

**Current Design (Production)**:

- All AI computation runs client-side via WebAssembly workers
- Eliminates network latency and server infrastructure costs
- Enables true offline play without server dependencies
- Simplified deployment and reduced attack surface

## Data Flow

### AI Turn Processing

1. `ConnectFour.tsx` detects AI turn
2. Calls `makeAIMove` in `game-store.ts`
3. Calls appropriate AI service (Classic AI or ML AI)
4. Chosen move processed by `makeMoveLogic`
5. UI updates

### Game Completion

1. Game state set to finished
2. Winning line highlighted and the win animation plays
3. Completion overlay shows the result
4. Current game state is persisted to `localStorage` (so a game in progress survives a refresh)

> **Note:** The app is currently pure client-side. There is no server round-trip on game completion — games are **not** written to a database in the running app today. See "Database System" below.

## Database System

> **Status:** The Drizzle schema, the D1 binding (`wrangler.toml`), the local SQLite connector (`src/lib/db/index.ts`), and the migration in `migrations/` all exist and are migration-ready — but **no application code currently calls the database.** `getDb()` is only exercised by tests; `src/app/` has no API routes or server actions. This is scaffolding for server-side game history/analytics that has not been wired into the client-only app. Treat the section below as the intended design, not current runtime behaviour. See [BACKLOG.md](./BACKLOG.md).

### Local Development

- **Database**: SQLite (`local.db`)
- **ORM**: Drizzle ORM
- **Setup**: `npm run db:local:reset`

### Production

- **Database**: Cloudflare D1
- **ORM**: Drizzle ORM
- **Migrations**: `npm run migrate:d1`

### Schema

```typescript
// src/lib/db/schema.ts
export const games = sqliteTable('games', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => nanoid()),
  playerId: text('playerId').notNull(),
  winner: text('winner', { enum: ['player1', 'player2'] }),
  completedAt: integer('completedAt', { mode: 'timestamp_ms' }),
  moveCount: integer('moveCount'),
  duration: integer('duration'),
  clientHeader: text('clientHeader'),
  history: text('history', { mode: 'json' }),
  gameType: text('gameType', { enum: ['classic', 'ml', 'watch', 'heuristic'] })
    .notNull()
    .default('classic'),
});
```

### Game Statistics (intended design)

The schema is designed to capture per-game records for later analytics:

- **Outcome + metadata**: winner, move count, duration, and game type (`classic` / `ml` / `watch` / `heuristic`)
- **Privacy-focused**: `playerId` generated with `nanoid()` for anonymous tracking — no PII

Today, the only thing that persists between sessions is the **current game state** (via the Zustand `persist` middleware → `localStorage`, key `connect-4-game-storage`). Aggregate win-rate/analytics and database writes are not yet implemented.

## Deployment

### Frontend Deployment

- **Platform**: Next.js 15 deployed to Cloudflare Workers via OpenNext (`@opennextjs/cloudflare`)
- **Build**: `npm run build:cf`
- **Domain**: `https://connect-4.tre.systems`

### WASM Security Headers

Set in `public/_headers`:

```
/wasm/*
  Cross-Origin-Embedder-Policy: require-corp
  Cross-Origin-Opener-Policy: same-origin
  Cross-Origin-Resource-Policy: same-origin
```

### Performance Considerations

AI runs **exclusively client-side** (WASM). This was a deliberate move away from the original hybrid client/server design: it removes network latency, eliminates server/infra cost, and enables true offline play. The trade-off — performance now depends on the user's device — is acceptable because the bitboard solver is fast even on mobile.

## Development vs Production

### Development Environment

- **Dev-only tools**: AI diagnostics, AI toggle, reset/test buttons (only on localhost)
- **Local database**: SQLite for development
- **Debug features**: Enhanced logging and diagnostics

### Production Environment

- **Clean UI**: No development tools
- **Classic AI default**: Most reliable AI opponent
- **Cloudflare D1**: Production database
- **Optimized builds**: Minified and optimized assets

## Summary

- Modern, maintainable, high-performance architecture
- All AI runs locally in the browser (WASM)
- Clear separation of concerns
- Full offline support (PWA)
- Privacy-focused, anonymous player IDs
- Database layer scaffolded (D1 + Drizzle) but not yet wired into the app
