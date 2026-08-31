---
trigger: always_on
---

# Gitea Primary SSoT & GitHub Mirror Governance Standard

This rule governs repository tracking, issue management, branching nomenclature, pull requests, and release distribution for the CDDM repository. AI coding agents and human engineers MUST strictly adhere to this hierarchy.

## 1. Single Source of Truth (SSoT) Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. PRIMARY SSoT: Gitea Portal (https://git.gt-web-dev.com/gt-dev/cddm)      │
│    - Authoritative Git repository (`origin`)                                │
│    - Primary Issue Tracker, Milestones, and Project Roadmaps                │
│    - Primary Pull Requests and Code Reviews                                 │
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

1. **Gitea-Based Branch Names**: Branch names MUST be derived from the primary Gitea issue number:
   - `fix/issue-<gitea-number>-<short-description>`
   - `feat/issue-<gitea-number>-<short-description>`
   - `refactor/issue-<gitea-number>-<short-description>`
2. **Zero Direct Main Commits**: Never commit directly to the default branch (`main`). Always create a branch based on the Gitea issue.
3. **Conventional Commits**: Commit messages must follow `@commitlint/cli` rules and reference the primary Gitea issue in the body or footer.

## 4. Push & Pull Request Protocol

1. **Gitea-First Push**: Always push working branches to `origin` (Gitea: `https://git.gt-web-dev.com/gt-dev/cddm.git`) first.
2. **Mirror Push**: Push to secondary remote `github` (`https://github.com/GrigorTonikyan/cddm.git`) after Gitea.
3. **Primary PR Creation**: Open the primary Pull Request on Gitea (`https://git.gt-web-dev.com/gt-dev/cddm/pulls`) merging into `main`.
4. **Mirror PR Creation**: Open or sync the secondary mirror PR on GitHub with references pointing to the primary Gitea PR.

## 5. Releases & Distribution

- Binary distribution, VSIX packages, and release metadata are published primarily to Gitea via `scripts/publish-release.ts` (`GITEA_HOST=git.gt-web-dev.com`).
- Release tags and artifacts are mirrored secondarily to GitHub.
