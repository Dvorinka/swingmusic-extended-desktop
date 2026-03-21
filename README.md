# 🖥️ SwingMusic Extended Desktop

A cross-platform desktop application for SwingMusic built with Tauri, providing native performance and deep system integration.

## ✨ Features

### Core Music Features
- **Universal Music Downloader** - Download from multiple streaming services
- **Advanced Audio Player** - High-quality playback with all formats
- **Smart Library Management** - Organize and browse your music collection
- **Real-time Sync** - Sync with web client and mobile apps
- **Analytics Dashboard** - Comprehensive listening statistics

### Desktop-Specific Features
- **Native Performance** - Optimized for desktop environments
- **System Tray Integration** - Background playback and controls
- **Global Hotkeys** - Media control from anywhere
- **File Associations** - Open music files with SwingMusic
- **Desktop Notifications** - Download completion and playback alerts
- **Auto-Start** - Launch on system startup
- **Native File System** - Direct access to music libraries

### User Interface
- **Modern Title Bar** - Custom window controls (Windows/Linux)
- **Collapsible Sidebar** - Space-efficient navigation
- **Mini Player** - Compact playback controls
- **Dark/Light Themes** - System-aware theming
- **Responsive Design** - Adapts to window size

## 🚀 Quick Start

### Prerequisites
- **Node.js** 19+ for frontend development
- **Rust** 1.70+ for Tauri development
- **Tauri CLI** for building and development

### Installation

```bash
# Clone repository
git clone https://github.com/Dvorinka/swingmusic-extended-desktop.git
cd swingmusic-extended-desktop

# Install dependencies
npm install

# Start development mode
npm run dev

# Build for production
npm run build
```

### Development Setup

```bash
# Install Tauri CLI globally
npm install -g @tauri-apps/cli

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install frontend dependencies
npm install

# Start development server
npm run dev
```

## 📁 Project Structure

```
swingmusic-desktop/
├── src/                          # Rust backend source
│   ├── main.rs                   # Main application entry point
│   ├── global_hotkeys.rs        # Global hotkey management
│   ├── native_features.rs        # Platform-specific features
│   ├── windows_integration.rs    # Windows-specific code
│   ├── macos_integration.rs      # macOS-specific code
│   └── linux_integration.rs      # Linux-specific code
├── desktop-components/           # Vue.js desktop components
│   ├── DesktopApp.vue           # Main desktop application
│   └── DesktopDownloadsView.vue # Desktop downloads interface
├── icons/                        # Application icons
├── Cargo.toml                   # Rust project configuration
├── tauri.conf.json             # Tauri configuration
├── package.json                 # Node.js dependencies
└── README.md                    # This file
```

## 🔧 Configuration

### Tauri Configuration (`tauri.conf.json`)
- **Window Settings**: Size, decorations, behavior
- **Security**: File system access, API permissions
- **Bundle**: Icons, metadata, platform-specific settings
- **System Tray**: Icon and menu configuration

### Build Configuration
- **Frontend**: Points to web client dist folder
- **Development**: Uses Vite dev server
- **Icons**: Multiple sizes for different platforms

## 🌐 Platform Support

### Windows
- ✅ Windows 10/11
- ✅ Custom title bar
- ✅ File associations
- ✅ Auto-start
- ✅ System notifications
- ✅ Windows Media Center integration

### macOS
- ✅ macOS 10.15+
- ✅ Native title bar
- ✅ Touch Bar support (planned)
- ✅ Quick Look integration
- ✅ macOS notifications
- ✅ Menu bar application

### Linux
- ✅ Ubuntu 20.04+
- ✅ Custom title bar
- ✅ MPRIS integration
- ✅ Desktop notifications
- ✅ File associations
- ✅ Auto-start support

## 🔌 API Integration

### Tauri Commands
```typescript
// Download management
invoke('download_universal_url', { request })
invoke('get_download_history')
invoke('pause_download', { downloadId })

// Settings
invoke('get_app_settings')
invoke('update_app_settings', { settings })

// Native features
invoke('show_notification', { title, body })
invoke('open_in_file_manager', { filePath })
invoke('set_auto_start', { enabled })
```

### Event Listeners
```typescript
// Download progress
listen('download-progress', (event) => {
  console.log('Progress:', event.payload)
})

// System tray events
listen('system-tray-event', (event) => {
  handleTrayEvent(event.payload)
})

// Media controls
listen('play_pause', () => togglePlayPause())
```

## 🎨 Desktop Components

### DesktopApp.vue
Main application component with:
- Custom title bar
- Collapsible sidebar
- Mini player
- System integration

### DesktopDownloadsView.vue
Desktop-optimized downloads interface with:
- Universal URL input
- Real-time progress tracking
- File system integration
- Native notifications

## 🔒 Security

### File System Access
- **Scoped Access**: Limited to music directories
- **User Confirmation**: File operations require user approval
- **Sandboxing**: Isolated from system files

### API Permissions
- **Minimal Scope**: Only necessary APIs enabled
- **User Control**: Settings control feature access
- **Audit Trail**: All actions logged

## 🚀 Performance

### Optimizations
- **Native Rendering**: GPU-accelerated graphics
- **Memory Management**: Efficient resource usage
- **Background Processing**: Non-blocking operations
- **Caching**: Intelligent data caching

### Benchmarks
- **Startup Time**: < 2 seconds
- **Memory Usage**: < 200MB idle
- **CPU Usage**: < 5% idle
- **File Operations**: Native filesystem speed

## 🐛 Troubleshooting

### Common Issues

#### Build Errors
```bash
# Clear build cache
cargo clean
npm run build

# Update dependencies
npm update
cargo update

# Reinstall Tauri CLI
npm uninstall -g @tauri-apps/cli
npm install -g @tauri-apps/cli
```

#### Development Issues
```bash
# Check frontend build
npm run build

# Verify Tauri setup
npm run tauri info

# Reset development environment
rm -rf src-tauri/target
npm run dev
```

#### Platform-Specific Issues
- **Windows**: Ensure Visual Studio Build Tools are installed
- **macOS**: Install Xcode Command Line Tools
- **Linux**: Install required system dependencies

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug npm run dev

# Build with debug symbols
npm run build:debug

# Inspect with developer tools
npm run dev -- --devtools
```

## 🛠️ Development Commands

### Development
```bash
npm run dev          # Start development mode
npm run tauri dev     # Alternative dev command
```

### Building

**⚠️ IMPORTANT: Build Order**

The desktop app requires the web client to be built first. The build process is:

```bash
# Step 1: Build the web client (required before Tauri build)
cd ../swingmusic-webclient
npm install
npm run build

# Step 2: Build the desktop app
cd ../swingmusic-desktop
npm install
npm run build
```

The `tauri.conf.json` is configured to automatically build the web client via `beforeBuildCommand`, but ensure dependencies are installed first.

### Build Commands
```bash
npm run build         # Build for production (runs web build first)
npm run build:debug   # Build with debug symbols
npm run build:release # Build optimized release
```

### Tauri Commands
```bash
npm run tauri build   # Build using Tauri CLI
npm run tauri dev      # Development mode
npm run tauri info     # Show build information
```

## 🧪 Testing

```bash
# Run Rust tests
cargo test

# Test frontend
npm run test

# Integration tests
npm run test:e2e
```

## 🤝 Contributing

1. Fork repository
2. Create feature branch
3. Make changes
4. Test on all platforms
5. Submit pull request

### Code Style
- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **TypeScript**: Follow Vue.js style guide
- **Vue.js**: Use Composition API with `<script setup>`

## 📦 Distribution

### Code Signing (REQUIRED for Production)

**⚠️ Without code signing, users will receive security warnings on Windows and macOS.**

#### Windows Code Signing

1. **Obtain a Code Signing Certificate**:
   - EV Certificate (recommended): Provides immediate trust, requires hardware token
   - Standard Certificate: Provides trust after reputation is built
   - Providers: DigiCert, Sectigo, GlobalSign

2. **Configure in `tauri.conf.json`**:
   ```json
   "windows": {
     "certificateThumbprint": "YOUR_CERTIFICATE_THUMBPRINT",
     "digestAlgorithm": "sha256",
     "timestampUrl": "http://timestamp.digicert.com"
   }
   ```

3. **Build signed Windows installer**:
   ```bash
   # Certificate must be installed in Windows Certificate Store
   npm run build:windows
   ```

#### macOS Code Signing

1. **Obtain Apple Developer Certificate**:
   - Enroll in Apple Developer Program ($99/year)
   - Create "Developer ID Application" certificate in Xcode

2. **Configure in `tauri.conf.json`**:
   ```json
   "macOS": {
     "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
     "providerShortName": "TEAM_ID",
     "entitlements": "entitlements.plist"
   }
   ```

3. **Create entitlements.plist** (for audio/network access):
   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
   <plist version="1.0">
   <dict>
       <key>com.apple.security.network.client</key>
       <true/>
       <key>com.apple.security.network.server</key>
       <true/>
       <key>com.apple.security.files.user-selected.read-write</key>
       <true/>
   </dict>
   </plist>
   ```

4. **Build signed macOS app**:
   ```bash
   # Requires Xcode and valid certificate
   npm run build:macos
   ```

5. **Notarize (required for distribution outside App Store)**:
   ```bash
   xcrun notarytool submit target/release/bundle/macos/SwingMusic.app \
     --apple-id "your@email.com" \
     --password "app-specific-password" \
     --team-id "TEAM_ID" \
     --wait
   ```

### Build Packages
```bash
# Build for all platforms
npm run build:all

# Platform-specific builds
npm run build:windows
npm run build:macos
npm run build:linux
```

### Release Process
1. Update version in `package.json` and `Cargo.toml`
2. Update changelog
3. Build signed release packages (see Code Signing section)
4. Test on all platforms
5. Create GitHub release with binaries
6. Update endpoint URL in `tauri.conf.json` for auto-updates

### Auto-Updates

The desktop app includes built-in auto-update functionality:

- **Update Endpoint**: Configured in `tauri.conf.json` under `plugins.updater.endpoints`
- **Update Check**: Automatic on startup, manual via Settings
- **Installation**: Downloads and installs in background, prompts to restart

To enable auto-updates for your releases:

1. **Generate signing key** (for update verification):
   ```bash
   npm run tauri signer generate
   ```

2. **Add public key to `tauri.conf.json`**:
   ```json
   "updater": {
     "pubkey": "YOUR_PUBLIC_KEY_HERE"
   }
   ```

3. **Sign your releases**:
   ```bash
   # Set private key as environment variable
   export TAURI_SIGNING_PRIVATE_KEY="your-private-key"
   npm run build
   ```

4. **Host update manifest** at your endpoint URL with format:
   ```json
   {
     "version": "1.0.1",
     "notes": "Bug fixes and improvements",
     "pub_date": "2024-01-15T12:00:00Z",
     "platforms": {
       "darwin-x86_64": {
         "signature": "base64-signature",
         "url": "https://github.com/.../SwingMusic_1.0.1_x64.app.tar.gz"
       },
       "windows-x86_64": {
         "signature": "base64-signature",
         "url": "https://github.com/.../SwingMusic_1.0.1_x64-setup.exe"
       }
     }
   }
   ```

## 📄 License

This project is licensed under AGPL-3.0 License - see the [LICENSE](../LICENSE) file for details.

## 🔗 Links

- **Backend**: [swingmusic-extended](https://github.com/Dvorinka/swingmusic-extended)
- **Web Client**: [swingmusic-extended-webclient](https://github.com/Dvorinka/swingmusic-extended-webclient)
- **Android App**: [swingmusic-extended-android](https://github.com/Dvorinka/swingmusic-extended-android)

---

**Built with ❤️ using Tauri and Vue.js**
