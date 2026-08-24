import React, { useEffect, useRef } from "react";
import {
  ContextMenuAction,
  WIN2X_CONTEXT_MENU_ACTIONS,
  WIN2X_DATA_ATTRS,
} from "../../constants/win2x-constants";
import {
  Minimize2,
  Maximize2,
  RotateCcw,
  Move,
  Scaling,
  Layers,
  LayoutGrid,
  X,
} from "lucide-react";
import styles from "./titlebar-context-menu.module.css";

export interface TitleBarContextMenuProps {
  isOpen: boolean;
  x: number;
  y: number;
  isMaximized: boolean;
  isSnapped: boolean;
  onAction: (action: ContextMenuAction) => void;
  onClose: () => void;
}

export const TitleBarContextMenu: React.FC<TitleBarContextMenuProps> = ({
  isOpen,
  x,
  y,
  isMaximized,
  isSnapped,
  onAction,
  onClose,
}) => {
  const menuRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!isOpen) return;

    const handleClickOutside = (e: MouseEvent | TouchEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    };

    window.addEventListener("mousedown", handleClickOutside);
    window.addEventListener("touchstart", handleClickOutside);
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("mousedown", handleClickOutside);
      window.removeEventListener("touchstart", handleClickOutside);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const canRestore = isMaximized || isSnapped;

  return (
    <div
      ref={menuRef}
      style={{ left: `${x}px`, top: `${y}px` }}
      className={styles.contextMenu}
      {...{ [WIN2X_DATA_ATTRS.CONTEXT_MENU]: true }}
      onClick={(e) => e.stopPropagation()}
    >
      <button
        type="button"
        disabled={!canRestore}
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.RESTORE);
          onClose();
        }}
      >
        <RotateCcw className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Restore</span>
      </button>

      <button
        type="button"
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.MOVE);
          onClose();
        }}
      >
        <Move className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Move</span>
      </button>

      <button
        type="button"
        disabled={isMaximized}
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.SIZE);
          onClose();
        }}
      >
        <Scaling className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Size</span>
      </button>

      <button
        type="button"
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.MINIMIZE);
          onClose();
        }}
      >
        <Minimize2 className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Minimize</span>
      </button>

      <button
        type="button"
        disabled={isMaximized}
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.MAXIMIZE);
          onClose();
        }}
      >
        <Maximize2 className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Maximize</span>
      </button>

      <div className={styles.divider} />

      <button
        type="button"
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.CASCADE_ALL);
          onClose();
        }}
      >
        <Layers className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Cascade All</span>
      </button>

      <button
        type="button"
        className={styles.menuItem}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.TILE_ALL);
          onClose();
        }}
      >
        <LayoutGrid className="w-3.5 h-3.5 mr-2.5 opacity-70" />
        <span>Tile All</span>
      </button>

      <div className={styles.divider} />

      <button
        type="button"
        className={`${styles.menuItem} ${styles.closeItem}`}
        onClick={() => {
          onAction(WIN2X_CONTEXT_MENU_ACTIONS.CLOSE);
          onClose();
        }}
      >
        <X className="w-3.5 h-3.5 mr-2.5 opacity-70 text-rose-400" />
        <span>Close</span>
        <span className={styles.shortcutHint}>Ctrl+W</span>
      </button>
    </div>
  );
};
