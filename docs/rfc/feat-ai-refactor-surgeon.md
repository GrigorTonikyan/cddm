# feat(ai-surgeon): automated AST clone cluster refactoring surgeon [EP-35]

## Summary
Implements automated AST refactoring surgeon that extracts multi-file duplicate clone clusters into standalone shared functions or modules with syntax preservation.

## Proposed Architecture
1. AST visitor identifies parameterizable variable differences across clone pairs.
2. Synthesizes canonical signature and extracted function body.
3. Rewrites call sites across all cluster files.
4. Executes `cargo test` / `bun test` to verify zero regressions.

## Checklist
- [x] AST difference parameterizer.
- [ ] Multi-language shared module code synthesizer.
- [ ] Automated rollback harness.

Branch: `feat/ai-refactor-surgeon`
Milestone: v1.10.0
