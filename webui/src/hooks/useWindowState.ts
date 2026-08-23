import { useCallback } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { ModalWindowState } from "../types/cddm-types";

export interface UseWindowStateOptions {
  initialWidth?: number;
  initialHeight?: number;
}

export interface UseWindowStateResult {
  windowState: ModalWindowState;
  updatePosition: (x: number, y: number) => void;
  updateSize: (rect: { x: number; y: number; width: number; height: number }) => void;
  toggleMaximize: () => void;
  toggleMinimize: () => void;
  resetState: () => void;
}

/**
 * Headless atomic hook managing persistent window bounds, maximize, and minimize states.
 */
export function useWindowState({
  initialWidth = 920,
  initialHeight = 680,
}: UseWindowStateOptions = {}): UseWindowStateResult {
  const windowState = useCDDMStore((s) => s.modalWindowState);
  const setPersistedState = useCDDMStore((s) => s.setModalWindowState);

  const effectiveState: ModalWindowState = {
    x:
      windowState.x >= 0
        ? windowState.x
        : typeof window !== "undefined"
          ? Math.max(20, (window.innerWidth - (windowState.width || initialWidth)) / 2)
          : 50,
    y:
      windowState.y >= 0
        ? windowState.y
        : typeof window !== "undefined"
          ? Math.max(30, (window.innerHeight - (windowState.height || initialHeight)) / 2)
          : 50,
    width: windowState.width || initialWidth,
    height: windowState.height || initialHeight,
    isMaximized: windowState.isMaximized || false,
    isMinimized: windowState.isMinimized || false,
  };

  const updatePosition = useCallback(
    (x: number, y: number) => {
      setPersistedState({ x, y });
    },
    [setPersistedState],
  );

  const updateSize = useCallback(
    (rect: { x: number; y: number; width: number; height: number }) => {
      setPersistedState(rect);
    },
    [setPersistedState],
  );

  const toggleMaximize = useCallback(() => {
    setPersistedState({
      isMaximized: !windowState.isMaximized,
      isMinimized: false,
    });
  }, [windowState.isMaximized, setPersistedState]);

  const toggleMinimize = useCallback(() => {
    setPersistedState({
      isMinimized: !windowState.isMinimized,
    });
  }, [windowState.isMinimized, setPersistedState]);

  const resetState = useCallback(() => {
    const cx =
      typeof window !== "undefined" ? Math.max(20, (window.innerWidth - initialWidth) / 2) : 50;
    const cy =
      typeof window !== "undefined" ? Math.max(30, (window.innerHeight - initialHeight) / 2) : 50;
    setPersistedState({
      x: cx,
      y: cy,
      width: initialWidth,
      height: initialHeight,
      isMaximized: false,
      isMinimized: false,
    });
  }, [initialWidth, initialHeight, setPersistedState]);

  return {
    windowState: effectiveState,
    updatePosition,
    updateSize,
    toggleMaximize,
    toggleMinimize,
    resetState,
  };
}
