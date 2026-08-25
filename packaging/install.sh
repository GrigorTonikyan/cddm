#!/usr/bin/env bash
set -euo pipefail

# CDDM Standalone Cross-Platform Shell Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/GrigorTonikyan/cddm/main/packaging/install.sh | bash

REPO="GrigorTonikyan/cddm"
INSTALL_DIR="${CDDM_INSTALL_DIR:-$HOME/.cddm/bin}"
VERSION="${CDDM_VERSION:-latest}"

echo "--> Initializing CDDM Installer..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)   TARGET_OS="unknown-linux-gnu" ;;
  darwin*)  TARGET_OS="apple-darwin" ;;
  msys*|mingw*|cygwin*) TARGET_OS="pc-windows-msvc" ;;
  *) echo "Error: Unsupported operating system: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) TARGET_ARCH="x86_64" ;;
  arm64|aarch64) TARGET_ARCH="aarch64" ;;
  *) echo "Error: Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"

if [ "$VERSION" = "latest" ]; then
  RELEASE_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$RELEASE_TAG" ]; then
    RELEASE_TAG="v1.7.0"
  fi
else
  RELEASE_TAG="$VERSION"
fi

TARBALL="cddm-${RELEASE_TAG}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${RELEASE_TAG}/${TARBALL}"

echo "--> Downloading CDDM ${RELEASE_TAG} for ${TARGET}..."
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$DOWNLOAD_URL" -o "${TMP_DIR}/${TARBALL}"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "${TMP_DIR}/${TARBALL}" "$DOWNLOAD_URL"
else
  echo "Error: Neither curl nor wget is available."; exit 1
fi

mkdir -p "$INSTALL_DIR"
tar -xzf "${TMP_DIR}/${TARBALL}" -C "$INSTALL_DIR"
chmod +x "${INSTALL_DIR}/cddm"*

echo "--> Successfully installed CDDM to: ${INSTALL_DIR}"
echo ""
echo "Please ensure ${INSTALL_DIR} is in your PATH environment variable:"
echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
echo ""
echo "Run 'cddm --help' to get started."
