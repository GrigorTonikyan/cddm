---
trigger: always_on
---

# Universal Test Architecture & Placement Standard

This rule governs the organization, structure, naming, and placement of all test suites across the CDDM repository. AI coding agents and human engineers MUST strictly adhere to this standard.

## 1. The Core Placement Hierarchy

CDDM enforces an industry-standard, multi-tier testing model:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. Co-located Unit & Component Tests                                       │
│    Located directly adjacent to the source code (*.test.ts / *.test.tsx)    │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. Script & Tooling Execution Tests                                         │
│    Located in `scripts/tests/*.test.ts` and `scripts/lib/*.test.ts`         │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. Subsystem Protocol & Contract Tests (MCP, REST)                          │
│    Placed under dedicated root `tests/<subsystem>/tools/*.test.ts`           │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. Rust Crates (Unit, Integration, Benchmarks, Doctests)                    │
│    Co-located `#[cfg(test)] mod tests`, `src/**/tests.rs`, `tests/*.rs`    │
├─────────────────────────────────────────────────────────────────────────────┤
│ 5. Full-System End-to-End (E2E) & Acceptance                                │
│    Isolated at workspace root `tests/e2e/*.spec.ts` (Playwright / CLI)     │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 2. WebUI & React Standards (`webui/src/`)

1. **Mandatory Co-location**: All React components, custom hooks, state stores, and utility functions MUST have their test files placed directly adjacent to the implementation file:
   - Component: `src/components/ui/badge.tsx` -> `src/components/ui/badge.test.tsx`
   - Store slice: `src/store/watch-slice.ts` -> `src/store/watch-slice.test.ts`
   - Utility: `src/utils/ide-links.ts` -> `src/utils/ide-links.test.ts`
2. **Prohibition of Legacy `__tests__/` Directories**: Arbitrary nested `__tests__/` subdirectories are strictly forbidden in `webui/`.
3. **Vitest + React Testing Library**: Test user-visible behavior and accessibility (`getByRole`, `findByText`), not internal state or implementation details.

## 3. Workspace Scripts & Tooling Standards (`scripts/`)

1. **Script Libraries & Utilities**: Reusable modules in `scripts/lib/` MUST be co-located (e.g., `scripts/lib/step-runner.test.ts` next to `scripts/lib/step-runner.ts`).
2. **CLI Executable Tests**: Tests that invoke script entrypoints via child process MUST be placed in `scripts/tests/*.test.ts` (e.g., `scripts/tests/check-no-emojis.test.ts`).
3. **Prohibition of `scripts/__tests__/`**: The dunder `__tests__` directory name is strictly forbidden.

## 4. MCP Protocol & Contract Standards (`tests/mcp/`)

1. **1:1 Tool Isolation**: Every Model Context Protocol tool MUST have an isolated test suite under `tests/mcp/tools/<tool-kebab-case>.test.ts` (governed in detail by `.agents/rules/test.mcp.md`).
2. **Dynamic Discovery Verification**: `tests/mcp/discovery.test.ts` MUST dynamically verify that 100% of tools exposed by `tools/list` have dedicated test files.
3. **Prohibition of Monolithic MCP Test Scripts**: Monolithic all-in-one test scripts (e.g., `scripts/mcp-*.ts`) are strictly forbidden.

## 5. Rust Engine & Crate Testing Standards (`crates/`)

1. **Unit Tests (White-box)**:
   - Module unit tests MUST be written in co-located `#[cfg(test)] mod tests { ... }` blocks at the bottom of the source file.
   - If unit tests cause a file to approach or exceed the 500-line ceiling (`.agents/rules/modularity-and-file-limits.md`), the tests MUST be extracted into a sibling `tests.rs` submodule file within the same module directory.
2. **Integration Tests (Black-box)**:
   - Tests exercising public crate APIs across multiple modules MUST be placed in `crates/<crate-name>/tests/*.rs`.
   - Shared test helpers and fixtures MUST be placed in `crates/<crate-name>/tests/common/mod.rs`.
3. **Benchmarks**:
   - Criterion micro-benchmarks MUST be placed in `crates/<crate-name>/benches/*.rs`.
4. **Documentation Tests**:
   - Public API documentation examples MUST be verified via `cargo test --doc`.

## 6. Living Documentation & Dynamic Test Matrix Synchronization

1. **Zero Manual Hardcoding**: Coding agents and engineers MUST NEVER manually hardcode, estimate, or hand-type test file inventories or test case counts in markdown documentation.
2. **Automated Discovery Engine**: All test matrices and capability mappings in [`docs/FEATURE_MATRIX.md`](../../docs/FEATURE_MATRIX.md) MUST be dynamically computed from AST and filesystem discovery using `bun scripts/sync-feature-matrix.ts`.
3. **Continuous CI Zero-Drift Gate**: `bun scripts/check-docs.ts` dynamically validates that documentation is 100% synchronized with actual test files and test counts. If drift is detected, CI and `vp run verify` will fail.

## 7. Naming Conventions & File Suffixes

| Pattern                      | Usage                                                                           |
| :--------------------------- | :------------------------------------------------------------------------------ |
| `*.test.ts` / `*.test.tsx`   | Standard unit, component, utility, and protocol contract test suites.           |
| `*.spec.ts` / `*.spec.tsx`   | End-to-end browser specifications and behavioral acceptance flows (Playwright). |
| `*.mock.ts` / `*.fixture.ts` | Test fixtures, mock API handlers, and synthetic data generators.                |
| `helpers.ts` / `setup.ts`    | Shared test harnesses, runner abstractions, and environment initializers.       |

## 8. Automated Pipeline Enforcement

Every test suite defined by this standard is strictly executed in `vp run verify` (or `bun scripts/verify.ts`):

- `[1/18]` `cargo fmt --check`
- `[2/18]` `cargo clippy`
- `[3/18]` `cargo test --workspace`
- `[4/18]` `tsc -p tsconfig.json`
- `[5/18]` `bun test scripts/tests`
- `[6/18]` `bun test tests/mcp`
- `[7/18]` `vp check`
- `[8/18]` `vp -C webui run test`
