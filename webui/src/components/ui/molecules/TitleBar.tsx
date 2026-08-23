import React from "react";
import { WindowControls } from "../atoms/WindowControls";

export interface TitleBarProps {
  icon?: React.ReactNode;
  title: string;
  subtitle?: string;
  isMaximized: boolean;
  onMouseDown?: (e: React.MouseEvent) => void;
  onDoubleClick?: () => void;
  onMinimize?: () => void;
  onMaximizeToggle: () => void;
  onClose: () => void;
  showMinimize?: boolean;
  className?: string;
}

/**
 * Composite molecular Windows 11 title bar with draggable handle and caption buttons.
 */
export const TitleBar: React.FC<TitleBarProps> = ({
  icon,
  title,
  subtitle,
  isMaximized,
  onMouseDown,
  onDoubleClick,
  onMinimize,
  onMaximizeToggle,
  onClose,
  showMinimize = true,
  className = "",
}) => {
  return (
    <div
      onMouseDown={onMouseDown}
      onDoubleClick={onDoubleClick}
      className={`shrink-0 px-4 py-2.5 bg-slate-950/90 border-b border-slate-800/80 flex items-center justify-between select-none cursor-move ${
        isMaximized ? "rounded-none" : "rounded-t-2xl"
      } ${className}`}
      data-window-titlebar
    >
      <div className="flex items-center gap-2.5 min-w-0 pr-4 pointer-events-none">
        {icon && (
          <div className="p-1.5 rounded-lg bg-indigo-950/80 border border-indigo-800/50 text-indigo-400 shrink-0">
            {icon}
          </div>
        )}
        <div className="min-w-0">
          <h3 className="text-xs font-bold text-slate-100 truncate tracking-wide flex items-center gap-2">
            <span>{title}</span>
          </h3>
          {subtitle && (
            <p className="text-[11px] text-slate-400 font-mono truncate leading-none mt-0.5">
              {subtitle}
            </p>
          )}
        </div>
      </div>

      <div className="shrink-0 pointer-events-auto">
        <WindowControls
          isMaximized={isMaximized}
          onMinimize={onMinimize}
          onMaximizeToggle={onMaximizeToggle}
          onClose={onClose}
          showMinimize={showMinimize}
        />
      </div>
    </div>
  );
};
