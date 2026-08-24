import React, { useEffect, useRef, useState, useMemo, useCallback } from "react";
import { Sparkles } from "lucide-react";
import { Portal } from "../../../atoms/portal/portal";
import { Backdrop } from "../../../atoms/backdrop/backdrop";
import {
  ContextMenuAction,
  PerformanceProfile,
  ResizeDirection,
  SnapLayoutPreset,
  SnapZone,
  Win2xTheme,
  WIN2X_CONTEXT_MENU_ACTIONS,
  WIN2X_DATA_ATTRS,
  WIN2X_DEFAULTS,
  WIN2X_KEYS,
  WIN2X_LAYOUT_MODES,
  WIN2X_PERFORMANCE_PROFILES,
  WIN2X_SNAP_ZONES,
} from "../../constants/win2x-constants";
import { TabItemData } from "../../core/types";
import { useBodyScrollLock } from "../../hooks/use-body-scroll-lock";
import { usePointerDrag } from "../../hooks/use-pointer-drag";
import { usePointerResize } from "../../hooks/use-pointer-resize";
import { useWindowManager } from "../../hooks/use-window-manager";
import { ResizeHandleGroup } from "../resize-handle-group/resize-handle-group";
import { TitleBar } from "../title-bar/title-bar";
import { TitleBarContextMenu } from "../titlebar-context-menu/titlebar-context-menu";
import { TabBar } from "../tab-bar/tab-bar";
import { SnapGhostGuide } from "../snap-ghost-guide/snap-ghost-guide";
import { computeSnapRect } from "../../core/geometry-engine";
import { defaultStorage, loadWindowState } from "../../core/storage-adapter";
import styles from "./win2x-window.module.css";

export interface Win2xWindowProps {
  id: string; // Required unique instance identifier
  windowType?: string; // Optional window category/type for template geometry
  isOpen: boolean;
  onClose: () => void;
  title: string;
  subtitle?: string;
  badge?: string;
  icon?: React.ReactNode;
  children: React.ReactNode;
  footer?: React.ReactNode;
  tabs?: TabItemData[];
  activeTabId?: string | null;
  onTabSelect?: (id: string) => void;
  onTabClose?: (id: string) => void;
  onTabAdd?: () => void;
  theme?: Win2xTheme;
  initialWidth?: number;
  initialHeight?: number;
  minWidth?: number;
  minHeight?: number;
  isModal?: boolean;
  showMinimize?: boolean;
  minimizeOnOutsideClick?: boolean;
  closeOnOutsideClick?: boolean;
  onOutsideClick?: () => void;
  performanceProfile?: PerformanceProfile;
  disableBlurWhileMoving?: boolean;
  className?: string;
}

const DEFAULT_WINDOW_ICON = <Sparkles className="w-4 h-4 text-indigo-400" />;

/**
 * Universal composed Windows 11 Acrylic Window organism.
 * Powered by pure CSS Modules, scoped CSS custom properties,
 * and a 120fps GPU compositor transform pipeline.
 *
 * STRICTLY REQUIRES Win2xManagerProvider for z-index, snapping, layout, and dock management.
 */
export const Win2xWindow: React.FC<Win2xWindowProps> = ({
  id,
  windowType,
  isOpen,
  onClose,
  title,
  subtitle,
  badge,
  icon = DEFAULT_WINDOW_ICON,
  children,
  footer,
  tabs,
  activeTabId = null,
  onTabSelect,
  onTabClose,
  onTabAdd,
  theme,
  initialWidth = WIN2X_DEFAULTS.INITIAL_WIDTH,
  initialHeight = WIN2X_DEFAULTS.INITIAL_HEIGHT,
  minWidth = WIN2X_DEFAULTS.MIN_WIDTH,
  minHeight = WIN2X_DEFAULTS.MIN_HEIGHT,
  isModal = false,
  showMinimize = true,
  minimizeOnOutsideClick,
  closeOnOutsideClick = false,
  onOutsideClick,
  performanceProfile = WIN2X_PERFORMANCE_PROFILES.BALANCED,
  disableBlurWhileMoving = true,
  className = "",
}) => {
  const windowContainerRef = useRef<HTMLDivElement | null>(null);
  const manager = useWindowManager();

  // Temporary local state for the snap ghost during drag
  const [dragSnapZone, setDragSnapZone] = useState<SnapZone>(WIN2X_SNAP_ZONES.NONE);

  // Context menu state
  const [contextMenuPos, setContextMenuPos] = useState<{ x: number; y: number } | null>(null);

  // Initial geometry with dual-tier storage loading
  const initialRect = useMemo(() => {
    const viewportW =
      typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
    const viewportH =
      typeof window !== "undefined" ? window.innerHeight : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;

    const savedState = loadWindowState(defaultStorage, id, windowType);
    if (savedState) {
      const w = Math.max(minWidth, savedState.width || initialWidth);
      const h = Math.max(minHeight, savedState.height || initialHeight);
      const posX = savedState.x >= 0 ? savedState.x : Math.max(0, Math.round((viewportW - w) / 2));
      const posY = savedState.y >= 0 ? savedState.y : Math.max(0, Math.round((viewportH - h) / 2));
      return { x: posX, y: posY, width: w, height: h };
    }

    return {
      x: Math.max(0, Math.round((viewportW - initialWidth) / 2)),
      y: Math.max(0, Math.round((viewportH - initialHeight) / 2)),
      width: initialWidth,
      height: initialHeight,
    };
  }, [id, windowType, initialWidth, initialHeight, minWidth, minHeight]);

  const { registerWindow, unregisterWindow, updateWindow } = manager;

  // Register with manager on mount / when isOpen changes to true
  useEffect(() => {
    if (isOpen) {
      registerWindow(id, {
        id,
        windowType,
        title,
        subtitle,
        badge,
        icon,
        isMinimized: false,
        isMaximized: false,
        isModal,
        rect: initialRect,
        preSnapRect: null,
        snappedZone: null,
        snappedPreset: null,
        onClose,
      });
    }
    return () => {
      unregisterWindow(id);
    };
  }, [isOpen, id, windowType, registerWindow, unregisterWindow, initialRect]);

  // Update title/subtitle/badge/icon/onClose in manager if they change
  useEffect(() => {
    if (isOpen) {
      updateWindow(id, { title, subtitle, badge, icon, onClose });
    }
  }, [isOpen, id, title, subtitle, badge, icon, onClose, updateWindow]);

  const winData = manager.windows.get(id);

  // Lock body scroll only if modal is active and open
  useBodyScrollLock(Boolean(isOpen && isModal && winData && !winData.isMinimized));

  // Other windows for magnetic snapping
  const otherWindows = useMemo(() => {
    return Array.from(manager.windows.values())
      .filter((w) => w.id !== id && !w.isMinimized)
      .map((w) => w.rect);
  }, [manager.windows, id]);

  const dragRect = winData ? winData.rect : initialRect;
  const { handlePointerDown, isDragging } = usePointerDrag({
    containerRef: windowContainerRef,
    x: dragRect.x,
    y: dragRect.y,
    width: dragRect.width,
    height: dragRect.height,
    isMaximized: winData?.isMaximized ?? false,
    enableSnapping: true,
    otherWindows,
    onSnapZoneChange: (zone) => {
      setDragSnapZone(zone);
    },
    onDragEnd: (finalX, finalY, snapZone) => {
      setDragSnapZone(WIN2X_SNAP_ZONES.NONE);
      if (!winData) return;

      let nextRect = { ...winData.rect, x: finalX, y: finalY };
      let finalZone: SnapZone = WIN2X_SNAP_ZONES.NONE;
      let preSnapRect = winData.preSnapRect;

      if (snapZone && snapZone !== WIN2X_SNAP_ZONES.NONE) {
        const viewportW =
          typeof window !== "undefined"
            ? window.innerWidth
            : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
        const viewportH =
          typeof window !== "undefined"
            ? window.innerHeight
            : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;
        const computed = computeSnapRect(snapZone, viewportW, viewportH);
        if (computed) {
          nextRect = computed;
          finalZone = snapZone;
          preSnapRect = winData.preSnapRect || { ...winData.rect, x: finalX, y: finalY };
        }
      } else {
        if (winData.snappedZone !== WIN2X_SNAP_ZONES.NONE && winData.snappedZone !== null) {
          preSnapRect = null;
        }
      }

      manager.updateWindow(id, {
        rect: nextRect,
        snappedZone: finalZone,
        snappedPreset: null,
        preSnapRect,
        isMaximized: finalZone === WIN2X_SNAP_ZONES.TOP_MAXIMIZE,
      });
    },
  });

  const { handleResizePointerDown, isResizing } = usePointerResize({
    containerRef: windowContainerRef,
    x: winData?.rect.x ?? initialRect.x,
    y: winData?.rect.y ?? initialRect.y,
    width: winData?.rect.width ?? initialRect.width,
    height: winData?.rect.height ?? initialRect.height,
    minWidth,
    minHeight,
    isMaximized: winData?.isMaximized ?? false,
    disabled: winData?.isMinimized ?? false,
    onResizeEnd: (finalRect) => {
      manager.updateWindow(id, {
        rect: finalRect,
        snappedZone: WIN2X_SNAP_ZONES.NONE,
        snappedPreset: null,
        preSnapRect: null,
      });
    },
  });

  const handleResizeDoubleClick = useCallback(
    (direction: ResizeDirection) => {
      manager.expandWindowInDirection(id, direction);
    },
    [manager, id],
  );

  // Handle Escape key
  useEffect(() => {
    if (!isOpen || !winData || winData.isMinimized) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === WIN2X_KEYS.ESCAPE && manager.activeWindowId === id) {
        onClose();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, winData?.isMinimized, manager.activeWindowId, id, onClose]);

  // Prevent render if not open or not registered yet
  if (!isOpen || !winData) return null;

  // The DockBar renders minimized states
  if (winData.isMinimized) {
    return null;
  }

  const toggleMaximize = () => {
    if (winData.isMaximized) {
      manager.updateWindow(id, {
        isMaximized: false,
        rect: winData.preSnapRect || initialRect,
        snappedZone: WIN2X_SNAP_ZONES.NONE,
        snappedPreset: null,
        preSnapRect: null,
      });
    } else {
      const viewportW =
        typeof window !== "undefined" ? window.innerWidth : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH;
      const viewportH =
        typeof window !== "undefined"
          ? window.innerHeight
          : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT;
      manager.updateWindow(id, {
        isMaximized: true,
        snappedZone: WIN2X_SNAP_ZONES.TOP_MAXIMIZE,
        snappedPreset: null,
        preSnapRect: winData.rect,
        rect: { x: 0, y: 0, width: viewportW, height: viewportH },
      });
    }
  };

  const handleTitleBarMinimize = () => {
    manager.updateWindow(id, { isMinimized: true });
  };

  const handleContextMenu = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    manager.focusWindow(id);
    setContextMenuPos({ x: e.clientX, y: e.clientY });
  };

  const handleTitleBarLongPress = (x: number, y: number) => {
    manager.focusWindow(id);
    setContextMenuPos({ x, y });
  };

  const handleContextMenuAction = (action: ContextMenuAction) => {
    switch (action) {
      case WIN2X_CONTEXT_MENU_ACTIONS.RESTORE:
        toggleMaximize();
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.MOVE:
      case WIN2X_CONTEXT_MENU_ACTIONS.SIZE:
        manager.focusWindow(id);
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.MINIMIZE:
        handleTitleBarMinimize();
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.MAXIMIZE:
        if (!winData.isMaximized) toggleMaximize();
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.CASCADE_ALL:
        manager.cascadeWindows();
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.TILE_ALL:
        manager.tileWindows(WIN2X_LAYOUT_MODES.TILE_GRID);
        break;
      case WIN2X_CONTEXT_MENU_ACTIONS.CLOSE:
        onClose();
        break;
    }
  };

  const handleBackdropClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.target !== e.currentTarget) return;

    if (onOutsideClick) {
      onOutsideClick();
      return;
    }

    const shouldMinimize = minimizeOnOutsideClick ?? showMinimize;
    if (shouldMinimize) {
      manager.updateWindow(id, { isMinimized: true });
    } else if (closeOnOutsideClick) {
      onClose();
    }
  };

  const handleSnapPresetSelect = (preset: SnapLayoutPreset, slotIndex: number) => {
    manager.applySnapPreset(id, preset, slotIndex);
  };

  const currentTheme = theme || manager.theme;

  const containerStyle: React.CSSProperties = {
    transform: `translate3d(${winData.rect.x}px, ${winData.rect.y}px, 0)`,
    width: `${winData.rect.width}px`,
    height: `${winData.rect.height}px`,
    zIndex: winData.zIndex,
  };

  const isMoving = isDragging || isResizing;
  const maximizedClass = winData.isMaximized ? styles.maximized || "" : "";
  const isActiveClass = manager.activeWindowId === id ? styles.active || "" : "";
  const combinedWindowClass =
    `${styles.window || ""} ${maximizedClass} ${isActiveClass} ${className}`.trim();
  const bodyMovingClass = isMoving ? styles.bodyMoving || "" : "";

  return (
    <Portal>
      {/* Optional Modal Backdrop overlay */}
      {isModal && (
        <Backdrop isOpen={isOpen && !winData.isMinimized} onClick={handleBackdropClick} />
      )}

      {/* Snap Ghost Guide */}
      {isDragging && <SnapGhostGuide snapZone={dragSnapZone} />}

      {/* TitleBar Desktop Context Menu */}
      {contextMenuPos && (
        <TitleBarContextMenu
          isOpen={Boolean(contextMenuPos)}
          x={contextMenuPos.x}
          y={contextMenuPos.y}
          isMaximized={winData.isMaximized}
          isSnapped={Boolean(winData.snappedZone || winData.snappedPreset)}
          onAction={handleContextMenuAction}
          onClose={() => setContextMenuPos(null)}
        />
      )}

      {/* Main Window Frame */}
      <div
        ref={windowContainerRef}
        style={containerStyle}
        className={combinedWindowClass}
        onPointerDownCapture={() => manager.focusWindow(id)}
        onPointerEnter={() => {
          if (!isMoving) {
            manager.focusWindow(id);
          }
        }}
        onWheel={(e) => e.stopPropagation()}
        {...{
          [WIN2X_DATA_ATTRS.WINDOW]: true,
          [WIN2X_DATA_ATTRS.ACTIVE]: manager.activeWindowId === id ? "true" : "false",
          [WIN2X_DATA_ATTRS.PROFILE]: performanceProfile,
          [WIN2X_DATA_ATTRS.THEME]: currentTheme,
          ...(isMoving && disableBlurWhileMoving ? { [WIN2X_DATA_ATTRS.MOVING]: "true" } : {}),
        }}
      >
        {/* Title Bar with Hardware Drag Handle */}
        <TitleBar
          icon={icon}
          title={title}
          subtitle={subtitle}
          badge={badge}
          isMaximized={winData.isMaximized}
          onPointerDown={handlePointerDown}
          onDoubleClick={toggleMaximize}
          onContextMenu={handleContextMenu}
          onLongPress={handleTitleBarLongPress}
          onMinimize={showMinimize ? handleTitleBarMinimize : undefined}
          onMaximizeToggle={toggleMaximize}
          onClose={onClose}
          showMinimize={showMinimize}
          enableSnapLayouts={manager.enableSnapLayouts}
          onSnapPresetSelect={handleSnapPresetSelect}
        />

        {/* Optional Tab Bar Header */}
        {tabs && tabs.length > 0 && onTabSelect && (
          <div className={styles.tabContainer}>
            <TabBar
              tabs={tabs}
              activeTabId={activeTabId}
              onTabSelect={onTabSelect}
              onTabClose={onTabClose}
              onTabAdd={onTabAdd}
            />
          </div>
        )}

        {/* Scrollable Window Body */}
        <div className={`${styles.body || ""} ${bodyMovingClass}`.trim()}>{children}</div>

        {/* Optional Window Footer */}
        {footer && <div className={styles.footer || ""}>{footer}</div>}

        {/* 8-Way Hardware Resize Handles with Double-Click Expansion */}
        {!winData.isMaximized && (
          <ResizeHandleGroup
            onPointerDown={handleResizePointerDown}
            onDoubleClick={handleResizeDoubleClick}
            isMaximized={winData.isMaximized}
          />
        )}
      </div>
    </Portal>
  );
};
