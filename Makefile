# PassLock Makefile

.PHONY: help build server dev check clean

RED    := \033[0;31m
GREEN  := \033[0;32m
YELLOW := \033[0;33m
BLUE   := \033[0;34m
NC     := \033[0m

.DEFAULT_GOAL := help

help:
	@printf "$(BLUE)PassLock v2.2.3$(NC)\n"
	@printf "$(BLUE)───────────────────────────────$(NC)\n"
	@printf "\n$(GREEN)Installation:$(NC)\n"
	@printf "  $(YELLOW)./install.sh$(NC)         Install to /usr/local/bin\n"
	@printf "  $(YELLOW)./install.sh --local$(NC) Install to ~/.local/bin\n"
	@printf "\n$(GREEN)Development:$(NC)\n"
	@printf "  $(YELLOW)make build$(NC)           Build CLI + server\n"
	@printf "  $(YELLOW)make server$(NC)          Start API server\n"
	@printf "  $(YELLOW)make dev$(NC)             Run TUI + server together\n"
	@printf "  $(YELLOW)make check$(NC)           Lint + format + test\n"
	@printf "  $(YELLOW)make clean$(NC)           Remove build files\n"

build:
	@printf "$(BLUE)Building...$(NC)\n"
	cargo build --release
	@mkdir -p bin
	CGO_ENABLED=0 go build -ldflags="-s -w" -o bin/passlock-server api_server.go
	@printf "$(GREEN)Done!$(NC)\n"

server:
	@printf "$(YELLOW)API server running on http://localhost:8080$(NC)\n"
	./bin/passlock-server

dev: build
	./bin/passlock-server &
	@sleep 1
	./target/release/passlock tui
	@pkill -f passlock-server || true

check:
	cargo fmt --all
	cargo clippy --all-targets
	cargo test
	go vet ./...
	go fmt ./...
	@printf "$(GREEN)All checks passed!$(NC)\n"

clean:
	cargo clean
	@rm -rf bin/
	@printf "$(GREEN)Clean!$(NC)\n"