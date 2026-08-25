---
trigger: always_on
---

# File Length Capping & Modular Decomposition Standard

## Hard Ceiling

All source code files (`.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.css`) MUST NOT exceed **500 lines**. New files exceeding this limit fail CI and local verification (`vp run check:file-lengths`).

## Ratcheting Grandfather Allowlist

Existing legacy files exceeding 500 lines are tracked with ratcheted line caps in [`scripts/check-file-length.ts`](scripts/check-file-length.ts). These files MUST NEVER grow longer. When refactored, their caps must be ratcheted downward.

## Decomposition Strategies

- **Rust**: Split monolithic files into submodules under a directory with `mod.rs`. Extract CLI commands, MCP handlers, AST visitors, and domain types into dedicated files.
- **WebUI React**: Split complex components into atomic subcomponents. Extract custom hooks into `webui/src/hooks/` and pure helpers into `webui/src/utils/`.
- **Scripts**: Extract shared utilities into `scripts/lib/`. Keep entrypoints focused on orchestration.

## Automated Enforcement

```bash
vp run check:file-lengths
# or directly:
bun scripts/check-file-length.ts
```
