# syntax=docker/dockerfile:1
# Multi-stage BuildKit-accelerated Dockerfile for CDDM (Code De-Duplication Meister)
# All external base images are pinned to immutable @sha256: digests.

ARG BUN_IMAGE=oven/bun:1.4.4-slim@sha256:1846adf22496bd711036aae8be147e57bdebbaea2e5264e3e0b5af1db187d9a4
ARG NODE_IMAGE=node:26-bookworm-slim@sha256:5df79989ed685e09f0a98019db4cebfb22fb9df1a53c0161664a011f7ff6b0f1
ARG RUST_IMAGE=rust:bookworm@sha256:ade39a710ef85f3506cffcfec3e8a24ba090badef1dff2fb798f71d878833fea
ARG RUNTIME_IMAGE=debian:bookworm-slim@sha256:734a72b3d6142a71993b4bc580ca01d8ca687f77a2c422d96a961d3376b179bc

# Stage 0: Verified Bun binary source
FROM ${BUN_IMAGE} AS bun-source

# Stage 1: Build WebUI distribution bundle (unified Node.js LTS + Bun builder)
FROM ${NODE_IMAGE} AS webui-builder
WORKDIR /app
COPY --from=bun-source /usr/local/bin/bun /usr/local/bin/bun
COPY package.json ./
COPY webui/package.json ./webui/
RUN --mount=type=cache,id=cddm-bun-cache,target=/root/.bun/install/cache,sharing=locked \
    cd webui && (bun install --frozen-lockfile || bun install)
COPY webui ./webui
RUN cd webui && bun run build

# Stage 2: Build Rust release binaries (cddm, cddm-mcp, cddm-lsp)
FROM ${RUST_IMAGE} AS rust-builder
WORKDIR /usr/src/cddm

RUN --mount=type=secret,id=internal_ca,required=false \
    --mount=type=cache,id=cddm-apt-builder,target=/var/cache/apt,sharing=locked \
    echo "force-unsafe-io" > /etc/dpkg/dpkg.cfg.d/force-unsafe-io && \
    apt-get update -qq && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      pkg-config \
      libssl-dev && \
    if [ -s /run/secrets/internal_ca ]; then \
      cp /run/secrets/internal_ca /usr/local/share/ca-certificates/gt-internal-ca.crt && \
      update-ca-certificates; \
    fi && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml README.md ./
COPY crates ./crates
COPY --from=webui-builder /app/webui/dist ./webui/dist

RUN --mount=type=cache,id=cddm-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cddm-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=cddm-cargo-target,target=/usr/src/cddm/target,sharing=locked \
    cargo build --release --locked \
      -p cddm-cli --bin cddm \
      -p cddm-mcp --bin cddm-mcp \
      -p cddm-lsp --bin cddm-lsp && \
    mkdir -p /out && \
    cp target/release/cddm target/release/cddm-mcp target/release/cddm-lsp /out/

# Stage 3: Minimal non-root production runtime container
FROM ${RUNTIME_IMAGE} AS runtime
LABEL org.opencontainers.image.title="CDDM" \
      org.opencontainers.image.description="Polyglot Code De-Duplication Meister & Architectural Governance Engine" \
      org.opencontainers.image.vendor="gt-dev" \
      org.opencontainers.image.source="https://git.gt-web-dev.com/gt-dev/cddm"

RUN --mount=type=cache,id=cddm-apt-runtime,target=/var/cache/apt,sharing=locked \
    echo "force-unsafe-io" > /etc/dpkg/dpkg.cfg.d/force-unsafe-io && \
    apt-get update -qq && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      git \
      curl && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd --system --gid 1001 cddm && \
    useradd --system --uid 1001 --gid cddm --create-home --shell /bin/bash cddm

COPY --link --from=rust-builder /out/cddm /usr/local/bin/cddm
COPY --link --from=rust-builder /out/cddm-mcp /usr/local/bin/cddm-mcp
COPY --link --from=rust-builder /out/cddm-lsp /usr/local/bin/cddm-lsp

USER cddm
WORKDIR /workspace

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD ["curl", "-fsS", "http://127.0.0.1:8080/api/health"]

ENTRYPOINT ["/usr/local/bin/cddm"]
CMD ["serve", "--port", "8080", "--host", "0.0.0.0"]
