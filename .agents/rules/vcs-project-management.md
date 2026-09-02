# VCS Project Management Governance Standard

This rule governs the exclusive use of Gitea as the Single Source of Truth (SSoT) for project management, issue tracking, and roadmap planning in the CDDM repository. AI coding agents and human engineers MUST strictly adhere to this standard.

## 1. Prohibition of Static Markdown Tracking

1. **Zero Markdown Task Lists**: Never create, maintain, or update static markdown files (e.g., \docs/TODO.md\, \docs/ROADMAP.md\, \docs/REQUIREMENTS.md\) to track active tasks, open bugs, or future milestones.
2. **Zero Redundant Documentation**: Do not duplicate requirement specifications or acceptance criteria across multiple files. All actionable specifications MUST live in Gitea Issues.
3. **No Scratchpad Commits**: Do not commit draft conversation logs, raw chat transcripts, or scratchpad RFCs (e.g., \docs/rfc/_.md\, \docs/win2x/_.md\) to the repository. These must be synthesized into official Gitea Wiki pages or Gitea PR descriptions.

## 2. Gitea Entity Mapping Mandate

1. **Milestones = Version Releases**:
   - Gitea Milestones are strictly reserved for time-bound, semantic version releases (e.g., \1.0.0\, \3.2.0\).
   - Every active milestone must have a corresponding semantic version number.
2. **Issues = Epics / Requirements / Tasks**:
   - **Requirements (FR)** and **Enhancement Proposals (EP)** are tracked as Gitea Issues (acting as "Epics").
   - Every Issue MUST be assigned to the target Version Milestone in which it is delivered.
3. **Labels = Categorization**:
   - Use standardized Gitea labels (\eat: *\, \ug\, \priority: *\) to categorize and filter Issues.

## 3. Project Management Workflow

1. **Discovery & Planning**: When planning new features or analyzing requirements, query Gitea (\git.gt-web-dev.com/api/v1/repos/gt-dev/cddm/issues\) to retrieve the canonical state.
2. **Documentation Pointers**: Files like \docs/ROADMAP.md\ or \docs/TODO.md\ MUST only contain high-level strategic summaries and absolute links pointing directly to the Gitea Milestone and Issue Tracker endpoints.
3. **Continuous Sync**: If the roadmap changes, update the Gitea Milestone/Issue first. Do not update a local markdown file as a substitute.
