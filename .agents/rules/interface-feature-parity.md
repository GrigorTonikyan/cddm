---
trigger: always_on
---

# Cross-Interface Feature Parity Mandate

In the CDDM repository, code deduplication and refactoring capabilities are delivered across four primary interaction pillars. Any contributor or AI coding agent adding, modifying, or extending core features MUST enforce strict **Feature Parity** across all four surfaces.

## The 4 Interface Pillars

1. **CLI Engine** (`crates/cddm-cli`): Dedicated subcommands and ergonomic terminal flags (`cddm scan`, `diff`, `semantic`, `refactor`, `extract`, `heal`, `tui`, `rules`, etc.).
2. **WebUI Studio** (`webui/` + `cddm-cli::serve`): Full-fidelity visual interface powered by React 19, `win2x-manager`, Axum REST/SSE endpoints, Monaco diffs, and interactive modals.
3. **MCP Server** (`crates/cddm-mcp`): JSON-RPC 2.0 Model Context Protocol tools (`tools/list`, `tools/call`), resources (`resources/list`, `resources/read`), and prompt workflows (`prompts/list`, `prompts/get`) for AI agents.
4. **TUI Studio** (`cddm tui` via `crates/cddm-cli/src/tui/`): High-speed, keyboard-driven terminal dashboard powered by `ratatui` and `crossterm` for terminal power-users and remote SSH workflows.

## Golden Rules for Feature Parity

1. **Zero Interface Orphans**: No new core engine feature may be added as a "CLI-only", "Web-only", "MCP-only", or "TUI-only" feature. Every capability must be accessible from all four pillars.
2. **Universal Synchronous Verification**: The feature parity matrix is strictly verified in CI and pre-commit pipelines via `bun scripts/check-feature-parity.ts` (invoked by `vp run verify`).
3. **Documentation Matrix Synchronization**: Any new capability must be registered in [docs/FEATURE_PARITY.md](../../docs/FEATURE_PARITY.md) and [docs/FEATURE_MATRIX.md](../../docs/FEATURE_MATRIX.md) with verified test mappings across all four interfaces.
4. **Consistent Domain Terminology**: Terms, flags, metric names (e.g., `DRY Health Score`, `Duplication %`, `Clone Cluster`, `Near-Miss`), and configuration options MUST remain identical across CLI flags, REST JSON schemas, MCP arguments, and TUI labels.
