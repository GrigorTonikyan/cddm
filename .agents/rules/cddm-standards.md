---
trigger: always_on
---

# CDDM Agent Standards

1. **Zero Halfway Implementations**: Complete all features end-to-end with unit tests, type safety, and error handling.
2. **Zero Bypasses**: Never use `--no-verify` or disable linter/compiler warnings.
3. **Vite Plus Toolchain Standard**: Always execute package scripts via `vp run <script>` (e.g. `vp run verify`, `vp run fix`, `vp run test`). Never use `bun run`, `npm`, `yarn`, `pnpm`, or `deno`. Direct script files must be run with `bun <script-path>`.
4. **Vite Plus Linter & Formatter**: Use `vp fmt`, `vp lint`, and `vp check` exclusively for all JS/TS/CSS/HTML formatting and linting.
5. **Conventional Commits Enforcement**: All git commit messages must satisfy `@commitlint/cli` rules configured in `commitlint.config.ts`.
6. **Automated Semantic Releases**: Version bumps and releases must use `vp run bump` or `vp run version:release`, synchronizing Cargo and npm manifests.
7. **Zero Magic Literals**: Use typed enums (`ScanPhase`, `CloneType`) and centralized constants (`API_ROUTES`, `DEFAULT_SCAN_CONFIG`, `APP_VERSION`).
8. **Dogfooding Quality Gate**: Always verify with `cddm scan . --min-tokens 50 --fail-threshold 15.0`.
9. **Strict TypeScript**: Full strict type safety with zero `any` across `webui/` and `scripts/`.
10. **Full Pipeline Verification**: Always validate with `vp run verify` (or `bun scripts/verify.ts`) before completing work.
