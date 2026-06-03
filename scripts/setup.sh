#!/usr/bin/env bash
# Nexus Setup Script (Linux/macOS)
# Usage: bash scripts/setup.sh

set -euo pipefail

echo "🦞 Nexus Setup"

# Check Rust
if ! command -v cargo &>/dev/null; then
    echo "✗ Rust not found. Install from https://rustup.rs"
    exit 1
fi
echo "✓ Rust: $(cargo --version)"

# Check Go
if ! command -v go &>/dev/null; then
    echo "✗ Go not found. Install from https://go.dev/dl"
    exit 1
fi
echo "✓ Go: $(go version)"

# Build
echo ""
echo "Building Nexus..."
cargo build --release
echo "✓ Build complete"

# Initialize
NEXUS_DIR="${HOME}/.nexus"
if [ ! -d "$NEXUS_DIR" ]; then
    ./target/release/nexus init --path "$NEXUS_DIR"
    echo "✓ Workspace initialized at $NEXUS_DIR"
fi

echo ""
echo "Nexus is ready!"
echo "  Run: nexus chat"
