import React from "react";
import {
  ResizeDirection,
  WIN2X_DATA_ATTRS,
  WIN2X_RESIZE_DIRECTIONS,
} from "../../constants/win2x-constants";
import styles from "./resize-handle.module.css";

export interface ResizeHandleProps {
  direction: ResizeDirection;
  onPointerDown: (direction: ResizeDirection, e: React.PointerEvent<HTMLDivElement>) => void;
  onDoubleClick?: (direction: ResizeDirection, e: React.MouseEvent<HTMLDivElement>) => void;
  className?: string;
}

const directionClassMap: Record<ResizeDirection, string> = {
  [WIN2X_RESIZE_DIRECTIONS.TOP]: styles.top || "",
  [WIN2X_RESIZE_DIRECTIONS.BOTTOM]: styles.bottom || "",
  [WIN2X_RESIZE_DIRECTIONS.LEFT]: styles.left || "",
  [WIN2X_RESIZE_DIRECTIONS.RIGHT]: styles.right || "",
  [WIN2X_RESIZE_DIRECTIONS.TOP_LEFT]: styles.topLeft || "",
  [WIN2X_RESIZE_DIRECTIONS.TOP_RIGHT]: styles.topRight || "",
  [WIN2X_RESIZE_DIRECTIONS.BOTTOM_LEFT]: styles.bottomLeft || "",
  [WIN2X_RESIZE_DIRECTIONS.BOTTOM_RIGHT]: styles.bottomRight || "",
};

/**
 * Universal atomic resize handle positioned along window edges and corners.
 */
export const ResizeHandle: React.FC<ResizeHandleProps> = ({
  direction,
  onPointerDown,
  onDoubleClick,
  className = "",
}) => {
  const combinedClass =
    `${styles.handle || ""} ${directionClassMap[direction]} ${className}`.trim();

  return (
    <div
      onPointerDown={(e) => onPointerDown(direction, e)}
      onDoubleClick={onDoubleClick ? (e) => onDoubleClick(direction, e) : undefined}
      className={combinedClass}
      {...{ [WIN2X_DATA_ATTRS.RESIZE_HANDLE]: direction }}
    />
  );
};
