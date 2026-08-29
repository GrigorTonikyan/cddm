import React, { useRef } from "react";
import { SnapLayoutPreset, WIN2X_DATA_ATTRS, WIN2X_TIMINGS } from "../../constants/win2x-constants";
import { WindowControls } from "../window-controls/window-controls";
import styles from "./title-bar.module.css";

export interface TitleBarProps {
  icon?: React.ReactNode;
  title: string;
  subtitle?: string;
  badge?: string;
  isMaximized: boolean;
  onPointerDown?: (e: React.PointerEvent<HTMLDivElement>) => void;
  onDoubleClick?: () => void;
  onContextMenu?: (e: React.MouseEvent<HTMLDivElement>) => void;
  onLongPress?: (x: number, y: number) => void;
  onMinimize?: () => void;
  onMaximizeToggle: () => void;
  onClose: () => void;
  showMinimize?: boolean;
  enableSnapLayouts?: boolean;
  onSnapPresetSelect?: (preset: SnapLayoutPreset, slotIndex: number) => void;
  className?: string;
}

/**
 * Composite molecular Windows 11 title bar with hardware draggable handle, context menu, and caption controls.
 */
export const TitleBar: React.FC<TitleBarProps> = ({
  icon,
  title,
  subtitle,
  badge,
  isMaximized,
  onPointerDown,
  onDoubleClick,
  onContextMenu,
  onLongPress,
  onMinimize,
  onMaximizeToggle,
  onClose,
  showMinimize = true,
  enableSnapLayouts = true,
  onSnapPresetSelect,
  className = "",
}) => {
  const longPressTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const roundedClass = isMaximized ? styles.notRounded || "" : styles.rounded || "";
  const combinedClass = `${styles.titleBar || ""} ${roundedClass} ${className}`.trim();

  const handleTouchStart = (e: React.TouchEvent<HTMLDivElement>) => {
    if (!onLongPress || e.touches.length === 0) return;
    const touch = e.touches[0];
    if (!touch) return;
    const { clientX, clientY } = touch;
    longPressTimerRef.current = setTimeout(() => {
      onLongPress(clientX, clientY);
    }, WIN2X_TIMINGS.LONG_PRESS_DELAY_MS);
  };

  const handleTouchEnd = () => {
    if (longPressTimerRef.current) {
      clearTimeout(longPressTimerRef.current);
      longPressTimerRef.current = null;
    }
  };

  return (
    <div
      onPointerDown={onPointerDown}
      onDoubleClick={onDoubleClick}
      onContextMenu={onContextMenu}
      onTouchStart={handleTouchStart}
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
      className={combinedClass}
      {...{ [WIN2X_DATA_ATTRS.TITLEBAR]: true }}
    >
      <div className={styles.infoArea}>
        {icon && <div className={styles.iconWrapper}>{icon}</div>}
        <div className={styles.textContainer}>
          <div className={styles.titleRow}>
            <h3 className={styles.title}>{title}</h3>
            {badge && <span className={styles.badge}>{badge}</span>}
          </div>
          {subtitle && <p className={styles.subtitle}>{subtitle}</p>}
        </div>
      </div>

      <div className={styles.controlsArea}>
        <WindowControls
          isMaximized={isMaximized}
          onMinimize={onMinimize}
          onMaximizeToggle={onMaximizeToggle}
          onClose={onClose}
          showMinimize={showMinimize}
          enableSnapLayouts={enableSnapLayouts}
          onSnapPresetSelect={onSnapPresetSelect}
        />
      </div>
    </div>
  );
};
