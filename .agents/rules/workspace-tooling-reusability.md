# Permanent Workspace Tooling & Continuous Reusability Standard

This rule governs the creation, evolution, consolidation, and reuse of automation scripts, developer tooling, and VCS integrations in the CDDM repository. AI coding agents and human contributors MUST strictly adhere to this standard.

## 1. Core Principles

1. **Zero Ephemeral / Throwaway Scripts**:
   - Coding agents MUST NEVER create temporary, one-off, or throwaway scripts in scratch, brain, or temp directories for repository tasks (e.g., CI monitoring, Gitea API calls, PR syncing, release packaging, cleanups).
   - All automation and developer utilities MUST reside permanently under `scripts/` with reusable library modules under `scripts/lib/`.

2. **Tooling Reusability & Continuous Evolution**:
   - Instead of re-implementing logic across tasks, agents MUST always discover, reuse, and extend the existing canonical workspace tools:
     - `scripts/monitor-ci.ts`: Live Gitea Actions CI/CD pipeline monitoring, polling, and job auditing.
     - `scripts/lib/gitea-client.ts`: Canonical Gitea REST API client, auth headers, and retry logic.
     - `scripts/version.ts` & `scripts/sync-version.ts`: Multi-manifest semantic release and version bumping.
     - `scripts/clean.ts` & `scripts/lib/clean-engine.ts`: Safe workspace cleaning and build artifact pruning.
     - `scripts/verify.ts`: 18-step synchronous workspace verification pipeline.
     - `scripts/sync-feature-matrix.ts` & `scripts/sync-docs.ts`: Living documentation and parity matrix synchronization.
   - When new options, parameters, or edge cases are needed, agents MUST enhance the existing canonical script rather than authoring a separate duplicate tool.

3. **Bun Native Runtime & Strict TypeScript**:
   - All workspace scripts MUST execute on the Bun runtime using native Bun APIs (`Bun.spawn`, `Bun.spawnSync`, `Bun.file`, `Bun.write`, `Bun.Glob`).
   - Legacy Node.js process APIs (`child_process`, `execSync`) are strictly forbidden.
   - Scripts must maintain strict TypeScript typing with zero `any`.

4. **Modularity & 500-Line Ceiling**:
   - All files under `scripts/` and `scripts/lib/` MUST NOT exceed 500 lines (governed by `.agents/rules/modularity-and-file-limits.md`).
   - Extract shared algorithms, types, and clients into focused helper modules in `scripts/lib/`.

5. **Mandatory Script Testing**:
   - Every executable script under `scripts/` MUST have a corresponding test suite under `scripts/tests/<script-name>.test.ts` or co-located in `scripts/lib/<module>.test.ts` (governed by `.agents/rules/test.md`).
   - Script tests must execute automatically during `vp run verify` (`bun test scripts/tests`).
