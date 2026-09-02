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
│ 1. Discover / Create Primary Issue on Gitea (`git.gt-web-dev.com`)          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. Create Issue-Derived Branch: `git checkout -b <type>/issue-<id>-<desc>`   │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. Implement Changes + Polyglot Tests + Verify (`vp run verify`)             │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. Push Issue Branch to Gitea `origin` (and downstream `github`)             │
├─────────────────────────────────────────────────────────────────────────────┤
│ 5. Open Pull Request on Gitea targeting `main` (with `Fixes #<id>`)          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 6. Merge via Gitea REST API (`POST /repos/gt-dev/cddm/pulls/{id}/merge`)    │
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
2. **API Merge Endpoint**: All merges MUST be executed via the authoritative Gitea REST API endpoint:
   `POST /repos/{owner}/{repo}/pulls/{index}/merge` with `{"Do": "merge", "delete_branch_after_merge": true}`.
3. **Traceability**: This ensures Gitea automatically marks the PR as merged, attaches all CI workflow logs to the PR, and auto-closes the linked issue.
