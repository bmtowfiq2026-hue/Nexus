# Contributing to Nexus

Thank you for considering contributing to Nexus! We welcome contributions of all shapes and sizes.

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Commit with a clear message
6. Push and open a Pull Request

## Development Setup

### Prerequisites

- **Rust 1.96+** — install via [rustup](https://rustup.rs)
- **Go 1.22+** — install via [go.dev](https://go.dev/dl)
- **Windows:** MinGW-w64 or MSVC build tools

### Build & Test

```bash
# Build the core runtime + CLI
cargo build

# Run all tests
cargo test

# Build the gateway
cd gateway && go build -o nexus-gateway .
```

## Code Style

- Rust: follow `rustfmt` and `clippy` — run `cargo fmt` before committing
- Go: follow `gofmt` — `gofmt -s -w .`
- Comments: explain **why**, not what. The code says what.
- No commented-out code. Delete it or explain it in a PR comment.

## Commit Messages

Use conventional commits:

```
feat: add graph memory entity extraction
fix: handle empty trajectory in skill extractor
docs: update README with channel configuration
```

## Pull Request Process

1. Update documentation if you change public API
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Request review from at least one maintainer
5. Squash commits before merge

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
