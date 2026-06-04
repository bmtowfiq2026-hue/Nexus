# Security

Nexus is designed with a **local-first, privacy-respecting** security model. Your conversations, memory, and data stay on your machine unless you explicitly configure otherwise.

## Default Security Posture

| Property | Default | Configurable |
|----------|---------|-------------|
| Data storage | Local only (`~/.nexus/`) | Yes |
| Network access | Sandbox blocked | Yes |
| API keys | In config file or env vars | Yes |
| Execution sandbox | Enabled (512MB max) | Yes |
| Audit trail | Coming | Future |

## API Keys

Nexus supports multiple ways to provide API keys:

### 1. Environment Variables (recommended for security)
```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GEMINI_API_KEY="AIza..."
export DEEPSEEK_API_KEY="sk-..."
export GROQ_API_KEY="gsk_..."
```

### 2. Config File (persistent)
```bash
nexus config set api_keys.openai "sk-..."
nexus config set api_keys.anthropic "sk-ant-..."
```

Keys stored in `~/.nexus/nexus.json` are masked in `config show` output. Environment variables take precedence over config file values.

All 24 providers use the `<PROVIDER>_API_KEY` naming convention (e.g. `PERPLEXITY_API_KEY`, `MISTRAL_API_KEY`, `XAI_API_KEY`).

### 3. Onboarding Wizard
```bash
nexus onboard
```

## Sandbox

The sandbox isolates agent command execution:

```json
{
  "sandbox": {
    "enabled": true,
    "max_memory_mb": 512,
    "max_cpu_cores": 2,
    "network_access": false
  }
}
```

- **Disabled**: Agent commands run on the host with full access
- **Enabled (default)**: Commands run with resource limits
- **Network blocked**: Prevents data exfiltration by default

## Channel Security

When using the Go gateway with messaging channels:

- **DM Pairing** (recommended): Unknown senders receive a pairing code
- **Allow lists**: Restrict which users/channels can interact
- **Token-based auth**: Each channel uses its own bot token

See [CHANNELS.md](CHANNELS.md) for per-channel security configuration.

## Privacy Modes (Coming)

| Mode | Data Storage | Network | Sharing |
|------|-------------|---------|---------|
| **Air-gapped** | Local only | Blocked | None |
| **Local-first** | Local | LLM only | None |
| **Selective** | Local + Cloud | LLM + Sync | Opt-in |
| **Full** | Cloud | All | Anonymized |

## Reporting Vulnerabilities

Report vulnerabilities to the project maintainers via GitHub Issues (for non-critical) or directly (for critical). We aim to respond within 48 hours.

**Do not** post critical security issues publicly.

## Best Practices

1. **Use environment variables** for API keys when possible
2. **Enable sandbox** when running untrusted agent commands
3. **Keep your config file** permissions restricted (`chmod 600 ~/.nexus/nexus.json`)
4. **Review channel allowlists** before enabling public DM access
5. **Run `nexus doctor`** periodically to check for misconfigurations
