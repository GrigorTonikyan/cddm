# `win2x-manager` Technical Guide & Architectural Reference

## Overview

`win2x-manager` is a standalone, ultra-high-performance, universal window management system designed for modern web applications. It replicates native operating system window ergonomics (draggable, 8-way resizable, minimizable to floating dock pill, full-screen maximize/restore, body scroll locking, and persistent bounds) while maintaining a strict **120fps hardware-accelerated rendering pipeline**.

---

## 1. Browser Rendering Pipeline Deep-Dive

```text
+-----------------------------------------------------------------------------------------------+
|                                  BROWSER FRAME PIPELINE                                       |
|  [JavaScript / Pointer] ---> [Style Calc] ---> [Layout / Reflow] ---> [Paint] ---> [Composite]|
|          |                           |                 |                 |              |     |
|    Pointer Events             CSS Properties     Box Geometries      Rasterize      GPU Layers  |
|  (setPointerCapture)         (Scoped Custom)       (containment)    (Decouple Blur) (translate3d)|
+-----------------------------------------------------------------------------------------------+
```

### The 16.6ms (60fps) vs 8.3ms (120fps) Frame Budget

At 120Hz refresh rates, a complete frame must process input, style calculation, layout, paint, and GPU compositing within **8.33 milliseconds**.

1. **Layout / Reflow**: Changing `top`, `left`, `width`, or `height` invalidates the document layout tree.
2. **Paint / Rasterization**: Changing colors, shadows, or background filters forces the CPU/GPU to rasterize bitmap pixels.
3. **Composite-Only Operations**: Changing `transform` (`translate3d`) or `opacity` allows the browser to bypass Layout and Paint entirely, manipulating pre-rasterized GPU textures directly on the compositor thread.

---

## 2. The Five Optimization Avenues (A, B, C, D, E)

### Avenue A: Hardware-Accelerated Composite-Only Transform Pipeline (`translate3d`)

- **Problem**: Traditional window positioning alters `style.left` and `style.top`. This triggers synchronous Layout Reflow and Paint passes on every mouse movement.
- **Solution**: The window is positioned at `top: 0; left: 0; position: fixed;` in CSS. Spatial movement is applied purely via `transform: translate3d(${x}px, ${y}px, 0)`.
- **Impact**: 0ms layout recalculation during dragging; movement runs on the GPU compositor.

### Avenue B: Configurable Motion State & Blur Decoupling

- **Problem**: `backdrop-filter: blur(...)` requires continuous real-time multi-pass Gaussian shader convolution over the pixels beneath the window. When an acrylic window moves over an acrylic backdrop, nested shader passes saturate GPU fill-rate bandwidth.
- **Solution**: During active drag/resize (`isDragging || isResizing`), the window dynamically sets `data-moving="true"`. In CSS, `backdrop-filter` is replaced with high-opacity solid acrylic (`var(--win2x-bg-acrylic-moving)`), and all CSS transition timers are bypassed (`transition: none !important`).
- **Configurability**: Users can customize this behavior via the `performanceProfile` prop:
  - `"extreme"`: Disables blur on both overlay and window during motion. Disables all transition easings.
  - `"balanced"` (default): Disables blur only on the moving window; background overlay retains soft blur.
  - `"quality"`: Retains real-time backdrop blur during motion for high-end workstations.

### Avenue C: Native `setPointerCapture` Engine

- **Problem**: Global `window.addEventListener('mousemove')` suffers from event bubbling latency, loses tracking if the cursor exits the viewport or hovers over iframes, and pollutes global window listeners.
- **Solution**: When a pointer initiates a drag or resize, the handle element calls `e.currentTarget.setPointerCapture(e.pointerId)`. All subsequent hardware pointer events (`pointermove`, `pointerup`, `pointercancel`) are redirected directly to that element by the OS.
- **Impact**: Zero dropped drag events, native multi-touch & stylus support, and zero global event noise.

### Avenue D: CSS Containment Sandbox (`contain: layout paint`)

- **Problem**: DOM mutations inside a modal window cause layout recalculation across the host document.
- **Solution**: The window container declares `contain: layout paint;`.
- **Impact**: The browser treats the window internal subtree as an isolated layout sandbox, preventing internal reflows from leaking into the host page.

### Avenue E: Universal Self-Contained Architecture (Zero-Dependency & CSS Modules)

- **Problem**: UI components tightly coupled to utility frameworks (Tailwind) or global stores (Zustand/Redux) cannot be extracted or reused in other projects.
- **Solution**:
  - 100% pure modern **CSS Modules (`*.module.css`)** with modern CSS nesting and parameterized **CSS Custom Properties (`--win2x-*`)**.
  - Framework-agnostic pure TypeScript math engine (`geometry-engine.ts`).
  - Pluggable storage abstraction (`storage-adapter.ts`) supporting LocalStorage, Memory, or custom store bridges.

---

## 3. Quick Start & Integration Guide

```tsx
import React, { useState } from "react";
import { Win2xWindow } from "./components/ui/win2x-manager";
import { CollapsibleCard, CodeBlock } from "./components/ui";
import { Sparkles } from "lucide-react";

export const ExampleModal = () => {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <Win2xWindow
      isOpen={isOpen}
      onClose={() => setIsOpen(false)}
      title="Code Refactoring Assistant"
      subtitle="Synthesized AST optimization"
      icon={<Sparkles className="w-4 h-4 text-indigo-400" />}
      initialWidth={920}
      initialHeight={680}
      performanceProfile="balanced"
    >
      <CollapsibleCard title="Suggested Abstraction" defaultOpen={true}>
        <CodeBlock
          filename="dedup_helper.rs"
          lineRange="L45-L89"
          code="pub fn extracted_shared_logic() { ... }"
          variant="added"
        />
      </CollapsibleCard>
    </Win2xWindow>
  );
};
```
