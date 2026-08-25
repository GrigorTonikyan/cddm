---
name: cddm-modular-refactoring
description: >-
  Use this skill when decomposing oversized files, reducing duplication,
  or refactoring non-adherent legacy code in the CDDM codebase.
---

# CDDM Modular Refactoring Runbook

## Rust Decomposition

1. Identify large file (e.g. `crates/<crate>/src/<module>.rs`)
2. Convert into a directory `crates/<crate>/src/<module>/` with `mod.rs`
3. Separate: `types.rs`, `handlers.rs`, `helpers.rs`
4. Export public symbols from `mod.rs`
5. Verify: `cargo check --workspace && cargo test --workspace`

## React / WebUI Decomposition

1. Identify large component (>500 lines) in `webui/src/components/`
2. Extract subcomponents into `components/<Feature>/`
3. Extract hooks into `webui/src/hooks/use<Feature>.ts`
4. Extract pure helpers into `webui/src/utils/<feature>-utils.ts`
5. Verify: `vp -C webui run test && vp -C webui run build`

## Ratchet Cap Updating

After decomposing a grandfathered file in [`scripts/check-file-length.ts`](scripts/check-file-length.ts):

1. Run `bun scripts/check-file-length.ts` to get the new line count
2. Lower the ceiling in `GRANDFATHERED_LINE_CAPS` to the new count
3. Verify: `bun test scripts/__tests__/file-length.test.ts`
