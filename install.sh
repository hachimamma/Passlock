#!/bin/bash

# PassLock Installer

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSLOCK_VERSION="2.2.3"
LOCAL_INSTALL=false

for arg in "$@"; do
    case $arg in
        --local) LOCAL_INSTALL=true ;;
        --help)
            echo "Usage: ./install.sh [--local]"
            echo ""
            echo "  --local    Install to ~/.local/bin (no sudo required)"
            echo "  (default)  Install to /usr/local/bin (requires sudo)"
            exit 0
            ;;
    esac
done

echo -e "${BLUE}PassLock v${PASSLOCK_VERSION} Installer${NC}"
echo -e "${BLUE}─────────────────────────────────${NC}"
echo ""

# Rust install

install_rust() {
    echo -e "${YELLOW}Rust not found. Installing...${NC}"
    if command -v curl &>/dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    elif command -v wget &>/dev/null; then
        wget -qO- https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo -e "${RED}[✗] curl and wget not found. Cannot install Rust automatically.${NC}"
        echo -e "${YELLOW}  Please install Rust manually: https://rustup.rs${NC}"
        exit 1
    fi
    echo -e "${GREEN}[✓] Rust installed!${NC}"
}

# Go install

install_go() {
    echo -e "${YELLOW}Go not found. Installing...${NC}"

    GO_VERSION="1.22.0"
    ARCH=$(uname -m)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')

    case $ARCH in
        x86_64)  ARCH="amd64" ;;
        aarch64) ARCH="arm64" ;;
        armv*)   ARCH="armv6l" ;;
        *)
            echo -e "${RED}[✗] Unsupported architecture: $ARCH${NC}"
            echo -e "${YELLOW}  Please install Go manually: https://go.dev/dl${NC}"
            exit 1
            ;;
    esac

    GO_TAR="go${GO_VERSION}.${OS}-${ARCH}.tar.gz"
    GO_URL="https://go.dev/dl/${GO_TAR}"

    echo -e "${BLUE}  Downloading Go ${GO_VERSION}...${NC}"

    if command -v curl &>/dev/null; then
        curl -fsSL "$GO_URL" -o "/tmp/${GO_TAR}"
    elif command -v wget &>/dev/null; then
        wget -q "$GO_URL" -O "/tmp/${GO_TAR}"
    else
        echo -e "${RED}[✗] curl and wget not found. Cannot install Go automatically.${NC}"
        echo -e "${YELLOW}  Please install Go manually: https://go.dev/dl${NC}"
        exit 1
    fi

    echo -e "${BLUE}  Extracting...${NC}"
    sudo rm -rf /usr/local/go
    sudo tar -C /usr/local -xzf "/tmp/${GO_TAR}"
    rm "/tmp/${GO_TAR}"

    export PATH=$PATH:/usr/local/go/bin

    SHELL_RC=""
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_RC="$HOME/.bashrc"
    fi

    if [ -n "$SHELL_RC" ]; then
        if ! grep -q "/usr/local/go/bin" "$SHELL_RC"; then
            echo 'export PATH=$PATH:/usr/local/go/bin' >> "$SHELL_RC"
            echo -e "${GREEN}  Added Go to PATH in ${SHELL_RC}${NC}"
        fi
    fi

    echo -e "${GREEN}[✓] Go ${GO_VERSION} installed!${NC}"
}

# Check deps

echo -e "${BLUE}Checking dependencies...${NC}"

if ! command -v rustc &>/dev/null; then
    read -rp "$(echo -e "${YELLOW}Rust is not installed. Install it now? [y/N]: ${NC}")" answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
        install_rust
    else
        echo -e "${RED}[✗] Rust is required. Exiting.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}[✓] Rust $(rustc --version | cut -d' ' -f2)${NC}"
fi

if ! command -v go &>/dev/null; then
    read -rp "$(echo -e "${YELLOW}Go is not installed. Install it now? [y/N]: ${NC}")" answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
        install_go
    else
        echo -e "${RED}[✗] Go is required. Exiting.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}[✓] Go $(go version | cut -d' ' -f3)${NC}"
fi

echo ""

# Build

echo -e "${BLUE}Building PassLock CLI...${NC}"
cargo build --release
echo -e "${GREEN}[✓] CLI built${NC}"

echo -e "${BLUE}Building API server...${NC}"
mkdir -p bin
CGO_ENABLED=0 go build -ldflags="-s -w" -o bin/passlock-server api_server.go
echo -e "${GREEN}[✓] Server built${NC}"

echo ""

# Install

if [ "$LOCAL_INSTALL" = true ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    cp target/release/passlock "$INSTALL_DIR/passlock"
    cp bin/passlock-server "$INSTALL_DIR/passlock-server"
    chmod +x "$INSTALL_DIR/passlock"
    chmod +x "$INSTALL_DIR/passlock-server"
    echo -e "${GREEN}[✓] Installed to ~/.local/bin${NC}"
    echo ""

    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        echo -e "${YELLOW}[⚠] ~/.local/bin is not in your PATH!${NC}"
        echo -e "${YELLOW}  Add this to your ~/.bashrc or ~/.zshrc:${NC}"
        echo -e "${YELLOW}  export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
        echo ""
    fi
else
    echo -e "${BLUE}Installing (requires sudo)...${NC}"
    sudo cp target/release/passlock /usr/local/bin/passlock
    sudo cp bin/passlock-server /usr/local/bin/passlock-server
    sudo chmod +x /usr/local/bin/passlock
    sudo chmod +x /usr/local/bin/passlock-server
    echo -e "${GREEN}[✓] Installed to /usr/local/bin${NC}"
    echo ""
fi


echo -e "${GREEN}PassLock v${PASSLOCK_VERSION} installed successfully!${NC}"
echo ""
echo -e "  Start TUI:     ${YELLOW}passlock tui${NC}"
echo -e "  Start server:  ${YELLOW}passlock-server${NC}"
echo -e "  Help:          ${YELLOW}passlock help${NC}"
echo ""