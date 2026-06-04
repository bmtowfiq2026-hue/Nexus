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
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "OPENAI_API_KEY not set" or provider key not found

```bash
# Option 1: Use demo mode (no setup needed)
nexus chat

# Option 2: Use Ollama (free, local)
nexus chat --provider ollama

# Option 3: Run the setup wizard
nexus onboard

# Option 4: Set the matching env var
# Replace with your provider: GEMINI, DEEPSEEK, GROQ, etc.
nexus config set api_keys.gemini "AIza..."
nexus chat --provider gemini
```

### "Connection refused" when using Ollama

Ollama is not running.

```bash
ollama serve

# Verify
curl http://localhost:11434/api/tags

# Or use nexus doctor
nexus doctor
```

### Provider returns "401 Unauthorized" or empty response

Your API key may be wrong or the provider requires a different base URL.

```bash
# Check your configured keys (masked)
nexus config show

# Verify the key is set in env
echo $GEMINI_API_KEY  # macOS/Linux
echo $env:GEMINI_API_KEY  # Windows

# Run doctor to see which providers are configured
nexus doctor

# For local providers (LM Studio, LocalAI, oobabooga),
# make sure the server is running on the expected port
```

### Build fails on Windows (linker errors)

```bash
rustup default stable-x86_64-pc-windows-gnu
```

### Build fails on macOS

```bash
xcode-select --install

# If OpenSSL errors
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

### Docker build fails

The Dockerfile uses multi-stage builds and requires `pkg-config` + `libssl-dev` in the Rust builder stage. These are installed automatically, but if you see OpenSSL errors:

```bash
# The Dockerfile already includes these, but make sure your Docker
# environment has network access to download apt packages.
docker build --no-cache -t nexus .
```

### Gateway won't start

```bash
# Check if port is in use
netstat -ano | findstr :8080   # Windows
lsof -i :8080                  # macOS/Linux

# Validate gateway.json syntax
cd gateway
nexus-gateway   # Shows startup errors on stderr

# Try verbose mode
NEXUS_GATEWAY_VERBOSE=1 ./nexus-gateway
```

### Agent responses are slow

```bash
# Check which provider you're using
nexus config show

# For faster responses, use a smaller model:
nexus chat --provider ollama --model llama3:7b

# Or use a faster provider like Groq (LPU inference):
nexus chat --provider groq --model llama3-70b-8192
```

### WebSocket chat doesn't connect

The gateway must be running on port 8080.

```bash
# Start the gateway
cd gateway && ./nexus-gateway

# Verify WebChat is enabled in gateway.json
# "webchat": { "enabled": true, "path": "/ws" }

# Open http://localhost:8080/ in your browser
```

### Webhook channels (WhatsApp, LINE, Messenger, Twilio) not working

These platforms require HTTPS webhook URLs. For local development:

```bash
# Install ngrok from https://ngrok.com
ngrok http 8080
# Output: https://abc123.ngrok.io -> http://localhost:8080
```

Then configure each platform's webhook URL as `https://abc123.ngrok.io/webhook/<channel>`.

Verify the webhook is reachable:
```bash
curl -k https://abc123.ngrok.io/health
```

### Memory / disk space

Nexus stores data in `~/.nexus/memory/`. To clear all memory:

```bash
# WARNING: This deletes all conversations and learned skills
rm -rf ~/.nexus/memory
nexus init  # Recreate workspace
```

### CI / Release workflow not triggering

Push a version tag to trigger the automated release pipeline:

```bash
git tag v0.6.0
git push origin v0.6.0
```

Check progress at https://github.com/bmtowfiq2026-hue/Nexus/actions.

### GitHub release binary not available

If the CI workflow failed or you're on a platform without a pre-built binary, build from source:

```bash
cargo build --release
./target/release/nexus --version
```

The release workflow builds Linux, macOS, and Windows binaries and publishes a Docker image to `ghcr.io/bmtowfiq2026-hue/nexus`.

## Getting Help

1. Run `nexus doctor` for a system health check
2. Open an issue at https://github.com/bmtowfiq2026-hue/Nexus/issues

## Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug nexus chat

# Enable trace logging (very verbose)
RUST_LOG=trace nexus chat

# For gateway debugging
NEXUS_GATEWAY_VERBOSE=1 ./nexus-gateway
```
