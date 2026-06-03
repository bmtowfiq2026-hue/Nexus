# Installation

## Prerequisites

| Dependency | Version | Install |
|------------|---------|---------|
| Rust | 1.86+ | [rustup.rs](https://rustup.rs) |
| Go | 1.22+ (optional) | [go.dev](https://go.dev/dl) |

## Option 1: Build from Source (Recommended)

```bash
git clone https://github.com/nexus/nexus.git
cd nexus
cargo build --release

# Verify
./target/release/nexus --version
./target/release/nexus init
./target/release/nexus chat    # Starts in demo mode
```

**Build times:** ~1-2 min debug, ~5 min release (first build downloads dependencies).

## Option 2: Docker

```bash
docker compose up nexus
```

## Option 3: Cargo Install (Coming Soon)

```bash
cargo install nexus-cli
```

## Post-Installation

```bash
# 1. Initialize workspace
nexus init

# 2. Check system health
nexus doctor

# 3. Start chatting (no API keys needed)
nexus chat

# 4. Or run the setup wizard
nexus onboard
```

## Platform Notes

### Windows
```powershell
# Use PowerShell or Windows Terminal
# If MSVC linker fails, try GNU:
rustup default stable-x86_64-pc-windows-gnu
```

### macOS
```bash
# You may need Xcode CLI tools:
xcode-select --install
```

### Linux
```bash
# Debian/Ubuntu:
sudo apt install pkg-config libssl-dev build-essential

# Fedora:
sudo dnf install pkg-config openssl-devel
```
