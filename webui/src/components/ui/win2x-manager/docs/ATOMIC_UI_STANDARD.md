# `win2x-manager` Atomic UI Component Standard & Agent Enforcement Standard

## 1. Purpose & Scope

This document establishes the strict, non-negotiable architectural boundaries between the general Atomic UI component library (`webui/src/components/ui/`) and the pure windowing subsystem (`webui/src/components/ui/win2x-manager/`).

---

## 2. Component System Separation

```text
webui/src/components/ui/
├── atoms/                                 # Pure presentational primitives (Portal, Backdrop, Badge, IconButton)
├── molecules/                             # Composed molecules (CollapsibleCard, CodeBlock)
└── win2x-manager/                         # Pure window management subsystem
    └── components/                        # Window-specific UI (TitleBar, WindowControls, ResizeHandles, Win2xWindow)
```

---

## 3. Strict Rules for AI Coding Agents

### Rule 1: Zero Tailwind Classes in `ui/` and `win2x-manager/`

- **FORBIDDEN**: Using Tailwind utility classes anywhere inside `ui/` or `win2x-manager/`.
- **MANDATORY**: Every component MUST have its own companion CSS Module (`<component-name>.module.css`) in the exact same directory.
- **MANDATORY**: All CSS Modules MUST use standard modern CSS nesting (`& { ...; &:hover { ... }; &[data-moving="true"] { ... } }`).

### Rule 2: Zero Magic Numbers or Hardcoded Literals

- **FORBIDDEN**: Inline magic numbers, raw strings, fallback literals, or arbitrary pixels in TypeScript or CSS.
- **MANDATORY**: Derive all constants from `ui/constants/ui-constants.ts` or `win2x-manager/constants/win2x-constants.ts`.
- **MANDATORY**: Derive all CSS values from `--cddm-ui-*` or `--win2x-*` custom properties.

### Rule 3: Pure Window Manager Domain Boundary

- `win2x-manager` MUST NOT own domain-agnostic UI primitives (badges, cards, code blocks).
- Window-specific components live under `win2x-manager/components/` and compose general atoms (`Portal`, `Backdrop`) from `ui/atoms/`.

### Rule 4: Strict Kebab-Case Directory & File Naming

- Directories and files must be named in kebab-case (`components/title-bar/title-bar.tsx`).
- Function and interface exports must use standard PascalCase.

### Rule 5: Zero Emojis Anywhere

- Absolutely no emojis in source files, CSS comments, markdown documentation, tests, or commit messages. Use clean text tags (`[PASS]`, `[FAIL]`, `[OK]`, `[WARN]`).

### Rule 6: Hardware-Accelerated Motion Enforcement

- Draggable motion MUST use `transform: translate3d(x, y, 0)`.
- Pointer events MUST use W3C Pointer Events with `setPointerCapture`.
- CSS transitions MUST be disabled when `[data-moving="true"]` is active.
- CSS containment (`contain: layout paint`) MUST be applied to the window container.
