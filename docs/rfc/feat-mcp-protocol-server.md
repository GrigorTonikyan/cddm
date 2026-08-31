# feat(mcp): Model Context Protocol server with 27 tools and dynamic discovery

## Summary
Implements complete Model Context Protocol (MCP) JSON-RPC 2.0 server with 27 dedicated tools for AI coding agents.

## Highlights
- 1:1 isolated test file per tool under `tests/mcp/tools/`.
- Dynamic discovery contract test asserting 100% test coverage.
- Full support for SARIF export, AST refactoring, and monorepo query tools.

## Test Plan
- [x] `bun test tests/mcp/discovery.test.ts`
- [x] Tested against live agent integration.

Branch: `feat/mcp-protocol-server`
Milestone: v1.9.0
