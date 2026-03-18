# Swing Music Desktop - CI/CD Setup

## 🚀 Automated Builds

This repository includes automated CI/CD pipelines for the Desktop application using GitHub Actions and Tauri.

### 📋 Workflows

#### 1. Desktop CI (`desktop-ci.yml`)
- **Triggers**: Push to main branch, Pull Requests, Manual dispatch
- **Jobs**: 
  - `test`: Runs Rust tests on Ubuntu
  - `build-linux`: Builds Linux x64 binaries
  - `build-windows`: Builds Windows x64 binaries  
  - `build-macos`: Builds macOS x64 binaries

#### 2. Desktop Release (`desktop-release.yml`)
- **Triggers**: Git tags (v*.*.*), Manual dispatch
- **Features**: Cross-platform builds and GitHub releases

#### 3. Build and Deploy (`build.yml`) - Legacy
- **Triggers**: Push to main, Tags, Manual dispatch
- **Features**: Full cross-platform builds with webclient integration

### 🔧 Build Process

1. **Environment Setup**
   - Rust stable toolchain with target-specific compilers
   - Tauri CLI for desktop app building
   - Platform-specific dependencies (GTK, WebKit, etc.)
   - Rust caching for faster builds

2. **Testing**
   - Rust unit tests with Cargo
   - Cross-platform compatibility checks

3. **Build**
   - **Linux**: AppImage, DEB, RPM packages
   - **Windows**: MSI installer
   - **macOS**: DMG disk image
   - All builds use Tauri's bundling system

4. **Release** (tags only)
   - Automatic GitHub release creation
   - Platform-specific installers attached
   - Version tagging and release notes

### 📱 Installing Desktop Apps

#### From GitHub Actions
1. Go to the **Actions** tab
2. Select the latest workflow run
3. Download artifacts from the **Artifacts** section
4. Extract and run the appropriate installer

#### From GitHub Releases
1. Go to the **Releases** tab
2. Download the latest release for your platform
3. Install using platform-specific instructions

### 🛠️ Local Development

#### Prerequisites
- Rust stable toolchain
- Node.js 20+
- Tauri CLI: `npm install -g @tauri-apps/cli`

#### Build Commands
```bash
# Clone the repository
git clone https://github.com/Dvorinka/swingmusic-extended-desktop.git
cd swingmusic-desktop

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for current platform
npm run tauri build

# Build for specific target
npm run tauri build -- --target x86_64-unknown-linux-gnu
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
```

#### Platform-specific Setup

##### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

##### Windows
- Install Microsoft Visual Studio C++ Build Tools
- Install Rust from rustup.rs

##### macOS
- Install Xcode Command Line Tools
- Install Rust from rustup.rs

### 📦 Build Artifacts

#### Linux
- **AppImage**: Portable executable
- **DEB**: Debian/Ubuntu package
- **RPM**: Red Hat/Fedora package

#### Windows
- **MSI**: Windows installer
- **EXE**: Portable executable (if enabled)

#### macOS
- **DMG**: Disk image with app bundle
- **APP**: Application bundle

### 🔄 Workflow Updates

To modify the CI/CD pipeline:
1. Edit `.github/workflows/desktop-ci.yml` or `.github/workflows/desktop-release.yml`
2. Test changes in a feature branch
3. Create a pull request for review

### 🎯 Release Process

#### Automated Releases (Tags)
```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0
```

#### Manual Releases
1. Go to **Actions** → **Desktop Release**
2. Click **Run workflow**
3. Enter version number (optional)
4. Download artifacts from created release

### 🐛 Troubleshooting

#### Build Failures
- Check the **Actions** tab for detailed logs
- Verify Rust toolchain and target installation
- Ensure platform-specific dependencies are installed

#### Runtime Issues
- Check system requirements for your platform
- Verify all dependencies are installed
- Check GitHub Issues for known problems

#### Artifact Issues
- Clear browser cache and retry download
- Verify file integrity after download
- Check release notes for platform-specific notes

### 📊 Build Status

- ✅ **CI builds**: Available on every push/PR
- ✅ **Multi-platform**: Linux, Windows, macOS
- ✅ **GitHub releases**: Created automatically on tags
- ✅ **Artifacts**: Stored for 30-90 days
- ✅ **Caching**: Rust and npm caches for faster builds

### 🔗 Integration

The desktop app integrates with:
- **SwingMusic Web Client**: Frontend UI components
- **SwingMusic Backend**: API and music services
- **System APIs**: Native notifications, file system, etc.

### 📞 Support

For issues related to:
- **CI/CD pipeline**: Create an issue in this repository
- **App functionality**: Check the main SwingMusic repository
- **Platform-specific issues**: Include system details in reports
- **Build problems**: Include logs and error details in issues
