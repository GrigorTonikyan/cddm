import React from "react";
import {
  ResizeDirection,
  WIN2X_DATA_ATTRS,
  WIN2X_RESIZE_DIRECTIONS,
} from "../../constants/win2x-constants";
import { ResizeHandle } from "../resize-handle/resize-handle";
import styles from "./resize-handle-group.module.css";

export interface ResizeHandleGroupProps {
  onPointerDown: (direction: ResizeDirection, e: React.PointerEvent<HTMLDivElement>) => void;
  onDoubleClick?: (direction: ResizeDirection, e: React.MouseEvent<HTMLDivElement>) => void;
  isMaximized: boolean;
}

const DIRECTIONS: ResizeDirection[] = [
  WIN2X_RESIZE_DIRECTIONS.TOP,
  WIN2X_RESIZE_DIRECTIONS.BOTTOM,
  WIN2X_RESIZE_DIRECTIONS.LEFT,
  WIN2X_RESIZE_DIRECTIONS.RIGHT,
  WIN2X_RESIZE_DIRECTIONS.TOP_LEFT,
  WIN2X_RESIZE_DIRECTIONS.TOP_RIGHT,
  WIN2X_RESIZE_DIRECTIONS.BOTTOM_LEFT,
  WIN2X_RESIZE_DIRECTIONS.BOTTOM_RIGHT,
];

/**
 * Molecular assembly of all 8-direction resize triggers.
 */
export const ResizeHandleGroup: React.FC<ResizeHandleGroupProps> = ({
  onPointerDown,
  onDoubleClick,
  isMaximized,
}) => {
  if (isMaximized) return null;

  return (
    <div className={styles.container || ""} {...{ [WIN2X_DATA_ATTRS.RESIZE_HANDLES]: true }}>
      {DIRECTIONS.map((direction) => (
        <ResizeHandle
          key={`resize-${direction}`}
          direction={direction}
          onPointerDown={onPointerDown}
          onDoubleClick={onDoubleClick}
          className={styles.handleItem || ""}
        />
      ))}
    </div>
  );
};
