import { useCallback, useRef, useEffect } from "react";

export type ResizeDirection =
  | "top"
  | "bottom"
  | "left"
  | "right"
  | "top-left"
  | "top-right"
  | "bottom-left"
  | "bottom-right";

export interface UseResizableOptions {
  containerRef?: React.RefObject<HTMLDivElement | null>;
  x: number;
  y: number;
  width: number;
  height: number;
  minWidth?: number;
  minHeight?: number;
  isMaximized: boolean;
  disabled?: boolean;
  onResizeEnd: (rect: { x: number; y: number; width: number; height: number }) => void;
  onResizeChange?: (rect: { x: number; y: number; width: number; height: number }) => void;
}

export interface UseResizableResult {
  handleResizeMouseDown: (direction: ResizeDirection, e: React.MouseEvent) => void;
  isResizing: boolean;
}

/**
 * Headless atomic hook providing hardware-accelerated 8-direction window resizing capabilities.
 * Uses requestAnimationFrame and direct DOM updates during motion to eliminate React render lag.
 */
export function useResizable({
  containerRef,
  x,
  y,
  width,
  height,
  minWidth = 440,
  minHeight = 320,
  isMaximized,
  disabled = false,
  onResizeEnd,
  onResizeChange,
}: UseResizableOptions): UseResizableResult {
  const isResizingRef = useRef(false);
  const rafId = useRef<number | null>(null);
  const resizeState = useRef<{
    direction: ResizeDirection;
    mouseX: number;
    mouseY: number;
    rect: { x: number; y: number; width: number; height: number };
  }>({
    direction: "bottom-right",
    mouseX: 0,
    mouseY: 0,
    rect: { x, y, width, height },
  });
  const currentRect = useRef<{ x: number; y: number; width: number; height: number }>({
    x,
    y,
    width,
    height,
  });

  useEffect(() => {
    currentRect.current = { x, y, width, height };
  }, [x, y, width, height]);

  // Clean up any pending animation frame on unmount
  useEffect(() => {
    return () => {
      if (rafId.current !== null) {
        cancelAnimationFrame(rafId.current);
      }
    };
  }, []);

  const handleResizeMouseDown = useCallback(
    (direction: ResizeDirection, e: React.MouseEvent) => {
      if (disabled || isMaximized || e.button !== 0) return;
      e.preventDefault();
      e.stopPropagation();

      isResizingRef.current = true;
      resizeState.current = {
        direction,
        mouseX: e.clientX,
        mouseY: e.clientY,
        rect: { x, y, width, height },
      };
      currentRect.current = { x, y, width, height };

      if (typeof document !== "undefined") {
        document.body.style.userSelect = "none";
      }

      const handleMouseMove = (moveEvent: MouseEvent) => {
        if (!isResizingRef.current) return;

        const deltaX = moveEvent.clientX - resizeState.current.mouseX;
        const deltaY = moveEvent.clientY - resizeState.current.mouseY;
        const { rect, direction: dir } = resizeState.current;

        let nextX = rect.x;
        let nextY = rect.y;
        let nextW = rect.width;
        let nextH = rect.height;

        if (dir.includes("right")) {
          nextW = Math.max(minWidth, rect.width + deltaX);
        }
        if (dir.includes("bottom")) {
          nextH = Math.max(minHeight, rect.height + deltaY);
        }
        if (dir.includes("left")) {
          const potentialW = rect.width - deltaX;
          if (potentialW >= minWidth) {
            nextW = potentialW;
            nextX = rect.x + deltaX;
          } else {
            nextW = minWidth;
            nextX = rect.x + (rect.width - minWidth);
          }
        }
        if (dir.includes("top")) {
          const potentialH = rect.height - deltaY;
          if (potentialH >= minHeight) {
            nextH = potentialH;
            nextY = Math.max(0, rect.y + deltaY);
          } else {
            nextH = minHeight;
            nextY = rect.y + (rect.height - minHeight);
          }
        }

        currentRect.current = { x: nextX, y: nextY, width: nextW, height: nextH };

        // Direct DOM update via requestAnimationFrame for 60fps/120fps smooth performance
        if (rafId.current === null) {
          rafId.current = requestAnimationFrame(() => {
            if (containerRef?.current) {
              containerRef.current.style.left = `${currentRect.current.x}px`;
              containerRef.current.style.top = `${currentRect.current.y}px`;
              containerRef.current.style.width = `${currentRect.current.width}px`;
              containerRef.current.style.height = `${currentRect.current.height}px`;
              containerRef.current.style.willChange = "left, top, width, height";
            }
            onResizeChange?.(currentRect.current);
            rafId.current = null;
          });
        }
      };

      const handleMouseUp = (upEvent: MouseEvent) => {
        if (!isResizingRef.current) return;
        isResizingRef.current = false;

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

        const deltaX = upEvent.clientX - resizeState.current.mouseX;
        const deltaY = upEvent.clientY - resizeState.current.mouseY;
        const { rect, direction: dir } = resizeState.current;

        let nextX = rect.x;
        let nextY = rect.y;
        let nextW = rect.width;
        let nextH = rect.height;

        if (dir.includes("right")) {
          nextW = Math.max(minWidth, rect.width + deltaX);
        }
        if (dir.includes("bottom")) {
          nextH = Math.max(minHeight, rect.height + deltaY);
        }
        if (dir.includes("left")) {
          const potentialW = rect.width - deltaX;
          if (potentialW >= minWidth) {
            nextW = potentialW;
            nextX = rect.x + deltaX;
          } else {
            nextW = minWidth;
            nextX = rect.x + (rect.width - minWidth);
          }
        }
        if (dir.includes("top")) {
          const potentialH = rect.height - deltaY;
          if (potentialH >= minHeight) {
            nextH = potentialH;
            nextY = Math.max(0, rect.y + deltaY);
          } else {
            nextH = minHeight;
            nextY = rect.y + (rect.height - minHeight);
          }
        }

        window.removeEventListener("mousemove", handleMouseMove);
        window.removeEventListener("mouseup", handleMouseUp);
        onResizeEnd({ x: nextX, y: nextY, width: nextW, height: nextH });
      };

      window.addEventListener("mousemove", handleMouseMove, { passive: true });
      window.addEventListener("mouseup", handleMouseUp);
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

  return { handleResizeMouseDown, isResizing: isResizingRef.current };
}
