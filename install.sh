#!/bin/bash
set -e

BINARY_NAME="extman"
INSTALL_DIR="/usr/local/bin"

echo "Building $BINARY_NAME..."
cargo build --release

echo "Installing to $INSTALL_DIR/$BINARY_NAME..."
sudo cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Installed: $(which $BINARY_NAME)"
