---
trigger: always_on
---

# CDDM General Agent Standards

1. **Zero Halfway Implementations**: Complete all features end-to-end with unit tests, strict type safety, and comprehensive error handling.
2. **Zero Bypasses**: Never use `--no-verify` or disable linter/compiler warnings.
3. **Vite Plus Toolchain Standard**: Always execute package scripts via `vp run <script>` (e.g., `vp run verify`, `vp run fix`, `vp run test`). Never use `bun run`, `npm`, `yarn`, `pnpm`, or `deno`. Direct script files must be run with `bun <script-path>`.
4. **Vite Plus Linter & Formatter**: Use `vp fmt`, `vp lint`, and `vp check` exclusively for all JS/TS/CSS/HTML formatting and linting.
5. **Conventional Commits Enforcement**: All git commit messages must satisfy `@commitlint/cli` rules configured in `commitlint.config.ts`.
6. **Automated Semantic Releases**: Version bumps and releases must use `vp run bump` or `vp run version:release`, synchronizing Cargo and npm manifests.
7. **Zero Magic Literals**: Use typed enums (`ScanPhase`, `CloneType`) and centralized constants (`API_ROUTES`, `DEFAULT_SCAN_CONFIG`, `APP_VERSION`).
8. **Dogfooding Quality Gate**: Always verify with `cddm scan . --min-tokens 50 --fail-threshold 5.0`.
9. **Strict TypeScript**: Full strict type safety with zero `any` across `webui/`, `scripts/`, and `tests/`.
10. **Full Pipeline Verification**: Always validate with `vp run verify` (or `bun scripts/verify.ts`) before completing work.
11. **Cross-Interface Feature Parity**: Every capability must be simultaneously available across CLI, WebUI Studio, MCP Server, and TUI Studio (governed by `.agents/rules/interface-feature-parity.md`).
12. **Unified Polyglot Test Architecture**: Strictly adhere to `.agents/rules/test.md` (Co-location for WebUI, `scripts/tests/` for scripts, `tests/mcp/` for MCP, and co-located/submodule unit testing for Rust).
13. **MCP Tool Testing Standard**: Every MCP tool must have an isolated, dedicated test suite under `tests/mcp/tools/<tool-kebab-case>.test.ts` dynamically discovered and verified in `vp run verify` (governed by `.agents/rules/test.mcp.md`).
14. **Living Documentation & Dynamic Test Discovery**: Never manually hardcode test inventories, suite lists, or test case counts in markdown documentation. All test matrices and capability mappings MUST be dynamically generated and synchronized via `bun scripts/sync-feature-matrix.ts` and verified in CI via `bun scripts/check-docs.ts`.
15. **Feature-Sliced Design & Public APIs**: Preserve strict separation between core engine logic (pure algorithms, SIMD, AST) and interaction surfaces (CLI, WebUI, MCP, TUI). In `webui/`, enforce Feature-Sliced Design with public API boundaries (`index.ts`), atomic components, dedicated custom hooks (`hooks/use*.ts`), and pure utilities (`utils/`).
16. **Bun-Only Scripting Runtime & APIs**: All automation scripts, test harnesses, and tooling in `scripts/` MUST run exclusively on the Bun runtime and use native Bun APIs (e.g., `Bun.spawn`, `Bun.spawnSync`, `Bun.$`, `Bun.file`, `Bun.write`, `Bun.Glob`, `Bun.serve`). Using Node.js child process modules (`node:child_process`, `child_process`, `execSync`, `exec`) or legacy Node.js fs/path patterns where Bun native equivalents exist in `scripts/` is strictly prohibited.
17. **Gitea Primary SSoT & GitHub Mirror Governance**: Gitea (`git.gt-web-dev.com`) is the authoritative Single Source of Truth for issues, pull requests, project tracking, and binary releases. GitHub is strictly a downstream replica mirror. All tasks, branch names, commit messages, and PRs MUST reference and prioritize Gitea first (governed by `.agents/rules/gitea-ssot-mirror-governance.md`).
