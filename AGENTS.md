# CDDM Agent Development Guidelines & Enforcement Standard

This document serves as the **Single Source of Truth (SSoT)** index and root governance standard for all AI coding agents and human engineers contributing to the **CDDM (Code De-Duplication Meister)** repository.

To prevent context bloat and maximize agent performance, detailed operational runbooks, prime directives, and modular rules are strictly organized under the `.agents/` directory.

> [!IMPORTANT]
> Coding agents **MUST** dynamically read and follow the detailed rules and skills linked below before performing relevant actions.

## 1. Modular Scoped Rules (`.agents/rules/`)

The following rules dictate coding standards, architectural limits, and workflow requirements:

- [Task Completion Workflow](.agents/rules/task-completion-workflow.md)
  - Details the mandatory 6-step sequential protocol for concluding any task.
- [Modularity & File Limits](.agents/rules/modularity-and-file-limits.md)
  - Details the 500-line absolute file ceiling, anti-monolith mandate, and the ratcheted grandfather allowlist (`scripts/check-file-length.ts`).
- [Legacy Remediation Protocol](.agents/rules/legacy-remediation.md)
  - Details the strict protocol for handling legacy code that violates workspace standards, prioritizing full remediation or structured logging in `docs/TODO.md`.
- [CDDM Agent Standards](.agents/rules/cddm-standards.md)
  - Details core coding standards: zero bypasses, Vite Plus toolchain, conventional commits, dogfooding, and strict TypeScript.

## 2. Workspace Skills (`.agents/skills/`)

The following skills are specialized runbooks that extend agent capabilities for specific CDDM workflows:

- [cddm-task-workflow](.agents/skills/cddm-task-workflow/SKILL.md)
  - On-demand skill for executing end-to-end task completion, `/browser` Chrome UI/UX testing, and MCP validation.
- [cddm-modular-refactoring](.agents/skills/cddm-modular-refactoring/SKILL.md)
  - On-demand skill for decomposing monolithic files, AST refactoring, and ratcheted limit maintenance.
