# `win2x-manager` Roadmap & Development Tasks

## Completed Tasks (Milestones v0.4.0 & v0.5.0)

- [x] Comprehensive architectural design and technical guide (`docs/GUIDE.md`)
- [x] System architecture specification (`docs/ARCHITECTURE.md`)
- [x] Atomic UI component and AI agent enforcement standard (`docs/ATOMIC_UI_STANDARD.md`)
- [x] System requirements specification (`docs/REQUIREMENTS.md`)
- [x] Dedicated AI agent contribution standard (`AGENTS.md`)
- [x] Core geometry engine with viewport clamping and 8-way resize math (`core/geometry-engine.ts`)
- [x] Hardware pointer capture and RAF dispatcher (`core/pointer-driver.ts`)
- [x] Universal pluggable storage adapter interface (`core/storage-adapter.ts`)
- [x] CSS custom properties token system with Dark, Light, and High-Contrast palettes (`styles/win2x-theme.css`)
- [x] Pure CSS Modules atomic primitives (Portal, Backdrop, Badge, IconButton, ResizeHandle, WindowControls)
- [x] Pure CSS Modules molecular components (ResizeHandleGroup, TitleBar, TabBar, CollapsibleCard, CodeBlock, SnapGhostGuide, SnapLayoutsMenu, SnapAssistModal, DockBar)
- [x] Composed Win2x Acrylic Window organism with 120fps compositor pipeline and tab support (`components/win2x-window/`)
- [x] Multi-Window Z-Index Stacking Manager (`context/win2x-manager-context.tsx`)
- [x] Window Snapping Grid, Edge Guides, and Windows 11 Snap Assist (`components/snap-assist/`, `components/snap-ghost-guide/`)
- [x] Tabbed Window Groups (`components/tab-bar/`, `components/win2x-window/`)
- [x] Custom Theme Inversion (`styles/win2x-theme.css`, `context/win2x-manager-context.tsx`)
- [x] Window Cascade & Tile Utility with global keyboard shortcuts (`context/win2x-manager-context.tsx`)
- [x] Comprehensive test suite covering core math, drivers, hooks, and components (41 tests across 9 test suites)

---

## Planned Capabilities & Future Enhancements

### Milestone v0.6.0: Enhanced Window Interactivity & Virtual Canvas

- [ ] **Cross-Tab Window Sync**: Synchronize active window state across browser tabs via `BroadcastChannel`.
- [ ] **Window Shake to Minimize**: Aero Shake interaction (shaking the active window title bar minimizes all other windows).
- [ ] **Floating Mini-Window Picture-in-Picture Mode**: Collapsible compact HUD mode for background scan monitoring.
- [ ] **Smooth Window Snapping Animation**: Spring-based transition when snapping into predefined layout zones.
- [ ] **Virtualized Infinite Window Workspace**: Panning and zoomable 2D canvas for large numbers of concurrent windows.
