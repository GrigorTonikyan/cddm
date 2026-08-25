# `win2x-manager`: Universal Self-Contained High-Performance Windowing System

> Comprehensive implementation plan -- audited against the full existing codebase.

---

## 1. Problem Statement & Core Requirements

The existing Window system (under `webui/src/components/ui/{atoms,molecules,organisms}/` and `webui/src/hooks/`) suffers from:

- **Performance**: Every `mousemove` triggers React state updates + `localStorage` writes. CSS transitions and double `backdrop-filter` blur add latency.
- **Tailwind Coupling**: All 10+ window components are styled with Tailwind utility classes, making them non-portable.
- **Scattered Layout**: Hooks live in `hooks/`, atoms in `ui/atoms/`, types in `types/cddm-types.ts`, store logic in `store/cddm-store.ts` -- the window subsystem is spread across 15+ files in 6 directories.

### User Requirements (Verbatim)

1. Feature named **`win2x-manager`**, fully self-contained under one directory.
2. **Zero Tailwind**: Pure CSS Modules (`.module.css`) with scoped CSS Custom Properties (`--win2x-*`).
3. **Self-contained docs** inside the feature directory, including:
   - Technical guide (teaching + reference)
   - Architecture specification
   - Atomic UI enforcement standard for coding agents
4. All **5 performance avenues** implemented (translate3d, blur decoupling, pointer capture, CSS containment, portable architecture).
5. Blur decoupling **configurable** (not forced -- default behavior but user can opt to keep blur during motion).
6. Kebab-case file/directory naming, standard names for exports.
7. Full test coverage (unit, integration, component tests).
8. Feature must be **universal, portable, and self-contained** -- copy to any React project.

---

## 2. Architecture & Directory Structure

```text
webui/src/components/ui/win2x-manager/
│
├── docs/                                  # [SELF-CONTAINED DOCUMENTATION]
│   ├── GUIDE.md                           # Comprehensive technical guide & teaching reference
│   │                                      #   - Browser rendering pipeline deep-dive
│   │                                      #   - Avenue A-E comparison matrix with pros/cons/benchmarks
│   │                                      #   - CSS containment & compositor layer analysis
│   │                                      #   - Configurable performance profiles explained
│   │                                      #   - Integration tutorial & usage examples
│   ├── ARCHITECTURE.md                    # System architecture specification
│   │                                      #   - Directory layout & file responsibility map
│   │                                      #   - Core engine contracts (GeometryEngine, PointerDriver, StorageAdapter)
│   │                                      #   - CSS custom properties contract (--win2x-* variable catalog)
│   │                                      #   - Hook composition diagram
│   │                                      #   - Data flow: pointer event -> RAF -> DOM -> state commit
│   └── ATOMIC_UI_STANDARD.md             # Atomic UI rules & coding agent enforcement
│                                          #   - Atom / Molecule / Organism strict boundary definitions
│                                          #   - What atoms CAN and CANNOT do (no hooks, no store, no side effects)
│                                          #   - What molecules CAN compose (atoms only, no organisms)
│                                          #   - Naming conventions (kebab-case dirs, PascalCase exports)
│                                          #   - CSS Module scoping rules (no global selectors, no Tailwind)
│                                          #   - Forbidden patterns checklist for AI agents
│
├── core/                                  # [FRAMEWORK-AGNOSTIC PURE LOGIC]
│   ├── types.ts                           # All types: Win2xRect, Win2xWindowState, Win2xConfig,
│   │                                      #   ResizeDirection, PerformanceProfile, StorageProvider
│   ├── geometry-engine.ts                 # Pure functions: clampToViewport(), computeResize(),
│   │                                      #   centerInViewport(), constrainMinSize()
│   ├── pointer-driver.ts                  # RAF-throttled pointer event dispatcher (no React dependency)
│   │                                      #   Uses setPointerCapture/releasePointerCapture
│   │                                      #   Provides: createDragSession(), createResizeSession()
│   └── storage-adapter.ts                # StorageProvider interface + LocalStorageAdapter + MemoryAdapter
│
├── hooks/                                 # [REACT HOOKS -- compose core engines]
│   ├── use-body-scroll-lock.ts            # Reference-counted document.body scroll locking
│   ├── use-pointer-drag.ts                # Wraps pointer-driver.createDragSession() in React lifecycle
│   │                                      #   Returns: { onPointerDown, isDragging }
│   ├── use-pointer-resize.ts              # Wraps pointer-driver.createResizeSession() in React lifecycle
│   │                                      #   Returns: { onResizePointerDown, isResizing }
│   └── use-window-state.ts                # Standalone persistent window bounds hook
│                                          #   Accepts StorageProvider (default: LocalStorageAdapter)
│                                          #   No dependency on useCDDMStore (fully decoupled)
│
├── styles/                                # [DESIGN TOKEN SYSTEM]
│   └── win2x-theme.css                    # Root-level CSS custom properties (--win2x-*)
│                                          #   Imported once by win2x-window.module.css
│
├── atoms/                                 # [ATOMIC PRIMITIVES -- zero logic, pure presentation]
│   ├── portal/
│   │   ├── portal.tsx                     # createPortal to document.body
│   │   └── portal.module.css              # (minimal, may be empty)
│   ├── backdrop/
│   │   ├── backdrop.tsx                   # Overlay with configurable blur/opacity
│   │   └── backdrop.module.css            # Fixed inset, backdrop-filter via CSS vars
│   ├── badge/
│   │   ├── badge.tsx                      # Color-variant pill badges
│   │   └── badge.module.css              # Variant classes via CSS vars
│   ├── icon-button/
│   │   ├── icon-button.tsx                # Accessible icon button with tooltip
│   │   └── icon-button.module.css
│   ├── resize-handle/
│   │   ├── resize-handle.tsx              # Single directional resize hit-target
│   │   └── resize-handle.module.css       # Positioning & cursor via CSS vars
│   └── window-controls/
│       ├── window-controls.tsx            # Win2x caption button triad (Min, Max/Restore, Close)
│       └── window-controls.module.css     # Button hover states, close button danger color
│
├── molecules/                             # [COMPOSED FROM ATOMS ONLY]
│   ├── resize-handle-group/
│   │   ├── resize-handle-group.tsx        # Assembles 8 ResizeHandle atoms
│   │   └── resize-handle-group.module.css
│   ├── title-bar/
│   │   ├── title-bar.tsx                  # Icon + Title/Subtitle + WindowControls
│   │   │                                  #   Receives onPointerDown for drag
│   │   │                                  #   touch-action: none for pointer capture
│   │   └── title-bar.module.css           # Titlebar layout, cursor-move, drag styling
│   ├── collapsible-card/
│   │   ├── collapsible-card.tsx           # Expandable accordion card
│   │   └── collapsible-card.module.css    # Chevron rotation, expand/collapse animation
│   └── code-block/
│       ├── code-block.tsx                 # Horizontal-scrolling code panel
│       └── code-block.module.css          # whitespace-pre, overflow-x-auto via CSS
│
├── organisms/                             # [TOP-LEVEL COMPOSED COMPONENTS]
│   └── win2x-window/
│       ├── win2x-window.tsx               # Full window organism composing all layers
│       │                                  #   Props: isOpen, onClose, title, subtitle, icon,
│       │                                  #          children, footer, initialWidth, initialHeight,
│       │                                  #          minWidth, minHeight, showMinimize,
│       │                                  #          performanceProfile ("extreme"|"balanced"|"quality"),
│       │                                  #          disableBlurWhileMoving (boolean, default: true),
│       │                                  #          storageKey, storageProvider
│       │                                  #   Uses: translate3d for position, contain: layout paint,
│       │                                  #          data-moving attribute for CSS motion state,
│       │                                  #          pointer-events-none on children during drag
│       └── win2x-window.module.css        # Window frame, maximized/restored states,
│                                          #   [data-moving="true"] blur decoupling rules,
│                                          #   minimization dock pill, CSS containment
│
├── __tests__/                             # [COMPREHENSIVE TEST SUITE]
│   ├── geometry-engine.test.ts            # Pure math: clamping, resizing, centering, min constraints
│   ├── pointer-driver.test.ts             # RAF dispatch, capture/release lifecycle, event coordinates
│   ├── storage-adapter.test.ts            # LocalStorage read/write/corruption recovery, MemoryAdapter
│   ├── use-body-scroll-lock.test.ts       # Reference counting, mount/unmount cleanup
│   ├── use-pointer-drag.test.ts           # Drag start/move/end lifecycle, boundary clamping
│   ├── use-pointer-resize.test.ts         # 8-direction resize, min size enforcement
│   ├── use-window-state.test.ts           # Persistence, default centering, reset, maximize/minimize
│   ├── win2x-window.test.tsx              # Full organism: open/close, escape key, minimize pill,
│   │                                      #   maximize/restore, data-moving attribute toggle,
│   │                                      #   backdrop click-to-dismiss, footer rendering
│   ├── collapsible-card.test.tsx           # Expand/collapse, controlled/uncontrolled, badge rendering
│   └── code-block.test.tsx                # Horizontal scroll enforcement, copy button, empty state
│
├── css.d.ts                               # TypeScript declaration for *.module.css imports
│                                          #   declare module "*.module.css" { ... }
│
└── index.ts                               # Barrel export: Win2xWindow, CollapsibleCard, CodeBlock,
                                           #   Badge, IconButton, useBodyScrollLock, usePointerDrag,
                                           #   usePointerResize, useWindowState, types, StorageAdapter
```

---

## 3. CSS Custom Properties Contract (`--win2x-*`)

All visual tokens are declared in [`win2x-theme.css`](../webui/src/components/ui/win2x-manager/styles/win2x-theme.css) and consumed by CSS Modules:

| Category   | Variable                     | Default Value             | Purpose                       |
| :--------- | :--------------------------- | :------------------------ | :---------------------------- |
| Background | `--win2x-bg-base`            | `#0f172a`                 | Window body background        |
| Background | `--win2x-bg-acrylic`         | `rgba(15, 23, 42, 0.88)`  | Acrylic glass effect (idle)   |
| Background | `--win2x-bg-acrylic-moving`  | `rgba(15, 23, 42, 0.98)`  | Solid fallback during motion  |
| Background | `--win2x-bg-titlebar`        | `rgba(2, 6, 23, 0.92)`    | Title bar background          |
| Background | `--win2x-bg-card`            | `rgba(2, 6, 23, 0.70)`    | Card body background          |
| Background | `--win2x-bg-card-header`     | `rgba(15, 23, 42, 0.60)`  | Card header background        |
| Background | `--win2x-bg-code`            | `rgba(2, 6, 23, 0.85)`    | Code block background         |
| Border     | `--win2x-border-window`      | `rgba(71, 85, 105, 0.75)` | Window frame border           |
| Border     | `--win2x-border-titlebar`    | `rgba(30, 41, 59, 0.80)`  | Title bar bottom border       |
| Border     | `--win2x-border-card`        | `rgba(30, 41, 59, 0.80)`  | Card border                   |
| Text       | `--win2x-text-primary`       | `#f8fafc`                 | Primary text color            |
| Text       | `--win2x-text-secondary`     | `#94a3b8`                 | Secondary / subtitle text     |
| Text       | `--win2x-text-muted`         | `#64748b`                 | Muted / disabled text         |
| Accent     | `--win2x-accent-primary`     | `#6366f1`                 | Primary accent (indigo)       |
| Accent     | `--win2x-accent-hover`       | `#818cf8`                 | Accent hover state            |
| Accent     | `--win2x-accent-active`      | `#4f46e5`                 | Accent active/pressed state   |
| Danger     | `--win2x-danger-bg`          | `#e11d48`                 | Close button hover background |
| Radius     | `--win2x-radius-window`      | `16px`                    | Window border-radius          |
| Radius     | `--win2x-radius-card`        | `12px`                    | Card border-radius            |
| Radius     | `--win2x-radius-button`      | `6px`                     | Button border-radius          |
| Shadow     | `--win2x-shadow-window`      | (deep shadow composite)   | Window drop shadow            |
| Effect     | `--win2x-blur-acrylic`       | `24px`                    | Window backdrop-filter blur   |
| Effect     | `--win2x-blur-backdrop`      | `8px`                     | Overlay backdrop blur         |
| Timing     | `--win2x-transition-fast`    | `100ms`                   | Fast UI transitions           |
| Timing     | `--win2x-transition-normal`  | `200ms`                   | Normal UI transitions         |
| Z-index    | `--win2x-z-backdrop`         | `9990`                    | Backdrop z-index              |
| Z-index    | `--win2x-z-window`           | `9995`                    | Window z-index                |
| Z-index    | `--win2x-z-pill`             | `9999`                    | Minimized pill z-index        |
| Size       | `--win2x-titlebar-height`    | `44px`                    | Title bar height              |
| Size       | `--win2x-resize-handle-size` | `6px`                     | Edge resize handle thickness  |
| Size       | `--win2x-resize-corner-size` | `14px`                    | Corner resize handle size     |

---

## 4. Five Performance Avenues -- Implementation Details

### Avenue A: `translate3d` Compositor Pipeline

- **Current problem**: `style.top` and `style.left` trigger Layout (Reflow) and Paint on the CPU main thread.
- **Solution**: Lock window at `top: 0; left: 0; position: fixed;`. Apply all position changes via `style.transform = translate3d(x, y, 0)`. Resizing still updates `style.width` and `style.height` (unavoidable for content reflow) but position is compositor-only.
- **CSS**: `will-change: transform` during active drag, reset to `auto` on release.

### Avenue B: Configurable Motion State & Blur Decoupling

- **Current problem**: Nested `backdrop-filter: blur(24px)` on the window + `backdrop-filter: blur(8px)` on the overlay forces double per-frame GPU shader passes.
- **Solution**: On `pointerdown`, set `data-moving="true"` attribute on the window container. CSS rule:
  ```css
  .window[data-moving="true"] {
    backdrop-filter: none;
    background: var(--win2x-bg-acrylic-moving);
    transition: none !important;
  }
  ```
  On `pointerup`, remove attribute to restore blur.
- **Configurable**: `disableBlurWhileMoving` prop (default: `true`). When `false`, blur is retained during motion (for users who prioritize visual quality over framerate). Additionally, `performanceProfile` prop:
  - `"extreme"`: Disables blur on both backdrop and window during motion. Disables ALL transitions during motion.
  - `"balanced"` (default): Disables blur only on window during motion. Backdrop retains low blur.
  - `"quality"`: Retains all blur effects during motion. No performance shortcuts.

### Avenue C: Native `setPointerCapture` Engine

- **Current problem**: `window.addEventListener("mousemove")` introduces event propagation delay, mouse-only support, and dropped events when cursor exits the browser.
- **Solution**: On `pointerdown`, call `e.currentTarget.setPointerCapture(e.pointerId)`. Listen for `pointermove` and `pointerup` on the same element (events are automatically redirected). On release, `releasePointerCapture()`.
- **Additional**: Set `touch-action: none` on title bar and resize handles in CSS to prevent browser touch gestures from interfering.

### Avenue D: CSS Containment Sandbox

- **Current problem**: Moving or resizing the window causes the browser to recalculate layout for ALL elements in the document.
- **Solution**: Apply `contain: layout paint;` on the window container. This tells the browser the window's subtree is independent and cannot affect external layout.
- **Additional**: Apply `content-visibility: auto` on the window body content area to defer rendering of off-screen collapsible sections.

### Avenue E: Universal Portable Architecture

- **Current problem**: Window state is coupled to `useCDDMStore` (Zustand), types live in `cddm-types.ts`, hooks reference CDDM-specific stores.
- **Solution**:
  - `use-window-state.ts` accepts a `StorageProvider` interface (`{ get(key): T | null, set(key, value): void }`) instead of importing `useCDDMStore`.
  - Default implementation: `LocalStorageAdapter` (reads/writes `localStorage`).
  - Alternative: `MemoryAdapter` (ephemeral, no persistence).
  - CDDM integration: In `RefactorPatchModal`, pass a custom adapter that delegates to `useCDDMStore` if desired.

---

## 5. TypeScript Infrastructure

### CSS Module Type Declaration (`css.d.ts`)

Required for TypeScript to understand `import styles from "./foo.module.css"`:

```typescript
declare module "*.module.css" {
  const classes: Readonly<Record<string, string>>;
  export default classes;
}
```

### Vite CSS Modules Support

Vite supports CSS Modules natively (any `.module.css` file). No configuration changes needed. The existing `@tailwindcss/vite` plugin remains for the rest of the app; win2x-manager simply does not use any Tailwind classes.

---

## 6. Migration & Cleanup Strategy

### Phase 1: Build `win2x-manager` (New)

Create the entire `webui/src/components/ui/win2x-manager/` tree from scratch. This is an additive operation -- the old components remain untouched during development.

### Phase 2: Migrate Consumer (`RefactorPatchModal`)

Update [`RefactorPatchModal.tsx`](../webui/src/components/RefactorPatchModal.tsx) to import from `win2x-manager/index.ts` instead of the old `./ui` barrel:

```diff
-import { Window, CollapsibleCard, CodeBlock } from "./ui";
+import { Win2xWindow, CollapsibleCard, CodeBlock } from "./ui/win2x-manager";
```

### Phase 3: Remove Old Window Components

Delete the following files that are now superseded by `win2x-manager`:

| Old File (DELETE)                                  | Replaced By                                               |
| :------------------------------------------------- | :-------------------------------------------------------- |
| `hooks/useBodyScrollLock.ts`                       | `win2x-manager/hooks/use-body-scroll-lock.ts`             |
| `hooks/useDraggable.ts`                            | `win2x-manager/hooks/use-pointer-drag.ts`                 |
| `hooks/useResizable.ts`                            | `win2x-manager/hooks/use-pointer-resize.ts`               |
| `hooks/useWindowState.ts`                          | `win2x-manager/hooks/use-window-state.ts`                 |
| `hooks/__tests__/useBodyScrollLock.test.ts`        | `win2x-manager/__tests__/use-body-scroll-lock.test.ts`    |
| `hooks/__tests__/useDraggable.test.ts`             | `win2x-manager/__tests__/use-pointer-drag.test.ts`        |
| `hooks/__tests__/useResizable.test.ts`             | `win2x-manager/__tests__/use-pointer-resize.test.ts`      |
| `components/ui/atoms/Portal.tsx`                   | `win2x-manager/atoms/portal/portal.tsx`                   |
| `components/ui/atoms/Backdrop.tsx`                 | `win2x-manager/atoms/backdrop/backdrop.tsx`               |
| `components/ui/atoms/Badge.tsx`                    | `win2x-manager/atoms/badge/badge.tsx`                     |
| `components/ui/atoms/IconButton.tsx`               | `win2x-manager/atoms/icon-button/icon-button.tsx`         |
| `components/ui/atoms/ResizeHandle.tsx`             | `win2x-manager/atoms/resize-handle/resize-handle.tsx`     |
| `components/ui/atoms/WindowControls.tsx`           | `win2x-manager/atoms/window-controls/window-controls.tsx` |
| `components/ui/molecules/ResizeHandleGroup.tsx`    | `win2x-manager/molecules/resize-handle-group/`            |
| `components/ui/molecules/TitleBar.tsx`             | `win2x-manager/molecules/title-bar/`                      |
| `components/ui/molecules/CollapsibleCard.tsx`      | `win2x-manager/molecules/collapsible-card/`               |
| `components/ui/molecules/CodeBlock.tsx`            | `win2x-manager/molecules/code-block/`                     |
| `components/ui/organisms/Window.tsx`               | `win2x-manager/organisms/win2x-window/`                   |
| `components/ui/__tests__/Window.test.tsx`          | `win2x-manager/__tests__/win2x-window.test.tsx`           |
| `components/ui/__tests__/CollapsibleCard.test.tsx` | `win2x-manager/__tests__/collapsible-card.test.tsx`       |
| `components/ui/__tests__/CodeBlock.test.tsx`       | `win2x-manager/__tests__/code-block.test.tsx`             |

### Phase 4: Update `ui/index.ts` Barrel

After cleanup, `ui/index.ts` re-exports from `win2x-manager`:

```typescript
// Re-export win2x-manager as the canonical UI window system
export * from "./win2x-manager";
```

### Phase 5: Remove `ModalWindowState` from CDDM Store

Remove `modalWindowState`, `setModalWindowState`, `loadPersistedWindowState()`, `persistWindowState()` from [`cddm-store.ts`](../webui/src/store/cddm-store.ts) and `ModalWindowState`/`DEFAULT_MODAL_WINDOW_STATE` from [`cddm-types.ts`](../webui/src/types/cddm-types.ts). The window state is now self-contained inside `win2x-manager` via its own `StorageAdapter`.

---

## 7. Accessibility Considerations

- **Keyboard**: Escape key closes the window. Focus is trapped inside the window when open. Tab cycling stays within the window boundary.
- **ARIA**: Window container has `role="dialog"`, `aria-modal="true"`, `aria-label={title}`. Close button has `aria-label="Close"`. Minimize/Maximize buttons have descriptive `aria-label` values.
- **Focus Management**: On open, focus moves to the first focusable element inside the window. On close, focus returns to the element that triggered the open.

---

## 8. Test Strategy

### Unit Tests (Core)

| Test File                 | Coverage Target                                                                                                                                        |
| :------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `geometry-engine.test.ts` | `clampToViewport`, `computeResize` for all 8 directions, `centerInViewport`, `constrainMinSize`, edge cases (zero viewport, negative coords)           |
| `pointer-driver.test.ts`  | `createDragSession` lifecycle, `createResizeSession` lifecycle, RAF throttle coalescing, `setPointerCapture`/`releasePointerCapture` mock verification |
| `storage-adapter.test.ts` | `LocalStorageAdapter` read/write/corruption recovery/missing key, `MemoryAdapter` ephemeral behavior                                                   |

### Hook Tests (React)

| Test File                      | Coverage Target                                                                                                   |
| :----------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `use-body-scroll-lock.test.ts` | Lock/unlock lifecycle, reference counting, nested mount/unmount                                                   |
| `use-pointer-drag.test.ts`     | `onPointerDown` handler type, `isDragging` state toggle, cleanup on unmount                                       |
| `use-pointer-resize.test.ts`   | `onResizePointerDown` handler type, `isResizing` state toggle, direction validation                               |
| `use-window-state.test.ts`     | Default centering, persistence via adapter, `toggleMaximize`, `toggleMinimize`, `resetState`, custom `storageKey` |

### Component Tests (DOM)

| Test File                   | Coverage Target                                                                                                                                                                                                  |
| :-------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `win2x-window.test.tsx`     | Renders when open, hidden when closed, Escape key closes, backdrop click closes, minimize creates pill, maximize toggles fullscreen, `data-moving` attribute toggles, footer renders, CSS module classes applied |
| `collapsible-card.test.tsx` | Default open/closed, toggle expands/collapses, controlled mode, badge renders, actions slot renders                                                                                                              |
| `code-block.test.tsx`       | Horizontal scroll enforced (`whitespace-pre`), copy button fires clipboard, empty placeholder renders                                                                                                            |

### CSS Module Verification

- All tests verify that CSS module classes are applied (not Tailwind utility classes).
- Snapshot-free: assertions check `data-*` attributes and DOM structure, not class name strings.

---

## 9. Execution Order

| Phase | Description                                                        | Files Created / Modified          |
| :---- | :----------------------------------------------------------------- | :-------------------------------- |
| 1     | Create `docs/GUIDE.md` (comprehensive technical guide)             | 1 new                             |
| 2     | Create `docs/ARCHITECTURE.md` (system architecture spec)           | 1 new                             |
| 3     | Create `docs/ATOMIC_UI_STANDARD.md` (agent enforcement rules)      | 1 new                             |
| 4     | Create `core/types.ts` (all types and interfaces)                  | 1 new                             |
| 5     | Create `core/geometry-engine.ts` + tests                           | 2 new                             |
| 6     | Create `core/pointer-driver.ts` + tests                            | 2 new                             |
| 7     | Create `core/storage-adapter.ts` + tests                           | 2 new                             |
| 8     | Create `styles/win2x-theme.css` + `css.d.ts`                       | 2 new                             |
| 9     | Create hooks (4 files)                                             | 4 new                             |
| 10    | Create atoms (6 dirs x 2 files = 12 files)                         | 12 new                            |
| 11    | Create molecules (4 dirs x 2 files = 8 files)                      | 8 new                             |
| 12    | Create `organisms/win2x-window/` (2 files)                         | 2 new                             |
| 13    | Create remaining tests (hooks + components)                        | 7 new                             |
| 14    | Create `index.ts` barrel export                                    | 1 new                             |
| 15    | Migrate `RefactorPatchModal.tsx` imports                           | 1 modified                        |
| 16    | Delete old components, hooks, and tests (22 files)                 | 22 deleted                        |
| 17    | Update `ui/index.ts` to re-export from win2x-manager               | 1 modified                        |
| 18    | Remove `ModalWindowState` from `cddm-store.ts` and `cddm-types.ts` | 2 modified                        |
| 19    | Run `bun scripts/fix.ts` then `bun scripts/verify.ts`              | 0                                 |
| 20    | Browser subagent live validation                                   | 0                                 |
|       | **Total**                                                          | ~45 new, ~4 modified, ~22 deleted |

---

## 10. Verification Plan

### Automated

1. `vp -C webui run test` -- All win2x-manager tests + existing tests pass.
2. `vp -C webui run build` -- Production bundle compiles with CSS Modules.
3. `bun scripts/verify.ts` -- All 11 quality gates pass.

### Live Browser Validation

1. Launch `cargo run -p cddm-cli -- serve --port 3000`.
2. Browser subagent opens `http://127.0.0.1:3000`:
   - Run scan, expand clone pair, open Refactor Advisor.
   - Verify 120fps drag smoothness (translate3d compositor path).
   - Verify blur decouples during drag (window becomes solid), restores on release.
   - Verify 8-way pointer-capture resize.
   - Verify collapsible sections toggle instantly.
   - Verify CSS Module classes (no Tailwind utility classes in win2x-manager DOM).
   - Verify state persists across modal close/reopen.
