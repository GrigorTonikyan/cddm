# CDDM Ecosystem Distribution, Registry & Adoption Strategy

> **Tracking Issue**: [`gt-dev/cddm#304`](https://git.gt-web-dev.com/gt-dev/cddm/issues/304)
> **Primary Benchmark Report**: [`docs/BENCHMARKS.md`](BENCHMARKS.md)

This document defines CDDM's developer adoption roadmap, public registry publication checklist, Model Context Protocol (MCP) directory submissions, and reproducible marketing/benchmark workflows.

---

## 1. Distribution Channels & Live Verification Matrix

CDDM maintains a multi-tier distribution architecture published from Gitea SSoT releases and mirrored downstream to GitHub (`GrigorTonikyan/cddm`):

| Channel                          | Identifier / Coordinates                         | Verification Mechanism                                           | Status                              |
| :------------------------------- | :----------------------------------------------- | :--------------------------------------------------------------- | :---------------------------------- |
| **Gitea Release Binaries**       | `gt-dev/cddm` + `SHA256SUMS.txt`                 | `bun scripts/verify-distribution-channels.ts --remote`           | Live (CD Automated)                 |
| **Linux / macOS Installer**      | `packaging/install.sh` (`CDDM_BASE_URL`)         | SHA-256 verification & smoke check                               | Live                                |
| **Windows PowerShell Installer** | `packaging/install.ps1` (`CDDM_BASE_URL`)        | `--live-install` ZIP + SHA-256 + binary test                     | Live                                |
| **Homebrew Tap**                 | `gt-dev/homebrew-tap` (`Formula/cddm.rb`)        | Gitea Contents/SSH dispatch + SHA-256 check                      | Live (CD Automated)                 |
| **Scoop Bucket**                 | `gt-dev/scoop-bucket` (`bucket/cddm.json`)       | Manifest JSON & asset HTTP 200 verification                      | Live (CD Automated)                 |
| **WinGet Bucket**                | `gt-dev/winget-pkgs` (v1.9.0 multi-file YAML)    | `winget validate --manifest` in verification suite               | Live (CD Automated)                 |
| **Container Registry (OCI)**     | `git.gt-web-dev.com/gt-dev/cddm:vX.Y.Z`          | `docker-build-push@v1` multi-stage BuildKit                      | Live (CD Automated)                 |
| **Cargo Crates**                 | `cddm-core`, `cddm-lsp`, `cddm-mcp`, `cddm-cli`  | Private Gitea Cargo Registry (CD) + `crates.io` (GitHub CD)      | Private Live / Public Token Pending |
| **NPM Packages**                 | `cddm` + platform `@cddm/*` binary wrappers      | Private Gitea NPM Registry (CD) + `npmjs` Provenance (GitHub CD) | Private Live / Public Token Pending |
| **VS Code Extension**            | `vscode-cddm` (`.vsix`, 24 language activations) | `bun scripts/package-vscode.ts` + Marketplace / Open VSX         | VSIX Live / Marketplace PAT Pending |

---

## 2. Public Registry Activation Playbook

GitHub downstream mirror release automation (`.github/workflows/release.yml`) is already wired for zero-touch public publishing once repository secrets are provisioned in GitHub Actions settings:

1. **crates.io Publishing (`CARGO_REGISTRY_TOKEN`)**:
   - Publish order enforced in CI: `cddm-core` -> `cddm-lsp` & `cddm-mcp` -> `cddm-cli`.
   - Allows `cargo install cddm-cli --locked` for every Rust developer worldwide without cloning from source.
2. **npm Registry with SLSA Provenance (`NPM_TOKEN` / Trusted Publisher OIDC)**:
   - Publishes platform-specific optional binaries (`@cddm/linux-x64`, `@cddm/win32-x64`, etc.) and root wrapper `cddm`.
   - Unlocks zero-install MCP execution: `npx -y -p cddm cddm-mcp` and `npx -y cddm scan .`.
3. **IDE Extensions (`VSCE_PAT` & `OVSX_PAT`)**:
   - Dual-publish `.vsix` to Microsoft Visual Studio Marketplace (`vsce publish`) and Eclipse Open VSX (`ovsx publish`) so VS Code, Cursor, Windsurf, VSCodium, and Antigravity users discover live clone squiggles, Quick Fix refactors, and CodeLens automatically.
4. **Upstream Core Package Managers (`microsoft/winget-pkgs`, `Homebrew/homebrew-core`)**:
   - Dispatch PRs cleanly after public binary releases (`GrigorTonikyan/cddm`) using the pre-validated v1.9.0 manifests emitted by `scripts/package-distribution.ts`.

---

## 3. MCP Registry & AI Ecosystem Readiness

CDDM ships 33 MCP tools, 14 static resources (+ 3 dynamic URI templates), and 3 guided workflows (`cddm-mcp`). Ecosystem discovery manifests are maintained under strict version lockstep:

- **Official MCP Registry**: [`server.json`](../server.json) conforms to `2025-12-11/server.schema.json` and is synchronized automatically across all 11 workspace manifests by `bun scripts/version.ts`. Submit with `mcp-publisher publish` once `cddm` is live on npmjs/OCI.
- **Smithery Directory**: [`smithery.yaml`](../smithery.yaml) declares the stdio command configuration for one-click Smithery installation.
- **Workspace Auto-Discovery**: [`.vscode/mcp.json`](../.vscode/mcp.json) enables instant local Copilot MCP attachment inside VS Code.
- **Client Snippets (`docs/MCP.md` & `README.md`)**: Documented turnkey configurations for Claude Desktop, Cursor, Antigravity, Windsurf, and Zed using both native `cddm-mcp` and `npx -y -p cddm cddm-mcp`.
- **Curated Directories**: Target submission PRs for `punkpeye/awesome-mcp-servers`, PulseMCP, and Cursor Directory citing CDDM's deterministic Tree-sitter AST rewriting + sandbox test verification (saving LLM tokens versus pure regex/LLM copy-paste).

---

## 4. Visual Showcase & Reproducible Benchmarks

Developer trust depends on live, reproducible evidence rather than static diagrams:

1. **Benchmark Harness (`scripts/benchmark-ecosystem.ts`)**:
   - Generates a deterministic 4-language polyglot corpus (Rust, TypeScript, Python, Go) with planted Type-1 exact and Type-2 renamed clones alongside realistic unique business logic.
   - Runs controlled cold (`--no-cache`) and warm (`redb` incremental mmap cache) timings comparing CDDM directly against installed ecosystem detectors (`jscpd`, optional `pmd`).
   - Writes verifiable results directly into [`docs/BENCHMARKS.md`](BENCHMARKS.md).
2. **Visual Demo Pipeline (`scripts/record-demos.ts`)**:
   - Records live `cddm` CLI terminal output into Asciinema v2 (`packaging/demos/cddm-cli-demo.cast`) and crisp dark terminal vector SVG (`docs/assets/cddm-cli-demo.svg`).
   - Launches an ephemeral headless Chromium Playwright session against `cddm serve` to capture 2x Retina screenshots of CDDM Studio (`docs/assets/cddm-webui-studio.png`) and the Interactive Auto-Refactor Sandbox (`docs/assets/cddm-webui-refactor.png`).

---

## 5. Community Launch & Adoption Funnel

1. **Core Differentiation Narrative**:
   - _Beyond grep/Rabin-Karp_: SIMD Mersenne M61 winnowing + Tree-sitter CST/AST hashing + Weisfeiler-Lehman semantic graph similarity (`Type-1` through `Type-4` + cross-language).
   - _Actionable End-to-End Remediation_: Unlike report-only scanners, CDDM synthesizes parameterized helpers, applies CST-preserving multi-file patches inside transactional `gix` branches, runs project test suites automatically to verify zero regressions, and ships identical capabilities across CLI, WebUI, TUI (`ratatui`), LSP, and MCP.
2. **Launch Cadence**:
   - **Wave 1 (Engineering Showcase)**: Publish technical architecture post covering SIMD AVX2/NEON/AVX-512 Mersenne M61 rolling hashes + Tree-sitter lossless refactoring; launch on **Show HN** and `r/rust`.
   - **Wave 2 (AI Tooling Audience)**: Publish `server.json` / `smithery.yaml` listings demonstrating token-saving codebase deduplication for coding agents on **PulseMCP**, `awesome-mcp-servers`, and `r/LocalLLaMA` / Cursor communities.
   - **Wave 3 (CI/CD Templates & Examples)**: Highlight drop-in Gitea Actions (`setup-cddm@v1`) and GitHub Actions PR comment quality gates (`--fail-threshold 5.0`).
3. **Adoption Telemetry Goals**:
   - Weekly npm downloads (`cddm`), `crates.io` installs (`cddm-cli`), Open VSX / VS Marketplace installs, Official MCP Registry & Smithery pulls, and community GitHub issues/PRs.
