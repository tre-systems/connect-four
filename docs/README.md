# Connect Four Documentation

Welcome to the Connect Four project documentation. This guide provides comprehensive information about the game, its AI system, development workflow, and deployment.

## 🎮 Quick Start

- **[Play Online](https://connect-4.tre.systems/)** - Live game
- **[Development Setup](./DEVELOPMENT.md#quick-start)** - Get started in 5 minutes
- **[Deployment Guide](./DEPLOYMENT.md)** - Deploy to production

## 📚 Documentation Structure

### Core Documentation

| Document                               | Purpose                                              | Audience      |
| -------------------------------------- | ---------------------------------------------------- | ------------- |
| **[DEVELOPMENT.md](./DEVELOPMENT.md)** | Complete development guide, testing, troubleshooting | Developers    |
| **[AI-SYSTEM.md](./AI-SYSTEM.md)**     | AI architecture, training, performance analysis      | AI developers |
| **[DEPLOYMENT.md](./DEPLOYMENT.md)**   | Cloudflare deployment, monitoring, troubleshooting   | DevOps        |
| **[GAME-GUIDE.md](./GAME-GUIDE.md)**   | Game rules, strategy, AI opponents                   | Players       |

### Reference Documentation

| Document                                 | Purpose                                  | Audience         |
| ---------------------------------------- | ---------------------------------------- | ---------------- |
| **[ARCHITECTURE.md](./ARCHITECTURE.md)** | System design, data flow, infrastructure | Architects       |
| **[TODO.md](./TODO.md)**                 | Current tasks and future plans           | Project managers |

## 🚀 Quick Reference

### Development Commands

```bash
npm run dev          # Start development server
npm run build        # Build for production
npm run check        # Run all checks (lint, test, type-check)
npm run deploy       # Deploy to Cloudflare
```

### AI Development

```bash
npm run test:ai-comparison    # Test AI performance
npm run train                 # Train ML models
```

### Database Management

```bash
npm run db:setup              # Setup local database
npm run db:migrate            # Run migrations
npm run db:shell              # Database shell
```

## 🎯 Current Status

### ✅ Completed Features

- **WASM AI Integration**: Rust/WASM AI system fully integrated and working
- **Dual AI System**: Classic minimax AI + ML neural network AI
- **Offline Support**: PWA with full offline gameplay
- **Cloudflare Deployment**: Production deployment with D1 database
- **Comprehensive Testing**: 89+ tests passing, E2E coverage

### 🔄 Active Development

- **ML AI Training**: Ongoing neural network training and optimization
- **Performance Optimization**: Continuous AI performance improvements
- **Feature Enhancement**: UI improvements and new game modes

### 📋 Next Steps

See **[TODO.md](./TODO.md)** for detailed roadmap and current tasks.

## 🏗️ Architecture Overview

The project uses a modern web architecture:

- **Frontend**: Next.js 15 with React, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly for client-side execution
- **Database**: Cloudflare D1 (production), SQLite (development)
- **Deployment**: Cloudflare Workers with GitHub Actions CI/CD

### Key Components

- **Game Logic**: Pure functions in `src/lib/game-logic.ts`
- **AI Services**: WASM integration in `src/lib/wasm-ai-service.ts`
- **State Management**: Zustand with Immer in `src/lib/game-store.ts`
- **Database**: Drizzle ORM with migrations

## 🤖 AI System

The game features a sophisticated dual AI system:

### Classic AI (Minimax)

- **Algorithm**: Minimax with alpha-beta pruning
- **Performance**: ~17ms per move, competitive play
- **Features**: Transposition tables, genetic parameters

### ML AI (Neural Network)

- **Architecture**: 4x128 ResNet-lite value + policy networks
- **Training**: Supervised learning from bitboard solver teacher
- **Performance**: Creative, unpredictable playstyle (~2-4s/move)

### Performance Results

Latest AI matrix test results show:

- **MM-Depth6**: 82.6% average win rate (strongest)
- **MM-Depth3**: 45.7% average win rate (balanced)
- **Heuristic**: 34.3% average win rate (educational)

## 🧪 Testing Strategy

The project uses a comprehensive testing approach:

- **Unit Tests**: Pure functions, schemas, game logic (Vitest)
- **Integration Tests**: Game store, AI services
- **E2E Tests**: Full game flows (Playwright)
- **AI Tests**: Performance matrix, competitive testing

### Test Commands

```bash
npm run test              # Unit tests
npm run test:e2e          # End-to-end tests
npm run test:ai-comparison # AI performance tests
npm run test:coverage     # Coverage report
```

## 📊 Performance

### Build Performance

- **Development**: Hot reload with WASM caching
- **Production**: Optimized builds with tree shaking
- **Deployment**: ~60KB worker bundle

### Runtime Performance

- **Classic AI Response**: < 20ms per move
- **ML AI Response**: ~2-4s per move (4000 MCTS simulations)
- **Game Loading**: < 2 seconds
- **Offline Support**: Full functionality without network

## 🔧 Troubleshooting

### Common Issues

| Issue             | Quick Fix                   |
| ----------------- | --------------------------- |
| WASM not loading  | `npm run build:wasm-assets` |
| Database errors   | `npm run db:setup`          |
| Build failures    | `npm run nuke`              |
| Deployment issues | Check `wrangler.toml`       |

### Getting Help

1. Check **[DEVELOPMENT.md](./DEVELOPMENT.md)** troubleshooting section
2. Review GitHub Issues for known problems
3. Check Cloudflare Workers logs: `npm run logs`

## 📈 Analytics

The game includes comprehensive analytics:

- **Game Statistics**: Win rates, move counts, duration
- **AI Performance**: Response times, evaluation metrics
- **User Behavior**: Game completion rates, AI selection

All data is privacy-focused with anonymous player IDs.

## 🤝 Contributing

Contributions are welcome! Please:

1. Read the documentation thoroughly
2. Run `npm run check` before submitting
3. Add tests for new features
4. Update documentation for changes

## 📄 License

MIT License - see [LICENSE](../LICENSE) for details.

---

**Last Updated**: July 2025  
**Version**: 1.0.0  
**Status**: Production Ready ✅
