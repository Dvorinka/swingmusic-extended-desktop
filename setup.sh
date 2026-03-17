#!/bin/bash

# SwingMusic Desktop Setup Script
# One-click setup for the desktop app development environment

set -e

echo "🎵 SwingMusic Desktop Setup"
echo "=========================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the swingmusic-desktop directory"
    exit 1
fi

print_status "Setting up SwingMusic Desktop development environment..."

# Check and install Rust
print_status "Checking Rust installation..."
if ! command -v cargo &> /dev/null; then
    print_status "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_success "Rust installed successfully"
else
    print_success "Rust is already installed"
fi

# Check and install Node.js
print_status "Checking Node.js installation..."
if ! command -v npm &> /dev/null; then
    print_status "Installing Node.js..."
    if command -v curl &> /dev/null; then
        curl -fsSL https://deb.nodesource.com/setup_19.x | sudo -E bash -
        sudo apt-get install -y nodejs
    elif command -v wget &> /dev/null; then
        wget -qO- https://deb.nodesource.com/setup_19.x | sudo -E bash -
        sudo apt-get install -y nodejs
    else
        print_error "Please install Node.js manually from https://nodejs.org/"
        exit 1
    fi
    print_success "Node.js installed successfully"
else
    print_success "Node.js is already installed"
fi

# Install Tauri CLI
print_status "Installing Tauri CLI..."
npm install -g @tauri-apps/cli
print_success "Tauri CLI installed successfully"

# Install Node.js dependencies
print_status "Installing Node.js dependencies..."
npm install
print_success "Node.js dependencies installed"

# Install Rust targets for cross-platform builds
print_status "Installing Rust targets for cross-platform builds..."
rustup target add x86_64-pc-windows-ms
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
print_success "Rust targets installed"

# Install platform-specific dependencies
print_status "Installing platform-specific dependencies..."
case "$(uname -s)" in
    Linux*)
        print_status "Installing Linux dependencies..."
        if command -v apt-get &> /dev/null; then
            sudo apt-get update
            sudo apt-get install -y \
                libwebkit2gtk-4.0-dev \
                build-essential \
                curl \
                wget \
                libssl-dev \
                libgtk-3-dev \
                libayatana-appindicator3-dev \
                librsvg2-dev
            print_success "Linux dependencies installed"
        else
            print_warning "Please install the following Linux dependencies manually:"
            echo "  - libwebkit2gtk-4.0-dev"
            echo "  - build-essential"
            echo "  - curl"
            echo "  - wget"
            echo "  - libssl-dev"
            echo "  - libgtk-3-dev"
            echo "  - libayatana-appindicator3-dev"
            echo "  - librsvg2-dev"
        fi
        ;;
    Darwin*)
        print_success "macOS detected - no additional dependencies needed"
        # Check for Xcode Command Line Tools
        if ! xcode-select -p &> /dev/null; then
            print_status "Installing Xcode Command Line Tools..."
            xcode-select --install
        fi
        ;;
    CYGWIN*|MINGW*|MSYS*)
        print_warning "Windows detected"
        print_warning "Please ensure Visual Studio Build Tools are installed with:"
        echo "  - C++ build tools"
        echo "  - Windows SDK"
        echo "  - CMake tools"
        ;;
esac

# Build frontend
print_status "Building frontend..."
if [ -d "../swingmusic-webclient" ]; then
    cd ../swingmusic-webclient
    npm install
    npm run build
    cd ../swingmusic-desktop
    print_success "Frontend built successfully"
else
    print_warning "Frontend directory not found. Please build the frontend manually."
fi

# Create icons directory if it doesn't exist
if [ ! -d "icons" ]; then
    mkdir -p icons
    print_status "Created icons directory"
    print_warning "Please add application icons to the icons directory:"
    echo "  - 32x32.png"
    echo "  - 128x128.png"
    echo "  - 128x128@2x.png"
    echo "  - icon.icns (macOS)"
    echo "  - icon.ico (Windows)"
    echo "  - tray-icon.png"
fi

# Test the setup
print_status "Testing the setup..."
if cargo check &> /dev/null; then
    print_success "Rust project compiles successfully"
else
    print_error "Rust project has compilation issues"
    exit 1
fi

if npm run tauri info &> /dev/null; then
    print_success "Tauri configuration is valid"
else
    print_warning "Tauri configuration may have issues"
fi

print_success "Setup completed successfully!"
echo ""
echo "🚀 Next steps:"
echo "  1. Add icons to the icons/ directory"
echo "  2. Run 'npm run dev' to start development"
echo "  3. Run './build.sh build' to create a production build"
echo "  4. Run './build.sh help' for more commands"
echo ""
echo "📚 For more information, see README.md"
