import React from "react";
import { Minus, Square, Copy, X } from "lucide-react";

export interface WindowControlsProps {
  isMaximized: boolean;
  onMinimize?: () => void;
  onMaximizeToggle: () => void;
  onClose: () => void;
  showMinimize?: boolean;
}

/**
 * Universal atomic Windows 11 window controls triad (Minimize, Maximize/Restore, Close).
 */
export const WindowControls: React.FC<WindowControlsProps> = ({
  isMaximized,
  onMinimize,
  onMaximizeToggle,
  onClose,
  showMinimize = true,
}) => {
  return (
    <div className="flex items-center space-x-1 select-none" data-window-controls>
      {showMinimize && onMinimize && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onMinimize();
          }}
          aria-label="Minimize"
          title="Minimize"
          className="w-8 h-7 flex items-center justify-center text-slate-400 hover:text-slate-100 hover:bg-slate-800/80 rounded transition-colors"
        >
          <Minus className="w-3.5 h-3.5" />
        </button>
      )}

      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          onMaximizeToggle();
        }}
        aria-label={isMaximized ? "Restore" : "Maximize"}
        title={isMaximized ? "Restore" : "Maximize"}
        className="w-8 h-7 flex items-center justify-center text-slate-400 hover:text-slate-100 hover:bg-slate-800/80 rounded transition-colors"
      >
        {isMaximized ? <Copy className="w-3 h-3 rotate-180" /> : <Square className="w-3 h-3" />}
      </button>

      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          onClose();
        }}
        aria-label="Close"
        title="Close"
        className="w-8 h-7 flex items-center justify-center text-slate-400 hover:text-white hover:bg-rose-600 rounded transition-colors"
      >
        <X className="w-4 h-4" />
      </button>
    </div>
  );
};
