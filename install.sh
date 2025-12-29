#!/bin/sh
set -e

REPO="AlexAxenti/ShellStash"
BINARY="st"
INSTALL_DIR="/usr/local/bin"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    echo "Currently only x86_64 is supported."
    exit 1
    ;;
esac

case "$OS" in
  linux)   TARGET="st-linux-$ARCH" ;;
  darwin)  TARGET="st-macos-$ARCH" ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/latest/download/$TARGET"

echo "Downloading $TARGET..."
curl -fL "$URL" -o "$BINARY"

chmod +x "$BINARY"

echo "Installing to $INSTALL_DIR (may require sudo)..."
sudo mv "$BINARY" "$INSTALL_DIR/$BINARY"

echo "ShellStash installed!"
echo "Run: st --help"
