# Connect Four

A modern, AI-powered implementation of the classic Connect Four game, built with Next.js, Rust/WASM, and advanced genetic algorithms.

![Connect Four Screenshot](screenshot.png)

## Features

- **Classic Gameplay**: Traditional Connect Four with modern UI
- **AI Opponents**: Multiple AI difficulty levels powered by Rust/WASM
- **Genetic Evolution**: AI parameters evolved through genetic algorithms
- **Parameter Tracking**: Comprehensive CSV logging and visualization of evolution
- **Offline-First**: Works completely offline once loaded
- **Mobile Optimized**: Responsive design for all devices
- **PWA Support**: Install as a native app
- **Elegant Win Animation**: Clean, professional highlighting of winning pieces
- **Improved UI**: Compact start screen with AI icons and better visual hierarchy

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

The game features a sophisticated AI system with multiple opponents to choose from:

- **Classic AI**: Traditional minimax algorithm with alpha-beta pruning. Fast and reliable.
- **ML AI**: Neural network trained on genetic algorithm data. Balanced performance.
- **Self-Play AI**: Advanced neural network trained through self-play with MCTS exploration. Most sophisticated.

### WASM AI Integration

The AI system is powered by Rust/WASM for optimal performance:

- **Rust Core**: High-performance game logic and AI algorithms
- **WASM Compilation**: Cross-platform compatibility with native performance
- **ML Integration**: Neural network weights loaded dynamically
- **Genetic Parameters**: Evolved parameters for heuristic evaluation

#### Recent Fixes (July 2025)

- ✅ **WASM Module Loading**: Fixed WASM compilation with proper feature flags
- ✅ **ML Weights Format**: Converted complex model format to simple flat arrays
- ✅ **Asset Serving**: Ensured all WASM assets are properly served
- ✅ **Error Handling**: Added comprehensive fallback mechanisms
- ✅ **Build Process**: Streamlined WASM build and asset deployment

### AI Selection Interface

Players can select their preferred AI opponent from a beautiful card-based interface that shows:

- AI type and description
- Performance characteristics
- Visual indicators for selection

### Genetic Algorithm Evolution

The AI parameters are evolved using genetic algorithms:

```bash
# Run genetic parameter evolution
npm run evolve:genetic-params

# Plot evolution results
python scripts/plot_evolution.py evolution_params_20241201_143022.csv
```

### Self-Play Training

Advanced AI models are trained through self-play:

```bash
# Quick self-play training (100 games, 10 epochs)
npm run train:self-play:quick

# Standard self-play training (1000 games, 50 epochs)
npm run train:self-play
```

### Simple ML Training

Lightweight neural networks for fast, efficient AI:

```bash
# Train basic simple model (20 epochs, 500 games)
python3 ml/scripts/simple_train.py --epochs 20 --num-games 500

# Train enhanced simple model (50 epochs, 1000 games)
python3 ml/scripts/simple_train.py --epochs 50 --num-games 1000 --batch-size 64 --learning-rate 0.0005 --output simple_model_enhanced.json
```

## Project Status

- **All lint, type checks, and tests pass** as of the latest commit.
- **AI Matrix**: All AI types tested, with MM-Depth6 and ML-Simple showing strong performance. See `docs/AI-MATRIX-RESULTS.md` for details.
- **Coverage**: 70% statements, 82% branches, 77% functions. All core logic and AI modules are well covered.
- **E2E**: All Playwright end-to-end tests pass.
- **AI vs AI Mode**: Now supports Classic AI vs ML AI battles with clear visual indicators.

## Features

### AI Opponents

- **Classic AI**: Minimax + Alpha-Beta (Depth 5) - Plays with classic board game strategy for a strong challenge.
- **ML AI**: Policy + Value Neural Network - Uses machine learning for fast, smart moves and adaptive play.

### Game Modes

- **Human vs AI**: Play against either Classic or ML AI
- **AI vs AI**: Watch Classic AI battle ML AI with clear visual indicators
- **Smooth Animations**: Enhanced animated background with no abrupt transitions

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

The application is deployed to Cloudflare Pages with automatic deployments from the main branch.

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
