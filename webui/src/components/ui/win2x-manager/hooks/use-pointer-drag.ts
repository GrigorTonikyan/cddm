import { useCallback } from "react";
import { startPointerDrag } from "../core/pointer-driver";
import { SnapZone, Win2xRect } from "../core/types";
import { useActiveStateWithCleanup } from "./use-active-cleanup";

export interface UsePointerDragOptions {
  containerRef: React.RefObject<HTMLElement | null>;
  x: number;
  y: number;
  width: number;
  height: number;
  isMaximized: boolean;
  disabled?: boolean;
  enableSnapping?: boolean;
  otherWindows?: Win2xRect[];
  onDragEnd: (finalX: number, finalY: number, snapZone?: SnapZone) => void;
  onDragChange?: (x: number, y: number) => void;
  onSnapZoneChange?: (zone: SnapZone) => void;
}

export interface UsePointerDragResult {
  handlePointerDown: (e: React.PointerEvent<HTMLElement>) => void;
  isDragging: boolean;
}

/**
 * Headless React hook wrapping W3C Pointer Events capture and RAF-throttled motion.
 */
export function usePointerDrag({
  containerRef,
  x,
  y,
  width,
  height,
  isMaximized,
  disabled = false,
  enableSnapping = true,
  otherWindows = [],
  onDragEnd,
  onDragChange,
  onSnapZoneChange,
}: UsePointerDragOptions): UsePointerDragResult {
  const {
    isActive: isDragging,
    setIsActive: setIsDragging,
    cleanupRef,
  } = useActiveStateWithCleanup();

  const handlePointerDown = useCallback(
    (e: React.PointerEvent<HTMLElement>) => {
      if (disabled || isMaximized || e.button !== 0) return;

      const target = e.target as HTMLElement;
      if (target.closest("button") || target.closest("input") || target.closest("a")) {
        return;
      }

      if (!containerRef.current) return;

      setIsDragging(true);

      const cleanup = startPointerDrag(e.currentTarget as HTMLElement, e, {
        containerElement: containerRef.current,
        initialX: x,
        initialY: y,
        width,
        height,
        enableSnapping,
        otherWindows,
        onDragEnd: (finalX, finalY, snapZone) => {
          setIsDragging(false);
          cleanupRef.current = null;
          onDragEnd(finalX, finalY, snapZone);
        },
        onDragChange,
        onSnapZoneChange,
      });

      cleanupRef.current = cleanup;
    },
    [
      containerRef,
      x,
      y,
      width,
      height,
      isMaximized,
      disabled,
      enableSnapping,
      otherWindows,
      onDragEnd,
      onDragChange,
      onSnapZoneChange,
    ],
  );

  return { handlePointerDown, isDragging };
}
