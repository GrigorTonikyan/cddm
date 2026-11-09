---
trigger: always_on
---

# Gitea Primary SSoT & GitHub Mirror Governance Standard

This rule governs repository tracking, issue management, branching nomenclature, pull requests, and automated release lifecycle for the CDDM repository. AI coding agents and human engineers MUST strictly adhere to this standard.

## 1. Single Source of Truth (SSoT) Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. PRIMARY SSoT: Gitea Portal (https://git.gt-web-dev.com/gt-dev/cddm)      │
│    - Authoritative Git repository (`origin`)                                │
│    - Primary Issue Tracker, Milestones, and Project Roadmaps                │
│    - Primary Pull Requests, Code Reviews, and Merges                        │
│    - Gitea Actions CI/CD matrix and cross-compilation                       │
│    - Authoritative binary release publisher and packaging assets            │
│    - Authoritative Private VCS Container Registry (git.gt-web-dev.com/...)  │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. SECONDARY DOWNSTREAM: GitHub Mirror (https://github.com/GrigorTonikyan/cddm)│
│    - Read-only / secondary replica mirror                                   │
│    - Mirrors commits, branches, and releases from Gitea                     │
│    - Cross-references primary Gitea issues and pull requests                │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 2. Issue Tracking & Task Tracing Mandate

1. **Gitea-First Issue Discovery**: When investigating changes or embarking on a task, always query and verify existing issues on Gitea first via Forge MCP (`gitea_list_issues` or `forge issue list --state open`).
2. **Mandatory Issue Recording**: If no issue exists for a required bugfix, refactor, or feature, the agent **MUST** create the issue on Gitea as the primary authoritative record via Forge MCP (`gitea_create_issue` or `forge issue create`) before writing code or creating a working branch.
3. **Primary Issue Identification**: All task discussions, commit messages, and PR summaries MUST cite the primary Gitea issue (e.g. `Fixes #16` pointing to Gitea Issue #16).

## 3. Branching & Commit Nomenclature

1. **Single Canonical Issue-Derived Branch**: Branch names MUST be derived strictly from the primary Gitea issue number:
   - `fix/issue-<gitea-number>-<short-description>`
   - `feat/issue-<gitea-number>-<short-description>`
   - `chore/issue-<gitea-number>-<short-description>`
   - `refactor/issue-<gitea-number>-<short-description>`
2. **Zero Redundant Branch Aliases**: Never push dual or redundant branch names (e.g., pushing both `feat/desc` and `feat/issue-X-desc`). Always maintain exactly ONE canonical branch per feature or issue to prevent UI clutter and duplicate PR prompts.
3. **Zero Direct Main Commits**: Never commit directly to the default branch (`main`). Always create a branch based on the Gitea issue.
4. **Conventional Commits**: Commit messages must follow `@commitlint/cli` rules and reference the primary Gitea issue in the body or footer.

## 4. Pull Request & API Merge Protocol

1. **Gitea Push (Sole Push Target)**: Always push working branches to `origin` (Gitea: `https://git.gt-web-dev.com/gt-dev/cddm.git`).
2. **Automated Server-Side Mirroring (Zero Manual Push)**: Gitea is configured with automated server-side push mirroring (`sync_on_commit: true`) to GitHub (`https://github.com/GrigorTonikyan/cddm.git`). Never push manually to the `github` remote—doing so causes redundant network traffic, race conditions, and reference lock collisions (e.g. `cannot lock ref ... reference already exists`).
3. **Primary PR Creation**: Open the primary Pull Request via Forge MCP (`gitea_create_pr` or `forge pr create`) targeting `main`.
4. **Auto-Closing Issue Citations**: PR descriptions MUST include closing keywords (`Fixes #<id>`, `Closes #<id>`, `Resolves #<id>`) pointing to the primary Gitea issue.
5. **CI Quality Gate Verification**: Verify all CI workflow checks pass before merging using Forge MCP (`gitea_wait_for_ci_gate` or `gitea_get_commit_statuses`). Never bypass CI gates (`--no-verify` and bypasses are strictly forbidden).
6. **API-Driven Merge Enforcement**:
   - Merging PRs into `main` MUST be executed via Forge MCP (`gitea_merge_pr` or `forge pr merge <id> --delete-branch`).
   - Merging via Forge MCP ensures Gitea automatically marks the PR as **`merged: true`** with state **`closed`**, auto-closes the linked issue, and prevents orphan PRs lingering in the UI.
   - Never bypass the Gitea merge endpoint with silent local fast-forward pushes to `main`.
7. **Automatic Branch Deletion**: Merged feature branches must be deleted immediately after merge (enforced by Gitea `default_delete_branch_after_merge: true`).

## 5. Semantic Versioning & Automated Release Lifecycle

1. **Issues Are Mandatory, Milestones Are Optional**: Every change MUST be tracked by a Gitea issue (PRs reference it via `Fixes #<id>`). Milestones are NOT required on issues or PRs; assign one only when an issue is targeted at a specific roadmap grouping.
2. **PR-Time Lockstep SemVer Bump & Automated Tag Release on `main`**:
   - Workspace versions are calculated deterministically from Conventional Commits since the latest release tag via `bun scripts/version.ts` (or `vp run version:bump`).
   - Every PR introducing features (`feat`), bug fixes (`fix`), or optimizations (`perf`, `refactor`) MUST bump the workspace version on the PR branch before opening the PR.
   - `bun scripts/version.ts` idempotently synchronizes all 10 project manifests (`Cargo.toml`, `Cargo.lock`, `package.json`, `webui/package.json`, `npm/cddm/package.json` + `optionalDependencies`, `editors/vscode/package.json`, Homebrew, Scoop, Winget, and `README.md` badges) and updates `CHANGELOG.md`.
   - `vp run verify` and PR CI enforce `bun scripts/version.ts --check` (100% manifest SSoT sync) and `bun scripts/version.ts --check-pr` (PR version bump gate).
   - When the PR is merged into `main`, the `auto-tag-release` CI job in `.gitea/workflows/ci.yaml` automatically detects the new version in `Cargo.toml`, creates and pushes the annotated Git tag `vX.Y.Z` to Gitea `origin`, and triggers `.gitea/workflows/release.yaml` to build and publish multi-platform binaries, VSIX packages, SHA-256 checksummed package manifests, and the Gitea release.
3. **Downstream Mirror Sync**: Release commits, tags, and assets pushed to Gitea `origin` are automatically mirrored downstream to GitHub via Gitea's server-side push mirror (`sync_on_commit: true`). Never push manually to `github`.
