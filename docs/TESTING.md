# CDDM Testing Architecture & Quality Assurance Standard

This document outlines the testing architecture, organizational conventions, execution strategies, and verification pipelines across all components in the **CDDM (Code De-Duplication Meister)** repository.

---

## 1. Multi-Tier Testing Hierarchy

CDDM adopts a polyglot, multi-tier testing model that enforces strict locality and scope-driven placement:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. Co-located Unit & Component Tests                                       │
│    Located directly adjacent to source code (*.test.ts / *.test.tsx)        │
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

---

## 2. Polyglot Testing Matrix

| Subsystem / Layer                                                 | Test Tier                          | Location Standard                                                    | Toolchain Runner                    |
| :---------------------------------------------------------------- | :--------------------------------- | :------------------------------------------------------------------- | :---------------------------------- |
| **Rust Crates** (`cddm-core`, `cddm-cli`, `cddm-lsp`, `cddm-mcp`) | **Unit Tests**                     | `#[cfg(test)] mod tests` or sibling `tests.rs` submodule             | `cargo test --workspace --lib`      |
| **Rust Crates**                                                   | **Integration Tests**              | `crates/<crate>/tests/*.rs` (`tests/common/mod.rs`)                  | `cargo test --workspace --test '*'` |
| **Rust Crates**                                                   | **Benchmarks**                     | `crates/<crate>/benches/*.rs`                                        | `cargo bench -p cddm-core`          |
| **Rust Crates**                                                   | **Doctests**                       | `/// ```rust` doc examples                                           | `cargo test --doc`                  |
| **WebUI Studio** (`webui/src/`)                                   | **Component Tests**                | **Direct Co-location**: `Component.test.tsx` next to `Component.tsx` | `vp -C webui run test` (Vitest)     |
| **WebUI Studio** (`webui/src/`)                                   | **Store / Hook / Util Tests**      | **Direct Co-location**: `*.test.ts` next to source file              | `vp -C webui run test` (Vitest)     |
| **Workspace Scripts** (`scripts/lib/`)                            | **Script Utility Tests**           | **Direct Co-location**: `scripts/lib/*.test.ts`                      | `bun test scripts/lib`              |
| **Workspace Scripts** (`scripts/`)                                | **CLI Script Execution Tests**     | **Canonical Directory**: `scripts/tests/*.test.ts`                   | `bun test scripts/tests`            |
| **MCP Server** (`tests/mcp/`)                                     | **Protocol & Tool Contract Tests** | `tests/mcp/tools/*.test.ts` + `discovery.test.ts`                    | `bun test tests/mcp`                |
| **Full System E2E** (`tests/e2e/`)                                | **Browser & Workflow Specs**       | `tests/e2e/*.spec.ts`                                                | Playwright / Bun                    |

---

## 3. Rust Testing Standards (`crates/`)

### Unit Tests

Unit tests in Rust crates reside in `#[cfg(test)] mod tests` blocks at the bottom of the source file. To strictly observe the 500-line modularity cap (`.agents/rules/modularity-and-file-limits.md`), modules exceeding ~400 lines extract their unit tests into a sibling `tests.rs` submodule file within the same directory.

### Integration Tests

Black-box tests exercising public crate APIs reside under `crates/<crate-name>/tests/`. Common helpers, mock file builders, and fixtures are grouped under `crates/<crate-name>/tests/common/mod.rs`.

### Benchmarks

Performance-critical subsystems (SIMD rolling hash computation, tree-sitter AST traversal, control flow graph construction) are benchmarked with Criterion under `crates/<crate-name>/benches/`.

---

## 4. WebUI Studio Testing Standards (`webui/`)

### Mandatory Co-Location

All UI components, custom hooks, Zustand store slices, and utility functions MUST place their test files directly adjacent to the implementation:

```text
webui/src/
├── components/
│   ├── App.tsx
│   ├── App.test.tsx                         # Co-located component test
│   ├── CloneClusterCard.tsx
│   ├── CloneClusterCard.test.tsx
│   └── ui/
│       ├── badge.tsx
│       ├── badge.test.tsx
│       └── win2x-manager/
│           ├── win2x-manager.tsx
│           ├── win2x-manager.test.tsx
│           ├── pointer-driver.ts
│           └── pointer-driver.test.ts
├── store/
│   ├── cddm-store.ts
│   ├── cddm-store.test.ts                   # Co-located store test
│   ├── watch-slice.ts
│   └── watch-slice.test.ts
└── utils/
    ├── ide-links.ts
    └── ide-links.test.ts                    # Co-located utility test
```

Legacy `__tests__/` subdirectories are strictly forbidden.

### Testing Philosophy

- **User-Centric Testing**: Test component behavior using React Testing Library queries (`getByRole`, `findByText`, `userEvent`) rather than testing internal state.
- **Strict TypeScript**: All test mocks and assertion fixtures must be strictly typed with zero `any`.

---

## 5. Workspace Scripts Testing (`scripts/`)

- Reusable helper libraries in `scripts/lib/` must have co-located unit tests (e.g. `scripts/lib/step-runner.test.ts`).
- End-to-end executable script validation suites reside in `scripts/tests/*.test.ts` (e.g. `scripts/tests/check-no-emojis.test.ts`, `scripts/tests/package-vscode.test.ts`).
- Legacy `scripts/__tests__/` directory is forbidden.

---

## 6. MCP Protocol & Tool Testing (`tests/mcp/`)

Model Context Protocol tool verification is organized into 1:1 dedicated test files under `tests/mcp/tools/<tool-kebab-case>.test.ts`:

- **Dynamic Discovery Quality Gate**: `tests/mcp/discovery.test.ts` dynamically queries `tools/list` at runtime and verifies that every exposed tool has a dedicated test suite.
- **Standard JSON-RPC 2.0 Runner**: `tests/mcp/helpers.ts` provides reusable stdio subprocess runners and error assertions.

## 7. Living Documentation & Dynamic Test Matrix Discovery

To eliminate brittle, manual hardcoding in documentation, CDDM employs a **Dynamic Test Matrix Discovery Engine** (`scripts/lib/test-matrix-generator.ts`):

- **Automatic Multi-Tier Scanning**: Scans Rust `crates/` for `#[test]` units, WebUI `webui/src/` for co-located test suites, `scripts/tests/` for tooling tests, and `tests/mcp/` for protocol contracts.
- **Dynamic Markdown Synchronization**: `bun scripts/sync-feature-matrix.ts` automatically computes live test counts and formats tables in `docs/FEATURE_MATRIX.md`.
- **Zero-Drift CI Validation**: `bun scripts/check-docs.ts` asserts that documentation is 100% in sync with code and test reality on every verification run.

---

## 8. Pipeline Execution

The complete testing suite is validated on every commit and pull request via `vp run verify` (or `bun scripts/verify.ts`), ensuring 100% test pass rates across all polyglot boundaries.
