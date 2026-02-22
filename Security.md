# Security Policy

## Security Overview

PassLock is designed with security as the top priority. This document outlines our security practices, threat model, and how to report vulnerabilities.

---

## Threat Model

### What PassLock Protects Against

**Unauthorized vault access** - AES-256-GCM or ChaCha20-Poly1305 encryption  
**Password cracking** - Argon2id key derivation (memory-hard, GPU-resistant)  
**Data tampering** - Authenticated encryption (AEAD) with Poly1305 MAC  
**Brute force attacks** - Strong key derivation makes offline attacks impractical  
**Memory dumps** - Sensitive data is zeroed after use  

### What PassLock Does NOT Protect Against

**Compromised operating system** - If your OS is compromised, all bets are off  
**Keyloggers** - Hardware/software keyloggers can capture your master password  
**Physical access attacks** - Cold boot attacks, hardware memory dumping  
**Malware on your system** - Malware with sufficient privileges can access decrypted data  
**Weak master passwords** - We can't protect against `password123`  

**Remember:** PassLock is local-only. Your security depends on securing your device!

---

## Encryption Details

### Adaptive Encryption

PassLock automatically selects the best cipher for your CPU:

| CPU Type | Cipher | Key Size | Mode | Performance |
|----------|--------|----------|------|-------------|
| **Modern (AES-NI)** | AES-256-GCM | 256-bit | Galois/Counter Mode | 3-5 GB/s |
| **Older (no AES-NI)** | ChaCha20-Poly1305 | 256-bit | IETF variant | 300 MB/s |

Both provide **256-bit security** and are approved by cryptographers worldwide.

### Key Derivation

**Algorithm:** Argon2id  
**Parameters:**
- Memory cost: 64 MB (hardcoded)
- Time cost: 3 iterations
- Parallelism: 1 thread
- Output: 32 bytes (256-bit key)

**Why Argon2id?**
- Winner of the Password Hashing Competition (2015)
- Resistant to GPU/ASIC attacks (memory-hard)
- Resistant to side-channel attacks
- Recommended by OWASP, NIST

### Authentication

**MAC:** Poly1305 (128-bit)  
**Nonce:** 96-bit random (generated via libsodium)  

Every vault operation is authenticated. Any tampering results in decryption failure.

---

## Security Features

### 1. **No Cloud, No Tracking**
- All data stored locally in `~/.passlock.vault`
- Zero telemetry or phone-home
- No external network requests (except web server in dev mode)

### 2. **Memory Safety**
- Written in Rust (memory-safe by design)
- Critical crypto operations in C (libsodium - audited & battle-tested)
- Sensitive data zeroed after use (`sodium_memzero`)

### 3. **Secure Random Number Generation**
- Uses `/dev/urandom` on Linux/macOS
- CryptGenRandom on Windows
- Provided by libsodium (secure by default)

### 4. **Vault File Format**

```
[1 byte: cipher type]      # 1=AES-256-GCM, 2=ChaCha20-Poly1305
[12 bytes: nonce]          # Random, unique per encryption
[N bytes: encrypted data]  # Your passwords (JSON)
[16 bytes: auth tag]       # Poly1305 MAC
```

**File permissions:** `600` (read/write owner only)

### 5. **Password Strength Validation**
- Real-time strength meter
- Checks for common patterns
- Recommends improvements
- Minimum 4 characters (we recommend 12+)

### 6. **Auto-lock (TUI)**
- TUI locks vault on exit
- Temporary files (`~/.passlock.temp`) deleted on clean exit
- Master password never stored (only held in memory during session)

### 7. **Configurable Security**
- User-controlled clipboard timeout
- Option to disable auto-clear for trusted environments
- Balance between security and usability

---

## Audit Status

### External Audits
**Status:** Not yet audited by a third-party security firm.

We welcome security researchers to review our code! See "Reporting Vulnerabilities" below.

### Dependencies
- **libsodium** - Widely audited, used by Signal, WireGuard, Tor
- **Rust std** - Memory-safe by design
- **ratatui** - TUI library (no crypto operations)

### Build Reproducibility
PassLock builds are **not yet reproducible**. This is on our roadmap for v3.0.

---

## Best Practices for Users

### Master Password
1. **Use a strong master password:**
   - Minimum 12 characters (20+ recommended)
   - Mix uppercase, lowercase, numbers, symbols
   - Avoid dictionary words
   - Consider a passphrase: `correct-horse-battery-staple-2024`

2. **Never reuse your master password** elsewhere

3. **Store a backup** of your master password in a secure location:
   - Physical safe
   - Trusted family member
   - Password manager (if using PassLock for specific use case)

### Vault Backup
1. **Regular backups:**
   ```bash
   cp ~/.passlock.vault ~/Backups/passlock-$(date +%Y%m%d).vault
   ```

2. **Encrypted backup storage:**
   ```bash
   # Encrypt backup with GPG
   gpg -c ~/.passlock.vault
   ```

3. **Test restores** periodically:
   ```bash
   cp ~/Backups/passlock-20240215.vault ~/.passlock.vault
   passlock unlock <password>
   ```

### System Security

1. **Enable full disk encryption:**
   - Linux: LUKS
   - macOS: FileVault
   - Windows: BitLocker

2. **Keep your OS updated:**
   ```bash
   sudo apt update && sudo apt upgrade  # Debian/Ubuntu
   ```

3. **Use a secure lock screen:**
   - Auto-lock after 5 minutes
   - Strong user password
   - Disable guest accounts

4. **Avoid untrusted systems:**
   - Don't use PassLock on shared/public computers
   - Don't run on compromised/infected systems

### TOTP/2FA Security
- TOTP secrets stored encrypted in vault (same as passwords)
- Base32 format validation
- Time-based code generation using system time
- 30-second refresh interval (RFC 6238 compliant)

### File Permissions

Verify vault permissions:
```bash
ls -l ~/.passlock.vault
# Should show: -rw------- (600)
```

Fix if needed:
```bash
chmod 600 ~/.passlock.vault
```

---

## Reporting Vulnerabilities

We take security seriously! If you discover a vulnerability:

### Reporting Process

1. **DO NOT** open a public GitHub issue
2. **Email:** security@passlock.dev (coming soon)
3. **For now:** Open a private security advisory on GitHub
4. **Include:**
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment:** Within 48 hours
- **Initial assessment:** Within 1 week
- **Fix timeline:** Depends on severity
  - Critical: 1-7 days
  - High: 1-4 weeks
  - Medium: 1-3 months
  - Low: Next release

### Security Advisory Process

1. We'll confirm the vulnerability
2. Develop a fix
3. Release a patch
4. Publish a security advisory
5. Credit you (if desired)

### Hall of Fame

Contributors who responsibly disclose vulnerabilities will be listed here:

- *No vulnerabilities reported yet!*

---

## Security Updates

### Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.x.x   | ✅ Active support  |
| 1.x.x   | ⚠️ Security fixes only |
| < 1.0   | ❌ Not supported   |

### Update Notifications

Watch our GitHub releases for security updates:
```bash
# Subscribe to releases
https://github.com/hachimamma/Passlock/releases
```

---

## Secure Development Practices

### Code Review
- All changes reviewed before merge
- Security-sensitive code gets extra scrutiny
- Dependency updates reviewed for security implications

### Dependency Management
```bash
# Check for vulnerable dependencies
cargo audit

# Update dependencies
cargo update
```

### Testing
- Crypto functions have unit tests
- Integration tests for vault operations
- Manual security testing before releases

### Build Security
- Builds use `--release` mode (optimizations enabled)
- Debug symbols stripped from releases
- Minimal dependencies (reduce attack surface)

---

## Cryptographic Standards

PassLock follows industry best practices:

- **NIST SP 800-175B** - Guideline for Using Cryptographic Standards
- **OWASP Password Storage Cheat Sheet** - Argon2id recommended
- **RFC 7539** - ChaCha20 and Poly1305
- **FIPS 197** - AES specification
- **RFC 5116** - AEAD Cipher Suites

---

## Known Limitations

1. **No HSM/TPM support** (yet)
   - Keys stored in memory during use
   - Future: Hardware key integration

2. **Configurable clipboard timeout (10s-5min or disabled)**
   - Clipboard cleared after 30s
   - Future: Secure clipboard API

3. **No panic protection**
   - Rust panics may leave temp files
   - Future: Panic handlers for cleanup

4. **No anti-debug protection**
   - Debuggers can read memory
   - Not a goal for open-source software

---

## Security Resources

### Learn More
- [libsodium documentation](https://libsodium.gitbook.io/)
- [Argon2 RFC](https://datatracker.ietf.org/doc/html/rfc9106)
- [OWASP Password Storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)

### Security Tools
```bash
# Audit Rust dependencies
cargo install cargo-audit
cargo audit

# Check for memory leaks
valgrind --leak-check=full ./target/release/passlock

# Static analysis
cargo clippy -- -W clippy::all
```

---

## Contact

- **Security Issues:** Open a private security advisory on GitHub
- **General Security Questions:** GitHub Discussions
- **Email:** security@passlock.dev (coming soon)

---

## License

This security policy is part of the PassLock project and follows the same MIT License.

---

**Last Updated:** February 22, 2026
**Version:** 2.3.5

---

*Security is a process, not a destination. We continuously improve PassLock's security posture and welcome your feedback!*