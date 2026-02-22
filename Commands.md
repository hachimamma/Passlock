# PassLock Command Reference

Complete guide to all PassLock commands and features.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Vault Management](#vault-management)
- [Password Operations](#password-operations)
- [System Information](#system-information)
- [TUI Mode](#tui-mode)
- [Advanced Usage](#advanced-usage)

---

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/hachimamma/Passlock
cd passlock

# Build and install
make install

# Or build manually
cargo build --release
sudo cp target/release/passlock /usr/local/bin/
```

### Quick Start

```bash
# 1. Create a vault
passlock create mySecurePassword123

# 2. Launch TUI
passlock tui

# 3. Start managing passwords!
```

---

## Vault Management

### Create a New Vault

```bash
passlock create <password>
```

Creates an encrypted vault at `~/.passlock.vault`.

**Example:**
```bash
passlock create mySecurePassword123
```

**Notes:**
- Use a strong master password (12+ characters)
- Master password is NEVER stored
- Vault is encrypted with AES-256-GCM or ChaCha20-Poly1305 (auto-selected)

---

### Unlock Vault (Verify Password)

```bash
passlock unlock <password>
```

Verifies your master password and shows number of entries.

**Example:**
```bash
passlock unlock mySecurePassword123
# Output: Vault unlocked! 5 entries found
```

---

### Sync Vault

```bash
passlock sync <password>
```

Re-encrypts and saves the vault (useful after manual edits).

**Example:**
```bash
passlock sync mySecurePassword123
```

---

## Password Operations

All password operations are done through the **TUI interface**.

### Launch TUI

```bash
passlock tui
```

**TUI Features:**
- View all passwords
- Add new passwords
- Edit existing passwords
- Delete passwords
- Search/filter
- Generate strong passwords
- Password strength checker
- Password history tracking
- Tags and categories
- Copy to clipboard (auto-clear after 30s at default)

### TUI Navigation

**Main Menu:**
- `↑/↓` or `j/k` - Navigate
- `Enter` - Select
- `q` - Quit

**Password List:**
- `↑/↓` or `j/k` - Scroll
- `Enter` - View details
- `/` - Search
- `n` - Add new password
- `e` - Edit selected
- `d` - Delete selected
- `f` - Filter by tags
- `q` - Back to menu
- `Right-click` - Context menu with quick actions
- `Scroll wheel` - Scroll through passwords

**Creating/Editing:**
- `Tab` - Next field
- `Shift+Tab` - Previous field
- `Ctrl+G` - Generate password
- `Enter` - Save
- `Esc` - Cancel

**Clipboard Settings**

Configure clipboard auto-clear timeout:
- Access: Esc → Options → Settings → Clipboard Timeout
- Options: 10s, 30s, 60s, 120s, 300s, Never
- Default: 30 seconds

---

## System Information

### Check CPU Features

```bash
passlock info cpu
```

Shows:
- CPU capabilities (AES-NI support)
- Recommended cipher for your system
- Vault status and size
- Version information

**Example Output:**
```
PassLock System Information
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

CPU Features:
  AES-NI: Supported
     (Hardware-accelerated AES encryption available)

Recommended Cipher:
  AES-256-GCM (hardware accelerated)

Vault Status:
  Vault exists at: ~/.passlock.vault
  Size: 2 KB

Version: PassLock v2.3.5
```

---

### Version

```bash
passlock version
```

Shows current PassLock version.

---

### Help

```bash
passlock help
```

Shows quick command reference.

---

## TUI Mode

### Features

#### 1. **View Passwords**
- Browse all saved passwords
- View details (username, URL, notes, tags)
- Copy password to clipboard (Ctrl+C)
- View password history

#### 2. **Add Password**
- Manual entry
- Generate strong password
- Add tags for organization
- Add notes/URLs

#### 3. **Edit Password**
- Modify any field
- Update password (old password saved to history)
- Change tags

#### 4. **Delete Password**
- Confirmation required
- Permanently removes entry

#### 5. **Search**
- Press `/` to search
- Searches name, username, URL, tags
- Real-time filtering

#### 6. **Filter by Tags**
- Press `f` to filter
- Shows only passwords with selected tags
- Useful for organizing (work, personal, banking, etc.)

#### 7. **Password Generator**
- Press `Ctrl+G` in password field
- Configurable length
- Includes: uppercase, lowercase, numbers, symbols
- Shows strength indicator

#### 8. **Password History**
- View previous passwords
- Restore old password if needed
- Tracks when password was changed

#### 9. **TOTP/2FA Codes**
- View live 2FA codes in password list
- Add 2FA secrets in add/edit screens
- 6-digit codes refresh every 30 seconds
- Compatible with Google Authenticator, Authy, etc.

#### 10. **Right-Click Context Menu**
- Right-click any password entry
- Quick actions: Copy Password, Copy Username, Copy URL, Edit, View History
- Keyboard navigation with ↑↓ arrows and Enter
- Mouse scroll support

---

## Advanced Usage

### Encryption Details

**PassLock uses adaptive encryption:**

| CPU Type | Cipher Used | Performance |
|----------|-------------|-------------|
| **Modern Intel/AMD** (with AES-NI) | AES-256-GCM | ⚡ 3-5 GB/s |
| **Older CPUs** (no AES-NI) | ChaCha20-Poly1305 | ⚡ 300 MB/s |

**Both are equally secure!** The cipher is automatically selected based on your CPU.

**Key Derivation:**
- Algorithm: Argon2id
- Memory: 64 MB
- Iterations: Tuned for ~100ms unlock time
- Salt: 16 bytes (unique per vault)

**Authentication:**
- Authenticated encryption (AEAD)
- Prevents tampering
- Wrong password = instant detection

---

### Vault File Format

Located at: `~/.passlock.vault`

**Structure:**
```
[1 byte: cipher type]
[12 bytes: nonce]
[N bytes: encrypted data]
[16 bytes: authentication tag]
```

**Cipher Types:**
- `1` = AES-256-GCM
- `2` = ChaCha20-Poly1305

**Encrypted Data Contains:**
- All password entries (JSON)
- Metadata (tags, history, timestamps)

---

### Backup Your Vault

```bash
# Backup vault file
cp ~/.passlock.vault ~/passlock-backup-$(date +%Y%m%d).vault

# Restore from backup
cp ~/passlock-backup-20240215.vault ~/.passlock.vault
```

**Important:** Keep backups in a secure location!

---

### Security Best Practices

1. **Strong Master Password**
   - Use 12+ characters
   - Mix uppercase, lowercase, numbers, symbols
   - Don't reuse from other services
   - Consider using a passphrase: "correct-horse-battery-staple"

2. **Regular Backups**
   - Backup vault file weekly
   - Store in encrypted location
   - Test restoring from backup

3. **Keep Software Updated**
   - Run `git pull` regularly
   - Check for new releases
   - Update dependencies: `cargo update`

4. **Physical Security**
   - Lock your computer when away
   - Enable full disk encryption
   - Don't share your master password

5. **Password Hygiene**
   - Generate unique passwords for each site
   - Use 16+ character passwords
   - Enable 2FA where available
   - Change passwords if compromised

---

## Troubleshooting

### "Vault not found"

Create a vault first:
```bash
passlock create yourPassword123
```

### "Wrong password"

- Check caps lock
- Try typing carefully
- No password recovery (by design!)

### "Permission denied"

Vault file permissions issue:
```bash
chmod 600 ~/.passlock.vault
```

### Slow encryption (old CPU)

Check which cipher is being used:
```bash
passlock info cpu
```

If you see "AES-NI: Not supported", PassLock will automatically use ChaCha20 (faster on your CPU).

### Build errors

Install dependencies:
```bash
# Ubuntu/Debian
sudo apt install build-essential libsodium-dev

# Fedora
sudo dnf install gcc libsodium-devel

# Arch
sudo pacman -S base-devel libsodium
```

---

## Environment Variables

### RUST_LOG

Enable debug logging:
```bash
RUST_LOG=debug passlock tui
```

Levels: `error`, `warn`, `info`, `debug`, `trace`

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Wrong password |
| `3` | Vault not found |

---

## Examples

### Daily Workflow

```bash
# Morning: Launch TUI
passlock tui

# Add new password (in TUI)
# Press 'a' -> Fill in details -> Save

# Search for password (in TUI)
# Press '/' -> Type "github" -> Enter

# Copy password
# Select entry -> Press 'Enter' -> Copy password shown
```

### Backup Script

```bash
#!/bin/bash
# backup-passlock.sh

DATE=$(date +%Y%m%d)
BACKUP_DIR=~/Backups/passlock
mkdir -p "$BACKUP_DIR"

cp ~/.passlock.vault "$BACKUP_DIR/passlock-$DATE.vault"
echo "Backup created: $BACKUP_DIR/passlock-$DATE.vault"

# Keep only last 7 backups
cd "$BACKUP_DIR"
ls -t passlock-*.vault | tail -n +8 | xargs rm -f
```

### Migration from Other Managers

Coming soon: Import from LastPass, 1Password, Bitworm, Chrome, etc.

---

## FAQ

**Q: Is PassLock secure?**
A: Yes! Uses industry-standard encryption (AES-256-GCM/ChaCha20-Poly1305 + Argon2id).

**Q: Where is my data stored?**
A: Locally at `~/.passlock.vault`. No cloud, no tracking.

**Q: What if I forget my master password?**
A: No recovery possible (by design). Keep backups and remember your password!

**Q: Can I sync across devices?**
A: Not built-in. Use your own sync solution (Git, Syncthing, Dropbox, etc.)

**Q: Is it audited?**
A: Open source! Review the code yourself or hire a security auditor.

**Q: Which cipher should I use?**
A: PassLock auto-detects! Run `passlock info cpu` to see what you're using.

---

## Contributing

Found a bug? Want a feature? [Open an issue!](https://github.com/hachimamma/Passlock/issues)

---

## License

MIT License - see LICENSE file

---

**Made with ❤️ by the PassLock community**