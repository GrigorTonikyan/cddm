---
trigger: always_on
---

# Legacy Code Remediation Protocol

When touching or editing existing code that violates workspace rules, follow one of these two paths.

## Path A: Direct Refactoring (Preferred)

Bring the touched module into full compliance: decompose files exceeding 500 lines, resolve warnings, add strict types, eliminate duplication. Update the ratcheted ceiling in [`scripts/check-file-length.ts`](scripts/check-file-length.ts) to lock in the reduced line count.

## Path B: Explicit Technical Debt Recording (Exception Only)

If full remediation is strictly impossible within the current task scope:

1. Pause. Do not silently ignore the violation.
2. Record a structured debt entry in [`docs/TODO.md`](docs/TODO.md) with: file path, current line count, specific rule violations, and a concrete decomposition plan.
3. Proceed with the minimal necessary change, ensuring the file does not grow beyond its registered ratchet cap.
