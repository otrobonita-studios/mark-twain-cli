#!/bin/bash
set -euo pipefail

# Repository configuration
REPO="otrobonita-studios/mark-twain-cli"
BINARY_NAME="mark-twain-cli"
CHECKSUM_FILE="SHA256SUMS"

echo "========================================="
echo " Installing Mark Twain CLI (mark-twain-cli)"
echo "========================================="

fail() {
    echo "Error: $*" >&2
    exit 1
}

# 1. Detect OS & Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64)   ARCH_LABEL="x86_64" ;;
    arm64|aarch64)  ARCH_LABEL="aarch64" ;;
    *)              fail "Unsupported architecture: $ARCH" ;;
esac

case "$OS" in
    Linux)
        PLATFORM="linux"
        EXTENSION="tar.gz"
        INSTALL_DIR="${HOME}/.local/bin"
        ;;
    Darwin)
        PLATFORM="macos"
        EXTENSION="tar.gz"
        INSTALL_DIR="${HOME}/.local/bin"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        ARCH_LABEL="x86_64"
        EXTENSION="zip"
        INSTALL_DIR="${HOME}/bin"
        ;;
    *)
        fail "Unsupported operating system: $OS"
        ;;
esac

echo "Detected platform: $PLATFORM ($ARCH_LABEL)"
echo "Target installation directory: $INSTALL_DIR"

# 2. Fetch the latest release metadata
echo "Fetching latest release from GitHub..."
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
RELEASE_JSON="$(curl -fsSL "$LATEST_RELEASE_URL")" \
    || fail "Could not reach the GitHub releases API for $REPO."

# `|| true` is load-bearing: with `set -o pipefail`, a grep that matches
# nothing fails the whole pipeline, and `set -e` would abort here silently --
# before the explicit, readable check below ever runs.
TAG="$(printf '%s' "$RELEASE_JSON" \
    | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' \
    | head -n 1 \
    | sed 's/.*"\([^"]*\)"$/\1/' || true)"

if [ -z "$TAG" ]; then
    fail "No published release found for $REPO. Nothing to install."
fi

echo "Latest version: $TAG"

# 3. Construct download URLs
# Format: mark-twain-cli-<tag>-<platform>-<arch>.<ext>
ASSET_NAME="${BINARY_NAME}-${TAG}-${PLATFORM}-${ARCH_LABEL}.${EXTENSION}"
BASE_URL="https://github.com/$REPO/releases/download/${TAG}"

TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT
cd "$TEMP_DIR"

echo "Downloading $ASSET_NAME ..."
curl -fsSL -o "$ASSET_NAME" "${BASE_URL}/${ASSET_NAME}" \
    || fail "Download failed. Release $TAG has no asset named $ASSET_NAME."

# 4. Verify the download against the release checksums
echo "Verifying checksum..."
curl -fsSL -o "$CHECKSUM_FILE" "${BASE_URL}/${CHECKSUM_FILE}" \
    || fail "Release $TAG publishes no $CHECKSUM_FILE; refusing to install an unverified binary."

if command -v sha256sum >/dev/null 2>&1; then
    SHA_CHECK=(sha256sum -c)
elif command -v shasum >/dev/null 2>&1; then
    SHA_CHECK=(shasum -a 256 -c)
else
    fail "Neither sha256sum nor shasum is available; cannot verify the download."
fi

grep " ${ASSET_NAME}\$" "$CHECKSUM_FILE" > "${CHECKSUM_FILE}.this" \
    || fail "$ASSET_NAME is not listed in $CHECKSUM_FILE."

"${SHA_CHECK[@]}" "${CHECKSUM_FILE}.this" \
    || fail "Checksum mismatch for $ASSET_NAME. The download is not what the release published."

echo "Checksum OK."

# 5. Extract and install
echo "Installing to $INSTALL_DIR ..."

if [ "$EXTENSION" = "tar.gz" ]; then
    tar -xzf "$ASSET_NAME"
else
    command -v unzip >/dev/null 2>&1 || fail "unzip is required to install on $PLATFORM."
    unzip -q "$ASSET_NAME"
fi

mkdir -p "$INSTALL_DIR"

if [ "$PLATFORM" = "windows" ]; then
    INSTALLED_PATH="$INSTALL_DIR/${BINARY_NAME}.exe"
    mv "${BINARY_NAME}.exe" "$INSTALLED_PATH"
else
    INSTALLED_PATH="$INSTALL_DIR/$BINARY_NAME"
    mv "$BINARY_NAME" "$INSTALLED_PATH"
fi

chmod +x "$INSTALLED_PATH"

echo "========================================="
echo " Installation complete: $INSTALLED_PATH"

case ":${PATH}:" in
    *":${INSTALL_DIR}:"*)
        echo " Type '${BINARY_NAME}' to get started."
        ;;
    *)
        echo
        echo " NOTE: $INSTALL_DIR is not on your PATH."
        echo " Add it to your shell profile to run the command by name:"
        echo
        echo "     export PATH=\"\$PATH:$INSTALL_DIR\""
        echo
        echo " Until then, run it directly: $INSTALLED_PATH"
        ;;
esac
echo "========================================="
