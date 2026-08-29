import { useCallback } from "react";
import { startPointerResize } from "../core/pointer-driver";
import { ResizeDirection, Win2xRect } from "../core/types";
import { useActiveStateWithCleanup } from "./use-active-cleanup";

export interface UsePointerResizeOptions {
  containerRef: React.RefObject<HTMLElement | null>;
  x: number;
  y: number;
  width: number;
  height: number;
  minWidth?: number;
  minHeight?: number;
  isMaximized: boolean;
  disabled?: boolean;
  onResizeEnd: (finalRect: Win2xRect) => void;
  onResizeChange?: (rect: Win2xRect) => void;
}

export interface UsePointerResizeResult {
  handleResizePointerDown: (direction: ResizeDirection, e: React.PointerEvent<HTMLElement>) => void;
  isResizing: boolean;
}

/**
 * Headless React hook wrapping W3C Pointer Events 8-way resize capture.
 */
export function usePointerResize({
  containerRef,
  x,
  y,
  width,
  height,
  minWidth = 460,
  minHeight = 340,
  isMaximized,
  disabled = false,
  onResizeEnd,
  onResizeChange,
}: UsePointerResizeOptions): UsePointerResizeResult {
  const {
    isActive: isResizing,
    setIsActive: setIsResizing,
    cleanupRef,
  } = useActiveStateWithCleanup();

  const handleResizePointerDown = useCallback(
    (direction: ResizeDirection, e: React.PointerEvent<HTMLElement>) => {
      if (disabled || isMaximized || e.button !== 0) return;
      e.preventDefault();
      e.stopPropagation();

      if (!containerRef.current) return;

      setIsResizing(true);

      const cleanup = startPointerResize(e.currentTarget, e, {
        containerElement: containerRef.current,
        initialRect: { x, y, width, height },
        direction,
        minWidth,
        minHeight,
        onResizeEnd: (finalRect) => {
          setIsResizing(false);
          cleanupRef.current = null;
          onResizeEnd(finalRect);
        },
        onResizeChange,
      });

      cleanupRef.current = cleanup;
    },
    [
      containerRef,
      x,
      y,
      width,
      height,
      minWidth,
      minHeight,
      isMaximized,
      disabled,
      onResizeEnd,
      onResizeChange,
    ],
  );

  return { handleResizePointerDown, isResizing };
}
