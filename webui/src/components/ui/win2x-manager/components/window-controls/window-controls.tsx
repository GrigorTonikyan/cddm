import React, { useRef, useState } from "react";
import { Minus, Square, Copy, X } from "lucide-react";
import {
  SnapLayoutPreset,
  WIN2X_ARIA_LABELS,
  WIN2X_DATA_ATTRS,
  WIN2X_TIMINGS,
} from "../../constants/win2x-constants";
import { SnapLayoutsMenu } from "../snap-layouts-menu/snap-layouts-menu";
import styles from "./window-controls.module.css";

export interface WindowControlsProps {
  isMaximized: boolean;
  onMinimize?: () => void;
  onMaximizeToggle: () => void;
  onClose: () => void;
  showMinimize?: boolean;
  enableSnapLayouts?: boolean;
  onSnapPresetSelect?: (preset: SnapLayoutPreset, slotIndex: number) => void;
  className?: string;
}

/**
 * Universal atomic Windows 11 caption controls triad (Minimize, Maximize/Restore, Close).
 */
export const WindowControls: React.FC<WindowControlsProps> = ({
  isMaximized,
  onMinimize,
  onMaximizeToggle,
  onClose,
  showMinimize = true,
  enableSnapLayouts = true,
  onSnapPresetSelect,
  className = "",
}) => {
  const [showSnapMenu, setShowSnapMenu] = useState(false);
  const hoverTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const longPressTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const maxBtnRef = useRef<HTMLButtonElement | null>(null);

  const handleMouseEnter = () => {
    if (!enableSnapLayouts) return;
    if (hoverTimerRef.current) clearTimeout(hoverTimerRef.current);
    hoverTimerRef.current = setTimeout(() => {
      setShowSnapMenu(true);
    }, WIN2X_TIMINGS.SNAP_LAYOUT_HOVER_DELAY_MS);
  };

  const handleMouseLeave = () => {
    if (hoverTimerRef.current) {
      clearTimeout(hoverTimerRef.current);
      hoverTimerRef.current = null;
    }
  };

  const handleContextMenu = (e: React.MouseEvent) => {
    if (enableSnapLayouts) {
      e.preventDefault();
      e.stopPropagation();
      setShowSnapMenu((prev) => !prev);
    }
  };

  const handleTouchStart = () => {
    if (!enableSnapLayouts) return;
    longPressTimerRef.current = setTimeout(() => {
      setShowSnapMenu(true);
    }, WIN2X_TIMINGS.LONG_PRESS_DELAY_MS);
  };

  const handleTouchEnd = () => {
    if (longPressTimerRef.current) {
      clearTimeout(longPressTimerRef.current);
      longPressTimerRef.current = null;
    }
  };

  return (
    <div
      className={`${styles.container || ""} ${className}`.trim()}
      {...{ [WIN2X_DATA_ATTRS.CONTROLS]: true }}
      onMouseLeave={() => setShowSnapMenu(false)}
    >
      {showMinimize && onMinimize && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onMinimize();
          }}
          aria-label={WIN2X_ARIA_LABELS.MINIMIZE}
          title={WIN2X_ARIA_LABELS.MINIMIZE}
          className={styles.button || ""}
        >
          <Minus className="w-3.5 h-3.5" />
        </button>
      )}

      <div className="relative inline-flex items-center">
        <button
          ref={maxBtnRef}
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            setShowSnapMenu(false);
            onMaximizeToggle();
          }}
          onMouseEnter={handleMouseEnter}
          onMouseLeave={handleMouseLeave}
          onContextMenu={handleContextMenu}
          onTouchStart={handleTouchStart}
          onTouchEnd={handleTouchEnd}
          aria-label={isMaximized ? WIN2X_ARIA_LABELS.RESTORE : WIN2X_ARIA_LABELS.MAXIMIZE}
          title={isMaximized ? WIN2X_ARIA_LABELS.RESTORE : WIN2X_ARIA_LABELS.MAXIMIZE}
          className={styles.button || ""}
        >
          {isMaximized ? <Copy className="w-3 h-3 rotate-180" /> : <Square className="w-3 h-3" />}
        </button>

        {enableSnapLayouts && showSnapMenu && onSnapPresetSelect && (
          <SnapLayoutsMenu
            isOpen={showSnapMenu}
            onSelect={(preset, slotIndex) => {
              setShowSnapMenu(false);
              onSnapPresetSelect(preset, slotIndex);
            }}
            onClose={() => setShowSnapMenu(false)}
          />
        )}
      </div>

      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          onClose();
        }}
        aria-label={WIN2X_ARIA_LABELS.CLOSE}
        title={WIN2X_ARIA_LABELS.CLOSE}
        className={`${styles.button || ""} ${styles.closeButton || ""}`}
      >
        <X className="w-4 h-4" />
      </button>
    </div>
  );
};
