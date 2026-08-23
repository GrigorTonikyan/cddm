import React, { useEffect } from "react";
import { Portal } from "../atoms/Portal";
import { Backdrop } from "../atoms/Backdrop";
import { TitleBar } from "../molecules/TitleBar";
import { ResizeHandleGroup } from "../molecules/ResizeHandleGroup";
import { useBodyScrollLock } from "../../../hooks/useBodyScrollLock";
import { useDraggable } from "../../../hooks/useDraggable";
import { useResizable } from "../../../hooks/useResizable";
import { useWindowState } from "../../../hooks/useWindowState";
import { Sparkles, Maximize2 } from "lucide-react";

export interface WindowProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  subtitle?: string;
  icon?: React.ReactNode;
  children: React.ReactNode;
  footer?: React.ReactNode;
  initialWidth?: number;
  initialHeight?: number;
  minWidth?: number;
  minHeight?: number;
  showMinimize?: boolean;
  className?: string;
}

/**
 * Universal composed Windows 11 Acrylic Window organism.
 * Features 8-way drag-resize, titlebar drag-move, full-screen maximize/restore,
 * minimization dock pill, body scroll-lock, and persistent configuration.
 */
export const Window: React.FC<WindowProps> = ({
  isOpen,
  onClose,
  title,
  subtitle,
  icon = <Sparkles className="w-4 h-4 text-indigo-400" />,
  children,
  footer,
  initialWidth = 920,
  initialHeight = 680,
  minWidth = 460,
  minHeight = 340,
  showMinimize = true,
  className = "",
}) => {
  const windowContainerRef = React.useRef<HTMLDivElement | null>(null);

  // Lock background body scroll whenever open and not minimized
  const { windowState, updatePosition, updateSize, toggleMaximize, toggleMinimize } =
    useWindowState({ initialWidth, initialHeight });

  useBodyScrollLock(isOpen && !windowState.isMinimized);

  const { handleMouseDown, isDragging } = useDraggable({
    containerRef: windowContainerRef,
    x: windowState.x,
    y: windowState.y,
    width: windowState.width,
    height: windowState.height,
    isMaximized: windowState.isMaximized,
    disabled: windowState.isMinimized,
    onDragEnd: (nextX, nextY) => updatePosition(nextX, nextY),
  });

  const { handleResizeMouseDown, isResizing } = useResizable({
    containerRef: windowContainerRef,
    x: windowState.x,
    y: windowState.y,
    width: windowState.width,
    height: windowState.height,
    minWidth,
    minHeight,
    isMaximized: windowState.isMaximized,
    disabled: windowState.isMinimized,
    onResizeEnd: (rect) => updateSize(rect),
  });

  // Handle Escape key to close window
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  // Minimized state dock pill at bottom right
  if (windowState.isMinimized) {
    return (
      <Portal>
        <div
          onClick={toggleMinimize}
          className="fixed bottom-4 right-4 z-[9999] px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700/80 shadow-2xl backdrop-blur-md flex items-center gap-3 cursor-pointer hover:bg-slate-800/90 transition-all animate-in slide-in-from-bottom-3 duration-200 select-none text-xs font-mono"
          data-window-minimized-pill
        >
          <div className="p-1.5 rounded-lg bg-indigo-950/80 border border-indigo-800/50 text-indigo-400">
            {icon}
          </div>
          <span className="font-bold text-slate-200 truncate max-w-xs">{title}</span>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              toggleMinimize();
            }}
            title="Restore Window"
            className="p-1 text-slate-400 hover:text-white rounded"
          >
            <Maximize2 className="w-3.5 h-3.5" />
          </button>
        </div>
      </Portal>
    );
  }

  // Active / Maximized Window container styling
  const containerStyle: React.CSSProperties = windowState.isMaximized
    ? {
        position: "fixed",
        top: 0,
        left: 0,
        width: "100vw",
        height: "100vh",
        zIndex: 9995,
      }
    : {
        position: "fixed",
        top: `${Math.max(0, windowState.y)}px`,
        left: `${Math.max(0, windowState.x)}px`,
        width: `${windowState.width}px`,
        height: `${windowState.height}px`,
        zIndex: 9995,
      };

  return (
    <Portal>
      {/* Backdrop overlay */}
      <Backdrop
        isOpen={isOpen && !windowState.isMinimized}
        onClick={(e) => {
          if (e.target === e.currentTarget) {
            onClose();
          }
        }}
      />

      {/* Main Window Frame */}
      <div
        ref={windowContainerRef}
        style={containerStyle}
        className={`bg-slate-900 border border-slate-700/80 flex flex-col shadow-2xl overflow-hidden backdrop-blur-xl animate-in zoom-in-95 duration-150 select-none ${
          windowState.isMaximized ? "rounded-none" : "rounded-2xl"
        } ${isDragging || isResizing ? "transition-none shadow-indigo-950/50" : ""} ${className}`}
        data-win11-window
      >
        {/* Title Bar with Drag & Caption Controls */}
        <TitleBar
          icon={icon}
          title={title}
          subtitle={subtitle}
          isMaximized={windowState.isMaximized}
          onMouseDown={handleMouseDown}
          onDoubleClick={toggleMaximize}
          onMinimize={showMinimize ? toggleMinimize : undefined}
          onMaximizeToggle={toggleMaximize}
          onClose={onClose}
          showMinimize={showMinimize}
        />

        {/* Scrollable Window Body */}
        <div
          className={`flex-1 overflow-y-auto overflow-x-hidden p-6 space-y-6 min-h-0 ${
            isDragging || isResizing ? "pointer-events-none select-none" : "select-text"
          }`}
        >
          {children}
        </div>

        {/* Optional Window Footer */}
        {footer && (
          <div className="shrink-0 px-6 py-3.5 bg-slate-950/90 border-t border-slate-800/80 flex items-center justify-between text-xs font-mono">
            {footer}
          </div>
        )}

        {/* 8-Way Resize Handles */}
        <ResizeHandleGroup
          onResizeMouseDown={handleResizeMouseDown}
          isMaximized={windowState.isMaximized}
        />
      </div>
    </Portal>
  );
};
