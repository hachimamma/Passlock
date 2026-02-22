# Changelog

All notable changes to PassLock will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.3.5] - 2026-02-22

### Added
- **Configurable Clipboard Timeout** - Users can now customize clipboard auto-clear duration
  - Available options: 10s, 30s, 60s, 120s, 300s, Never (disabled)
  - Accessible from Settings screen (Esc → Options → Settings)
  - Default: 30 seconds

### Fixed
- **Right-Click Context Menu** - Complete rewrite of clickable area detection
  - Now dynamically calculates clickable regions based on actual entry content
  - Properly handles entries with varying fields (URL, 2FA, History)
  - Maintains proper spacing regardless of entry length
  - Fixed issue where clickable areas were misaligned for entries with different field counts
  - Context menu now accurately responds to mouse clicks on any part of an entry

### Changed
- Right-click detection now uses dynamic row mapping instead of hardcoded calculations
- Settings screen now displays current clipboard timeout value

---

## [2.3.4] - 2026-02-21

### Added
- **TOTP/2FA Support** - Time-based One-Time Password generation
  - Base32 secret key support
  - Automatic 6-digit code generation every 30 seconds
  - Live countdown timer showing seconds remaining
  - Invalid key detection and error display
  - Uses `SystemTime` for accurate time-based generation
  - Compatible with Google Authenticator, Authy, and other TOTP apps
  - Accessible from Add/Edit/View screens

- **Right-Click Context Menu** - Mouse-driven password management
  - 5 quick actions: Copy Password, Copy Username, Copy URL, Edit Entry, View History
  - Full mouse integration with right-click detection
  - Keyboard navigation support (↑/↓ arrows, Enter, Esc)
  - URL option grayed out when not available
  - Mouse scrolling support in password list
  - Clickable area detection for each entry

### Fixed
- TOTP field integration in add/edit forms
- Tab navigation now includes 2FA secret field
- Context menu positioning and boundary detection

### Technical
- Added dependencies: `totp-lite` (v2.0), `base32` (v0.4)
- Created `src/totp.rs` module for TOTP functionality
- Enhanced `Entry` model with `totp_secret` field
- Implemented dynamic clickable regions for context menu

---

## [2.2.3] - 2026-02-16

### Added
- Options menu with ASCII art
- 8 beautiful themes (Gruvbox, Dracula, Nord, Tokyo Night, Catppuccin, Solarized, One Dark, Cyberpunk)
- Settings screen with theme selector
- Comprehensive help screen with all keybinds
- Shift+Tab to navigate backwards through fields
- Ctrl+S to save from any field in add/edit screens

### Changed
- Removed redundant help text from main menu (now in Help screen)
- Esc in main menu now opens Options menu
- Exit moved to '7' key, theme selector on 'T' or '8'

### Fixed
- Terminal cleanup issues when pressing Esc
- Ctrl+S now properly saves instead of typing 's'

---

## [2.0.0] - 2026-02-15

### Major Release - Complete Rewrite

PassLock v2.0.0 is a complete rewrite with massive improvements in security, performance, and usability!

### Added

#### Core Features
- **Adaptive Encryption** - Automatically selects AES-256-GCM (with AES-NI) or ChaCha20-Poly1305 (without)
- **CPU Feature Detection** - Detects AES-NI support for optimal performance
- **Beautiful TUI** - Complete terminal user interface with Gruvbox theme
- **Password History** - Track up to 5 previous passwords per entry
- **Tags & Categories** - Organize passwords with custom tags
- **Advanced Search** - Fast search across names, usernames, URLs, and tags
- **Filter by Tags** - View passwords by category
- **Password Generator** - Generate strong, random passwords (4-64 characters)
- **Password Strength Meter** - Real-time feedback on password quality
- **Go API Server** - RESTful API for web frontend
- **HTML Frontend** - Modern web interface (development)

#### Security Improvements
- **Argon2id Key Derivation** - GPU/ASIC-resistant password hashing
- **Authenticated Encryption** - AEAD with Poly1305 MAC prevents tampering
- **Secure Memory Zeroing** - Sensitive data cleared after use
- **Vault File Versioning** - Support for multiple encryption formats
- **Auto-lock on Exit** - TUI properly cleans up temporary files

#### Developer Experience
- **Modular UI Architecture** - 10 separate UI modules for maintainability
- **Comprehensive Documentation** - README, COMMANDS, CONTRIBUTING, SECURITY
- **Professional Makefile** - Easy build, test, install, and release
- **Zero Clippy Warnings** - Clean, idiomatic Rust code
- **GitHub Actions Ready** - CI/CD workflows prepared

#### Commands
- `passlock info cpu` - Show CPU features and recommended cipher
- `passlock version` - Show version information
- `passlock help` - Improved help text
- `passlock tui` - Launch TUI interface

### Changed

- **Encryption** - Migrated from AES-256-GCM-only to adaptive encryption
- **UI Framework** - Switched to ratatui with custom Gruvbox theme
- **Vault Format** - New format with cipher metadata (backward compatible)
- **C Crypto Core** - Optimized vault_engine.c for better performance
- **Build System** - Enhanced Makefile with color output and better targets
- **Exit Handling** - Fixed TUI cleanup (no more unicode mess on exit!)

### Fixed

- Terminal not properly cleaned up on exit
- ESC key causing terminal corruption
- Password visibility in edit mode
- Vault locking issues
- Memory leaks in crypto operations
- Compiler warnings and clippy issues

### Performance

- **6x faster** encryption on CPUs without AES-NI (ChaCha20-Poly1305)
- **3-5 GB/s** encryption speed on modern CPUs with AES-NI
- Optimized vault loading and saving
- Reduced memory allocations in hot paths

### Documentation

- Complete command reference (COMMANDS.md)
- Security documentation (SECURITY.md)
- Contributing guidelines (CONTRIBUTING.md)
- Updated README with features comparison
- Installation instructions for multiple distros
- Troubleshooting guide

### Infrastructure

- Professional Makefile with 20+ targets
- Support for both system-wide and local installation
- Go server build integration
- Dependency checking
- Code formatting and linting
- Size optimization

### Contributors

Special thanks to:
- Community members who reported issues
- RokyBeast who submitted PRs for ChaCha20-Poly1305 support
- Early testers who provided feedback

---

## [1.0.0] - 2025-XX-XX

### Initial Release

- Basic password management
- AES-256-GCM encryption
- CLI interface
- Vault creation and unlocking
- Password CRUD operations

---

## [Unreleased]

### Planned for v2.4.0
- Attachment support for credentials
- Secure notes functionality
- Export vault to encrypted formats
- Import from other password managers

### Planned for v3.0.0
- Browser extension (Chrome/Firefox)
- Mobile apps (iOS/Android)
- Team/family sharing
- Emergency access
- Hardware key support (YubiKey)
- Passkey/WebAuthn support
- HaveIBeenPwned breach checker

---

[2.3.5]: https://github.com/hachimamma/Passlock/releases/tag/v2.3.5
[2.3.4]: https://github.com/hachimamma/Passlock/releases/tag/v2.3.4
[2.2.3]: https://github.com/hachimamma/Passlock/releases/tag/v2.2.3
[2.0.0]: https://github.com/hachimamma/Passlock/releases/tag/v2.0.0
[1.0.0]: https://github.com/hachimamma/Passlock/releases/tag/v1.0.0