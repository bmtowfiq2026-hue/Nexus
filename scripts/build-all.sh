#!/usr/bin/env sh
# Nexus Build-All Script (macOS/Linux/WSL2)
# Builds both the Rust CLI and Go gateway
set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "${CYAN}${BOLD}== Building Nexus ==${NC}"
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
echo ""

# 1. Build Rust CLI
echo "${YELLOW}[1/2] Building Rust CLI...${NC}"
cd "$ROOT_DIR"
cargo build --release 2>&1 | tail -3
if [ -f "$ROOT_DIR/target/release/nexus" ]; then
    echo "  ${GREEN}nexus binary:${NC} $ROOT_DIR/target/release/nexus"
else
    echo "  ${RED}Rust build failed${NC}"
    exit 1
fi

# 2. Build Go gateway
echo ""
echo "${YELLOW}[2/2] Building Go gateway...${NC}"
if command -v go >/dev/null 2>&1; then
    cd "$ROOT_DIR/gateway"
    go build -o "$ROOT_DIR/target/release/nexus-gateway" .
    echo "  ${GREEN}gateway binary:${NC} $ROOT_DIR/target/release/nexus-gateway"
else
    echo "  ${YELLOW}Go not found. Installing...${NC}"
    echo "  Install Go manually from https://go.dev/dl, then run:"
    echo "    cd gateway && go build -o ../target/release/nexus-gateway ."
    exit 1
fi

echo ""
echo "${GREEN}${BOLD}Build complete!${NC}"
echo ""
echo "  ${GREEN}nexus${NC}           Rust CLI agent"
echo "  ${GREEN}nexus-gateway${NC}   Go multi-channel gateway"
echo ""
echo "Run:"
echo "  ./target/release/nexus start"
