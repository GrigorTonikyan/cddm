import React from "react";
import { UI_DATA_ATTRS } from "../../constants/ui-constants";
import styles from "./backdrop.module.css";

export interface BackdropProps {
  isOpen: boolean;
  onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
  children?: React.ReactNode;
  className?: string;
  transparent?: boolean;
}

/**
 * Universal atomic backdrop overlay with acrylic blur and click-to-dismiss support.
 */
export const Backdrop: React.FC<BackdropProps> = ({
  isOpen,
  onClick,
  children,
  className = "",
  transparent = false,
}) => {
  if (!isOpen) return null;

  const transparentClass = transparent ? styles.transparent || "" : "";
  const combinedClass = `${styles.backdrop || ""} ${transparentClass} ${className}`.trim();

  return (
    <div onClick={onClick} className={combinedClass} {...{ [UI_DATA_ATTRS.BACKDROP]: true }}>
      {children}
    </div>
  );
};
