---
trigger: always_on
---

# Protected Branch & Mandatory Pull Request Governance Standard

This rule strictly governs branch protection and pull request enforcement for all AI coding agents contributing to the CDDM repository.

## 1. Absolute Prohibition of Direct Commits to Protected Branches

1. **Strict Ban on Direct `main` Commits**: AI coding agents MUST NEVER commit directly to default or protected branches (`main`, `master`, `release/*`, `production`).
2. **Explicit User Approval Exemption Only**: Direct commits or direct pushes to `main` are ONLY permitted if the human user explicitly, unambiguously, and in writing commands the agent to commit directly to `main` in their prompt.
3. **Automatic Rejection**: Any agent action attempting to bypass branch creation, commit directly to `main`, or push un-reviewed code directly to `origin/main` without explicit user instruction is a critical governance violation.

## 2. Mandatory Feature/Fix Branch & PR Lifecycle

For every task, fix, refactor, or feature:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. Discover / Create Issue via Forge MCP (`gitea_list/create_issue`)        │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. Create Issue-Derived Branch: `git checkout -b <type>/issue-<id>-<desc>`   │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. Implement Changes + Polyglot Tests + Verify (`vp run verify`)             │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. Bump Workspace SemVer & Sync 10 Manifests (`bun scripts/version.ts`)      │
├─────────────────────────────────────────────────────────────────────────────┤
│ 5. Push Issue Branch to Gitea `origin` (auto-mirrored to `github`)           │
├─────────────────────────────────────────────────────────────────────────────┤
│ 6. Open Pull Request via Forge MCP (`gitea_create_pr`)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ 7. Verify CI Quality Gate via Forge MCP (`gitea_wait_for_ci_gate`)           │
├─────────────────────────────────────────────────────────────────────────────┤
│ 8. Merge via Forge MCP (`gitea_merge_pr`) -> Auto-Tag `vX.Y.Z` Release CI   │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 3. Branch Nomenclature Standard

All working branches MUST strictly adhere to the canonical naming format derived from the primary Gitea issue ID:

- `feat/issue-<gitea-id>-<short-description>`
- `fix/issue-<gitea-id>-<short-description>`
- `chore/issue-<gitea-id>-<short-description>`
- `refactor/issue-<gitea-id>-<short-description>`
- `perf/issue-<gitea-id>-<short-description>`

## 4. API-Driven PR Merge Protocol

1. **Never Merge via Local Fast-Forward Push**: Agents must not merge locally and push `main` directly to remotes.
2. **Never Bypass CI Gates**: All commits and PRs must satisfy CI quality gates verified through `gitea_wait_for_ci_gate` or `gitea_get_commit_statuses`.
3. **API Merge Endpoint**: All merges MUST be executed via Forge MCP (`gitea_merge_pr`) or `forge pr merge <id> --delete-branch`.
4. **Traceability & Automated Tag Release**: This ensures Gitea automatically marks the PR as merged, attaches all CI workflow logs to the PR, auto-closes the linked issue, and triggers `auto-tag-release` in `.gitea/workflows/ci.yaml` to cut the release tag `vX.Y.Z` and publish distribution assets.
