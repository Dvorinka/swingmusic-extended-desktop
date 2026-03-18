# Commit Message Guidelines

## Semantic Commit Messages for SwingMusic

This project uses semantic versioning based on commit messages. Follow these guidelines to ensure proper version bumps:

## Format
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

## Types

### 🚨 Major Version (Breaking Changes)
- `feat!:` or `feat!: breaking change`
- `BREAKING CHANGE:`
- `major:`
- `breaking:`

**Examples:**
```
feat!: remove deprecated API
feat!: change authentication flow
BREAKING CHANGE: migrate to new configuration format
```

### ✨ Minor Version (New Features)
- `feat:` or `feature:`
- `add:` or `new:`
- `enhance:`

**Examples:**
```
feat: add playlist sharing feature
feat(desktop): add system tray support
add: support for Apple Silicon
enhance: improve search functionality
```

### 🐛 Patch Version (Bug Fixes & Improvements)
- `fix:` or `bug:`
- `patch:`
- `update:`, `improve:`, `refactor:`
- `docs:`, `style:`, `test:`, `chore:`

**Examples:**
```
fix: resolve crash on startup
fix(ui): fix button alignment issue
patch: update dependencies
improve: optimize database queries
docs: update installation guide
test: add integration tests
chore: update build configuration
```

## Examples

### Good Commit Messages
```
feat: add cross-platform desktop builds

- Configure Linux x86_64 builds (DEB, RPM, AppImage)
- Configure Windows x86_64 cross-compilation builds
- Configure macOS builds (x86_64, ARM64) via CI/CD
- Add Linux ARM64 builds via GitHub Actions

Fixes #123

fix: resolve icon format issues for Windows builds

Convert icons to proper RGBA format to fix Tauri build errors
on Windows platform. Update icon generation script for all
platforms.

BREAKING CHANGE: remove legacy configuration format

The old configuration format is no longer supported. Users must
migrate to the new YAML-based configuration.
```

### Bad Commit Messages
```
fixed stuff
update
wip
fix bugs
add thing
```

## How Version Bumping Works

1. **Major**: Any breaking change → `1.0.0` → `2.0.0`
2. **Minor**: Any new feature → `1.0.0` → `1.1.0`
3. **Patch**: Bug fixes, improvements → `1.0.0` → `1.0.1`

## Tips

- Use the imperative mood ("add feature" not "added feature")
- Keep descriptions short and clear
- Include scope when relevant (feat(ui):, feat(desktop):, etc.)
- Use body for detailed explanations
- Reference issues when applicable

## Automatic Detection

The CI/CD system automatically:
1. Analyzes commits since last release
2. Counts each type of change
3. Determines the appropriate version bump
4. Generates release notes based on commit types
5. Creates a new release with the calculated version

**Example:**
- Commits: 2 `feat:`, 3 `fix:`, 1 `docs:`
- Result: `v1.1.0` (minor bump due to features)
