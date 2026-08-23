import { useCallback, useRef, useEffect } from "react";

export interface UseDraggableOptions {
  containerRef?: React.RefObject<HTMLDivElement | null>;
  x: number;
  y: number;
  width: number;
  height: number;
  isMaximized: boolean;
  disabled?: boolean;
  onDragEnd: (x: number, y: number) => void;
  onDragChange?: (x: number, y: number) => void;
}

export interface UseDraggableResult {
  handleMouseDown: (e: React.MouseEvent) => void;
  isDragging: boolean;
}

/**
 * Headless atomic hook providing hardware-accelerated 60fps/120fps dragging capabilities.
 * Uses requestAnimationFrame and direct DOM transforms during motion to eliminate React render overhead.
 */
export function useDraggable({
  containerRef,
  x,
  y,
  width,
  height,
  isMaximized,
  disabled = false,
  onDragEnd,
  onDragChange,
}: UseDraggableOptions): UseDraggableResult {
  const isDraggingRef = useRef(false);
  const rafId = useRef<number | null>(null);
  const dragStartPos = useRef<{ mouseX: number; mouseY: number; windowX: number; windowY: number }>(
    {
      mouseX: 0,
      mouseY: 0,
      windowX: 0,
      windowY: 0,
    },
  );
  const currentPos = useRef<{ x: number; y: number }>({ x, y });

  useEffect(() => {
    currentPos.current = { x, y };
  }, [x, y]);

  // Clean up any pending animation frame on unmount
  useEffect(() => {
    return () => {
      if (rafId.current !== null) {
        cancelAnimationFrame(rafId.current);
      }
    };
  }, []);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (disabled || isMaximized || e.button !== 0) return;

      // Don't initiate drag if clicked on button or interactive control
      const target = e.target as HTMLElement;
      if (target.closest("button") || target.closest("input") || target.closest("a")) {
        return;
      }

      isDraggingRef.current = true;
      dragStartPos.current = {
        mouseX: e.clientX,
        mouseY: e.clientY,
        windowX: x,
        windowY: y,
      };
      currentPos.current = { x, y };

      if (typeof document !== "undefined") {
        document.body.style.userSelect = "none";
      }

      const handleMouseMove = (moveEvent: MouseEvent) => {
        if (!isDraggingRef.current) return;

        const deltaX = moveEvent.clientX - dragStartPos.current.mouseX;
        const deltaY = moveEvent.clientY - dragStartPos.current.mouseY;

        let nextX = dragStartPos.current.windowX + deltaX;
        let nextY = dragStartPos.current.windowY + deltaY;

        // Viewport clamping (keep at least 100px of title bar visible)
        const minX = -width + 100;
        const maxX = window.innerWidth - 100;
        const minY = 0;
        const maxY = Math.max(0, window.innerHeight - 50);

        nextX = Math.max(minX, Math.min(nextX, maxX));
        nextY = Math.max(minY, Math.min(nextY, maxY));

        currentPos.current = { x: nextX, y: nextY };

        // Direct DOM update via requestAnimationFrame for 60fps/120fps smooth performance
        if (rafId.current === null) {
          rafId.current = requestAnimationFrame(() => {
            if (containerRef?.current) {
              containerRef.current.style.left = `${currentPos.current.x}px`;
              containerRef.current.style.top = `${currentPos.current.y}px`;
              containerRef.current.style.willChange = "left, top";
            }
            onDragChange?.(currentPos.current.x, currentPos.current.y);
            rafId.current = null;
          });
        }
      };

      const handleMouseUp = (upEvent: MouseEvent) => {
        if (!isDraggingRef.current) return;
        isDraggingRef.current = false;

        if (rafId.current !== null) {
          cancelAnimationFrame(rafId.current);
          rafId.current = null;
        }

        if (typeof document !== "undefined") {
          document.body.style.userSelect = "";
        }

        if (containerRef?.current) {
          containerRef.current.style.willChange = "auto";
        }

        const deltaX = upEvent.clientX - dragStartPos.current.mouseX;
        const deltaY = upEvent.clientY - dragStartPos.current.mouseY;

        let nextX = dragStartPos.current.windowX + deltaX;
        let nextY = dragStartPos.current.windowY + deltaY;

        const minX = -width + 100;
        const maxX = window.innerWidth - 100;
        const minY = 0;
        const maxY = Math.max(0, window.innerHeight - 50);

        nextX = Math.max(minX, Math.min(nextX, maxX));
        nextY = Math.max(minY, Math.min(nextY, maxY));

        window.removeEventListener("mousemove", handleMouseMove);
        window.removeEventListener("mouseup", handleMouseUp);
        onDragEnd(nextX, nextY);
      };

      window.addEventListener("mousemove", handleMouseMove, { passive: true });
      window.addEventListener("mouseup", handleMouseUp);
    },
    [containerRef, x, y, width, height, isMaximized, disabled, onDragEnd, onDragChange],
  );

  return { handleMouseDown, isDragging: isDraggingRef.current };
}
