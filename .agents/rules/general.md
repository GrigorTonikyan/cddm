---
trigger: always_on
---

# CDDM General Agent Standards

1. **Zero Halfway Implementations**: Complete all features end-to-end with unit tests, strict type safety, and comprehensive error handling. No stubs, mocks in production, or half-measures.
2. **Zero Bypasses**: Never use `--no-verify`, `--skip-checks`, or disable linter/compiler warnings.
3. **Vite Plus Toolchain Standard**:
   - Always execute package scripts and toolchain operations exclusively via **`vp`** (`vp run <task>`, `vp test`, `vp check`, `vp fmt`, `vp lint`, `vp dev`, `vp build`, `vp outdated`, `vp hooks`).
   - Never use `bun run`, `npm`, `yarn`, `pnpm`, or `deno` for script execution or toolchain management.
   - **Standalone Script Invocation**: When executing standalone script files directly from disk, use **`bun <script-path>`** (e.g., `bun scripts/verify.ts`). Never run `vp <script-path>`.
4. **Vite Plus Linter & Formatter (`oxfmt` & `oxlint`)**:
   - Use `vp fmt`, `vp lint`, and `vp check` exclusively for all JS, TS, JSX, TSX, JSON, Markdown, CSS, and HTML formatting and linting.
   - Formatting and linting on staged files must be dispatched via `vp staged`.
5. **Vite Plus Native Hooks Dispatcher**:
   - Git hook dispatching must be managed via Vite Plus hooks (`.vite-hooks/` enabled via `vp hooks enable`). Third-party hook managers (Husky, simple-git-hooks) are strictly banned.
6. **Package Manager Audit Standard**:
   - Use **`vp outdated`** or **`bun outdated`** to check for package updates. Never execute `bun pm outdated`.
7. **Modular Docker Containerization & Private VCS Registry Standard**:
   - Image versions in Dockerfiles must NOT be pinned to brittle specific hashes or patch digests; least-specific versions, preferably **`latest`** tags (e.g. `rust:latest`, `oven/bun:latest`, `debian:latest`), must be used.
   - VCS Action runners on Gitea (`highperf`/DinD) are responsible for building container images from `Dockerfile`.
   - Built container images MUST be published directly into the private VCS Container Registry (`git.gt-web-dev.com/<owner>/<repo>:latest`) and consumed from that same registry.
8. **VCS-Driven Development (Gitea SSoT)**:
   - Gitea is the authoritative Single Source of Truth for issues, milestones, branches, PRs, and releases.
   - Every change must be preceded by an issue on Gitea.
   - Canonical branches MUST be derived strictly from the issue: `feat/issue-<id>-<slug>`, `fix/issue-<id>-<slug>`.
   - Commits must adhere to Conventional Commits referencing the issue (`Fixes #<id>`).
   - Direct commits/pushes to `main` are strictly banned. All merges must proceed through Gitea PRs and be executed via the Gitea REST API (`POST /repos/{owner}/{repo}/pulls/{index}/merge` with `delete_branch_after_merge: true`).
9. **Strict Dogfooding Quality Gate**:
   - Every verification must run `cddm scan . --min-tokens 50 --fail-threshold 5.0` on release binaries. Loose thresholds (e.g. 15.0%) are strictly prohibited.
10. **Zero Hardcoding Enforcement**:
    - NOTHING, in NO CASE, may be hardcoded into code. All ports, paths, URLs, timeouts, retry limits, and string literals must be centralized in typed enums (`StrEnum`, Rust enums) or typed configuration structs.
11. **Strict Prohibition of Inline Scripts**:
    - AI coding agents MUST NEVER execute inline scripts (`bun -e "..."`, `node -e "..."`, or `python -c "..."`).
    - Any workflow lacking a script or CLI subcommand must be tracked as an issue and implemented as a reusable, permanent CLI tool or script under `scripts/`.
12. **Strict TypeScript & Polyglot Type Safety**:
    - Full strict type safety with zero `any` across `webui/`, `scripts/`, `crates/`, and `tests/`.
13. **Comprehensive Test Suite & Quality Gate**:
    - Full test coverage is mandatory: co-located unit tests, integration tests, MCP protocol tests, AND Playwright E2E browser tests (`vp -C webui run test:e2e`).
    - In addition to tests, the quality gate must execute: security vulnerability audits (`bun pm scan`), license compliance audits (`bun pm licenses`), 500-line modularity ceilings, zero-emoji policy, 4-pillar parity, and documentation synchronization.
14. **Cross-Interface Feature Parity**:
    - Every capability must be simultaneously available across CLI, WebUI Studio, MCP Server, and TUI Studio (governed by `.agents/rules/interface-feature-parity.md`).
15. **MCP Tool Testing Standard**:
    - Every MCP tool must have an isolated, dedicated test suite under `tests/mcp/tools/<tool-kebab-case>.test.ts` dynamically discovered and verified in `vp run verify` (governed by `.agents/rules/test.mcp.md`).
16. **Living Documentation & Dynamic Test Discovery**:
    - Never manually hardcode test inventories or test case counts in markdown. All test matrices and capability mappings MUST be dynamically generated via `bun scripts/sync-feature-matrix.ts` and verified in CI via `bun scripts/check-docs.ts`.
17. **Feature-Sliced Design & Public APIs**:
    - Preserve strict separation between core engine logic (pure algorithms, SIMD, AST) and interaction surfaces (CLI, WebUI, MCP, TUI). In `webui/`, enforce Feature-Sliced Design with public API boundaries (`index.ts`), atomic components, dedicated custom hooks (`hooks/use*.ts`), and pure utilities (`utils/`).
18. **Bun-Only Scripting Runtime & APIs**:
    - All automation scripts, test harnesses, and tooling in `scripts/` MUST run exclusively on the Bun runtime and use native Bun APIs (`Bun.spawn`, `Bun.spawnSync`, `Bun.$`, `Bun.file`, `Bun.write`, `Bun.Glob`, `Bun.serve`). Using Node.js child process modules (`child_process`, `execSync`) in `scripts/` is strictly prohibited.
19. **Interface Documentation Decoupling & Automated Generation**:
    - All four interaction pillars (CLI, WebUI, MCP, TUI) maintain dedicated reference guides under `docs/` (`docs/CLI.md`, `docs/MCP.md`, `docs/WEBUI.md`, `docs/TUI.md`, `docs/LSP_SETUP.md`). All matrices MUST be dynamically generated via `bun scripts/sync-docs.ts` and validated via `bun scripts/check-docs.ts`.
20. **Tightly Coupled Git Wiki Synchronization**:
    - The Gitea/Git Wiki MUST be tightly coupled to repository `docs/` and synchronized via `bun scripts/sync-wiki.ts` with zero static drift.
21. **Permanent Workspace Tooling & Reusability**:
    - Never write one-off scratch scripts in external or brain directories for workspace automation, CI polling, or API calls. All developer tooling and automation MUST reside permanently under `scripts/`, be continuously enhanced and extended with reusable CLI parameters, and maintain 100% test coverage in `scripts/tests/` (governed by `.agents/rules/workspace-tooling-reusability.md`).
22. **Protected Branch & Mandatory Pull Request Governance**:
    - AI coding agents MUST NEVER commit or push directly to default/protected branches (`main`, `master`, `release/*`) unless explicitly and unambiguously commanded by the human user in their prompt. All work must proceed through canonical issue branches, primary Gitea PRs, and API merges (governed by `.agents/rules/protected-branch-pr-enforcement.md`).
