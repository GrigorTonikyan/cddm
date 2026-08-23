import React from "react";
import { ResizeDirection } from "../../../hooks/useResizable";
import { ResizeHandle } from "../atoms/ResizeHandle";

export interface ResizeHandleGroupProps {
  onResizeMouseDown: (direction: ResizeDirection, e: React.MouseEvent) => void;
  isMaximized?: boolean;
}

const ALL_DIRECTIONS: ResizeDirection[] = [
  "top",
  "bottom",
  "left",
  "right",
  "top-left",
  "top-right",
  "bottom-left",
  "bottom-right",
];

/**
 * Composite molecular component rendering all 8-direction resize handles.
 */
export const ResizeHandleGroup: React.FC<ResizeHandleGroupProps> = ({
  onResizeMouseDown,
  isMaximized = false,
}) => {
  if (isMaximized) return null;

  return (
    <>
      {ALL_DIRECTIONS.map((dir) => (
        <ResizeHandle
          key={`resize-handle-${dir}`}
          direction={dir}
          onMouseDown={onResizeMouseDown}
        />
      ))}
    </>
  );
};
