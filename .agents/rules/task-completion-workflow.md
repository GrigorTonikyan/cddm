---
trigger: always_on
---

# Mandatory Task Completion Protocol

When any task is completed in this codebase, execute the following 6 steps in strict sequential order before concluding.

## Step 1: Automated Verification Suite

Run `vp run verify` (or `bun scripts/verify.ts`). Inspect the output and confirm every dynamically discovered check passes with zero errors and zero warnings.

## Step 2: Interactive Browser UI/UX Verification

For frontend or user-facing changes, spawn a `/browser` subagent in Chrome to interactively verify the WebUI at `http://localhost:3000`. Exercise all pages, modals, buttons, keyboard shortcuts, themes, and responsive states. Confirm zero console errors and zero visual defects.

## Step 3: MCP Server Live Testing

Dynamically discover available `cddm` MCP tools from the `<mcp_servers>` context block or by listing schema files in the MCP configuration directory. Call relevant tools corresponding to modified functionality. Verify input schemas, response structures, and error codes.

## Step 4: Manual Diff and Semantic Inspection

Review `git diff` output. Verify zero hallucinated APIs, partial stubs, placeholder mocks, or broken edge cases.

## Step 5: Living Documentation & Test Matrix Synchronization

Whenever tests are added, modified, renamed, or moved, run `bun scripts/sync-feature-matrix.ts` to dynamically regenerate and synchronize [`docs/FEATURE_MATRIX.md`](../../docs/FEATURE_MATRIX.md).
Then run `bun scripts/check-docs.ts` to validate full documentation integrity, link resolution, and zero-drift verification across all markdown files. Update all relevant documentation in `docs/` and root to reflect the exact state of the codebase.

## Step 6: Version Sync, Conventional Commit, Gitea Push, PR & Merge

1. **Version Bump & Manifest Sync**: Run `bun scripts/version.ts` (or `vp run version:bump`) to idempotently bump the workspace SemVer from Conventional Commits since the latest tag and synchronize all 10 manifests + `CHANGELOG.md`. Verify with `bun scripts/version.ts --check-pr`.
2. **Conventional Commit**: Commit using strict Conventional Commits referencing the primary Gitea issue (`Fixes #<gitea-id>`).
3. **Gitea Push (Sole Push Target)**: Push to `origin` (Gitea: `https://git.gt-web-dev.com/gt-dev/cddm.git`) without bypassing Git hooks (`--no-verify` is forbidden), using strictly ONE canonical branch name (`feat/issue-<num>-<desc>`). Do NOT push manually to `github`—Gitea automatically replicates all branches, commits, and tags to GitHub via server-side push mirror (`sync_on_commit: true`).
4. **Primary Pull Request**: Open the primary Pull Request via Forge MCP (`gitea_create_pr` or `forge pr create`) merging into `main`. Include `Fixes #<id>` in the description; the linked issue is mandatory. Milestones are optional and never required on issues or PRs.
5. **Verify CI Quality Gate**: Monitor and verify CI gate status using Forge MCP (`gitea_wait_for_ci_gate` or `gitea_get_commit_statuses`). Never bypass CI gates (`--no-verify` or manual fast-forwards are strictly prohibited).
6. **API-Driven Merge & Automated Tag Release**: Conclude the PR merge via Forge MCP (`gitea_merge_pr` or `forge pr merge <id> --delete-branch`) to register the PR as `merged: true`, auto-delete the remote branch, and automatically close the linked issue. Upon merge to `main`, Gitea Actions (`auto-tag-release` in `.gitea/workflows/ci.yaml`) automatically tags `vX.Y.Z` and triggers the release build pipeline.
7. **Release Asset Whitelist, Dynamic Variables & Multi-App Distribution**: In release workflows, release download assets are strictly compiled binaries (`.tar.gz`, `.zip`, `.vsix`) and `SHA256SUMS.txt`. All publishers, repository targets, and secrets are resolved dynamically from VCS variable/secret stores (`vars.*`, `secrets.*`) with `.env` as last resort. Package manager manifests (`.rb`, `.json`, `.yaml`) are dispatched to central, multi-app repositories (`homebrew-tap`, `scoop-bucket`, `winget-pkgs`), never uploaded as binary download assets. All secrets must be injected strictly via `env:` mapping with universal log masking.
