/**
 * Hardware Pointer Capture & RAF-throttled motion driver for win2x-manager.
 * Zero React dependency.
 */

import {
  WIN2X_DATA_ATTRS,
  WIN2X_DEFAULTS,
  WIN2X_SNAP_ZONES,
  WIN2X_TIMINGS,
} from "../constants/win2x-constants";
import {
  clampToViewport,
  computeResize,
  detectSnapZone,
  detectWindowToWindowSnap,
} from "./geometry-engine";
import { ResizeDirection, SnapZone, Win2xRect } from "./types";

export interface DragSessionOptions {
  containerElement: HTMLElement;
  initialX: number;
  initialY: number;
  width: number;
  height: number;
  enableSnapping?: boolean;
  otherWindows?: Win2xRect[];
  onDragEnd: (finalX: number, finalY: number, snapZone?: SnapZone) => void;
  onDragChange?: (x: number, y: number) => void;
  onSnapZoneChange?: (zone: SnapZone, rect: Win2xRect | null) => void;
}

export interface ResizeSessionOptions {
  containerElement: HTMLElement;
  initialRect: Win2xRect;
  direction: ResizeDirection;
  minWidth?: number;
  minHeight?: number;
  onResizeEnd: (finalRect: Win2xRect) => void;
  onResizeChange?: (rect: Win2xRect) => void;
}

function setupPointerMotion(
  captureElement: HTMLElement,
  containerElement: HTMLElement,
  pointerId: number,
  willChange: string,
): void {
  if (typeof captureElement.setPointerCapture === "function") {
    try {
      captureElement.setPointerCapture(pointerId);
    } catch {
      // Safe fallback in unsupported environments
    }
  }
  containerElement.setAttribute(WIN2X_DATA_ATTRS.MOVING, "true");
  containerElement.style.willChange = willChange;
  if (typeof document !== "undefined") {
    document.body.style.userSelect = "none";
  }
}

function teardownPointerCapture(
  containerElement: HTMLElement,
  captureElement: HTMLElement,
  pointerId: number,
  handlePointerMove: (e: PointerEvent) => void,
  handlePointerUp: (e: PointerEvent) => void,
): void {
  containerElement.removeAttribute(WIN2X_DATA_ATTRS.MOVING);
  containerElement.style.willChange = "auto";
  if (typeof document !== "undefined") {
    document.body.style.userSelect = "";
  }

  if (typeof captureElement.releasePointerCapture === "function") {
    try {
      captureElement.releasePointerCapture(pointerId);
    } catch {
      // Safe fallback
    }
  }

  captureElement.removeEventListener("pointermove", handlePointerMove);
  captureElement.removeEventListener("pointerup", handlePointerUp);
  captureElement.removeEventListener("pointercancel", handlePointerUp);
}

/**
 * Initiates a hardware-captured, RAF-throttled pointer drag session.
 */
export function startPointerDrag(
  captureElement: HTMLElement,
  e: PointerEvent | React.PointerEvent,
  options: DragSessionOptions,
): () => void {
  const {
    containerElement,
    initialX,
    initialY,
    width,
    height,
    enableSnapping = true,
    otherWindows = [],
    onDragEnd,
    onDragChange,
    onSnapZoneChange,
  } = options;

  const startMouseX = e.clientX;
  const startMouseY = e.clientY;
  const pointerId = e.pointerId;

  let currentX = initialX;
  let currentY = initialY;
  let currentSnapZone: SnapZone = WIN2X_SNAP_ZONES.NONE;
  let pendingSnapZone: SnapZone = WIN2X_SNAP_ZONES.NONE;
  let snapTimer: ReturnType<typeof setTimeout> | null = null;
  let rafId: number | null = null;
  let isMoving = true;

  setupPointerMotion(captureElement, containerElement, pointerId, "transform");

  const handlePointerMove = (moveEvent: PointerEvent) => {
    if (!isMoving) return;

    const deltaX = moveEvent.clientX - startMouseX;
    const deltaY = moveEvent.clientY - startMouseY;

    const rawX = initialX + deltaX;
    const rawY = initialY + deltaY;

    const viewportW =
      typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
    const viewportH =
      typeof window !== "undefined" ? window.innerHeight : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

    const clamped = clampToViewport(rawX, rawY, width, height, viewportW, viewportH);
    let finalX = clamped.x;
    let finalY = clamped.y;

    // Window-to-window magnetic snapping
    if (otherWindows.length > 0) {
      const magnet = detectWindowToWindowSnap(
        { x: finalX, y: finalY, width, height },
        otherWindows,
      );
      finalX = magnet.x;
      finalY = magnet.y;
    }

    currentX = finalX;
    currentY = finalY;

    if (enableSnapping) {
      const newZone = detectSnapZone(moveEvent.clientX, moveEvent.clientY, viewportW, viewportH);
      if (newZone !== pendingSnapZone) {
        pendingSnapZone = newZone;
        if (snapTimer) {
          clearTimeout(snapTimer);
          snapTimer = null;
        }

        if (newZone === WIN2X_SNAP_ZONES.NONE) {
          currentSnapZone = WIN2X_SNAP_ZONES.NONE;
          onSnapZoneChange?.(WIN2X_SNAP_ZONES.NONE, null);
        } else {
          // Delay hint animation by WIN2X_TIMINGS.SNAP_HINT_DELAY_MS
          snapTimer = setTimeout(() => {
            currentSnapZone = newZone;
            onSnapZoneChange?.(newZone, null);
          }, WIN2X_TIMINGS.SNAP_HINT_DELAY_MS);
        }
      }
    }

    if (rafId === null) {
      rafId = requestAnimationFrame(() => {
        containerElement.style.transform = `translate3d(${currentX}px, ${currentY}px, 0)`;
        onDragChange?.(currentX, currentY);
        rafId = null;
      });
    }
  };

  const handlePointerUp = () => {
    cleanUp();
    onDragEnd(currentX, currentY, currentSnapZone);
  };

  const cleanUp = () => {
    isMoving = false;
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }
    if (snapTimer) {
      clearTimeout(snapTimer);
      snapTimer = null;
    }

    teardownPointerCapture(
      containerElement,
      captureElement,
      pointerId,
      handlePointerMove,
      handlePointerUp,
    );
  };

  captureElement.addEventListener("pointermove", handlePointerMove, {
    passive: true,
  });
  captureElement.addEventListener("pointerup", handlePointerUp);
  captureElement.addEventListener("pointercancel", handlePointerUp);

  return cleanUp;
}

/**
 * Initiates a hardware-captured, RAF-throttled pointer resize session.
 */
export function startPointerResize(
  captureElement: HTMLElement,
  e: PointerEvent | React.PointerEvent,
  options: ResizeSessionOptions,
): () => void {
  const {
    containerElement,
    initialRect,
    direction,
    minWidth = WIN2X_DEFAULTS.MIN_WIDTH,
    minHeight = WIN2X_DEFAULTS.MIN_HEIGHT,
    onResizeEnd,
    onResizeChange,
  } = options;

  const startMouseX = e.clientX;
  const startMouseY = e.clientY;
  const pointerId = e.pointerId;

  let currentRect: Win2xRect = { ...initialRect };
  let rafId: number | null = null;
  let isResizing = true;

  setupPointerMotion(captureElement, containerElement, pointerId, "transform, width, height");

  const handlePointerMove = (moveEvent: PointerEvent) => {
    if (!isResizing) return;

    const deltaX = moveEvent.clientX - startMouseX;
    const deltaY = moveEvent.clientY - startMouseY;

    currentRect = computeResize(initialRect, direction, deltaX, deltaY, minWidth, minHeight);

    if (rafId === null) {
      rafId = requestAnimationFrame(() => {
        containerElement.style.transform = `translate3d(${currentRect.x}px, ${currentRect.y}px, 0)`;
        containerElement.style.width = `${currentRect.width}px`;
        containerElement.style.height = `${currentRect.height}px`;
        onResizeChange?.(currentRect);
        rafId = null;
      });
    }
  };

  const cleanUp = () => {
    isResizing = false;
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }

    teardownPointerCapture(
      containerElement,
      captureElement,
      pointerId,
      handlePointerMove,
      handlePointerUp,
    );
  };

  const handlePointerUp = () => {
    cleanUp();
    onResizeEnd(currentRect);
  };

  captureElement.addEventListener("pointermove", handlePointerMove, {
    passive: true,
  });
  captureElement.addEventListener("pointerup", handlePointerUp);
  captureElement.addEventListener("pointercancel", handlePointerUp);

  return cleanUp;
}
