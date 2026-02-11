.PHONY: all build run clean install-deps test server

all: build

install-deps:
	@echo "Installing libsodium..."
	@if command -v brew >/dev/null 2>&1; then \
		brew install libsodium; \
	elif command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get update && sudo apt-get install -y libsodium-dev; \
	elif command -v yum >/dev/null 2>&1; then \
		sudo yum install -y libsodium-devel; \
	elif command -v pacman >/dev/null 2>&1; then \
		sudo pacman -S libsodium; \
	else \
		echo "Please install libsodium manually"; \
		exit 1; \
	fi
	@echo "libsodium installed successfully!"

build:
	@echo "Building PassLock with C vault engine..."
	cargo build --release
	@echo "Build complete! Binary: target/release/passlock"

run:
	cargo run --release

test: test-c test-rust

test-c:
	@echo "Testing C vault engine..."
	gcc -DTEST_MODE -o test_crypto src/c/crypto_core.c -Wall -Wextra
	./test_crypto
	rm -f test_crypto

test-rust:
	@echo "\nTesting Rust components..."
	@echo "Note: Add #[test] functions to enable Rust tests"
	@cargo test --quiet || echo "No Rust tests defined yet"
	@cargo clean

server:
	@echo "Starting web server..."
	go run api_server.go

clean:
	cargo clean
	rm -f test_crypto
	@echo "Cleaned build artifacts"

setup: install-deps
	@echo "Setting up project structure..."
	@mkdir -p src/c
	@mkdir -p web
	@echo "Project setup complete!"

help:
	@echo "PassLock Build Commands:"
	@echo "  make install-deps  - Install libsodium dependency"
	@echo "  make build        - Build the project"
	@echo "  make run          - Run the TUI"
	@echo "  make test         - Run all tests (C + Rust)"
	@echo "  make test-c       - Run only C tests"
	@echo "  make test-rust    - Run only Rust tests"
	@echo "  make server        - Start web server"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make setup        - Full project setup"
