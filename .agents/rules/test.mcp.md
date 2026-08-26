---
trigger: always_on
---

# MCP Server Tool Testing & Dynamic Discovery Standard

This rule governs Model Context Protocol (MCP) server testing in the CDDM repository, specializing Section 4 of `.agents/rules/test.md`.

## Golden Rules for MCP Tool Testing

1. **Dedicated 1:1 Test File Per Tool**: Every registered MCP tool (e.g. `scan_codebase`, `cddm_get_clone_pair`, `cddm_suggest_refactor`) MUST have a corresponding, isolated test file under `tests/mcp/tools/<tool-kebab-case>.test.ts`.
2. **Canonical Test Home**: All MCP tests MUST reside under `tests/mcp/`. Placing MCP tests or validation scripts under `scripts/` is strictly forbidden.
3. **Automated Dynamic Discovery Test**: A dedicated test suite [`tests/mcp/discovery.test.ts`](../../tests/mcp/discovery.test.ts) MUST dynamically query the MCP server's `tools/list` endpoint at runtime and assert that 100% of discovered tools have a corresponding test suite in `tests/mcp/tools/`.
4. **Mandatory Schema & Execution Coverage**:
   - Each per-tool test suite MUST execute the tool with valid arguments and assert that the response payload matches the expected schema.
   - Each per-tool test suite MUST test negative/boundary cases (e.g. missing required parameters, non-existent paths) and assert standard JSON-RPC 2.0 error codes (`-32602` INVALID_PARAMS, `-32603` INTERNAL_ERROR).
5. **No Monolithic Multi-Tool Scripts**: Monolithic scripts attempting to test all tools in a single file (e.g., `scripts/mcp-*.ts`) are strictly forbidden.
6. **Continuous Pipeline Verification**: The MCP test suite is executed automatically via `bun test tests/mcp/` as a blocking step in `vp run verify` (or `bun scripts/verify.ts`).

## Test Suite Structure

```
tests/mcp/
├── helpers.ts                        # Standard JSON-RPC stdio runner & assertion helpers
├── discovery.test.ts                 # Dynamic discovery & 1:1 test presence verification
└── tools/                            # 1:1 dedicated test suites
    ├── scan-codebase.test.ts
    ├── get-clone-pair.test.ts
    ├── suggest-refactor.test.ts
    ├── get-clone-cluster.test.ts
    ├── suggest-cluster-refactor.test.ts
    ├── export-sarif.test.ts
    ├── diff-scan.test.ts
    ├── get-timeline.test.ts
    ├── check-suppression.test.ts
    ├── apply-cluster-refactor.test.ts
    ├── generate-ai-prompt.test.ts
    ├── ast-refactor.test.ts
    ├── verify-refactor.test.ts
    ├── check-policies.test.ts
    ├── heal-refactor.test.ts
    ├── export-cache-pack.test.ts
    ├── import-cache-pack.test.ts
    ├── scan-monorepo.test.ts
    ├── get-semantic-graph.test.ts
    ├── compare-semantic-graphs.test.ts
    ├── scan-cross-language.test.ts
    └── extract-shared-module.test.ts
```
