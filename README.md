# PASSLOCK

**A multi-language, security-first password manager with a native TUI and optional HTTP API**

PASSLOCK is a **locally encrypted password manager** built as a systems-level project.
It combines **Rust**, **C**, and **Go** to demonstrate clean FFI boundaries, layered cryptography, and multiple interfaces over a single secure core.

No cloud dependency.
No hidden services.
Your vault stays on your machine.

---

## Key Highlights

* Layered cryptography with explicit trust boundaries
* Rust-first architecture for safety, correctness, and performance
* C crypto core for low-level control and secure memory handling
* Optional Go HTTP API for browser or remote access
* Clean, keyboard-driven TUI
* Single encrypted vault with persistent storage

---

## Architecture Overview

```
┌──────────────────┐
│   Rust CLI / TUI │   ← Primary interface
└─────────┬────────┘
          │ FFI
          │
┌─────────▼────────┐
│   C Crypto Core  │   ← Secure primitives & memory ops
└─────────┬────────┘
          │
┌─────────▼────────┐
│   Go HTTP API    │   ← Optional local web access
└──────────────────┘
```

### Design Goals

* Keep cryptographic material isolated
* Avoid unsafe abstractions in high-level code
* Make data flow explicit and auditable

---

## Technology Stack

### Rust

* CLI and TUI
* Vault logic and persistent storage
* ChaCha20-Poly1305 (IETF) encryption
* Argon2 key derivation
* Safe wrappers around C FFI

### C

* Low-level crypto utilities
* XOR obfuscation layer
* Secure random generation
* Timing-safe comparisons
* Explicit zero-on-free memory wiping

### Go

* Lightweight HTTP API
* Local-only web interface
* Simple concurrency model

---

## Directory Structure

```
Passlock/
├── Cargo.toml
├── build.rs
├── Makefile
├── README.md
├── .gitignore
│
├── c_src/
│   ├── crypto_core.c
│   └── crypto_core.h
│
├── go_src/
│   └── api_server.go
│
├── src/
│   ├── main.rs
│   ├── ui.rs
│   ├── crypto.rs
│   ├── storage.rs
│   └── models.rs
│
└── web/
    └── index.html
```

---

## Features

* ChaCha20-Poly1305 (IETF) authenticated encryption
* Argon2 password-based key derivation
* XOR obfuscation layer (defense in depth)
* Secure memory wiping in C
* Timing-safe comparisons
* Password generator
* Password strength meter (TUI and Web)
* Search and filter
* Persistent encrypted vault
* Local HTTP API with web interface

---

## Build and Run

### Prerequisites

* Rust toolchain
* Go
* C compiler (gcc or clang)
* Make

### Commands

```
make build     # Build everything
make run       # Run CLI / TUI
make server    # Start HTTP API + Web UI
make test      # Test C crypto layer
make clean     # Clean build artifacts
```

---

## API Usage (Optional Go Server)

Start the server:

```
make server
```

Then open:

```
http://localhost:8080
```

Example API usage:

```
curl -X POST http://localhost:8080/api \
  -d '{"act":"list","pwd":"<master-password>"}'
```

```
curl -X POST http://localhost:8080/api \
  -d '{"act":"search","pwd":"<master-password>","q":"github"}'
```

---

## Security Model

* Master password is never stored
* Key derivation via Argon2
* Vault encrypted at rest
* Multi-layer encryption pipeline
* Explicit memory zeroing for secrets
* No telemetry, no cloud, no external services

This project prioritizes **clarity, control, and auditability** over convenience.

---

## Why Multi-Language?

| Language | Purpose                                           |
| -------- | ------------------------------------------------- |
| Rust     | Memory safety, correctness, expressive APIs       |
| C        | Precise control over memory and crypto primitives |
| Go       | Simple and robust local HTTP services             |
| Overall  | Clean FFI and real-world systems design           |

---

## Status

![Rust CI](https://github.com/hachimamma/Passlock/actions/workflows/rust.yml/badge.svg)
![C CI](https://github.com/hachimamma/Passlock/actions/workflows/c.yml/badge.svg)
![Go CI](https://github.com/hachimamma/Passlock/actions/workflows/go.yml/badge.svg)
![Integration](https://github.com/hachimamma/Passlock/actions/workflows/integration.yml/badge.svg)

PASSLOCK is actively evolving and intended as:

* A serious personal password manager
* A reference for Rust ↔ C ↔ Go interoperability
* A security-focused systems project

Contributions, audits, and architectural feedback are welcome.
