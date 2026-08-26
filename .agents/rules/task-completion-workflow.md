---
trigger: always_on
---

# Mandatory Task Completion Protocol

When any task is completed in this codebase, execute the following 6 steps in strict sequential order before concluding.

## Step 1: Automated Verification Suite

Run `vp run verify` (or `bun scripts/verify.ts`). Inspect the output and confirm every dynamically discovered check passes with zero errors and zero warnings.

## Step 2: Interactive Browser UI/UX Verification

For frontend or user-facing changes, spawn a `/browser` subagent in Chrome to interactively verify the WebUI at `http://localhost:5173`. Exercise all pages, modals, buttons, keyboard shortcuts, themes, and responsive states. Confirm zero console errors and zero visual defects.

## Step 3: MCP Server Live Testing

Dynamically discover available `cddm` MCP tools from the `<mcp_servers>` context block or by listing schema files in the MCP configuration directory. Call relevant tools corresponding to modified functionality. Verify input schemas, response structures, and error codes.

## Step 4: Manual Diff and Semantic Inspection

Review `git diff` output. Verify zero hallucinated APIs, partial stubs, placeholder mocks, or broken edge cases.

## Step 5: Living Documentation & Test Matrix Synchronization

Whenever tests are added, modified, renamed, or moved, run `bun scripts/sync-feature-matrix.ts` to dynamically regenerate and synchronize [`docs/FEATURE_MATRIX.md`](../../docs/FEATURE_MATRIX.md).
Then run `bun scripts/check-docs.ts` to validate full documentation integrity, link resolution, and zero-drift verification across all markdown files. Update all relevant documentation in `docs/` and root to reflect the exact state of the codebase.

## Step 6: Version Sync and Conventional Commit

Synchronize version manifests if needed (`bun scripts/sync-version.ts` or `vp run bump`). Commit using strict Conventional Commits. Push without bypassing Git hooks (`--no-verify` is forbidden).
