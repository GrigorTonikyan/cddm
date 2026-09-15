# syntax=docker/dockerfile:1
# Multi-stage modular Dockerfile for CDDM (Code De-Duplication Meister)
# Uses least-specific image tags and builds cleanly on VCS Action runners.

# Stage 1: Build WebUI distribution bundle
FROM oven/bun:latest AS webui-builder
WORKDIR /app
COPY webui/package.json ./webui/
COPY package.json ./
RUN cd webui && bun install --frozen-lockfile || bun install
COPY webui ./webui
RUN cd webui && bun run build

# Stage 2: Build Rust release binaries (cddm-cli, cddm-mcp, cddm-lsp)
FROM rust:latest AS rust-builder
WORKDIR /usr/src/cddm

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY --from=webui-builder /app/webui/dist ./webui/dist

RUN cargo build --release \
    -p cddm-cli --bin cddm \
    -p cddm-mcp --bin cddm-mcp \
    -p cddm-lsp --bin cddm-lsp

# Stage 3: Minimal production runtime container
FROM debian:latest AS runtime
LABEL maintainer="CDDM Development Team <gt-dev@gt-web-dev.com>"
LABEL org.opencontainers.image.source="https://git.gt-web-dev.com/gt-dev/cddm"
LABEL org.opencontainers.image.description="Polyglot Code De-Duplication Meister & Architectural Governance Engine"

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1001 cddm && \
    useradd -u 1001 -g cddm -m -s /bin/bash cddm

COPY --from=rust-builder /usr/src/cddm/target/release/cddm /usr/local/bin/cddm
COPY --from=rust-builder /usr/src/cddm/target/release/cddm-mcp /usr/local/bin/cddm-mcp
COPY --from=rust-builder /usr/src/cddm/target/release/cddm-lsp /usr/local/bin/cddm-lsp

USER cddm
WORKDIR /workspace

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/cddm"]
CMD ["serve", "--port", "8080", "--host", "0.0.0.0"]
