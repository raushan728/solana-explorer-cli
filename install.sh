#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing Raushan Explorer (Binary)...${NC}"

# 1. Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

if [ "$MACHINE" != "Linux" ] && [ "$MACHINE" != "Mac" ]; then
    echo -e "${RED}Error: Unsupported operating system: $OS${NC}"
    echo "Please build from source using Cargo."
    exit 1
fi
echo "Detected OS: $MACHINE"

# 2. Configuration
VERSION="v0.1.0"
BINARY_NAME="raushan"
REPO="raushan728/solana-explorer-cli"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"
INSTALL_DIR="/usr/local/bin"

# 3. Download Binary
echo "Downloading binary from GitHub Releases..."
if ! curl -sSfL -o "$BINARY_NAME" "$DOWNLOAD_URL"; then
    echo -e "${RED}Error: Failed to download binary.${NC}"
    echo -e "Please ensure you have an active internet connection."
    echo -e "Alternatively, you can install from source using Cargo:"
    echo -e "  ${GREEN}cargo install --git https://github.com/$REPO${NC}"
    exit 1
fi

# 4. Make Executable & Install
chmod +x "$BINARY_NAME"

echo "Installing to $INSTALL_DIR (may require sudo)..."
if [ ! -d "$INSTALL_DIR" ]; then
    sudo mkdir -p "$INSTALL_DIR"
fi

if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
fi

echo -e "${GREEN}Success! Raushan Explorer ($VERSION) is installed.${NC}"
echo -e "Run ${GREEN}raushan --version${NC} to verify."