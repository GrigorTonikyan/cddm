# `win2x-manager` System Requirements Specification

## 1. Functional Requirements

### 1.1 Window Lifecycle & Controls

- **FR-1.1 (Open/Close)**: The window must smoothly mount into the DOM via a React Portal to `document.body` and unmount on close.
- **FR-1.2 (Keyboard Dismissal)**: Pressing the `Escape` key must dismiss the topmost active window.
- **FR-1.3 (Minimize to Dock Pill & Outside-Click Minimization)**: Clicking the Minimize button (`-`) or clicking outside the window (backdrop overlay) transitions the window into a compact floating acrylic dock pill in the bottom-right viewport corner. Minimized windows stack cleanly in a dedicated dock container. Clicking or pressing `Enter`/`Space` on the pill restores the window to its previous coordinates. An optional close button on the pill allows instant dismissal.
- **FR-1.4 (Maximize / Restore Toggle)**: Clicking the Maximize button ([Maximize]/[Restore]) or double-clicking the title bar must toggle full-screen viewport mode (`100vw x 100vh`, `0, 0`). Toggling again must restore exact previous dimensions and coordinates.
- **FR-1.5 (Background Scroll Lock)**: When the window is open and not minimized, scrolling on the underlying page body must be locked (`document.body.style.overflow = "hidden"`). When closed or minimized, body scroll must be restored with reference counting.

### 1.2 Motion & Manipulation

- **FR-2.1 (Draggability)**: The window must be draggable by its title bar across all 2D screen coordinates with viewport clamping (retaining at least 100px of the title bar visible).
- **FR-2.2 (8-Way Resizability)**: The window must be resizable from all 4 edges (`top`, `bottom`, `left`, `right`) and all 4 corners (`top-left`, `top-right`, `bottom-left`, `bottom-right`).
- **FR-2.3 (Minimum Dimension Bounds)**: Resizing must strictly enforce configurable minimum bounds (`minWidth`, `minHeight`, default 460px x 340px).

### 1.3 State Persistence

- **FR-3.1 (Bounds & State Persistence)**: Window coordinates `(x, y)`, dimensions `(width, height)`, and maximized state must automatically persist across page reloads via pluggable storage adapters (`LocalStorageAdapter`, `MemoryAdapter`).

### 1.4 Composability & Atomic Modules

- **FR-4.1 (Collapsible Cards)**: Provide universal molecular collapsible cards with header badges, action slots, and animated chevron indicators supporting independent expand/collapse states.
- **FR-4.2 (Zero-Wrapping Code Panels)**: Provide monospace code blocks strictly enforcing `whitespace: pre` and horizontal scrolling without text wrapping.

---

## 2. Non-Functional Requirements

### 2.1 Performance & Frame Rate

- **NFR-1.1 (120fps Rendering)**: Window dragging must operate on the GPU compositor thread using `transform: translate3d(...)` via `requestAnimationFrame` with zero layout reflows during active motion.
- **NFR-1.2 (Motion Blur Decoupling)**: During active drag or resize, CSS `backdrop-filter` must be dynamically decoupled (swapping to solid high-opacity acrylic) to prevent shader fill-rate saturation, fully configurable via `performanceProfile`.
- **NFR-1.3 (Input Latency)**: Must utilize the W3C Pointer Events API with `setPointerCapture` to eliminate event queue delay.

### 2.2 Styling & Portability

- **NFR-2.1 (Zero Tailwind Dependency)**: All styling inside `win2x-manager` must use pure CSS Modules (`*.module.css`) with scoped CSS custom properties (`--win2x-*`).
- **NFR-2.2 (Self-Contained & Universal)**: The module must have zero dependencies on application-specific stores or frameworks and be drop-in portable to any React project.

### 2.3 Quality & Engineering Standards

- **NFR-3.1 (Strict TypeScript)**: 100% strict type safety with `strict: true`, `noImplicitAny: true`, and zero compiler warnings.
- **NFR-3.2 (Zero Emoji Policy)**: Zero emojis or pictographs anywhere in code, styles, tests, or documentation.
- **NFR-3.3 (Comprehensive Test Coverage)**: 100% test coverage across unit, integration, and component test suites.
