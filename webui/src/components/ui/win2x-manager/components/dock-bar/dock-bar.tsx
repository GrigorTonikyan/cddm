import React, { useRef, useState } from "react";
import { useWindowManager } from "../../hooks/use-window-manager";
import { Maximize2, X, LayoutGrid, Layers, MonitorUp, GripVertical } from "lucide-react";
import {
  WIN2X_ARIA_LABELS,
  WIN2X_DATA_ATTRS,
  WIN2X_LAYOUT_MODES,
} from "../../constants/win2x-constants";
import { WindowMetaDisplay } from "../common/window-meta";
import styles from "./dock-bar.module.css";

export const DockBar: React.FC = () => {
  const manager = useWindowManager();
  const windowsArray = Array.from(manager.windows.values());
  const minimizedWindows = windowsArray.filter((w) => w.isMinimized);

  const [dockOffset, setDockOffset] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const isDraggingDock = useRef(false);
  const dragStartPos = useRef<{ x: number; y: number }>({ x: 0, y: 0 });
  const initialOffset = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  if (windowsArray.length === 0) return null;

  const handleDockPointerDown = (e: React.PointerEvent) => {
    isDraggingDock.current = true;
    dragStartPos.current = { x: e.clientX, y: e.clientY };
    initialOffset.current = { ...dockOffset };

    const handlePointerMove = (moveEvent: PointerEvent) => {
      if (!isDraggingDock.current) return;
      const deltaX = moveEvent.clientX - dragStartPos.current.x;
      const deltaY = moveEvent.clientY - dragStartPos.current.y;
      setDockOffset({
        x: initialOffset.current.x + deltaX,
        y: initialOffset.current.y + deltaY,
      });
    };

    const handlePointerUp = () => {
      isDraggingDock.current = false;
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", handlePointerUp);
    };

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", handlePointerUp);
  };

  const dockStyle: React.CSSProperties = {
    transform: `translate3d(calc(-50% + ${dockOffset.x}px), ${dockOffset.y}px, 0)`,
  };

  return (
    <div
      style={dockStyle}
      className={styles.dockBar}
      {...{
        [WIN2X_DATA_ATTRS.DOCK_CONTAINER]: true,
        [WIN2X_DATA_ATTRS.THEME]: manager.theme,
      }}
    >
      <div
        className={styles.dockDragHandle}
        onPointerDown={handleDockPointerDown}
        title="Drag DockBar"
      >
        <GripVertical size={14} className={styles.dragHandleIcon} />
      </div>

      <div className={styles.dockGlobalControls}>
        <button
          type="button"
          title="Cascade Layout"
          className={styles.layoutBtn}
          onClick={() => manager.cascadeWindows()}
        >
          <Layers size={16} />
        </button>
        <button
          type="button"
          title="Tile Layout"
          className={styles.layoutBtn}
          onClick={() => manager.tileWindows(WIN2X_LAYOUT_MODES.TILE_GRID)}
        >
          <LayoutGrid size={16} />
        </button>
        <div className={styles.divider} />
        <button
          type="button"
          title="Minimize All"
          className={styles.layoutBtn}
          onClick={() => manager.minimizeAllWindows()}
        >
          <MonitorUp size={16} className={styles.rotate180} />
        </button>
      </div>

      {minimizedWindows.length > 0 && <div className={styles.divider} />}

      <div className={styles.dockPillContainer}>
        {minimizedWindows.map((win) => (
          <div
            key={win.id}
            role="button"
            tabIndex={0}
            className={styles.pill}
            title={`${win.title}${win.subtitle ? ` (${win.subtitle})` : ""}`}
            {...{ [WIN2X_DATA_ATTRS.MINIMIZED_PILL]: true }}
            onClick={() => {
              manager.updateWindow(win.id, { isMinimized: false });
              manager.focusWindow(win.id);
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                manager.updateWindow(win.id, { isMinimized: false });
                manager.focusWindow(win.id);
              }
            }}
          >
            <WindowMetaDisplay
              win={win}
              iconWrapperClass={styles.pillIconWrapper}
              infoClass={styles.pillTextGroup}
              titleClass={styles.pillTitle}
              subtitleClass={styles.pillSubtitle}
              badgeClass={styles.pillBadge}
            />
            <div className={styles.pillControls}>
              <button
                type="button"
                className={styles.pillRestoreBtn}
                title={WIN2X_ARIA_LABELS.RESTORE_WINDOW}
                onClick={(e) => {
                  e.stopPropagation();
                  manager.updateWindow(win.id, { isMinimized: false });
                  manager.focusWindow(win.id);
                }}
              >
                <Maximize2 size={14} />
              </button>
              <button
                type="button"
                className={styles.pillCloseBtn}
                title={WIN2X_ARIA_LABELS.CLOSE_MINIMIZED}
                onClick={(e) => {
                  e.stopPropagation();
                  manager.closeWindow(win.id);
                }}
              >
                <X size={14} />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
