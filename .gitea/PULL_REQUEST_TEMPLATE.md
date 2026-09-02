# Pull Request

### Primary Gitea Issue Reference

Fixes #<!-- insert primary issue ID here, e.g. Fixes #28 -->

---

### Description & Motivation

Summarize the core architectural changes, algorithms introduced, or bug resolved.

---

### 4-Pillar Cross-Interface Parity Verification

- [ ] **CLI Engine** (`crates/cddm-cli`): Dedicated command / terminal flags implemented and tested.
- [ ] **WebUI Studio** (`webui/`): React 19 visual component, Monaco diffs, or modal integrated.
- [ ] **MCP Server** (`crates/cddm-mcp`): JSON-RPC 2.0 tool registered and 1:1 tested under `tests/mcp/tools/`.
- [ ] **TUI Studio** (`crates/cddm-cli/src/tui/`): Terminal dashboard view and keyboard navigation updated.

---

### Quality & Governance Checklist

- [ ] Passes full verification pipeline (`vp run verify` / `bun scripts/verify.ts`).
- [ ] Code formatting & linting clean (`cargo fmt --check`, `cargo clippy`, `vp check`).
- [ ] Modularity Standard satisfied (all files <= 500 lines, verified with `bun scripts/check-file-length.ts`).
- [ ] Living documentation synchronized (`bun scripts/sync-feature-matrix.ts`, `bun scripts/check-docs.ts`).
- [ ] Conventional Commits formatting strictly adhered to (`@commitlint/cli`).
- [ ] Dogfooding quality gate passes (`cddm scan . --min-tokens 50`).
