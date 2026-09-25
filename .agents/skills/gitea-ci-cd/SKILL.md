---
name: gitea-ci-cd
description: >-
  Authoritative Single Source of Truth (SSoT) operational skill and standard runbooks
  for Gitea Actions CI/CD pipelines, Runner 4.0.0 architecture (builtin:checkout,
  self: action refs, doublestar volumes, umask, OTel tracing), dual-tier runner routing,
  zero-trust loopback proxying, and multi-repo quality gate workflows.
---

# Gitea Actions CI/CD Skill (`gitea-ci-cd`)

This skill defines the authoritative **Single Source of Truth (SSoT)** operational standards, runbooks, and workflow templates for running **Gitea Actions CI/CD pipelines** on the **Gitea Enterprise Stack** and **TrueNAS SCALE 26.0**.

---

## 1. Stack Topology & Zero-Trust Ingress Loopback

The Gitea CI/CD infrastructure enforces a 3-tier zero-trust network boundary with high-throughput local container registry loopback:

```text
               Internet (Clients / Git Push)
                             │
                             ▼ [QUIC / HTTP3]
                   [ cloudflared Tunnel ]
                             │
                      (gitea-ingress)
                             │
                             ▼
                     [ gitea-proxy:443 ] (client_max_body_size 0;)
                             │
                             ▼
                     [ gitea:30008 ] ◄──────────┐
                   /                 \          │ (Direct Local Loopback: 500+ MB/s,
        (gitea-backend)           (gitea-egress)│  Bypasses Cloudflare 100MB cap)
          /         \                   │       │
         ▼           ▼                  ▼       │
    [postgres]    [redis]      [ CI/CD Runners ]┘
                                (172.16.12.100)
```

### Key Architectural Tenets

1. **Zero-Cost 413 Bypass (`172.16.12.100`)**:
   - Cloudflare Free and Pro plans hard-cap HTTP request bodies to 100 MB.
   - Gitea Actions runners on `gitea-egress` resolve `git.gt-web-dev.com` directly to `172.16.12.100` (`gitea-proxy`) via `--add-host`.
   - Nginx handles TLS termination and enforces `client_max_body_size 0;`, uploading large container images at physical SSD/RAM speeds (500+ MB/s) with **zero WAN bandwidth** and **zero Cloudflare fees**.
2. **Network Isolation**:
   - Runners exist strictly on `gitea-egress` (`172.16.12.0/24`).
   - Runners have **zero network access** to PostgreSQL (`gitea-backend`) or Redis.
3. **SSH Ingress (`30009:30009`)**:
   - Exposes Gitea rootless SSH on host port `30009` for secure Git deploy-key pulls from PaaS runners (e.g. Coolify) without token interpolation.

---

## 2. Dual-Tier Runner Topology & Workload Routing

The cluster maintains two dedicated runner tiers:

| Dimension                      | Standard Runner (`gitea-runner-standard`)   | High-Performance Runner (`gitea-runner-highperf`)                            |
| :----------------------------- | :------------------------------------------ | :--------------------------------------------------------------------------- |
| **Compute Limit**              | `4` vCPUs (CFS quota)                       | `12` vCPUs (CFS quota)                                                       |
| **Memory Limit**               | `8192M` (8 GB)                              | `16384M` (16 GB)                                                             |
| **Concurrency (`capacity`)**   | `2` concurrent jobs                         | `1` dedicated thread (100% unshared compute)                                 |
| **Shared Memory (`shm_size`)** | `2G`                                        | `4G`                                                                         |
| **Docker Socket Isolation**    | `docker_host: "-"` (isolated)               | `docker_host: ""` (native DinD socket mounted)                               |
| **Target Labels**              | `ubuntu-latest`, `ubuntu-24.04`, `standard` | `ubuntu-latest-12-cores`, `ubuntu-latest-8-cores`, `highperf`, `heavy-build` |

### Workload Placement Rules

```yaml
# Standard Lightweight Jobs: Linting, Unit Tests, Typecheck, Security Scans
jobs:
  lint-and-test:
    runs-on: ubuntu-latest

# Heavy Compilation Jobs: Rust builds, Webpack/esbuild bundling, Multi-arch Docker builds
jobs:
  compile-and-publish:
    runs-on: ubuntu-latest-12-cores # or runs-on: highperf
```

---

## 3. Gitea Runner 4.0.0 Architecture & Standards

All repositories on `git.gt-web-dev.com` MUST adhere to the Runner 4.0.0 capabilities and standards:

### 1. Native Built-In Actions (`builtin:checkout`)

Runner 4.0.0 introduces native compiled Go actions that execute directly within the runner binary:

```yaml
steps:
  - name: Checkout Repository
    uses: builtin:checkout
    with:
      fetch-depth: 0
```

- **Zero Action Download**: Skips git cloning `actions/checkout` on every run.
- **Zero Node.js Dependency**: Runs in scratch, Alpine, distroless, or minimal Python/Rust images without requiring Node.js.
- **Supported Parameters**: `repository`, `ref`, `token`, `path`, `fetch-depth`.

### 2. Instance-Relative `self:` Action References

Reference shared internal composite actions across repositories on `git.gt-web-dev.com` without hardcoding domain names or embedding tokens:

```yaml
steps:
  - name: Setup Enterprise Toolchains
    uses: self:gt-dev/actions/setup-toolchains@v1
```

### 3. In-Runner Cache Proxying

- In Runner 4.0.0, job containers send cache requests to the parent runner daemon, which proxies to local disk or S3.
- `container.network` MUST remain `""` (empty) in runner configs so jobs get an isolated bridge network that the runner daemon automatically joins.
- Never set `container.network: bridge` in job specifications.

### 4. Doublestar Volume Matching

In `valid_volumes`, single `*` matches strictly within one path segment. Recursive path matching requires `**`:

```yaml
container:
  valid_volumes:
    - /opt/hostedtoolcache/**
    - /opt/hostedtoolcache
    - /etc/docker/certs.d/git.gt-web-dev.com/ca.crt
    - /etc/ssl/certs/ca-certificates.crt
```

### 5. Container Umask Enforcement

Set `--umask=0022` in `container.options` to ensure files created in host dataset mounts maintain standard permissions (`0755` dirs, `0644` files) readable by TrueNAS `apps` user (`568`).

---

## 4. Mandatory Repository CI/CD Rules (STRICT MANDATE)

1. **Workflow Directory Standard**:
   - Workflows MUST reside strictly in `.gitea/workflows/*.yaml`.
   - Never use `.github/workflows/` on Gitea SSoT repositories.
   - Use `.yaml` extension exclusively (`.yml` is deprecated).
2. **Absolute Ban on `services: docker:dind`**:
   - The runners are already full Docker-in-Docker daemons with internal CA certificates mounted at `/etc/docker/certs.d/git.gt-web-dev.com/ca.crt`.
   - Workflows that declare `services: docker: image: docker:dind` spawn an unconfigured sidecar that lacks local CA trust, causing `x509: certificate signed by unknown authority`.
   - Always rely on the runner's native Docker socket.
3. **Absolute Ban on Credentials in URLs**:
   - Never interpolate tokens or credentials inside URLs (`https://${TOKEN}@git...`). Secret transmission MUST occur via HTTP headers or SSH keys.
4. **Zero Inline Scripts**:
   - Never write inline throwaway scripts (`python -c "..."`, `node -e "..."`). Use standard CLI tools (`forge`) or reusable scripts.
5. **Deterministic Merge Gates**:
   - All PR workflows MUST define an explicit `merge-gate` job that aggregates status dependencies.

---

## 5. Golden Workflow Templates

### Template A: Web & Modern Full-Stack CI Quality Gate

```yaml
name: CI & Quality Gate

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  security-scan:
    name: Gitleaks Secret Scan
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Repository
        uses: builtin:checkout
        with:
          fetch-depth: 0

      - name: Install Pinned Gitleaks
        run: |
          GITLEAKS_VERSION="8.30.1"
          GITLEAKS_SHA256="551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb"
          curl -sSLO "https://github.com/gitleaks/gitleaks/releases/download/v${GITLEAKS_VERSION}/gitleaks_${GITLEAKS_VERSION}_linux_x64.tar.gz"
          echo "${GITLEAKS_SHA256}  gitleaks_${GITLEAKS_VERSION}_linux_x64.tar.gz" | sha256sum -c -
          tar -xzf "gitleaks_${GITLEAKS_VERSION}_linux_x64.tar.gz" -C /usr/local/bin gitleaks
          rm -f "gitleaks_${GITLEAKS_VERSION}_linux_x64.tar.gz"
          chmod +x /usr/local/bin/gitleaks

      - name: Execute Secret Scan
        run: gitleaks dir . --redact -v

  test-and-lint:
    name: Lint & Test Suite
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Repository
        uses: builtin:checkout
        with:
          fetch-depth: 0

      - name: Setup Bun Environment
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Install Dependencies
        run: bun install --frozen-lockfile

      - name: Format & Lint Verification
        run: bun run check

      - name: Run Test Suite
        run: bun test

  merge-gate:
    name: PR Quality & Merge Gate
    needs: [security-scan, test-and-lint]
    runs-on: ubuntu-latest
    steps:
      - name: Verification Gate
        run: echo "All quality gates passed successfully."
```

### Template B: Python & Backend CI Quality Gate

```yaml
name: Python Quality Gate

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test-and-lint:
    name: Python Lint & Tests
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Repository
        uses: builtin:checkout
        with:
          fetch-depth: 0

      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.13"

      - name: Setup uv
        uses: astral-sh/setup-uv@v5
        with:
          enable-cache: true

      - name: Install Dependencies
        run: uv sync --frozen

      - name: Verify Formatting & Linting
        run: |
          uv lock --check
          uv run ruff check
          uv run ruff format --check
          uv run mypy src

      - name: Execute Tests
        run: uv run pytest -v

  merge-gate:
    name: Merge Gate
    needs: [test-and-lint]
    runs-on: ubuntu-latest
    steps:
      - run: echo "Quality gate passed."
```

### Template C: Container Build & Direct Local Registry Loopback

```yaml
name: Container Publish

on:
  push:
    branches: [main]

jobs:
  build-and-push:
    name: Build & Push Container Image
    runs-on: ubuntu-latest-12-cores # Highperf runner
    steps:
      - name: Checkout Repository
        uses: builtin:checkout
        with:
          fetch-depth: 0

      # Native DinD runner connects directly to gitea-proxy (172.16.12.100)
      - name: Authenticate Container Registry
        run: echo "${{ secrets.GITHUB_TOKEN }}" | docker login git.gt-web-dev.com -u "${{ github.actor }}" --password-stdin

      - name: Build & Tag Image
        run: docker build -t git.gt-web-dev.com/${{ github.repository }}:latest .

      - name: Push Image across High-Speed Loopback
        run: docker push git.gt-web-dev.com/${{ github.repository }}:latest
```

---

## 6. Operational CLI Commands & Diagnostics

All pipeline and runner operations are managed via unified `forge` CLI:

```bash
# 1. Pipeline Telemetry
forge gitea runs [-o owner] [-r repo] [--limit 10] [--json]
forge gitea run <run_id> [--json]
forge gitea jobs <run_id> [--json]
forge gitea job-logs <job_id>
forge gitea rerun <run_id>

# 2. Runner Health & Inspection
forge gitea runners [--json]
forge gitea logs --runner standard -n 50
forge gitea logs --runner highperf -n 50

# 3. Synchronize Runner Configs to TrueNAS Host
forge truenas app sync-runners [app_name]

# 4. Strict Zero-Trust TLS & Registry Loopback Verification
forge gitea tls-check [--json]
```
