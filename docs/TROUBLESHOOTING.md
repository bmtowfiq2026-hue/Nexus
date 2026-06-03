# Troubleshooting

## Common Issues

### "nexus: command not found"

The binary is not in your PATH.

```bash
# Add to PATH (Linux/macOS)
export PATH="$PATH:$HOME/.cargo/bin"
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc

# Add to PATH (Windows - PowerShell)
$env:Path += ";$env:USERPROFILE\.cargo\bin"
[Environment]::SetEnvironmentVariable("Path", "$env:Path;$env:USERPROFILE\.cargo\bin", "User")
```

### "cargo: command not found"

Rust is not installed.

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "OPENAI_API_KEY not set"

You need to provide API credentials for the LLM provider.

```bash
# Option 1: Use demo mode (no setup needed)
nexus chat

# Option 2: Use Ollama (free, local)
nexus chat --provider ollama

# Option 3: Set your API key
export OPENAI_API_KEY="sk-..."
nexus chat --provider openai
```

### "Connection refused" when using Ollama

Ollama is not running.

```bash
# Start Ollama
ollama serve

# Verify it's running
curl http://localhost:11434/api/tags

# Or use nexus doctor to check
nexus doctor
```

### Build fails on Windows

If you get linker errors, try the GNU toolchain:

```bash
rustup default stable-x86_64-pc-windows-gnu
```

### Build fails on macOS

```bash
# Install required system libraries
xcode-select --install

# Or if OpenSSL errors occur
brew install pkg-config openssl
```

### Build fails on Linux

```bash
# Debian/Ubuntu
sudo apt update
sudo apt install pkg-config libssl-dev build-essential

# Fedora/RHEL
sudo dnf install pkg-config openssl-devel gcc
```

### Gateway won't start

```bash
# Check if port is in use
netstat -ano | findstr :8080   # Windows
lsof -i :8080                  # macOS/Linux

# Check gateway.json syntax
cd gateway
nexus-gateway   # Will show startup errors

# Try verbose mode
./nexus-gateway --verbose
```

### Agent responses are slow

```bash
# Check which provider you're using
nexus config show | grep default_provider

# For faster responses, use a smaller model:
nexus chat --provider ollama --model llama3:7b

# Or use GPT-4o-mini instead of GPT-4o:
nexus chat --provider openai --model gpt-4o-mini
```

### Memory / disk space

Nexus stores data in `~/.nexus/memory/`. To clear all memory:

```bash
# WARNING: This deletes all conversations and learned skills
rm -rf ~/.nexus/memory
```

## Getting Help

1. Run `nexus doctor` for a system health check
2. Run `nexus doctor` to detect available providers
3. Open an issue at https://github.com/nexus/nexus/issues
4. Join the community Discord

## Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug nexus chat

# Enable trace logging (very verbose)
RUST_LOG=trace nexus chat

# For gateway debugging
NEXUS_GATEWAY_VERBOSE=1 ./nexus-gateway
```
