# `win2x-manager`: Universal Self-Contained High-Performance Windowing System

## Problem Description & Core Requirements

The user has specified the requirements for the **`win2x-manager`** subsystem:

1. **Self-Contained & Fully Portable**:
   - Placed under [`webui/src/components/ui/win2x-manager/`](file:///x:/projects/cddm/webui/src/components/ui/win2x-manager/).
   - Contains its own complete documentation suite directly inside `webui/src/components/ui/win2x-manager/docs/`.
   - Zero Tailwind dependency: 100% pure modern **CSS Modules (`*.module.css`)** with scoped **CSS Custom Properties (`--win2x-*`)**.
   - Strict kebab-case naming for all directories and files (`win2x-window.tsx`, `win2x-window.module.css`).
2. **Implementation of All 5 Avenues (A, B, C, D, and E)**:
   - **Avenue A (`translate3d` Compositor Pipeline)**: GPU compositor-only motion without layout reflows.
   - **Avenue B (Configurable Motion State & Blur Decoupling)**: Configurable performance profiles (`extreme`, `balanced`, `quality` / `disableBlurWhileMoving`), allowing users to choose between maximum 120fps throughput and visual blur retention.
   - **Avenue C (Native `setPointerCapture` Engine)**: Direct hardware pointer events without window listener latency.
   - **Avenue D (CSS Containment Sandbox)**: `contain: layout paint;` to prevent document-wide reflows.
   - **Avenue E (Universal Portable Architecture)**: Zero external styling dependencies, pluggable storage adapters (`LocalStorage`, `Memory`, `Zustand`).
3. **Comprehensive Documentation & Coding Agent Guardrails**:
   - `docs/GUIDE.md`: Deep architectural guide, benchmarks, pipeline comparison, teaching & reference.
   - `docs/ARCHITECTURE.md`: Core system architecture, geometry engine, pointer driver, storage adapter.
   - `docs/ATOMIC_UI_STANDARD.md`: Strict rules and enforcement standard for Atomic UI component hierarchy, explicit boundaries, preventing agent drift.
4. **Comprehensive Test Coverage**:
   - Unit tests, integration tests, component tests, and E2E browser verification with 100% pass rate.

---

## Architecture & Directory Structure

```text
webui/src/components/ui/win2x-manager/
├── docs/
│   ├── GUIDE.md                        # Comprehensive technical guide, benchmark comparison, teaching & reference
│   ├── ARCHITECTURE.md                 # System architecture, engine specs, storage abstractions & CSS variables
│   └── ATOMIC_UI_STANDARD.md           # Atomic UI component rules & coding agent enforcement standards
│
├── core/
│   ├── types.ts                        # Universal window types, options, rects, directions, config modes
│   ├── geometry-engine.ts              # Pure math: viewport clamping, boundary calculation, snapping, docking
│   ├── pointer-driver.ts               # W3C Pointer Events capture driver & RAF throttle dispatcher
│   └── storage-adapter.ts              # Universal storage interface (LocalStorage, Memory, Custom)
│
├── hooks/
│   ├── use-body-scroll-lock.ts         # Reference-counted background scroll locking
│   ├── use-pointer-drag.ts             # Hardware-accelerated pointer drag hook
│   ├── use-pointer-resize.ts           # Hardware-accelerated 8-way resize hook
│   └── use-window-state.ts             # Persistent bounds and window lifecycle hook
│
├── styles/
│   └── win2x-theme.css                 # Scoped CSS custom properties (--win2x-*)
│
├── atoms/
│   ├── portal/
│   │   ├── portal.tsx
│   │   └── portal.module.css
│   ├── backdrop/
│   │   ├── backdrop.tsx
│   │   └── backdrop.module.css
│   ├── badge/
│   │   ├── badge.tsx
│   │   └── badge.module.css
│   ├── icon-button/
│   │   ├── icon-button.tsx
│   │   └── icon-button.module.css
│   ├── resize-handle/
│   │   ├── resize-handle.tsx
│   │   └── resize-handle.module.css
│   └── window-controls/
│       ├── window-controls.tsx
│       └── window-controls.module.css
│
├── molecules/
│   ├── resize-handle-group/
│   │   ├── resize-handle-group.tsx
│   │   └── resize-handle-group.module.css
│   ├── title-bar/
│   │   ├── title-bar.tsx
│   │   └── title-bar.module.css
│   ├── collapsible-card/
│   │   ├── collapsible-card.tsx
│   │   └── collapsible-card.module.css
│   └── code-block/
│       ├── code-block.tsx
│       └── code-block.module.css
│
├── organisms/
│   └── win2x-window/
│       ├── win2x-window.tsx
│       └── win2x-window.module.css
│
├── __tests__/
│   ├── geometry-engine.test.ts
│   ├── pointer-driver.test.ts
│   ├── storage-adapter.test.ts
│   ├── use-body-scroll-lock.test.ts
│   ├── use-pointer-drag.test.ts
│   ├── use-pointer-resize.test.ts
│   ├── win2x-window.test.tsx
│   ├── collapsible-card.test.tsx
│   └── code-block.test.tsx
│
└── index.ts                            # Universal standalone package entry point
```

---

## Scoped CSS Custom Properties System (`--win2x-*`)

```css
:root {
  --win2x-bg-base: #0f172a;
  --win2x-bg-acrylic: rgba(15, 23, 42, 0.88);
  --win2x-bg-acrylic-moving: rgba(15, 23, 42, 0.98);
  --win2x-bg-titlebar: rgba(2, 6, 23, 0.92);
  --win2x-bg-card: rgba(2, 6, 23, 0.7);
  --win2x-bg-card-header: rgba(15, 23, 42, 0.6);
  --win2x-bg-code: rgba(2, 6, 23, 0.85);

  --win2x-border-window: rgba(71, 85, 105, 0.75);
  --win2x-border-titlebar: rgba(30, 41, 59, 0.8);
  --win2x-border-card: rgba(30, 41, 59, 0.8);

  --win2x-text-primary: #f8fafc;
  --win2x-text-secondary: #94a3b8;
  --win2x-text-muted: #64748b;
  --win2x-accent-primary: #6366f1;
  --win2x-accent-hover: #818cf8;
  --win2x-accent-active: #4f46e5;
  --win2x-danger-bg: #e11d48;

  --win2x-radius-window: 16px;
  --win2x-radius-card: 12px;
  --win2x-radius-button: 6px;

  --win2x-shadow-window: 0 25px 50px -12px rgba(0, 0, 0, 0.75), 0 0 0 1px rgba(255, 255, 255, 0.05);
  --win2x-blur-acrylic: 24px;
  --win2x-blur-backdrop: 8px;

  --win2x-z-backdrop: 9990;
  --win2x-z-window: 9995;
  --win2x-z-pill: 9999;
}
```

---

## Verification Plan

### Automated Tests

- Full test suite for core algorithms, hooks, and CSS Module components under `__tests__/`.
- Refactor `RefactorPatchModal` to use `win2x-manager`.
- Full repository quality verification: `bun scripts/verify.ts` (all 11 quality gates).

### Live Browser Validation

- Test smooth 120fps drag/resize motion via browser subagent.
- Validate configurable blur decoupling modes (`extreme`, `balanced`, `quality`).
- Validate complete self-containment with pure CSS Modules.
