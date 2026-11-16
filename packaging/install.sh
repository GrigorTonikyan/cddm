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

case "$ARCH" in
  x86_64|amd64) TARGET_ARCH="x86_64" ;;
  arm64|aarch64) TARGET_ARCH="aarch64" ;;
  *) echo "Error: Unsupported CPU architecture: ${ARCH}" >&2; exit 1 ;;
esac

ARCHIVE_EXT=".tar.gz"
FALLBACK_TARGET=""
case "$OS" in
  linux*)
    TARGET="${TARGET_ARCH}-unknown-linux-gnu"
    ;;
  darwin*)
    TARGET="${TARGET_ARCH}-apple-darwin"
    ;;
  msys*|mingw*|cygwin*)
    TARGET="${TARGET_ARCH}-pc-windows-msvc"
    FALLBACK_TARGET="${TARGET_ARCH}-pc-windows-gnu"
    ARCHIVE_EXT=".zip"
    ;;
  *)
    echo "Error: Unsupported operating system: ${OS}" >&2
    exit 1
    ;;
esac

fetch_url() {
  local url="$1"
  local dest="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$dest"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$dest" "$url"
  else
    echo "Error: Neither curl nor wget is available." >&2
    exit 1
  fi
}

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/cddm-install.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

if [ "$VERSION" = "latest" ]; then
  API_URL="https://${GITEA_HOST}/api/v1/repos/${REPO}/releases/latest"
  META_FILE="${TMP_DIR}/release.json"
  if ! fetch_url "$API_URL" "$META_FILE"; then
    echo "Error: Failed to query latest release metadata from ${API_URL}. Refusing silent version downgrade." >&2
    exit 1
  fi
  RELEASE_TAG="$(grep -oE '"tag_name"[[:space:]]*:[[:space:]]*"[^"]+"' "$META_FILE" | head -n1 | sed -E 's/.*"([^"]+)"$/\1/')"
  if [ -z "$RELEASE_TAG" ]; then
    echo "Error: Could not parse release tag_name from ${API_URL}. Refusing silent version downgrade." >&2
    exit 1
  fi
else
  case "$VERSION" in
    v*) RELEASE_TAG="$VERSION" ;;
    *)  RELEASE_TAG="v${VERSION}" ;;
  esac
fi

BASE_DOWNLOAD_URL="https://${GITEA_HOST}/${REPO}/releases/download/${RELEASE_TAG}"
SUMS_FILE="${TMP_DIR}/SHA256SUMS.txt"

echo "--> Fetching SHA256SUMS.txt for ${RELEASE_TAG}..."
if ! fetch_url "${BASE_DOWNLOAD_URL}/SHA256SUMS.txt" "$SUMS_FILE"; then
  echo "Error: Failed to download ${BASE_DOWNLOAD_URL}/SHA256SUMS.txt for integrity verification." >&2
  exit 1
fi

ARCHIVE_NAME="cddm-${RELEASE_TAG}-${TARGET}${ARCHIVE_EXT}"
if ! grep -qF "$ARCHIVE_NAME" "$SUMS_FILE" && [ -n "$FALLBACK_TARGET" ]; then
  CANDIDATE_FALLBACK="cddm-${RELEASE_TAG}-${FALLBACK_TARGET}${ARCHIVE_EXT}"
  if grep -qF "$CANDIDATE_FALLBACK" "$SUMS_FILE"; then
    ARCHIVE_NAME="$CANDIDATE_FALLBACK"
  fi
fi

EXPECTED_HASH="$(awk -v target="$ARCHIVE_NAME" '{
  fname=$2; sub(/^\*/, "", fname); sub(/^\.\//, "", fname);
  if (fname == target) { print tolower($1); exit }
}' "$SUMS_FILE")"

if [ -z "$EXPECTED_HASH" ]; then
  echo "Error: Release ${RELEASE_TAG} does not provide a prebuilt binary archive for ${ARCHIVE_NAME}." >&2
  exit 1
fi

if printf '%s\n' "$EXPECTED_HASH" | grep -qE '^0+$'; then
  echo "Error: SHA256SUMS.txt contains a forbidden all-zero digest for ${ARCHIVE_NAME}." >&2
  exit 1
fi

ARCHIVE_PATH="${TMP_DIR}/${ARCHIVE_NAME}"
echo "--> Downloading ${ARCHIVE_NAME}..."
fetch_url "${BASE_DOWNLOAD_URL}/${ARCHIVE_NAME}" "$ARCHIVE_PATH"

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_HASH="$(sha256sum "$ARCHIVE_PATH" | awk '{print tolower($1)}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_HASH="$(shasum -a 256 "$ARCHIVE_PATH" | awk '{print tolower($1)}')"
elif command -v openssl >/dev/null 2>&1; then
  ACTUAL_HASH="$(openssl dgst -sha256 "$ARCHIVE_PATH" | awk '{print tolower($NF)}')"
else
  echo "Error: No SHA-256 verification tool (sha256sum, shasum, or openssl) found." >&2
  exit 1
fi

if [ "$ACTUAL_HASH" != "$EXPECTED_HASH" ]; then
  echo "Error: SHA-256 checksum mismatch for ${ARCHIVE_NAME}: expected ${EXPECTED_HASH}, got ${ACTUAL_HASH}." >&2
  exit 1
fi
echo "--> [PASS] Verified SHA-256 digest: ${ACTUAL_HASH}"

mkdir -p "$INSTALL_DIR"
if [ "$ARCHIVE_EXT" = ".zip" ]; then
  unzip -qo "$ARCHIVE_PATH" -d "$INSTALL_DIR"
else
  tar -xzf "$ARCHIVE_PATH" -C "$INSTALL_DIR"
fi
chmod +x "${INSTALL_DIR}/cddm"* 2>/dev/null || true

echo "--> Successfully installed CDDM ${RELEASE_TAG} to: ${INSTALL_DIR}"
echo ""
echo "Please ensure ${INSTALL_DIR} is in your PATH environment variable:"
echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
echo ""
echo "Run 'cddm --help' to get started."
