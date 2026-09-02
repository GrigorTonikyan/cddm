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

## Step 6: Version Sync, Conventional Commit, Gitea Push, PR & Merge

1. **Version Sync**: Synchronize version manifests if needed (`bun scripts/sync-version.ts` or `vp run bump`).
2. **Conventional Commit**: Commit using strict Conventional Commits referencing the primary Gitea issue (`Fixes #<gitea-id>`).
3. **Gitea-First Push & Mirror**: Push to `origin` (Gitea: `https://git.gt-web-dev.com/gt-dev/cddm.git`) first without bypassing Git hooks (`--no-verify` is forbidden), using strictly ONE canonical branch name (`feat/issue-<num>-<desc>`). Then mirror to `github`.
4. **Primary Pull Request**: Open the primary Pull Request on Gitea (`https://git.gt-web-dev.com/gt-dev/cddm/pulls`) merging the feature/fix branch into `main`. Include `Fixes #<id>` in the description and assign the target milestone.
5. **API-Driven Merge**: When concluding a PR merge, execute the merge through the official Gitea REST API (`POST /repos/gt-dev/cddm/pulls/{id}/merge`) to register the PR as `merged: true`, close it in the UI, and automatically close the linked issue.
6. **Milestone Release & Version Enforcement**: When concluding a release milestone, you **MUST** run `bun scripts/version.ts --release-as <version>` or `vp run version:release` to synchronize all 10 project manifests (Cargo.toml, package.json, webui/package.json, VSIX, etc.) and tag `vX.Y.Z` **BEFORE** closing the milestone on Gitea. Never close a milestone without syncing the codebase version to match it.
