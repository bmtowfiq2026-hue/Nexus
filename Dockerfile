# Stage 1: Build Rust core + CLI
FROM rust:1.86-slim-bookworm AS rust-builder
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml core/
COPY cli/Cargo.toml cli/
RUN mkdir -p core/src cli/src && echo "fn main() {}" > cli/src/main.rs && touch core/src/lib.rs
RUN cargo build --release 2>/dev/null || true
COPY core/src core/src
COPY cli/src cli/src
RUN cargo build --release

# Stage 2: Build Go gateway
FROM golang:1.22-bookworm AS go-builder
WORKDIR /app
COPY gateway/go.mod gateway/go.sum ./
COPY gateway/ ./gateway/
RUN cd gateway && go build -o /nexus-gateway .

# Stage 3: Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /app/target/release/nexus /usr/local/bin/nexus
COPY --from=go-builder /nexus-gateway /usr/local/bin/nexus-gateway
EXPOSE 8080
ENTRYPOINT ["nexus"]
CMD ["--help"]
