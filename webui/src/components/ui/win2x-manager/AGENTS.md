# `win2x-manager` AI Agent Development Guidelines & Enforcement Standard

## 1. Prime Directives for AI Agents

1. **Zero Tailwind Classes**:
   - Under no circumstances may Tailwind utility classes be introduced inside `win2x-manager/`.
   - All styling must reside in companion CSS Modules (`<component-name>.module.css`).

2. **Zero Hardcoded Values & Strict Constants / Enums**:
   - Absolutely no inline magic numbers, raw strings, fallback literals, or arbitrary pixels.
   - All defaults, directions, profiles, timeouts, and bounds MUST derive from `constants/win2x-constants.ts`.
   - In CSS, all visual attributes MUST derive from `--win2x-*` custom properties in `styles/win2x-theme.css`.

3. **Modern Nested CSS Scoping**:
   - All CSS Modules MUST use standard CSS nesting syntax (`& { ...; &:hover { ... }; &[data-moving="true"] { ... } }`).

4. **Pure Window Management Boundary**:
   - `win2x-manager` is a pure windowing subsystem and MUST NOT own generic UI components (badges, cards, code blocks, etc.).
   - Window-specific UI lives strictly in `components/` (`title-bar`, `window-controls`, `resize-handle`, `resize-handle-group`, `win2x-window`).
   - Generic UI primitives are consumed from `../atoms/` and `../molecules/`.

5. **Strict Kebab-Case Directory and File Structure**:
   - Component directories and files must use kebab-case (`components/title-bar/title-bar.tsx`).
   - Exports must use standard PascalCase.

6. **Zero Emoji Policy**:
   - Absolutely no emojis or pictographs anywhere in code, markdown documentation, tests, or comments.
   - Use clean text tags (`[PASS]`, `[FAIL]`, `[OK]`, `[WARN]`).

7. **Hardware-Accelerated 120fps Motion Enforcement**:
   - Window movement must execute via `transform: translate3d(...)` on the compositor thread.
   - Pointer events must use W3C Pointer Events with `setPointerCapture`.
   - The moving state must set `[data-moving="true"]` on the container to decouple `backdrop-filter` and disable CSS transitions during motion.
   - The container must declare `contain: layout paint;`.

8. **100% Test Coverage & Zero Quality Bypasses**:
   - Every core engine, hook, and component must have comprehensive unit/component tests in `__tests__/`.
   - All tests must pass with zero warnings (`vp -C webui run test`).
   - The master verification script (`bun scripts/verify.ts`) must pass all 11 quality gates.
