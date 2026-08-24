import React, { createContext, useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  ResizeDirection,
  SnapLayoutPreset,
  Win2xTheme,
  WIN2X_DATA_ATTRS,
  WIN2X_DEFAULTS,
  WIN2X_KEYS,
  WIN2X_LAYOUT_MODES,
  WIN2X_SNAP_ZONES,
  WIN2X_THEMES,
  WIN2X_Z_INDEX,
} from "../constants/win2x-constants";
import {
  SnapAssistSession,
  WindowRegistration,
  Win2xManagerContextValue,
  WindowLayoutMode,
} from "../core/types";
import {
  calculateCascadePositions,
  calculateTileGridPositions,
  calculateTileSplitPositions,
  computeSnapLayoutSlotRect,
  expandHandleToEdges,
  getSnapLayoutDefinitions,
} from "../core/geometry-engine";
import { DockBar } from "../components/dock-bar/dock-bar";
import { SnapAssistModal } from "../components/snap-assist/snap-assist-modal";
import { defaultStorage, saveWindowState } from "../core/storage-adapter";

export const Win2xManagerContext = createContext<Win2xManagerContextValue | null>(null);

export interface Win2xManagerProviderProps {
  children: React.ReactNode;
  enableSnapLayouts?: boolean;
  enableKeyboardShortcuts?: boolean;
  initialTheme?: Win2xTheme;
}

export const Win2xManagerProvider: React.FC<Win2xManagerProviderProps> = ({
  children,
  enableSnapLayouts = true,
  enableKeyboardShortcuts = true,
  initialTheme = WIN2X_THEMES.DARK,
}) => {
  const [theme, setTheme] = useState<Win2xTheme>(initialTheme);
  const [windows, setWindows] = useState<Map<string, WindowRegistration>>(new Map());
  const [activeWindowId, setActiveWindowId] = useState<string | null>(null);
  const [snapAssistSession, setSnapAssistSession] = useState<SnapAssistSession | null>(null);

  // Synchronous ref to prevent stale closures and unnecessary re-renders
  const windowsRef = useRef<Map<string, WindowRegistration>>(windows);
  windowsRef.current = windows;

  // Keep track of the z-index stack ordered from bottom to top
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

      // Persist on 2 levels
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

  const expandWindowInDirection = useCallback(
    (id: string, direction: ResizeDirection) => {
      const current = windowsRef.current.get(id);
      if (!current || current.isMaximized || current.isMinimized) return;

      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

      const otherWindows = Array.from(windowsRef.current.values())
        .filter((w) => w.id !== id && !w.isMinimized)
        .map((w) => w.rect);

      const expandedRect = expandHandleToEdges(
        current.rect,
        direction,
        viewportW,
        viewportH,
        otherWindows,
      );

      updateWindow(id, {
        rect: expandedRect,
        snappedZone: WIN2X_SNAP_ZONES.NONE,
        preSnapRect: current.rect,
      });
      focusWindow(id);
    },
    [updateWindow, focusWindow],
  );

  const applySnapPreset = useCallback(
    (id: string, preset: SnapLayoutPreset, slotIndex: number) => {
      const current = windowsRef.current.get(id);
      if (!current) return;

      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

      const targetRect = computeSnapLayoutSlotRect(preset, slotIndex, viewportW, viewportH);
      if (!targetRect) return;

      updateWindow(id, {
        rect: targetRect,
        isMaximized: false,
        snappedZone: null,
        snappedPreset: { preset, slotIndex },
        preSnapRect: current.preSnapRect || current.rect,
      });
      focusWindow(id);

      // Check if there are remaining open unminimized windows for Snap Assist
      const def = getSnapLayoutDefinitions().find((d) => d.preset === preset);
      if (def && def.slots.length > 1) {
        const nextSlot = def.slots.find((s) => s.index !== slotIndex);
        const remainingWindows = Array.from(windowsRef.current.values()).filter(
          (w) => w.id !== id && !w.isMinimized,
        );

        if (nextSlot && remainingWindows.length > 0) {
          const filled = new Map<number, string>();
          filled.set(slotIndex, id);
          setSnapAssistSession({
            preset,
            activeSlotIndex: nextSlot.index,
            filledSlots: filled,
            sourceWindowId: id,
          });
        }
      }
    },
    [updateWindow, focusWindow],
  );

  const dismissSnapAssist = useCallback(() => {
    setSnapAssistSession(null);
  }, []);

  const assignWindowToSnapAssistSlot = useCallback(
    (windowId: string, slotIndex: number) => {
      if (!snapAssistSession) return;

      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

      const targetRect = computeSnapLayoutSlotRect(
        snapAssistSession.preset,
        slotIndex,
        viewportW,
        viewportH,
      );
      if (targetRect) {
        const current = windowsRef.current.get(windowId);
        updateWindow(windowId, {
          rect: targetRect,
          isMaximized: false,
          snappedPreset: { preset: snapAssistSession.preset, slotIndex },
          preSnapRect: current?.preSnapRect || current?.rect,
        });
      }

      // Check if there are more slots in this preset
      const def = getSnapLayoutDefinitions().find((d) => d.preset === snapAssistSession.preset);
      const nextFilled = new Map(snapAssistSession.filledSlots);
      nextFilled.set(slotIndex, windowId);

      const nextUnfilledSlot = def?.slots.find((s) => !nextFilled.has(s.index));
      const remainingCandidates = Array.from(windowsRef.current.values()).filter(
        (w) => !Array.from(nextFilled.values()).includes(w.id) && !w.isMinimized,
      );

      if (nextUnfilledSlot && remainingCandidates.length > 0) {
        setSnapAssistSession({
          ...snapAssistSession,
          activeSlotIndex: nextUnfilledSlot.index,
          filledSlots: nextFilled,
        });
      } else {
        setSnapAssistSession(null);
      }
    },
    [snapAssistSession, updateWindow],
  );

  const cascadeWindows = useCallback(() => {
    setWindows((prev) => {
      const activeIds = zIndexStackRef.current.filter(
        (id) => !prev.get(id)?.isMinimized && !prev.get(id)?.isMaximized,
      );
      if (activeIds.length === 0) return prev;

      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

      const newPositions = calculateCascadePositions(
        activeIds,
        viewportW,
        viewportH,
        WIN2X_DEFAULTS.CASCADE_STEP,
      );
      const next = new Map(prev);

      activeIds.forEach((id) => {
        const win = next.get(id)!;
        const pos = newPositions.get(id)!;
        next.set(id, {
          ...win,
          rect: { ...win.rect, x: pos.x, y: pos.y },
          preSnapRect: null,
          snappedZone: null,
          snappedPreset: null,
        });
      });
      return next;
    });
  }, []);

  const tileWindows = useCallback((mode: WindowLayoutMode) => {
    setWindows((prev) => {
      const activeIds = zIndexStackRef.current.filter(
        (id) => !prev.get(id)?.isMinimized && !prev.get(id)?.isMaximized,
      );
      if (activeIds.length === 0) return prev;

      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

      let newRects = new Map<string, { x: number; y: number; width: number; height: number }>();
      if (mode === "tile-grid") {
        newRects = calculateTileGridPositions(activeIds, viewportW, viewportH);
      } else if (mode === "tile-horizontal" || mode === "tile-vertical") {
        newRects = calculateTileSplitPositions(
          activeIds,
          viewportW,
          viewportH,
          mode === "tile-horizontal" ? "horizontal" : "vertical",
        );
      }

      if (newRects.size === 0) return prev;

      const next = new Map(prev);
      activeIds.forEach((id) => {
        const win = next.get(id)!;
        const rect = newRects.get(id);
        if (rect) {
          next.set(id, { ...win, rect, preSnapRect: null, snappedZone: null, snappedPreset: null });
        }
      });
      return next;
    });
  }, []);

  const minimizeAllWindows = useCallback(() => {
    setWindows((prev) => {
      const next = new Map(prev);
      prev.forEach((win, id) => {
        if (!win.isMinimized) {
          next.set(id, { ...win, isMinimized: true });
        }
      });
      return next;
    });
    setActiveWindowId(null);
  }, []);

  const restoreAllWindows = useCallback(() => {
    setWindows((prev) => {
      const next = new Map(prev);
      prev.forEach((win, id) => {
        if (win.isMinimized) {
          next.set(id, { ...win, isMinimized: false });
        }
      });
      return next;
    });
    if (zIndexStackRef.current.length > 0) {
      setActiveWindowId(zIndexStackRef.current[zIndexStackRef.current.length - 1] ?? null);
    }
  }, []);

  const closeWindow = useCallback(
    (id: string) => {
      const win = windowsRef.current.get(id);
      if (win?.onClose) {
        win.onClose();
      } else {
        unregisterWindow(id);
      }
    },
    [unregisterWindow],
  );

  // Global keyboard shortcuts for cascade, tile, minimize all, restore all
  useEffect(() => {
    if (!enableKeyboardShortcuts || typeof window === "undefined") return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.altKey && e.shiftKey) {
        const key = e.key.toLowerCase();
        if (key === WIN2X_KEYS.KEY_C) {
          e.preventDefault();
          cascadeWindows();
        } else if (key === WIN2X_KEYS.KEY_G) {
          e.preventDefault();
          tileWindows(WIN2X_LAYOUT_MODES.TILE_GRID);
        } else if (key === WIN2X_KEYS.KEY_H) {
          e.preventDefault();
          tileWindows(WIN2X_LAYOUT_MODES.TILE_HORIZONTAL);
        } else if (key === WIN2X_KEYS.KEY_V) {
          e.preventDefault();
          tileWindows(WIN2X_LAYOUT_MODES.TILE_VERTICAL);
        } else if (key === WIN2X_KEYS.KEY_M) {
          e.preventDefault();
          minimizeAllWindows();
        } else if (key === WIN2X_KEYS.KEY_R) {
          e.preventDefault();
          restoreAllWindows();
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [enableKeyboardShortcuts, cascadeWindows, tileWindows, minimizeAllWindows, restoreAllWindows]);

  // Sync active theme to document.documentElement for global tokens
  useEffect(() => {
    if (typeof document !== "undefined" && document.documentElement) {
      document.documentElement.setAttribute(WIN2X_DATA_ATTRS.THEME, theme);
    }
  }, [theme]);

  const snapAssistCandidates = useMemo(() => {
    if (!snapAssistSession) return [];
    const filledIds = Array.from(snapAssistSession.filledSlots.values());
    return Array.from(windows.values()).filter((w) => !filledIds.includes(w.id) && !w.isMinimized);
  }, [snapAssistSession, windows]);

  const value: Win2xManagerContextValue = useMemo(
    () => ({
      windows,
      activeWindowId,
      snapAssistSession,
      enableSnapLayouts,
      theme,
      setTheme,
      focusWindow,
      registerWindow,
      unregisterWindow,
      updateWindow,
      cascadeWindows,
      tileWindows,
      minimizeAllWindows,
      restoreAllWindows,
      closeWindow,
      expandWindowInDirection,
      applySnapPreset,
      dismissSnapAssist,
      assignWindowToSnapAssistSlot,
    }),
    [
      windows,
      activeWindowId,
      snapAssistSession,
      enableSnapLayouts,
      theme,
      setTheme,
      focusWindow,
      registerWindow,
      unregisterWindow,
      updateWindow,
      cascadeWindows,
      tileWindows,
      minimizeAllWindows,
      restoreAllWindows,
      closeWindow,
      expandWindowInDirection,
      applySnapPreset,
      dismissSnapAssist,
      assignWindowToSnapAssistSlot,
    ],
  );

  return (
    <Win2xManagerContext.Provider value={value}>
      {children}
      {snapAssistSession && snapAssistCandidates.length > 0 && (
        <SnapAssistModal
          session={snapAssistSession}
          candidateWindows={snapAssistCandidates}
          onSelectWindow={(winId) =>
            assignWindowToSnapAssistSlot(winId, snapAssistSession.activeSlotIndex)
          }
          onDismiss={dismissSnapAssist}
        />
      )}
      <DockBar />
    </Win2xManagerContext.Provider>
  );
};
