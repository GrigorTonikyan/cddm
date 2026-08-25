import { useEffect } from "react";
import { WIN2X_KEYS, WIN2X_LAYOUT_MODES } from "../constants/win2x-constants";
import type { WindowLayoutMode } from "../core/types";

export interface UseWin2xShortcutsOptions {
  enabled: boolean;
  cascadeWindows: () => void;
  tileWindows: (mode: WindowLayoutMode) => void;
  minimizeAllWindows: () => void;
  restoreAllWindows: () => void;
}

export function useWin2xShortcuts({
  enabled,
  cascadeWindows,
  tileWindows,
  minimizeAllWindows,
  restoreAllWindows,
}: UseWin2xShortcutsOptions): void {
  useEffect(() => {
    if (!enabled || typeof window === "undefined") return;

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
  }, [enabled, cascadeWindows, tileWindows, minimizeAllWindows, restoreAllWindows]);
}
