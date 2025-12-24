# Development Guide

This document provides comprehensive guidance for developers working on the Connect Four project.

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run all checks
npm run check

# Run tests
npm run test
npm run test:e2e
```

## Project Structure

```
connect-four/
├── src/                    # Frontend source code
│   ├── app/               # Next.js app directory
│   ├── components/        # React components
│   └── lib/               # Core logic and utilities
├── worker/                # Rust AI engine (WASM source)
│   ├── src/               # Core AI logic
│   └── tests/             # Rust integration tests
├── resources/             # Static assets (AI weights, params)
├── e2e/                   # End-to-end tests
└── docs/                  # Documentation
```

## AI Development

### Genetic Algorithm Evolution

The project uses a genetic algorithm to evolve AI parameters for optimal gameplay. The parameters are pre-calculated and stored in `resources/ai/evolved.json`.

#### Parameter Tracking

The evolution process tracks:

- **Parameters**: 14 genetic parameters plus fitness and diversity metrics
- **Convergence**: Parameter changes between generations to show stability

#### Expected Convergence Patterns

- **Parameter stabilization**: Should occur after ~20-30 generations
- **Fitness plateau**: Should reach 0.8-0.9 range
- **Diversity decrease**: Population should converge over time

#### Genetic Parameter IDs

Genetic parameters use UUID-based identifiers (36 characters) for uniqueness and readability.

### AI Testing

```bash
# Run comprehensive AI comparison
npm run test:ai-comparison:comprehensive
```

## Database

### Local Development

```bash
# Reset local database
npm run db:local:reset

# Generate migrations
npm run migrate:generate

# Apply local migrations
npm run migrate:local
```

### Production

```bash
# Apply production migrations
npm run migrate:d1

# Database shell
npm run db:shell
```

## Testing

### Unit Tests

```bash
# Run all unit tests
npm run test

# Run with coverage
npm run test:coverage

# Watch mode
npm run test:watch
```

### End-to-End Tests

```bash
# Run e2e tests
npm run test:e2e

# Run with UI
npm run test:e2e:ui
```

### Rust Tests

```bash
# Run Rust tests
npm run test:rust

# Run slow tests
npm run test:rust:slow
```

## Building

### Development Build

```bash
# Build WASM assets (compiles Rust, copies weights from resources/ai)
npm run build:wasm-assets

# Build for development
npm run build
```

### Production Build

```bash
# Build for Cloudflare
npm run build:cf
```

## Deployment

```bash
# Deploy to Cloudflare
npm run deploy

# Quick deploy
npm run deploy:quick
```

## Code Quality

### Linting and Type Checking

```bash
# Run linting
npm run lint

# Fix linting issues
npm run lint:fix

# Type checking
npm run type-check
```

### Full Quality Check

```bash
# Run all quality checks
npm run check
```

This includes:

- ESLint linting
- TypeScript type checking
- Rust AI tests
- Unit test coverage
- End-to-end tests

## Troubleshooting

### WASM Issues

If you encounter WASM loading issues:

1. Rebuild WASM assets:

   ```bash
   npm run build:wasm-assets
   ```

2. Check that WASM files exist:

   ```bash
   ls -la public/wasm/
   ```

3. Verify WASM compilation:
   ```bash
   cd worker && cargo check
   ```

### Database Issues

If database operations fail:

1. Reset local database:

   ```bash
   npm run db:local:reset
   ```

2. Check migration status:
   ```bash
   npm run db:shell
   ```

### Test Failures

If tests are failing:

1. Check WASM compilation
2. Reset local database
3. Clear test cache:
   ```bash
   rm -rf coverage/ test-results/
   ```

## Performance Optimization

### Rust AI Performance

- Use `cargo build --release` for production builds
- Monitor AI matrix test results for performance regressions
- Use `npm run test:ai-comparison` to benchmark AI performance

### Frontend Performance

- Monitor bundle size with `npm run build`
- Use React DevTools for component profiling
- Check WASM loading times in browser dev tools

## Contributing

1. Follow the existing code style
2. Add tests for new functionality
3. Update documentation for significant changes
4. Run `npm run check` before submitting changes
5. Ensure all tests pass locally

## Useful Commands

```bash
# Full system check
npm run check

# Quick development cycle
npm run dev

# Test specific functionality
npm run test:ai-comparison:fast

# Monitor logs
npm run logs
```

## Helper Scripts

```bash
# Generate TypeScript types from Rust
npm run generate:types

# Verify WASM loading in node environment
npm run test:wasm-loading

# Convert PyTorch models to JSON for WASM
npm run utils:convert-model
```
