---
name: cddm-task-workflow
description: >-
  Use this skill when completing any feature, refactor, or bugfix in CDDM
  to execute the mandatory 6-step post-task completion sequence.
---

# CDDM Task Completion Workflow

Execute these 6 steps in strict order after completing any task.

## Step 1: Run Automated Verification

```bash
vp run verify
```

Inspect all output. Every check must pass with zero errors and zero warnings.

## Step 2: Browser UI/UX Verification

For UI or user-facing changes:

1. Start the dev server: `vp -C webui run dev`
2. Spawn a `/browser` subagent to navigate `http://localhost:5173`
3. Exercise all pages, modals, buttons, keyboard shortcuts, themes
4. Confirm zero console errors and zero visual defects

## Step 3: MCP Server Live Testing

1. Discover available tools from the `<mcp_servers>` context block
2. Call relevant `cddm` MCP tools via `call_mcp_tool` for any modified functionality
3. Verify input schemas and response structures

## Step 4: Manual Diff Inspection

Review `git diff` output. Confirm zero hallucinated APIs, stubs, or broken edge cases.

## Step 5: Documentation Sync

```bash
bun scripts/check-docs.ts
```

Update docs in `docs/` and root as needed.

## Step 6: Version Sync, Commit, Gitea Push, PR & Merge

1. Sync versions if needed: `vp run bump`
2. Commit with Conventional Commits referencing the primary Gitea issue (`Fixes #<gitea-id>`)
3. Push to `origin` (Gitea) first (never use `--no-verify`), using strictly ONE canonical branch (`feat/issue-<num>-<desc>`). Then mirror to `github`.
4. Open the primary Pull Request on Gitea (`https://git.gt-web-dev.com/gt-dev/cddm/pulls`), include `Fixes #<id>`, assign target milestone, and mirror to GitHub secondarily.
5. Merge PR via Gitea REST API (`POST /repos/{owner}/{repo}/pulls/{id}/merge`) to ensure auto-closure of issues and clean UI state.
6. For milestone releases, execute `vp run version:release` to synchronize all 10 manifests and publish release artifacts.
