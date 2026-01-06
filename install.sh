#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Raushan Explorer...${NC}"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed.${NC}"
    exit 1
fi

# Build release binary
cargo build --release

BIN_PATH="target/release/raushan"
INSTALL_DIR="/usr/local/bin"

if [ ! -f "$BIN_PATH" ]; then
    echo -e "${RED}Error: Binary not found at $BIN_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}Installing binary to $INSTALL_DIR...${NC}"

# Check for write permissions or sudo
if [ -w "$INSTALL_DIR" ]; then
    cp "$BIN_PATH" "$INSTALL_DIR/raushan"
else
    echo "Sudo permissions required to install to $INSTALL_DIR"
    sudo cp "$BIN_PATH" "$INSTALL_DIR/raushan"
fi

echo -e "${GREEN}Success! Raushan Explorer is installed.${NC}"
echo -e "Run ${GREEN}raushan --help${NC} to get started."
