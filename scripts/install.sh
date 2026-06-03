#!/usr/bin/env sh
# Nexus Install Script (macOS/Linux/WSL2)
# Usage: git clone https://github.com/bmtowfiq2026-hue/Nexus.git ~/.nexus-repo && bash ~/.nexus-repo/scripts/install.sh

set -e

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "${CYAN}${BOLD}== Installing Nexus ==${NC}"
echo ""

# Detect platform
case "$(uname -s)" in
  Darwin)  OS="macos" ;;
  Linux)   OS="linux" ;;
  *)       echo "Unsupported OS: $(uname -s)"; exit 1 ;;
esac

case "$(uname -m)" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)            echo "Unsupported architecture: $(uname -m)"; exit 1 ;;
esac

echo "${YELLOW}Detected:${NC} $OS ($ARCH)"

# Install Rust if missing
if ! command -v cargo >/dev/null 2>&1; then
  echo ""
  echo "${YELLOW}Installing Rust...${NC}"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"
  echo "${GREEN}Rust installed${NC}"
} else {
  echo "${GREEN}Rust found: $(cargo --version)${NC}"
fi

# Clone or update repo
NEXUS_DIR="$HOME/.nexus-repo"
if [ -d "$NEXUS_DIR/.git" ]; then
  echo "${YELLOW}Updating Nexus repository...${NC}"
  cd "$NEXUS_DIR" && git pull --ff-only
else
  echo "${YELLOW}Cloning Nexus repository...${NC}"
  git clone --depth 1 https://github.com/bmtowfiq2026-hue/Nexus.git "$NEXUS_DIR"
fi

# Build
echo "${YELLOW}Building Nexus (this may take a few minutes)...${NC}"
cd "$NEXUS_DIR"
cargo build --release 2>&1 | tail -1

# Install binary
cp "$NEXUS_DIR/target/release/nexus-cli" "$HOME/.nexus-bin" 2>/dev/null || mkdir -p "$HOME/.nexus-bin"
cp "$NEXUS_DIR/target/release/nexus-cli" "$HOME/.nexus-bin/nexus"
chmod +x "$HOME/.nexus-bin/nexus"

# Add to PATH
SHELL_PROFILE=""
case "$SHELL" in
  */zsh) SHELL_PROFILE="$HOME/.zshrc" ;;
  */bash) 
    if [ -f "$HOME/.bash_profile" ]; then SHELL_PROFILE="$HOME/.bash_profile"
    elif [ -f "$HOME/.bashrc" ]; then SHELL_PROFILE="$HOME/.bashrc"
    fi ;;
esac

if echo "$PATH" | grep -q "$HOME/.nexus-bin"; then
  echo "${GREEN}Already in PATH${NC}"
else
  if [ -n "$SHELL_PROFILE" ]; then
    echo 'export PATH="$HOME/.nexus-bin:$PATH"' >> "$SHELL_PROFILE"
    echo "${GREEN}Added to PATH in $SHELL_PROFILE${NC}"
  else
    echo "${YELLOW}Add to PATH manually: export PATH=\"\$HOME/.nexus-bin:\$PATH\"${NC}"
  fi
  export PATH="$HOME/.nexus-bin:$PATH"
fi

# Init workspace
echo ""
nexus init 2>&1
echo ""

echo "${CYAN}${BOLD}Nexus installed!${NC}"
echo ""
echo "  ${GREEN}nexus chat${NC}        Start chatting (demo mode)"
echo "  ${GREEN}nexus doctor${NC}      Check system health"
echo "  ${GREEN}nexus onboard${NC}     Guided setup wizard"
echo ""
echo "  ${YELLOW}Quick start:${NC}"
echo "    ${GREEN}nexus chat${NC}"
echo ""
