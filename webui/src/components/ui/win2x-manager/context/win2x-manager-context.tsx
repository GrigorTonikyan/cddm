import React, { createContext, useCallback, useEffect, useMemo, useState } from "react";
import {
  ResizeDirection,
  SnapLayoutPreset,
  Win2xTheme,
  WIN2X_DATA_ATTRS,
  WIN2X_DEFAULTS,
  WIN2X_SNAP_ZONES,
  WIN2X_THEMES,
} from "../constants/win2x-constants";
import type { SnapAssistSession, Win2xManagerContextValue, WindowLayoutMode } from "../core/types";
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
import { useWin2xRegistry } from "../hooks/use-win2x-registry";
import { useWin2xShortcuts } from "../hooks/use-win2x-shortcuts";

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
  const [snapAssistSession, setSnapAssistSession] = useState<SnapAssistSession | null>(null);

  const {
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
  } = useWin2xRegistry();

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
    [windowsRef, updateWindow, focusWindow],
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
    [windowsRef, updateWindow, focusWindow],
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
    [snapAssistSession, windowsRef, updateWindow],
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
  }, [setWindows, zIndexStackRef]);

  const tileWindows = useCallback(
    (mode: WindowLayoutMode) => {
      setWindows((prev) => {
        const activeIds = zIndexStackRef.current.filter(
          (id) => !prev.get(id)?.isMinimized && !prev.get(id)?.isMaximized,
        );
        if (activeIds.length === 0) return prev;

        const viewportW =
          typeof window !== "undefined"
            ? window.innerWidth
            : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
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
            next.set(id, {
              ...win,
              rect,
              preSnapRect: null,
              snappedZone: null,
              snappedPreset: null,
            });
          }
        });
        return next;
      });
    },
    [setWindows, zIndexStackRef],
  );

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
  }, [setWindows, setActiveWindowId]);

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
  }, [setWindows, zIndexStackRef, setActiveWindowId]);

  const closeWindow = useCallback(
    (id: string) => {
      const win = windowsRef.current.get(id);
      if (win?.onClose) {
        win.onClose();
      } else {
        unregisterWindow(id);
      }
    },
    [windowsRef, unregisterWindow],
  );

  useWin2xShortcuts({
    enabled: enableKeyboardShortcuts,
    cascadeWindows,
    tileWindows,
    minimizeAllWindows,
    restoreAllWindows,
  });

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
