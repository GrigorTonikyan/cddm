# VCS Project Management Governance Standard

This rule governs the exclusive use of Gitea as the Single Source of Truth (SSoT) for project management, issue tracking, and roadmap planning in the CDDM repository. AI coding agents and human engineers MUST strictly adhere to this standard.

## 1. Prohibition of Static Markdown Tracking

1. **Zero Markdown Task Lists**: Never create, maintain, or update static markdown files (e.g., `docs/TODO.md`, `docs/ROADMAP.md`, `docs/REQUIREMENTS.md`) to track active tasks, open bugs, or future milestones.
2. **Zero Redundant Documentation**: Do not duplicate requirement specifications or acceptance criteria across multiple files. All actionable specifications MUST live in Gitea Issues.
3. **No Scratchpad Commits**: Do not commit draft conversation logs, raw chat transcripts, or scratchpad RFCs (e.g., `docs/rfc/*.md`, `docs/win2x/*.md`) to the repository. These must be synthesized into official Gitea Wiki pages or Gitea PR descriptions.

## 2. Gitea Entity Mapping Mandate

1. **Milestones (Optional Roadmap Groupings)**:
   - Gitea Milestones are strictly optional roadmap or epoch groupings (e.g., `v5.0-initiative`, `q4-performance`).
   - Versioning and releases are NOT gated by milestones. Releases are triggered automatically upon merge to `main` when PRs bump SemVer in lockstep via `bun scripts/version.ts`.
2. **Issues = Epics / Requirements / Tasks**:
   - **Requirements (FR)** and **Enhancement Proposals (EP)** are tracked as Gitea Issues (acting as "Epics").
   - Every change MUST be tracked by an Issue. Assigning a Milestone is optional and only done when the Issue aligns with a specific strategic roadmap grouping.
3. **Labels = Categorization**:
   - Use standardized Gitea labels (`feat: *`, `bug`, `priority: *`) to categorize and filter Issues.

## 3. Project Management Workflow

1. **Discovery & Planning**: When planning new features or analyzing requirements, query Gitea via Forge MCP (`gitea_list_issues` or `forge issue list --state open`) to retrieve the canonical state.
2. **Documentation Pointers**: Files like `docs/ROADMAP.md` or `docs/TODO.md` MUST only contain high-level strategic summaries and absolute links pointing directly to the Gitea Milestone and Issue Tracker endpoints.
3. **Continuous Sync**: If the roadmap changes, update the Gitea Milestone/Issue first via Forge MCP (`gitea_update_issue` or `forge issue`). Do not update a local markdown file as a substitute.

## 4. PR-Time Lockstep Versioning & Automated Tag Releases

1. **Issues Mandatory, Milestones Optional**:
   - Every change MUST be tracked by a Gitea issue; this is the hard requirement.
   - Milestones are optional roadmap groupings and do not gate or trigger releases.
2. **PR-Level SemVer Bump & Automated Release on `main`**:
   - Workspace versions are calculated deterministically from Conventional Commits since the latest Git tag via `bun scripts/version.ts` (or `vp run version:bump`).
   - Every PR introducing features, fixes, or optimizations MUST run `bun scripts/version.ts` before opening the PR to synchronize all 10 manifests (`Cargo.toml`, `Cargo.lock`, `package.json`, `webui/package.json`, `npm/cddm/package.json`, `editors/vscode/package.json`, Homebrew, Scoop, Winget, and `README.md`) and `CHANGELOG.md`.
   - `bun scripts/version.ts --check` and `bun scripts/version.ts --check-pr` enforce 100% SSoT version synchronization and PR version bumps in local verification and CI.
   - Merging a bumped PR into `main` automatically triggers the `auto-tag-release` job in `.gitea/workflows/ci.yaml`, which creates and pushes tag `vX.Y.Z` to Gitea `origin` and triggers `.gitea/workflows/release.yaml` to publish release binaries, VSIX packages, and SHA-256 checksummed distribution manifests.
