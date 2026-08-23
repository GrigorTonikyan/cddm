# CDDM Agent Development Guidelines & Enforcement Standard

This document defines the strict, non-negotiable engineering standards, conventions, rules, and quality gates for any AI agent or developer contributing to the **CDDM (Code De-Duplication Meister)** codebase.

---

## 1. Prime Directives (Non-Negotiable)

1. **Zero Halfway Implementations**:
   - Every feature, refactor, or bug fix must be implemented end-to-end.
   - Never leave `TODO` placeholders, stub functions, mock implementations in production paths, or partial types.
   - Every new component or API endpoint must include automated unit/integration tests and type definitions.

2. **Zero Quality Bypasses**:
   - **NEVER** use `--no-verify` or bypass Git hooks under any circumstances.
   - **NEVER** suppress compiler or linter errors using `#![allow(...)]`, `// @ts-ignore`, or `any` unless explicitly justified in architectural reviews.
   - All code must compile with **zero warnings** (`-D warnings`).

3. **Mandatory Dogfooding Self-Scan**:
   - CDDM must pass its own duplication analysis on its own codebase with every change.
   - Command: `cargo run -p cddm-cli -- scan . --min-tokens 50 --fail-threshold 15.0`.
   - Duplication percentage must remain below 15.0% and DRY Health Score must remain high.

4. **Vite Plus Toolchain Enforcement (Strict)**:
   - All package script execution across the repository **MUST use `vp run <script>`** (e.g. `vp run verify`, `vp run fix`, `vp run test`, `vp run build`, `vp -C webui run lint`).
   - Running package scripts via `bun run`, `npm run`, `yarn`, `pnpm`, or `deno` is strictly **forbidden**.
   - Direct execution of standalone TypeScript scripts MUST use `bun <script-path>` (e.g., `bun scripts/verify.ts`, `bun scripts/fix.ts`, `bun scripts/setup-hooks.ts`) since `vp` does not execute arbitrary files directly.
   - All JavaScript, TypeScript, JSX, and CSS formatting and linting MUST use Vite Plus (`vp fmt`, `vp lint`, `vp check`) exclusively.

5. **Single Source of Truth (Cross-Platform)**:
   - Do NOT duplicate scripts into `.ps1` and `.sh` variants.
   - All repository automation scripts reside in `scripts/` as cross-platform TypeScript executed via `bun` (e.g., `bun scripts/verify.ts`, `bun scripts/setup-hooks.ts`).

6. **Strict Conventional Commits & Semantic Versioning**:
   - All commit messages MUST strictly adhere to the Conventional Commits specification, validated via `@commitlint/cli` and `commitlint.config.ts`.
   - Valid types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
   - Breaking changes MUST use `!` (e.g. `feat(core)!: ...`) or a `BREAKING CHANGE:` footer.
   - Releases and version bumps MUST use `vp run bump` or `vp run version:release`, keeping `Cargo.toml`, `package.json`, `webui/package.json`, `npm/cddm/package.json`, and `CHANGELOG.md` in lockstep.

7. **Strict Zero-Emoji Policy Across Entire Codebase**:
   - **NO EMOJIS OR PICTOGRAPHS** are permitted anywhere in the repository (Rust crates, WebUI, documentation, scripts, changelogs, CLI terminal output, or commit messages).
   - Codebase cleanliness is continuously scanned and enforced via `bun scripts/check-no-emojis.ts` (`vp run check:emojis`).
   - Use clean, professional text tags (`[PASS]`, `[FAIL]`, `[OK]`, `[ERROR]`, `[WARN]`) for status indicators.

8. **Strict Zero-Downgrade & Latest Dependencies Policy Across Entire Codebase**:
   - **NEVER DOWNGRADE ANY DEPENDENCY** under any circumstances across any subsystem (Rust `Cargo.toml`, npm `package.json`, React `webui/package.json`, E2E `tests/e2e/package.json`, or root scripts).
   - Only the latest compatible versions of all libraries, devDependencies, crates, and toolchain components must be used across the entire repository.
   - When encountering upstream linter, compiler, or runner version mismatches, always upgrade, pin, or resolve with the latest versions rather than reverting or downgrading.

---

## 2. Code Architecture & Style Standards

### A. Constants & Enums (Zero Magic Literals)

- **Rust**:
  - Phase names must use `cddm_core::ScanPhase` enum.
  - Clone classifications must use `cddm_core::CloneType` enum.
  - Route paths must use constants (`ROUTE_API_HEALTH`, `ROUTE_API_SCAN`).
  - JSON-RPC 2.0 error codes and methods in `cddm-mcp` must use `rpc_errors` and `mcp_methods` modules.
  - Package versions must derive dynamically from `env!("CARGO_PKG_VERSION")`.

- **TypeScript / WebUI**:
  - API routes must use `API_ROUTES` from `src/constants/cddm-constants.ts`.
  - Default configs must use `DEFAULT_SCAN_CONFIG`.
  - Phases must use the `ScanPhase` union type.
  - App version must use `APP_VERSION` from `src/constants/cddm-constants.ts`.

### B. Rust Engine Rules (`cddm-core`, `cddm-cli`, `cddm-mcp`)

- **Unsafe Code**: Forbidden across all crates (`unsafe_code = "forbid"`).
- **Formatting**: Strictly enforce `rustfmt.toml` (2024 edition, 100 max line width).
- **Linter**: Strictly enforce Clippy with workspace deny rules (`all = "deny"`, `correctness = "deny"`).
- **Debug Derivations**: All public structs and enums must implement `std::fmt::Debug`.

### C. WebUI & TypeScript Rules (`webui/`)

- **Strict Type Checking**: Must pass `tsc -b` with maximum strictness:
  - `strict: true`
  - `noImplicitAny: true`
  - `noUncheckedIndexedAccess: true`
  - `noUnusedLocals: true`
  - `noUnusedParameters: true`
  - `noImplicitReturns: true`
- **Styling**: TailwindCSS utility classes only. Maintain the sleek dark mode aesthetic (slate-950 base, indigo/purple accents).
- **State Management**: Zustand store (`useCDDMStore`) with atomic action setters.

---

## 3. Git Hooks & Verification Pipeline

### Vite+ Git Hooks Enforcement (`.vite-hooks/`)

Git hooks are installed and configured via `vp config` / `vp run prepare`:

- **`pre-commit`**: Runs `vp staged`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `vp check`.
- **`pre-push`**: Runs `cargo test --workspace`, `vp -C webui run test`, `vp -C webui run build`, and `cddm scan .`.
- **`commit-msg`**: Runs `@commitlint/cli` against `commitlint.config.ts` to enforce Conventional Commits.

### Canonical Master Runners

The repository provides canonical master commands:

1. **Verify All Checks (Read-Only)**:

   ```bash
   vp run verify
   # or directly:
   bun scripts/verify.ts
   ```

2. **Auto-Fix All Fixable Issues & Validate**:

   ```bash
   vp run fix
   # or directly:
   bun scripts/fix.ts
   ```

3. **Deep Clean All Artifacts & Lockfiles**:

   ```bash
   vp run clean
   # or directly:
   bun scripts/clean.ts
   ```

4. **Full Workspace Reset & Default State Preparation**:

   ```bash
   vp run reset
   # or directly:
   bun scripts/reset.ts
   ```

All 11 verification checks must pass cleanly:

1. `cargo fmt --check` (Rust formatting check)
2. `cargo clippy --workspace --all-targets -- -D warnings` (Strict Clippy zero-warning linter)
3. `cargo test --workspace` (Rust unit & integration test suite)
4. `tsc -p tsconfig.json` (Scripts strict typecheck)
5. `bun test scripts/__tests__` (Repository scripts unit tests)
6. `vp check` (Workspace-wide type-aware Oxlint + Oxfmt check)
7. `vp -C webui run test` (WebUI Vitest unit test suite)
8. `vp -C webui run build` (WebUI production bundle compilation)
9. `bun scripts/check-no-emojis.ts` (Zero-Emoji policy codebase enforcement)
10. `bun scripts/check-docs.ts` (Documentation integrity & cross-reference sync validation)
11. `cddm scan . --min-tokens 50 --fail-threshold 15.0` (Dogfooding self-scan)

---

## 4. Documentation Integrity Rules

- Never write speculative or unimplemented feature claims into documentation.
- All options, commands, and code samples in `README.md`, `docs/API.md`, `docs/ARCHITECTURE.md`, and `docs/FEATURE_MATRIX.md` must be verified and true.
- Table headers and column pipes in markdown documents must be properly aligned without markdownlint warnings (`MD060`).
