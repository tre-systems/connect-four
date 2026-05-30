# Connect Four

A modern, AI-powered implementation of the classic Connect Four game, built with Next.js, Rust/WASM, and advanced genetic algorithms.

![Connect Four Screenshot](screenshot.png)

## Features

- **Classic Gameplay**: Traditional Connect Four with modern UI and smooth animations
- **Advanced AI Opponents**:
  - **Bitboard Solver**: Strictly optimized [Minimax](https://en.wikipedia.org/wiki/Minimax) with [Bitboards](https://en.wikipedia.org/wiki/Bitboard) (Depth 6)
  - **ML-MCTS AI**: [AlphaZero](https://en.wikipedia.org/wiki/AlphaZero)-style [Monte Carlo Tree Search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search) with neural networks
- **Genetic Evolution**: AI parameters evolved through genetic algorithms
- **Offline-First**: Works completely offline once loaded as a PWA
- **Analytics**: Comprehensive parameter tracking and CSV logging
- **Game Modes**: Human vs AI, and AI vs AI (Watch Mode)
- **Responsive**: Fully optimized for mobile and desktop

## Quick Start

```bash
# Install dependencies
npm install

# Setup database and build WASM
npm run db:setup
npm run build:wasm-assets

# Start development server
npm run dev
```

Visit [http://localhost:3000](http://localhost:3000) to play!

## Troubleshooting

### WASM AI Issues

If you encounter issues with the WASM AI system:

1. **Rebuild WASM Assets**:

   ```bash
   npm run build:wasm-assets
   ```

2. **Test WASM Loading**:

   ```bash
   node scripts/test-wasm-loading.js
   ```

3. **Check Console Logs**: Look for WASM loading messages in browser console

4. **Verify Assets**: Ensure all files are accessible:
   - `/wasm/connect_four_ai_core.js`
   - `/wasm/connect_four_ai_core_bg.wasm`
   - `/ml/data/weights/ml_ai_weights_best.json`
   - `/ml/data/genetic_params/evolved.json`

## AI System

The game features a sophisticated Dual AI system powered by Rust and WebAssembly:

1.  **Bitboard Solver**: High-performance solver using [bitboard optimizations](https://github.com/denkspuren/BitboardC4) (Negamax + alpha-beta).
2.  **ML-MCTS AI**: [AlphaZero](https://en.wikipedia.org/wiki/AlphaZero)-style neural (Value/Policy) + MCTS.
3.  **Supervised Training**: Models trained using bitboard solver as teacher.

> For detailed architecture and performance stats, see [AI System Documentation](docs/AI-SYSTEM.md).

### Recent Updates

- **WASM Integration**: Pure client-side execution for zero latency.
- **Model Optimization**: ML models converted to efficient flat arrays.

### Genetic Evolution

The AI parameters are evolved using genetic algorithms. Results are included in the release.

## Project Status

- **CI**: Every push to `main` runs the full gate (`npm run check`: lint, type-check, Rust AI matrix tests, unit coverage, and Playwright e2e) before deploying — see [deploy.yml](.github/workflows/deploy.yml).
- **AI Matrix**: All AI types are exercised by `cargo test`, with the **Bitboard Solver (Depth 6)** and **ML-MCTS (AlphaZero)** as the strongest opponents.
- **Game Modes**: Human vs AI and AI vs AI ("Watch Mode"), including Classic vs ML battles.
- **Persistence**: Current game state is saved to `localStorage`. A D1/Drizzle database layer is scaffolded but not yet wired in — see [docs/BACKLOG.md](docs/BACKLOG.md).

## Development

### Testing

```bash
# Run all tests
npm run test

# Run tests with coverage
npm run test:coverage

# Run E2E tests
npm run test:e2e

# Run AI comparison tests
npm run test:ai-comparison:fast
```

### Building

```bash
# Build for development
npm run build

# Build for production
npm run build:cf

# Build WASM assets
npm run build:wasm-assets
```

### Database

```bash
# Setup local database
npm run db:setup

# Run migrations
npm run db:migrate

# Database shell
npm run db:shell
```

## Deployment

The application is deployed to Cloudflare Workers with automatic deployments from the main branch.

```bash
# Deploy manually
npm run deploy

# Quick deploy
npm run deploy:quick
```

## Architecture

- **Frontend**: Next.js 15 with React 19, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly for client-side execution
- **Persistence**: `localStorage` for game state; a Cloudflare D1 + Drizzle layer is scaffolded for future server-side history (not yet wired in)
- **Deployment**: Cloudflare Workers (via OpenNext) with GitHub Actions CI/CD

See [docs/](docs/) for the full architecture, AI system, development, and deployment guides.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `npm run check`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
