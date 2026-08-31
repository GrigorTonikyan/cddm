# feat(hub): federation hub and multi-repository clone correlation matrix [EP-36]

## Summary

Introduces Organization Federation Hub for tracking, analyzing, and deduplicating code across multiple distinct repositories in an organization.

## Capabilities & Architecture

- **Central Hub Cache Scanner**: Aggregates duplicate code discovery across multiple distinct Git repositories (`cddm hub scan` / `cddm_scan_hub`).
- **Inter-Repository Duplication Matrix**: Real-time cross-repository clone correlation and duplication matrix computation.
- **Shared Package Synthesis**: Automated extraction of cross-repository duplicate clusters into standalone npm or Cargo packages with caller rewrite plans (`cddm hub extract` / `cddm_extract_hub_package`).
- **4-Pillar Parity**: Full integration across CLI (`cddm hub`), WebUI Studio (`HubFederationModal`), MCP Server (`cddm_scan_hub`, `cddm_extract_hub_package`), and TUI Studio (`Hub` tab).

## Test Verification

- [x] CLI integration: `crates/cddm-cli/src/commands/hub.rs`
- [x] MCP tools: `tests/mcp/tools/scan-hub.test.ts` and `tests/mcp/tools/extract-hub-package.test.ts`
- [x] WebUI Component: `webui/src/components/HubFederationModal.test.tsx`
- [x] TUI View: `crates/cddm-cli/src/tui/views/hub.rs`

## References

- Fixes #7 (`[FEAT] Federation Hub & Cross-Repository Clone Correlation Matrix [EP-36]`)

Branch: `feat/monorepo-federation-hub`
Milestone: v1.11.0
