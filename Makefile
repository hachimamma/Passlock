# PassLock Makefile
# Comprehensive build system for development and production

.PHONY: help build build-release install uninstall test test-c test-rust test-all clean lint format check run dev docs

# Default target
.DEFAULT_GOAL := help

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
NC := \033[0m

# Paths
PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
CARGO = cargo
GO = go
CC = gcc

help: ## Show this help message
	@printf "$(BLUE)PassLock Build System$(NC)\n"
	@printf "$(BLUE)=====================$(NC)\n\n"
	@printf "$(GREEN)Available targets:$(NC)\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'
	@printf "\n$(GREEN)Examples:$(NC)\n"
	@printf "  make build          # Build Rust CLI + Go server\n"
	@printf "  make build-release  # Build optimized release\n"
	@printf "  make install        # Install everything to system\n"
	@printf "  make server         # Run API server\n"
	@printf "  make dev            # Run CLI + server together\n"

# ============================================================
# Build Targets - Rust CLI
# ============================================================

build: ## Build debug version (Rust CLI)
	@printf "$(BLUE)Building PassLock CLI (debug)...$(NC)\n"
	$(CARGO) build
	@printf "$(GREEN)CLI build complete!$(NC)\n"
	@printf "$(GREEN)Binary: target/debug/passlock$(NC)\n"

build-release: ## Build optimized release (Rust CLI)
	@printf "$(BLUE)Building PassLock CLI (release)...$(NC)\n"
	$(CARGO) build --release
	@printf "$(GREEN)CLI release build complete!$(NC)\n"
	@printf "$(GREEN)Binary: target/release/passlock$(NC)\n"

# ============================================================
# Build Targets - Go Server
# ============================================================

build-server: ## Build Go API server
	@printf "$(BLUE)Building Go API server...$(NC)\n"
	$(GO) build -o bin/passlock-server api_server.go
	@printf "$(GREEN)Server build complete!$(NC)\n"
	@printf "$(GREEN)Binary: bin/passlock-server$(NC)\n"

build-server-release: ## Build optimized Go server
	@printf "$(BLUE)Building Go API server (optimized)...$(NC)\n"
	CGO_ENABLED=0 $(GO) build -ldflags="-s -w" -o bin/passlock-server api_server.go
	@printf "$(GREEN)Server release build complete!$(NC)\n"
	@printf "$(GREEN)Binary: bin/passlock-server$(NC)\n"

build-all: build build-server ## Build everything (CLI + server)
	@printf "$(GREEN)All components built!$(NC)\n"

build-all-release: build-release build-server-release ## Build everything (optimized)
	@printf "$(GREEN)All components built (release)!$(NC)\n"

# ============================================================
# Installation
# ============================================================

install: build-release build-server-release ## Install PassLock CLI + server to system
	@printf "$(BLUE)Installing PassLock...$(NC)\n"
	@printf "  → Installing CLI to $(BINDIR)\n"
	sudo cp target/release/passlock $(BINDIR)/passlock
	sudo chmod +x $(BINDIR)/passlock
	@printf "  → Installing server to $(BINDIR)\n"
	sudo cp bin/passlock-server $(BINDIR)/passlock-server
	sudo chmod +x $(BINDIR)/passlock-server
	@printf "$(GREEN)PassLock installed successfully!$(NC)\n\n"
	@printf "$(GREEN)Quick start:$(NC)\n"
	@printf "  1. Create vault:  passlock create <password>\n"
	@printf "  2. Start server:  passlock-server\n"
	@printf "  3. Launch TUI:    passlock tui\n\n"
	@printf "$(GREEN)Run 'passlock help' for more info$(NC)\n"

install-local: build-release build-server-release ## Install to ~/.local/bin (no sudo)
	@printf "$(BLUE)Installing PassLock locally...$(NC)\n"
	@mkdir -p ~/.local/bin
	@printf "  → Installing CLI to ~/.local/bin\n"
	cp target/release/passlock ~/.local/bin/passlock
	chmod +x ~/.local/bin/passlock
	@printf "  → Installing server to ~/.local/bin\n"
	cp bin/passlock-server ~/.local/bin/passlock-server
	chmod +x ~/.local/bin/passlock-server
	@printf "$(GREEN)PassLock installed to ~/.local/bin!$(NC)\n\n"
	@printf "$(YELLOW)Note: Make sure ~/.local/bin is in your PATH$(NC)\n"

uninstall: ## Uninstall PassLock from system
	@printf "$(BLUE)Uninstalling PassLock...$(NC)\n"
	sudo rm -f $(BINDIR)/passlock
	sudo rm -f $(BINDIR)/passlock-server
	@printf "$(GREEN)PassLock uninstalled$(NC)\n"

# ============================================================
# Testing
# ============================================================

test: test-rust ## Run all tests

test-rust: ## Run Rust tests
	@printf "$(BLUE)Running Rust tests...$(NC)\n"
	$(CARGO) test
	@printf "$(GREEN)Rust tests passed!$(NC)\n"

test-go: ## Run Go tests
	@printf "$(BLUE)Running Go tests...$(NC)\n"
	$(GO) test ./... -v
	@printf "$(GREEN)Go tests passed!$(NC)\n"

test-all: test-rust test-go ## Run all tests (Rust + Go)
	@printf "$(GREEN)All tests passed!$(NC)\n"

# ============================================================
# Code Quality
# ============================================================

lint: lint-rust lint-go ## Run all linters

lint-rust: ## Run Rust clippy
	@printf "$(BLUE)Running clippy...$(NC)\n"
	$(CARGO) clippy --all-targets --all-features
	@printf "$(GREEN)Rust lint complete!$(NC)\n"

lint-go: ## Run Go linters
	@printf "$(BLUE)Running Go linters...$(NC)\n"
	$(GO) vet ./...
	@printf "$(GREEN)Go lint complete!$(NC)\n"

format: format-rust format-go ## Format all code

format-rust: ## Format Rust code
	@printf "$(BLUE)Formatting Rust code...$(NC)\n"
	$(CARGO) fmt --all
	@printf "$(GREEN)Rust code formatted!$(NC)\n"

format-go: ## Format Go code
	@printf "$(BLUE)Formatting Go code...$(NC)\n"
	$(GO) fmt ./...
	@printf "$(GREEN)Go code formatted!$(NC)\n"

check: format lint test ## Run all checks (format, lint, test)
	@printf "$(GREEN)All checks passed! Ready to commit.$(NC)\n"

# ============================================================
# Development
# ============================================================

run: build ## Build and run CLI
	@printf "$(BLUE)Running PassLock CLI...$(NC)\n"
	./target/debug/passlock

run-release: build-release ## Run CLI (release)
	@printf "$(BLUE)Running PassLock CLI (release)...$(NC)\n"
	./target/release/passlock

run-tui: build ## Run TUI interface
	@printf "$(BLUE)Launching TUI...$(NC)\n"
	./target/debug/passlock tui

server: build-server ## Run API server
	@printf "$(BLUE)Starting PassLock API server...$(NC)\n"
	@printf "$(YELLOW)Server will run on http://localhost:8080$(NC)\n"
	./bin/passlock-server

dev: build-all ## Run CLI + server together
	@printf "$(BLUE)Starting development environment...$(NC)\n"
	@printf "$(YELLOW)Starting server in background...$(NC)\n"
	./bin/passlock-server &
	@sleep 1
	@printf "$(GREEN)Server running on http://localhost:8080$(NC)\n"
	@printf "$(BLUE)Launching TUI...$(NC)\n"
	./target/debug/passlock tui
	@printf "$(YELLOW)Stopping server...$(NC)\n"
	@pkill -f passlock-server || true

# ============================================================
# Documentation
# ============================================================

docs: ## Generate Rust documentation
	@printf "$(BLUE)Generating documentation...$(NC)\n"
	$(CARGO) doc --no-deps --open
	@printf "$(GREEN)Documentation generated!$(NC)\n"

# ============================================================
# Cleaning
# ============================================================

clean: ## Clean build artifacts
	@printf "$(BLUE)Cleaning build artifacts...$(NC)\n"
	$(CARGO) clean
	@rm -rf bin/
	@printf "$(GREEN)Clean complete!$(NC)\n"

clean-all: clean ## Clean everything
	@printf "$(BLUE)Cleaning everything...$(NC)\n"
	@rm -rf target/
	@rm -f Cargo.lock
	@rm -rf bin/
	@printf "$(GREEN)Deep clean complete!$(NC)\n"

# ============================================================
# Utility
# ============================================================

deps: ## Check dependencies
	@printf "$(BLUE)Checking dependencies...$(NC)\n\n"
	@printf "$(BLUE)Rust toolchain:$(NC)\n"
	@rustc --version || (printf "$(RED)Rust not installed$(NC)\n" && exit 1)
	@$(CARGO) --version || (printf "$(RED)Cargo not installed$(NC)\n" && exit 1)
	@printf "\n$(BLUE)Go toolchain:$(NC)\n"
	@$(GO) version || (printf "$(RED)Go not installed$(NC)\n" && exit 1)
	@printf "\n$(BLUE)System libraries:$(NC)\n"
	@pkg-config --exists libsodium && printf "$(GREEN)libsodium found$(NC)\n" || printf "$(YELLOW)libsodium not found$(NC)\n"

info: ## Show system information
	@./target/release/passlock info cpu 2>/dev/null || (printf "$(YELLOW)Build first: make build-release$(NC)\n")

version: ## Show version
	@printf "$(BLUE)PassLock Version:$(NC)\n"
	@grep '^version' Cargo.toml | head -1

size: build-all-release ## Show binary sizes
	@printf "$(BLUE)Binary Sizes:$(NC)\n"
	@printf "$(BLUE)=============$(NC)\n"
	@ls -lh target/release/passlock | awk '{print "CLI:    " $$5}'
	@ls -lh bin/passlock-server | awk '{print "Server: " $$5}'

all: build-all test ## Build and test everything
	@printf "$(GREEN)Everything complete!$(NC)\n"