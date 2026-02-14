# Contributing to PassLock

Thank you for considering contributing to PassLock!

We love contributions from the community. Whether it's bug reports, feature requests, documentation improvements, or code contributions - all are welcome!

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Guidelines](#coding-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)

---

## Code of Conduct

Be respectful, be kind, be constructive. We're all here to make PassLock better!

---

## How Can I Contribute?

### Reporting Bugs

**Before submitting:**
- Check if the bug has already been reported
- Collect information about the bug:
  - PassLock version (`passlock version`)
  - Operating System
  - Rust version (`rustc --version`)
  - Steps to reproduce
  - Expected vs actual behavior

**Submit a bug report:**
1. Go to [Issues](https://github.com/hachimamma/Passlock/issues)
2. Click "New Issue"
3. Choose "Bug Report"
4. Fill in the template

### Suggesting Features

We love new ideas! Before suggesting:
- Check if it's already been suggested
- Think about how it fits PassLock's philosophy (local-first, privacy-focused)

**Submit a feature request:**
1. Go to [Issues](https://github.com/hachimamma/Passlock/issues)
2. Click "New Issue"
3. Choose "Feature Request"
4. Describe your idea!

### Improving Documentation

Documentation is crucial! Help us by:
- Fixing typos
- Clarifying confusing sections
- Adding examples
- Translating to other languages

Just submit a PR with your changes!

### Contributing Code

See below for development setup and guidelines.

---

## Development Setup

### Prerequisites

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies
# Ubuntu/Debian:
sudo apt install build-essential libsodium-dev

# Fedora:
sudo dnf install gcc libsodium-devel

# macOS:
brew install libsodium

# Install dev tools
rustup component add clippy rustfmt
```

### Clone and Build

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/passlock
cd passlock

# Build
make build

# Run tests
make test

# Run lint
make lint
```

### Project Structure

```
passlock/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── models.rs        # Data structures
│   ├── storage.rs       # Vault persistence
│   ├── crypto.rs        # Crypto wrapper
│   ├── vault_ffi.rs     # C FFI bindings
│   ├── ui/              # TUI interface
│   │   ├── mod.rs
│   │   ├── app.rs       # App state
│   │   ├── colors.rs    # Gruvbox theme
│   │   ├── screens.rs   # Screen enums
│   │   ├── handlers.rs  # Input handlers
│   │   └── widgets/     # UI components
│   └── c/               # C vault engine
│       ├── vault_engine.c
│       ├── vault_engine.h
│       └── crypto_core.c
├── Cargo.toml           # Rust dependencies
├── Makefile             # Build system
├── README.md            # Main docs
└── COMMANDS.md          # Command reference
```

---

## Coding Guidelines

### Rust Style

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Use meaningful variable names
- Add comments for complex logic
- Keep functions small and focused

**Example:**
```rust
// Good
pub fn encrypt_vault(vault: &Vault, password: &str) -> Result<Vec<u8>, CryptoError> {
    let salt = generate_salt()?;
    let encrypted = encrypt_data(&vault.serialize()?, password, &salt)?;
    Ok(encrypted)
}

// Bad
pub fn e(v: &Vault, p: &str) -> Result<Vec<u8>, CryptoError> {
    let s = generate_salt()?;  // What's s?
    let e = encrypt_data(&v.serialize()?, p, &s)?;  // Unclear
    Ok(e)
}
```

### C Style

- Follow kernel coding style
- Use clear variable names
- Comment complex crypto operations
- Always check return values
- Use `vault_` prefix for all functions
- Keep functions under 100 lines when possible

### Documentation

- Public functions MUST have doc comments
- Use `///` for Rust doc comments
- Use `/** */` for C doc comments
- Include examples where helpful

**Example:**
```rust
/// Encrypts the vault with the given password.
///
/// # Arguments
///
/// * `vault` - The vault to encrypt
/// * `password` - Master password for encryption
///
/// # Returns
///
/// Encrypted vault data or error
///
/// # Examples
///
/// ```
/// let vault = Vault::new();
/// let encrypted = encrypt_vault(&vault, "password123")?;
/// ```
pub fn encrypt_vault(vault: &Vault, password: &str) -> Result<Vec<u8>, Error> {
    // ...
}
```

---

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semi-colons, etc
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Updating build tasks, etc

### Examples

```
feat(crypto): add ChaCha20-Poly1305 cipher support

Add ChaCha20-Poly1305 as alternative to AES-256-GCM for CPUs
without AES-NI hardware support. Automatically selects best
cipher based on CPU capabilities.

Closes #123
```

```
fix(ui): prevent crash on empty password list

Check if password list is empty before rendering to prevent
panic when accessing index 0.

Fixes #456
```

```
docs: update installation instructions

Add instructions for Fedora and Arch Linux.
```

---

## Pull Request Process

### Before Submitting

1. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**

3. **Run checks:**
   ```bash
   make check  # Runs format, lint, and tests
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

### Submitting

1. Go to [Pull Requests](https://github.com/hachimamma/passlock/pulls)
2. Click "New Pull Request"
3. Select your branch
4. Fill in the PR template:
   - What does this PR do?
   - Why is this change needed?
   - How has it been tested?
   - Screenshots (if UI changes)
5. Submit!

### Review Process

- Maintainers will review your PR
- They may request changes
- Make requested changes and push
- Once approved, it will be merged!

### PR Checklist

- [ ] Code follows style guidelines
- [ ] `make check` passes
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Commit messages follow format
- [ ] No breaking changes (or clearly documented)

---

## Testing

### Running Tests

```bash
# All tests
make test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests
cargo test --test '*'
```

### Writing Tests

**Unit tests** go in the same file:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let data = b"test data";
        let encrypted = encrypt(data, "password", &salt()).unwrap();
        assert_ne!(data, encrypted.as_slice());
    }
}
```

**Integration tests** go in `tests/`:
```rust
// tests/vault_operations.rs
use passlock::*;

#[test]
fn test_create_and_unlock_vault() {
    // Test end-to-end workflow
}
```

### Test Coverage

We aim for >80% test coverage on core functionality:
- Crypto operations
- Vault management
- Password operations
- UI code (harder to test, manual testing OK)

---

## Good First Issues

Looking for something to work on? Check out issues labeled:
- `good first issue`
- `help wanted`
- `documentation`

---

## Getting Help

- **Questions:** Open a [Discussion](https://github.com/hachimamma/Passlock/discussions)
- **Email:** subhodisha2062@gmail.com

---

## Contributors

Thank you to all contributors! See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the list.

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Happy coding!**