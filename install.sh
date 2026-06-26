#!/bin/bash
set -e

# Repository configuration
REPO="otrobonita-studios/mark-twain-cli"
BINARY_NAME="mark-twain-cli"

echo "========================================="
echo " Installing Mark Twain CLI (mark-twain-cli)"
echo "========================================="

# 1. Detect OS & Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        PLATFORM="macos"
        if [ "$ARCH" = "arm64" ]; then
            ARCH_LABEL="aarch64"
        else
            ARCH_LABEL="x86_64"
        fi
        EXTENSION="tar.gz"
        INSTALL_DIR="/usr/local/bin"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        ARCH_LABEL="x86_64"
        EXTENSION="zip"
        INSTALL_DIR="/c/Windows/System32" # Fallback or custom user folder in path
        if [ -d "$HOME/bin" ]; then
            INSTALL_DIR="$HOME/bin"
        elif [ -d "/usr/bin" ]; then
            INSTALL_DIR="/usr/bin"
        fi
        ;;
    *)
        echo "Error: Unsupported Operating System: $OS"
        exit 1
        ;;
esac

echo "Detected Platform: $PLATFORM ($ARCH_LABEL)"
echo "Target Installation Directory: $INSTALL_DIR"

# 2. Fetch the latest release metadata
echo "Fetching latest release from GitHub..."
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
RELEASE_JSON=$(curl -s "$LATEST_RELEASE_URL")

# Extract tag name
TAG=$(echo "$RELEASE_JSON" | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$')

if [ -z "$TAG" ]; then
    echo "Warning: Could not fetch latest release tag. Using fallback 'v0.1.0'."
    TAG="v0.1.0"
fi

echo "Latest version: $TAG"

# 3. Construct download URL
# Format: mark-twain-cli-<tag>-<platform>-<arch>.<ext>
ASSET_NAME="${BINARY_NAME}-${TAG}-${PLATFORM}-${ARCH_LABEL}.${EXTENSION}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/${TAG}/${ASSET_NAME}"

TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "Downloading binary from: $DOWNLOAD_URL"
if ! curl -L -f -o "$ASSET_NAME" "$DOWNLOAD_URL"; then
    echo "Error: Download failed! Please check if release assets are uploaded correctly."
    exit 1
fi

# 4. Extract and Install
echo "Extracting and installing to $INSTALL_DIR..."

if [ "$EXTENSION" = "tar.gz" ]; then
    tar -xzf "$ASSET_NAME"
else
    unzip -q "$ASSET_NAME"
fi

# Ensure install directory exists
mkdir -p "$INSTALL_DIR"

# Copy binary to destination
if [ "$PLATFORM" = "windows" ]; then
    mv "${BINARY_NAME}.exe" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/${BINARY_NAME}.exe"
else
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

# Clean up
cd - > /dev/null
rm -rf "$TEMP_DIR"

echo "========================================="
echo " Installation Complete!"
echo " Type '${BINARY_NAME}' to get started."
echo "========================================="
