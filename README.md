# Connect Four

A modern, AI-powered implementation of the classic Connect Four game, built with Next.js, Rust/WASM, and advanced genetic algorithms.

![Connect Four Screenshot](screenshot.png)

## Features

- **Classic Gameplay**: Traditional Connect Four with modern UI and smooth animations
- **Advanced AI Opponents**:
  - **Bitboard Solver**: Strictly optimized Minimax with bitboards (Depth 6)
  - **ML-MCTS AI**: AlphaZero-style Monte Carlo Tree Search with neural networks
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

1.  **Bitboard Solver**: High-performance solver using 64-bit operations.
2.  **ML-MCTS AI**: AlphaZero-style neural (Value/Policy) + MCTS.
3.  **Self-Play**: Advanced training capability.

> For detailed architecture and performance stats, see [AI System Documentation](docs/AI-SYSTEM.md).

### Recent Updates

- **WASM Integration**: Pure client-side execution for zero latency.
- **Model Optimization**: ML models converted to efficient flat arrays.

### Genetic Evolution

The AI parameters are evolved using genetic algorithms. Results are included in the release.

## Project Status

- **All lint, type checks, and tests pass** as of the latest commit.
- **AI Matrix**: All AI types tested, with **Bitboard-Solver (Depth 6)** and **ML-MCTS (AlphaZero)** showing strong performance. See [`docs/AI-MATRIX-RESULTS.md`](docs/AI-MATRIX-RESULTS.md) for details.
- **Coverage**: 70% statements, 82% branches, 77% functions. All core logic and AI modules are well covered.
- **E2E**: All Playwright end-to-end tests pass.
- **AI vs AI Mode**: Now supports Classic AI vs ML AI battles with clear visual indicators.

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

- **Frontend**: Next.js 15 with React, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly for client-side execution
- **Database**: Cloudflare D1 (production), SQLite (development)
- **Deployment**: Cloudflare Workers with GitHub Actions CI/CD

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `npm run check`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
