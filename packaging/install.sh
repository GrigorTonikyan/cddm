#!/usr/bin/env bash
set -euo pipefail

# CDDM Standalone Cross-Platform Shell Installer
# Usage: curl -fsSL https://git.gt-web-dev.com/gt-dev/cddm/raw/branch/main/packaging/install.sh | bash

GITEA_HOST="${CDDM_GITEA_HOST:-git.gt-web-dev.com}"
REPO="${CDDM_REPO:-gt-dev/cddm}"
INSTALL_DIR="${CDDM_INSTALL_DIR:-$HOME/.cddm/bin}"
VERSION="${CDDM_VERSION:-latest}"

echo "--> Initializing CDDM Installer (host: ${GITEA_HOST})..."

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
  RELEASE_TAG=$(curl -s "https://${GITEA_HOST}/api/v1/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$RELEASE_TAG" ]; then
    RELEASE_TAG="v1.9.0"
  fi
else
  RELEASE_TAG="$VERSION"
fi

TARBALL="cddm-${RELEASE_TAG}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://${GITEA_HOST}/${REPO}/releases/download/${RELEASE_TAG}/${TARBALL}"

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
