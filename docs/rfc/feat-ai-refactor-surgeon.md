# feat(ai-surgeon): automated AST clone cluster refactoring surgeon [EP-35]

## Summary

Implements automated AST refactoring surgeon that extracts multi-file duplicate clone clusters into standalone shared functions or modules with syntax preservation, automated rollback, and zero-regression verification.

## Capabilities & Architecture

- **AST Parameterization**: Identifies parameterizable variable and identifier differences across multi-file clone occurrences.
- **Shared Module Synthesizer**: Generates deduplicated helper functions, TypeScript/Rust modules, and accompanying unit tests/benchmarks (`cddm extract` / `cddm_extract_shared_module`).
- **Autonomous Surgeon Loop**: AI-guided code transformation with AST validation and automated rollback on test failures (`cddm heal` / `cddm_heal_refactor`).
- **4-Pillar Parity**: Full surface support across CLI (`cddm refactor`, `cddm extract`, `cddm heal`), WebUI (`RefactorSandboxModal`, `ExtractModuleTab`, `RefactorPatchModal`), MCP Server (`cddm_ast_refactor`, `cddm_suggest_cluster_refactor`, `cddm_apply_cluster_refactor`, `cddm_extract_shared_module`, `cddm_heal_refactor`, `cddm_verify_refactor`), and TUI (`Refactor` and `Extract` tabs).

## Test Verification

- [x] CLI commands: `crates/cddm-cli/src/commands/refactor.rs` and `crates/cddm-cli/src/commands/extract.rs`
- [x] MCP tools: `tests/mcp/tools/ast-refactor.test.ts`, `tests/mcp/tools/suggest-cluster-refactor.test.ts`, `tests/mcp/tools/apply-cluster-refactor.test.ts`, `tests/mcp/tools/extract-shared-module.test.ts`, `tests/mcp/tools/heal-refactor.test.ts`, `tests/mcp/tools/verify-refactor.test.ts`
- [x] WebUI Components: `webui/src/components/RefactorSandboxModal.test.tsx`, `webui/src/components/sandbox/ExtractModuleTab.test.tsx`, `webui/src/components/RefactorPatchModal.test.tsx`
- [x] TUI Views: `crates/cddm-cli/src/tui/views/refactor.rs` and `crates/cddm-cli/src/tui/views/extract.rs`

## References

- Fixes #6 (`[RFC] AI Refactor Surgeon: Automated AST Cluster Extraction [EP-35]`)

Branch: `feat/ai-refactor-surgeon`
Milestone: v1.11.0
