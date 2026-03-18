#!/bin/bash

# Cross-platform build script for SwingMusic Desktop
set -e

echo "🎵 Building SwingMusic Desktop for supported platforms..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build for Linux (native)
echo "🐧 Building for Linux (x86_64)..."
npm run tauri build -- --target x86_64-unknown-linux-gnu

# Build for Windows (cross-compilation)
echo "🪟 Building for Windows (x86_64)..."
npm run tauri build -- --target x86_64-pc-windows-gnu

# Note: macOS builds require a macOS host
echo "🍎 macOS builds require a macOS host - skipping on Linux"
echo "🐧 Linux ARM64 builds require complex cross-compilation setup - skipping"

echo "✅ Available builds completed successfully!"
echo ""
echo "📦 Generated packages:"
echo "Linux: target/x86_64-unknown-linux-gnu/release/bundle/"
echo "Windows: target/x86_64-pc-windows-gnu/release/bundle/"
echo ""
echo "🔧 For full cross-platform support, use GitHub Actions CI/CD:"
