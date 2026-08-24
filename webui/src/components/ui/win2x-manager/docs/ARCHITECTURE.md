# `win2x-manager` System Architecture Specification

## 1. System Component Hierarchy

```text
webui/src/components/ui/
│
├── constants/                             # Shared UI enums, variants, and constants
│   └── ui-constants.ts
├── styles/                                # Shared UI design tokens (--cddm-ui-*)
│   └── theme.css
├── atoms/                                 # General Atomic UI primitives
│   ├── portal/
│   ├── backdrop/
│   ├── badge/
│   └── icon-button/
├── molecules/                             # General Molecular UI components
│   ├── collapsible-card/
│   └── code-block/
│
└── win2x-manager/                         # [PURE UNIVERSAL WINDOWING SUBSYSTEM]
    ├── AGENTS.md                          # Strict AI contributor guidelines
    ├── constants/                         # Window-specific constants & enums
    │   └── win2x-constants.ts
    ├── core/                              # Framework-agnostic pure logic
    │   ├── types.ts
    │   ├── geometry-engine.ts
    │   ├── pointer-driver.ts
    │   └── storage-adapter.ts
    ├── hooks/                             # React lifecycle hooks
    │   ├── use-body-scroll-lock.ts
    │   ├── use-pointer-drag.ts
    │   ├── use-pointer-resize.ts
    │   └── use-window-state.ts
    ├── styles/                            # Window design tokens (--win2x-*)
    │   └── win2x-theme.css
    ├── components/                        # Window-specific components
    │   ├── resize-handle/
    │   ├── resize-handle-group/
    │   ├── window-controls/
    │   ├── title-bar/
    │   └── win2x-window/
    ├── docs/                              # Technical documentation
    │   ├── GUIDE.md
    │   ├── ARCHITECTURE.md
    │   ├── ATOMIC_UI_STANDARD.md
    │   ├── REQUIREMENTS.md
    │   └── TODO.md
    └── index.ts                           # Pure window manager barrel export
```

---

## 2. Core Engine Specifications

### A. Geometry Engine (`core/geometry-engine.ts`)

A collection of pure mathematical functions referencing `WIN2X_DEFAULTS`:

- `clampToViewport(x, y, width, _height, viewportWidth, viewportHeight)`: Keeps the title bar accessible (at least 100px visible horizontally, 0 to `viewportHeight - 50` vertically).
- `computeResize(rect, direction, deltaX, deltaY, minWidth, minHeight)`: Calculates next `(x, y, width, height)` for all 8 resize directions.
- `centerInViewport(width, height, viewportWidth, viewportHeight)`: Calculates initial centered coordinates with safety offsets.
- `constrainMinSize(width, height, minWidth, minHeight)`: Enforces dimensional lower bounds.

### B. Pointer Driver (`core/pointer-driver.ts`)

A zero-dependency W3C Pointer Events capture and dispatch engine:

- `startPointerDrag(targetElement, e, options)`:
  1. Calls `targetElement.setPointerCapture(pointerId)`.
  2. Binds `pointermove`, `pointerup`, `pointercancel` directly to `targetElement`.
  3. Coalesces rapid pointer movements via `requestAnimationFrame`.
  4. Directly updates `containerElement.style.transform = translate3d(x, y, 0)`.
  5. Releases capture on `pointerup` and invokes `onDragEnd(finalX, finalY)`.
- `startPointerResize(targetElement, e, options)`:
  1. Captures pointer on the active resize handle.
  2. Runs RAF-throttled delta calculations.
  3. Directly updates `containerElement.style` (`transform`, `width`, `height`).
  4. Releases capture on `pointerup` and invokes `onResizeEnd(finalRect)`.

### C. Storage Adapter (`core/storage-adapter.ts`)

A pluggable 2-method interface for window geometry persistence:

```typescript
export interface StorageProvider {
  getItem<T>(key: string): T | null;
  setItem<T>(key: string, value: T): void;
  removeItem(key: string): void;
}
```

- `LocalStorageAdapter`: Safely reads/writes JSON to `window.localStorage` with auto-healing fallback on corruption.
- `MemoryAdapter`: In-memory ephemeral storage for isolated test environments or SSR.

---

## 3. Data Flow: Pointer Event to GPU Render

```text
User initiates Drag on TitleBar (pointerdown)
  │
  ├──► e.currentTarget.setPointerCapture(e.pointerId)
  ├──► Set container data-moving="true" (Disables transitions & blur)
  │
  ▼
Pointer moves (pointermove events at 60-240Hz)
  │
  ├──► GeometryEngine computes clamped (nextX, nextY)
  ├──► Pending coords stored in PointerDriver ref
  ├──► requestAnimationFrame schedules next vsync tick
  │
  ▼
Browser VSync Tick (RAF callback)
  │
  ├──► Mutate container.style.transform = `translate3d(${x}px, ${y}px, 0)`
  ├──► GPU Compositor updates position on hardware layer (0ms layout reflow)
  │
  ▼
User releases pointer (pointerup)
  │
  ├──► e.currentTarget.releasePointerCapture(e.pointerId)
  ├──► Remove container data-moving attribute (Restores blur)
  ├──► Single commit to StorageAdapter & React state
```
