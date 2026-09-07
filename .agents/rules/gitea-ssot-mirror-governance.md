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
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. SECONDARY DOWNSTREAM: GitHub Mirror (https://github.com/GrigorTonikyan/cddm)│
│    - Read-only / secondary replica mirror                                   │
│    - Mirrors commits, branches, and releases from Gitea                     │
│    - Cross-references primary Gitea issues and pull requests                │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 2. Issue Tracking & Task Tracing Mandate

1. **Gitea-First Issue Discovery**: When investigating changes or embarking on a task, always query and verify existing issues on Gitea first (`git.gt-web-dev.com/api/v1/repos/gt-dev/cddm/issues`).
2. **Mandatory Issue Recording**: If no issue exists for a required bugfix, refactor, or feature, the agent **MUST** create the issue on Gitea as the primary authoritative record before writing code or creating a working branch.
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
3. **Primary PR Creation**: Open the primary Pull Request on Gitea (`https://git.gt-web-dev.com/gt-dev/cddm/pulls`) merging into `main`.
4. **Auto-Closing Issue Citations**: PR descriptions MUST include closing keywords (`Fixes #<id>`, `Closes #<id>`, `Resolves #<id>`) pointing to the primary Gitea issue.
5. **API-Driven Merge Enforcement**:
   - Merging PRs into `main` MUST be executed via the official Gitea REST API (`POST /repos/{owner}/{repo}/pulls/{index}/merge`).
   - Merging via the API ensures Gitea automatically marks the PR as **`merged: true`** with state **`closed`**, auto-closes the linked issue, and prevents orphan PRs lingering in the UI.
   - Never bypass the Gitea merge endpoint with silent local fast-forward pushes to `main`.
6. **Automatic Branch Deletion**: Merged feature branches must be deleted immediately after merge (enforced by Gitea `default_delete_branch_after_merge: true`).

## 5. Milestone & Release Lifecycle

1. **Mandatory Milestone Assignment**: Every issue and PR must be assigned to an active Gitea milestone (e.g. `v3.4.0`). Verified in `vp run verify` via `bun scripts/check-milestones.ts`. Run `bun scripts/sync-milestones.ts` to auto-assign unassigned issues.
2. **Milestone Closure & Automagic Release**:
   - Workspace versions are determined strictly by milestones.
   - When all assigned issues and PRs for a milestone reach 100% completion (0 open issues remaining), the release MUST be triggered via `bun scripts/milestone-release.ts` (or `vp run release:milestone`).
   - The engine automatically synchronizes all 10 project manifests (`package.json`, `Cargo.toml`, `webui/package.json`, NPM packages, VS Code VSIX, Homebrew, Scoop, Winget, and README badges) via `bun scripts/sync-version.ts`.
   - Generates the commit `chore(release): vX.Y.Z [skip ci]`, creates the signed semantic Git tag `vX.Y.Z`, pushes to Gitea `origin` (auto-mirrored to GitHub), closes the milestone, and publishes the Gitea release.
3. **Downstream Mirror Sync**: Release commits, tags, and assets pushed to Gitea `origin` are automatically mirrored downstream to GitHub via Gitea's server-side push mirror (`sync_on_commit: true`). Never push manually to `github`.
