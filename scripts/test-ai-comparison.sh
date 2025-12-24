#!/bin/bash

# AI Comparison Test Script
# Tests different AI types against each other

set -e

echo "🤖 Running AI Comparison Tests..."

# Build the Rust core
echo "🔨 Building Rust AI core..."
cd worker/rust_ai_core
cargo build
cd ../..

# Run the AI matrix test
echo "🧪 Running AI Matrix Test..."
cd worker/rust_ai_core
cargo test test_ai_matrix -- --ignored --nocapture
cd ../..

echo "✅ AI Comparison Tests Complete!" 