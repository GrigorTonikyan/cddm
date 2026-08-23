import React from "react";
import { ResizeDirection } from "../../../hooks/useResizable";

export interface ResizeHandleProps {
  direction: ResizeDirection;
  onMouseDown: (direction: ResizeDirection, e: React.MouseEvent) => void;
  className?: string;
}

const directionStyles: Record<ResizeDirection, string> = {
  top: "top-0 left-2 right-2 h-1.5 cursor-ns-resize",
  bottom: "bottom-0 left-2 right-2 h-1.5 cursor-ns-resize",
  left: "top-2 bottom-2 left-0 w-1.5 cursor-ew-resize",
  right: "top-2 bottom-2 right-0 w-1.5 cursor-ew-resize",
  "top-left": "top-0 left-0 w-3 h-3 cursor-nwse-resize z-10",
  "top-right": "top-0 right-0 w-3 h-3 cursor-nesw-resize z-10",
  "bottom-left": "bottom-0 left-0 w-3 h-3 cursor-nesw-resize z-10",
  "bottom-right": "bottom-0 right-0 w-3.5 h-3.5 cursor-nwse-resize z-10",
};

/**
 * Universal atomic resize handle positioned along window edges and corners.
 */
export const ResizeHandle: React.FC<ResizeHandleProps> = ({
  direction,
  onMouseDown,
  className = "",
}) => {
  return (
    <div
      onMouseDown={(e) => onMouseDown(direction, e)}
      className={`absolute select-none ${directionStyles[direction]} ${className}`}
      data-resize-handle={direction}
    />
  );
};
