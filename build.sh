#!/bin/bash

# SwingMusic Desktop Build Script
# This script helps build the desktop app for different platforms

set -e

echo "🎵 SwingMusic Desktop Build Script"
echo "=================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the swingmusic-desktop directory"
    exit 1
fi

# Check dependencies
echo "🔍 Checking dependencies..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Node.js is installed
if ! command -v npm &> /dev/null; then
    echo "❌ Error: Node.js is not installed. Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Check if Tauri CLI is installed
if ! command -v tauri &> /dev/null; then
    echo "📦 Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
fi

# Install Node.js dependencies
echo "📦 Installing Node.js dependencies..."
npm install

# Build frontend first
echo "🏗️ Building frontend..."
cd ../swingmusic-webclient
npm install
npm run build
cd ../swingmusic-desktop

# Parse command line arguments
COMMAND=${1:-"dev"}
PLATFORM=${2:-"current"}

echo "🚀 Running: $COMMAND for platform: $PLATFORM"

case $COMMAND in
    "dev")
        echo "🔧 Starting development mode..."
        npm run dev
        ;;
    "build")
        echo "📦 Building for production..."
        case $PLATFORM in
            "all")
                echo "🏗️ Building for all platforms..."
                npm run build
                ;;
            "windows")
                echo "🪟 Building for Windows..."
                npm run build -- --target x86_64-pc-windows-ms
                ;;
            "macos")
                echo "🍎 Building for macOS..."
                npm run build -- --target x86_64-apple-darwin
                npm run build -- --target aarch64-apple-darwin
                ;;
            "linux")
                echo "🐧 Building for Linux..."
                npm run build -- --target x86_64-unknown-linux-gnu
                ;;
            *)
                echo "🏗️ Building for current platform..."
                npm run build
                ;;
        esac
        ;;
    "clean")
        echo "🧹 Cleaning build artifacts..."
        cargo clean
        rm -rf src-tauri/target
        cd ../swingmusic-webclient
        rm -rf dist
        cd ../swingmusic-desktop
        echo "✅ Clean completed"
        ;;
    "test")
        echo "🧪 Running tests..."
        cargo test
        cd ../swingmusic-webclient
        npm run test
        cd ../swingmusic-desktop
        ;;
    "info")
        echo "📊 Build information:"
        npm run tauri info
        ;;
    "setup")
        echo "⚙️ Setting up development environment..."
        
        # Install Rust targets
        echo "📦 Installing Rust targets..."
        rustup target add x86_64-pc-windows-ms
        rustup target add x86_64-apple-darwin
        rustup target add aarch64-apple-darwin
        rustup target add x86_64-unknown-linux-gnu
        
        # Install platform-specific dependencies
        case "$(uname -s)" in
            Linux*)
                echo "🐧 Installing Linux dependencies..."
                sudo apt-get update
                sudo apt-get install -y libwebkit2gtk-4.0-dev \
                    build-essential \
                    curl \
                    wget \
                    libssl-dev \
                    libgtk-3-dev \
                    libayatana-appindicator3-dev \
                    librsvg2-dev
                ;;
            Darwin*)
                echo "🍎 macOS detected - no additional dependencies needed"
                ;;
            CYGWIN*|MINGW*|MSYS*)
                echo "🪟 Windows detected - please ensure Visual Studio Build Tools are installed"
                ;;
        esac
        
        echo "✅ Setup completed"
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [COMMAND] [PLATFORM]"
        echo ""
        echo "Commands:"
        echo "  dev       Start development mode"
        echo "  build     Build for production"
        echo "  clean     Clean build artifacts"
        echo "  test      Run tests"
        echo "  info      Show build information"
        echo "  setup     Setup development environment"
        echo "  help      Show this help message"
        echo ""
        echo "Platforms (for build command):"
        echo "  current   Build for current platform (default)"
        echo "  all       Build for all platforms"
        echo "  windows   Build for Windows"
        echo "  macos     Build for macOS"
        echo "  linux     Build for Linux"
        echo ""
        echo "Examples:"
        echo "  $0 dev              # Start development"
        echo "  $0 build            # Build for current platform"
        echo "  $0 build all        # Build for all platforms"
        echo "  $0 build windows    # Build for Windows only"
        echo "  $0 setup            # Setup development environment"
        ;;
    *)
        echo "❌ Unknown command: $COMMAND"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

echo "✅ Command completed successfully!"
