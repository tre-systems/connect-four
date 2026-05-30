#!/bin/bash

# AI Comparison Test Script
# Tests different AI types against each other

set -e

echo "🤖 Running AI Comparison Tests..."

echo "🔨 Building Rust AI core..."
cd worker
cargo build
cd ..

echo "🧪 Running AI Matrix Test..."
cd worker
cargo test test_ai_matrix -- --ignored --nocapture
cd ..

echo "✅ AI Comparison Tests Complete!" 