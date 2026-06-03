# Installation Guide

## Prerequisites

| Dependency | Version | Install |
|------------|---------|---------|
| Rust | 1.96+ | https://rustup.rs |
| Go | 1.22+ | https://go.dev/dl |
| Git | any | https://git-scm.com |

## Option 1: Pre-built Binaries

Download from [GitHub Releases](https://github.com/nexus/nexus/releases):

```bash
# Linux/macOS
curl -L https://github.com/nexus/nexus/releases/latest/download/nexus-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv nexus /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri https://github.com/nexus/nexus/releases/latest/download/nexus-x86_64-pc-windows-msvc.zip -OutFile nexus.zip
Expand-Archive nexus.zip -DestinationPath .
```

## Option 2: Build from Source

```bash
git clone https://github.com/nexus/nexus.git
cd nexus

# Build the Rust core + CLI
cargo build --release

# Build the Go gateway (optional)
cd gateway
go build -o nexus-gateway .
cd ..

# (Optional) Add to PATH
echo 'export PATH="$PATH:'$(pwd)'/target/release"' >> ~/.bashrc
```

### Build Times

| Component | Debug | Release |
|-----------|-------|---------|
| Core + CLI | ~1 min | ~5 min |
| Gateway | ~5 sec | ~10 sec |

## Option 3: Cargo Install (coming soon)

```bash
cargo install nexus-cli
```

## Post-Installation

```bash
# Verify installation
nexus --help

# Initialize workspace
nexus init

# Set your LLM provider API key
export OPENAI_API_KEY="sk-..."
# or
export ANTHROPIC_API_KEY="sk-ant-..."
# or use local Ollama (no key needed)

# Start chatting
nexus chat
```

## Platform-Specific Notes

### Windows

Use **PowerShell** or **Windows Terminal**. If building with MSVC fails, try the GNU toolchain:

```bash
rustup default stable-x86_64-pc-windows-gnu
```

### macOS

If you get code signing warnings, run:
```bash
xattr -d com.apple.quarantine /usr/local/bin/nexus
```

### Linux

You may need `pkg-config` and `libssl-dev`:
```bash
sudo apt install pkg-config libssl-dev   # Debian/Ubuntu
sudo dnf install pkg-config openssl-devel  # Fedora
```

## Troubleshooting

**"ld: cannot find crt2.o"** — Install MinGW-w64:
```bash
# Windows
scoop install mingw
```

**"cargo: error while loading shared libraries"** — Update rustup:
```bash
rustup update
```

**Gateway connection refused** — Ensure the gateway is running before connecting channels:
```bash
cd gateway && ./nexus-gateway.exe
```
