#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo "Installing Raushan Explorer v0.1.0..."

# Configuration
REPO="raushan728/solana-explorer-cli"
BINARY_NAME="raushan"
VERSION="v0.1.0"
INSTALL_DIR="/usr/local/bin"

# 1. Download the pre-built binary from GitHub Release
echo "Downloading binary from GitHub..."
URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"

curl -L -o "$BINARY_NAME" "$URL"

# 2. Make it executable
chmod +x "$BINARY_NAME"

# 3. Move to system path
echo "Moving binary to $INSTALL_DIR (may require sudo)..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
fi

echo -e "${GREEN}Success! Raushan Explorer is installed system-wide.${NC}"
echo -e "Run ${GREEN}raushan --version${NC} to verify."