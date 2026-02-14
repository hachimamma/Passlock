# PassLock

**Secure, local-first password manager with a beautiful TUI, adaptive encryption, and zero cloud dependencies.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Security](https://img.shields.io/badge/encryption-AES--256--GCM%20%7C%20ChaCha20-green.svg)]()

---

## Features

- **Military-grade encryption** - AES-256-GCM or ChaCha20-Poly1305 (auto-selected)
- **Beautiful TUI** - Intuitive terminal interface with Gruvbox theme
- **Blazing fast** - Written in Rust with C crypto core
- **Local-only** - No cloud, no telemetry, no BS
- **Password generator** - Strong, unique passwords every time
- **Password strength meter** - Real-time feedback
- **Tags & organization** - Categorize your passwords
- **Password history** - Track changes, restore old passwords
- **Fast search** - Find passwords instantly
- **CPU-aware** - Uses hardware acceleration when available
- **Cross-platform** - Linux, macOS, Windows (coming soon)

---

## Why PassLock?

| Feature | LastPass | 1Password | Bitwarden | **PassLock** |
|---------|----------|-----------|-----------|--------------|
| **Price** | $3/mo | $3/mo | $1/mo | **FREE** ✅ |
| **Open Source** | ❌ | ❌ | ✅ | ✅ |
| **Local-only** | ❌ | ❌ | ❌ | **✅** |
| **No cloud** | ❌ | ❌ | ❌ | **✅** |
| **No telemetry** | ❌ | ❌ | ⚠️ | **✅** |
| **CLI + TUI** | ❌ | ❌ | ⚠️ | **✅** |
| **Adaptive encryption** | ❌ | ❌ | ❌ | **✅** |

---

## Quick Start

### Installation

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install build-essential libsodium-dev

# Clone and build
git clone https://github.com/hachimamma/Passlock
cd passlock
make install
```

### First Use

```bash
# Create your vault
passlock create mySecurePassword123

# Launch TUI
passlock tui
```

**That's it!**

---

## Screenshots

### Main Menu
```
┌─────────────────────────────────────────┐
│  PassLock v2.0.0                     │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                          │
│  View Passwords                       │
│  Add Password                         │
│  Edit Password                       │
│  Delete Password                     │
│  View History                         │
│  Search Passwords                     │
│  Generate Password                    │
│  Filter by Tags                      │
│  Lock Vault                           │
│  Exit                                  │
│                                          │
│  Use ↑↓ or j/k to navigate              │
│  Press Enter to select, q to quit       │
└─────────────────────────────────────────┘
```

### Password List (Gruvbox Theme)
```
 ┌─ Passwords (5 entries) ─────────────────┐
 │ Search: _                             │
 ├──────────────────────────────────────────┤
 │ ▶ GitHub (hachimamma)                    │
 │     work, dev                          │
 │                                           │
 │   Gmail (personal@gmail.com)             │
 │     personal                            │
 │                                           │
 │   AWS Console (admin)                    │
 │     work, cloud                         │
 │                                           │
 │   Bank Account (customer123)             │
 │     banking                             │
 │                                           │
 │   Reddit (user2024)                      │
 │     personal, social                    │
 └──────────────────────────────────────────┘
   ↑↓:Navigate  Enter:View  /:Search  q:Back
```

---

## Security

### Encryption

PassLock uses **adaptive encryption** based on your CPU:

- **Modern CPUs** (with AES-NI): **AES-256-GCM** - Hardware accelerated, 3-5 GB/s
- **Older CPUs** (no AES-NI): **ChaCha20-Poly1305** - 6x faster than software AES

**Both are equally secure!** (256-bit security, used by Signal, WireGuard, TLS 1.3)

Check what you're using:
```bash
passlock info cpu
```

### Key Derivation

- **Algorithm:** Argon2id (winner of Password Hashing Competition)
- **Memory:** 64 MB (resistant to GPU/ASIC attacks)
- **Iterations:** Auto-tuned for ~100ms unlock time
- **Salt:** 16 bytes, unique per vault

### Authentication

- **AEAD:** Authenticated Encryption with Associated Data
- **Tag:** 128-bit Poly1305 MAC
- **Tampering protection:** Any modification = decryption fails

---

## Documentation

- **[Command Reference](Commands.md)** - Complete guide to all commands
- **[Security Details](SECURITY.md)** - In-depth security analysis (coming soon)
- **[Architecture](ARCHITECTURE.md)** - How PassLock works (coming soon)
- **[Contributing](CONTRIBUTING.md)** - Help improve PassLock!

---

## Use Cases

### Personal Use
- Manage all your passwords securely
- Generate strong, unique passwords
- Keep everything local (no cloud sync)
- Free and open source

### Developer/SysAdmin
- Store SSH keys, API tokens, database credentials
- CLI-friendly workflow
- Git-syncable vault file
- Fast search and filtering

### Small Teams
- Share vault file via Git/Syncthing
- No subscription fees
- Full control over data
- Audit trail (password history)

### Privacy-Conscious Users
- Zero telemetry
- No phone-home
- No account required
- No cloud storage

---

## ⚡ Performance

```
Encryption Speed (10 MB file):

CPU                     Cipher              Speed
────────────────────────────────────────────────────
Intel i7 (AES-NI)       AES-256-GCM        0.002s
Intel i7 (AES-NI)       ChaCha20-Poly1305  0.010s
Intel Celeron (no AES)  AES-256-GCM        0.200s ❌
Intel Celeron (no AES)  ChaCha20-Poly1305  0.033s ✅

PassLock automatically picks the fastest option!
```

---

## 🛠️ Building from Source

### Prerequisites

**Ubuntu/Debian:**
```bash
sudo apt install build-essential libsodium-dev
```

**Fedora:**
```bash
sudo dnf install gcc libsodium-devel
```

**macOS:**
```bash
brew install libsodium
```

**Arch:**
```bash
sudo pacman -S base-devel libsodium
```

### Build

```bash
# Clone
git clone https://github.com/hachimamma/Passlock
cd passlock

# Build release version
cargo build --release

# Install (optional)
sudo cp target/release/passlock /usr/local/bin/
```

### Development

```bash
# Run tests
make test

# Run with debug logging
RUST_LOG=debug cargo run -- tui

# Check code quality
make lint

# Format code
make format
```

---

## Roadmap

### Version 2.0 (Current)
- [x] TUI interface with Gruvbox theme
- [x] Adaptive encryption (AES/ChaCha20)
- [x] Password generator
- [x] Password history
- [x] Tags and categories
- [x] Search and filter
- [x] CPU feature detection

### Version 2.1 (In Progress)
- [ ] Browser extension (Chrome/Firefox)
- [ ] Import from LastPass/1Password/Bitwarden
- [ ] Export to various formats
- [ ] Breach checker (HaveIBeenPwned integration)
- [ ] TOTP/2FA support

### Version 3.0 (Planned)
- [ ] Mobile app (iOS/Android)
- [ ] Secure notes & files
- [ ] Team/family sharing
- [ ] Emergency access
- [ ] Hardware key support (YubiKey)
- [ ] Passkey/WebAuthn support

---

## Contributing

We love contributions! Here's how you can help:

1. **Report bugs** - [Open an issue](https://github.com/hachimamma/Passlock/issues)
2. **Suggest features** - Tell us what you need!
3. **Submit PRs** - Fix bugs, add features
4. **Improve docs** - Help others understand PassLock
5. **Spread the word** - Star the repo, tell friends!

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## Acknowledgments

- **libsodium** - Robust crypto library
- **ratatui** - Beautiful TUI framework
- **Gruvbox** - Amazing color scheme
- **Rust** - Memory-safe systems programming
- **Community contributors** - Thank you!

Special thanks to:
- [@rokybeast] - ChaCha20-Poly1305 optimization for CPUs without AES-NI

---

## License

MIT License - see [LICENSE](LICENSE) file

**TL;DR:** Free to use, modify, distribute. No warranty.

---

## Links

- **Homepage:** https://passlock.dev (coming soon)
- **Documentation:** https://docs.passlock.dev (coming soon)
- **Issues:** https://github.com/hachimamma/Passlock/issues
- **Discussions:** https://github.com/hachimamma/Passlock/discussions

---

## Support

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - General questions and ideas
- **Email** - your.email@example.com

---

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=hachimamma/Passlock&type=Date)](https://star-history.com/#hachimamma/Passlock&Date)

---

**Made with ❤️ by the PassLock community**

*Secure your digital life. Own your data. Stay free.*