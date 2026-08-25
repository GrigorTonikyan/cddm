import { useCallback, useRef, useState } from "react";
import { WIN2X_Z_INDEX } from "../constants/win2x-constants";
import type { WindowRegistration } from "../core/types";
import { defaultStorage, saveWindowState } from "../core/storage-adapter";

export function useWin2xRegistry() {
  const [windows, setWindows] = useState<Map<string, WindowRegistration>>(new Map());
  const [activeWindowId, setActiveWindowId] = useState<string | null>(null);

  const windowsRef = useRef<Map<string, WindowRegistration>>(windows);
  windowsRef.current = windows;

  const zIndexStackRef = useRef<string[]>([]);

  const recomputeZIndices = useCallback(
    (stack: string[], currentWindows: Map<string, WindowRegistration>) => {
      const nextWindows = new Map(currentWindows);
      stack.forEach((id, index) => {
        const win = nextWindows.get(id);
        if (win) {
          nextWindows.set(id, {
            ...win,
            zIndex: WIN2X_Z_INDEX.BASE_WINDOW + index * WIN2X_Z_INDEX.ACTIVE_STEP,
          });
        }
      });
      return nextWindows;
    },
    [],
  );

  const registerWindow = useCallback(
    (id: string, initialData: Omit<WindowRegistration, "zIndex">) => {
      setWindows((prev) => {
        if (prev.has(id)) return prev;
        const stack = [...zIndexStackRef.current, id];
        zIndexStackRef.current = stack;
        const next = new Map(prev);
        next.set(id, { ...initialData, zIndex: 0 });
        return recomputeZIndices(stack, next);
      });
      setActiveWindowId(id);
    },
    [recomputeZIndices],
  );

  const unregisterWindow = useCallback(
    (id: string) => {
      setWindows((prev) => {
        if (!prev.has(id)) return prev;
        const stack = zIndexStackRef.current.filter((windowId) => windowId !== id);
        zIndexStackRef.current = stack;
        const next = new Map(prev);
        next.delete(id);
        return recomputeZIndices(stack, next);
      });
      setActiveWindowId((prev) => {
        if (prev === id) {
          const stack = zIndexStackRef.current;
          return stack.length > 0 ? (stack[stack.length - 1] ?? null) : null;
        }
        return prev;
      });
    },
    [recomputeZIndices],
  );

  const updateWindow = useCallback((id: string, updates: Partial<WindowRegistration>) => {
    setWindows((prev) => {
      const current = prev.get(id);
      if (!current) return prev;

      let hasChanges = false;
      for (const key of Object.keys(updates) as (keyof WindowRegistration)[]) {
        if (current[key] !== updates[key]) {
          hasChanges = true;
          break;
        }
      }
      if (!hasChanges) return prev;

      const updated = { ...current, ...updates };
      const next = new Map(prev);
      next.set(id, updated);

      saveWindowState(
        defaultStorage,
        id,
        {
          x: updated.rect.x,
          y: updated.rect.y,
          width: updated.rect.width,
          height: updated.rect.height,
          isMaximized: updated.isMaximized,
          isMinimized: updated.isMinimized,
        },
        updated.windowType,
      );

      return next;
    });
  }, []);

  const focusWindow = useCallback(
    (id: string) => {
      if (activeWindowId === id) return;

      setWindows((prev) => {
        if (!prev.has(id)) return prev;

        const stack = zIndexStackRef.current.filter((windowId) => windowId !== id);
        stack.push(id);
        zIndexStackRef.current = stack;

        return recomputeZIndices(stack, prev);
      });
      setActiveWindowId(id);
    },
    [activeWindowId, recomputeZIndices],
  );

  return {
    windows,
    setWindows,
    windowsRef,
    zIndexStackRef,
    activeWindowId,
    setActiveWindowId,
    registerWindow,
    unregisterWindow,
    updateWindow,
    focusWindow,
  };
}
