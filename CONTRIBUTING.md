# Contributing to Nexus

Thank you for considering contributing to Nexus! We welcome contributions of all shapes and sizes.

## How It Works

Nexus uses a **fork + pull request** workflow. All changes must be reviewed and approved before merging.

```
1. Fork the repo on GitHub
2. Create a feature branch in your fork
3. Make changes, commit, push
4. Open a Pull Request to bmtowfiq2026-hue/Nexus
5. Maintainer reviews, requests changes if needed
6. Maintainer approves and merges
```

> **You do not have merge access.** Only the maintainer merges PRs after review. This ensures quality and consistency.

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt` (Rust), `gofmt -s -w .` (Go)
6. Lint: `cargo clippy -- -D warnings` (Rust), `go vet ./...` (Go)
7. Commit with a clear message
8. Push and open a Pull Request

## Development Setup

### Prerequisites

- **Rust 1.86+** — install via [rustup](https://rustup.rs)
- **Go 1.22+** — install via [go.dev](https://go.dev/dl)
- **Windows:** MinGW-w64 or MSVC build tools

### Build & Test

```bash
# Build the core runtime + CLI
cargo build

# Run all tests
cargo test

# Lint
cargo clippy -- -D warnings
cargo fmt --check

# Build the gateway
cd gateway && go build -o nexus-gateway .
cd gateway && go vet ./...
```

## Code Style

- **Rust:** follow `rustfmt` and `clippy` — run `cargo fmt` before committing. No `#[allow(...)]` without a comment explaining why.
- **Go:** follow `gofmt` — `gofmt -s -w .` before committing. Run `go vet ./...` to catch issues.
- **Error handling:** never use `.unwrap()` or `.expect()` in library code — propagate errors with `?` or `anyhow`.
- **Comments:** explain **why**, not what. The code says what.
- No commented-out code. Delete it or explain it in a PR comment.

## Commit Messages

Use conventional commits:

```
feat: add graph memory entity extraction
fix: handle empty trajectory in skill extractor
docs: update README with channel configuration
```

## Pull Request Process

1. **Update documentation** if you change public API
2. **Add tests** for new functionality
3. Ensure all **CI checks pass** (check, test, fmt, clippy, build)
4. Fill out the **PR template** fully
5. A maintainer will review your PR within a few days
6. Address review feedback by pushing new commits to the same branch
7. Once approved, the maintainer will merge (usually with squash)

### What Gets Reviewed

- Correctness — does the code do what it claims?
- Tests — are there tests for new functionality?
- Style — does it follow project conventions?
- Scope — is the change focused? (Small PRs are reviewed faster.)

### What Won't Be Merged

- PRs that break CI without explanation
- PRs with commented-out code, debug prints, or secrets
- PRs that add dependencies without justification
- PRs that change formatting across unrelated files

## Good First Issues

Look for issues labeled [`good-first-issue`](https://github.com/bmtowfiq2026-hue/Nexus/labels/good-first-issue) — these are small, self-contained, and well-documented. Great places to start:

- **Add a new provider** — copy `openai_compat.rs`, fill in the config, done.
- **Add a new channel** — implement the `Channel` interface in Go.
- **Fix a warning** — address clippy warnings in the codebase.

## Need Help?

Open a [Discussion](https://github.com/bmtowfiq2026-hue/Nexus/discussions) for questions, or mention `@bmtowfiq2026-hue` in your PR for a faster response.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
