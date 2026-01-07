#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Detected OS: $(uname -s)${NC}"
echo -e "${GREEN}Installing Raushan Explorer...${NC}"

# 1. Check for Cargo (Rust)
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: 'cargo' is not installed.${NC}"
    echo -e "Please install Rust and Cargo first:"
    echo -e "  ${GREEN}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    echo -e "After installing, restart your terminal and try this command again."
    exit 1
fi

# Variables
INSTALL_DIR="/usr/local/bin"
REPO_URL="https://github.com/raushan728/solana-explorer-cli.git"
TEMP_DIR=""

# Cleanup function
cleanup() {
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        echo -e "Cleaning up temporary files..."
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# 2. Check if we are in the repo directory or need to clone
if [ ! -f "Cargo.toml" ]; then
    echo -e "Cargo.toml not found. Cloning repository to temporary directory..."
    TEMP_DIR=$(mktemp -d)
    git clone --depth 1 "$REPO_URL" "$TEMP_DIR"
    cd "$TEMP_DIR"
else
    echo -e "Cargo.toml found. Building from current directory..."
fi

# 3. Build release binary
echo -e "${GREEN}Building release binary...${NC}"
cargo build --release

BIN_PATH="target/release/raushan"

if [ ! -f "$BIN_PATH" ]; then
    echo -e "${RED}Error: Build failed. Binary not found at $BIN_PATH${NC}"
    exit 1
fi

# 4. Install binary
echo -e "${GREEN}Installing binary to $INSTALL_DIR...${NC}"

if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "Creating directory $INSTALL_DIR..."
    sudo mkdir -p "$INSTALL_DIR"
fi

if [ -w "$INSTALL_DIR" ]; then
    cp "$BIN_PATH" "$INSTALL_DIR/raushan"
else
    echo "Sudo permissions required to install to $INSTALL_DIR"
    sudo cp "$BIN_PATH" "$INSTALL_DIR/raushan"
fi

echo -e "${GREEN}Success! Raushan Explorer is installed.${NC}"
echo -e "Run ${GREEN}raushan --help${NC} to get started."
