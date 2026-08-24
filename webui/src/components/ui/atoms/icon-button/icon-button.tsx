import React from "react";
import {
  BUTTON_SIZES,
  BUTTON_VARIANTS,
  ButtonSize,
  ButtonVariant,
  UI_DATA_ATTRS,
} from "../../constants/ui-constants";
import styles from "./icon-button.module.css";

export interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  icon: React.ReactNode;
  title?: string;
  variant?: ButtonVariant;
  size?: ButtonSize;
  className?: string;
}

/**
 * Universal atomic icon button component with accessible labels and hover states.
 */
export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  title,
  variant = BUTTON_VARIANTS.DEFAULT,
  size = BUTTON_SIZES.MD,
  className = "",
  type = "button",
  ...props
}) => {
  const sizeClass = size === BUTTON_SIZES.SM ? styles.sizeSm || "" : styles.sizeMd || "";
  const variantClass = variant === BUTTON_VARIANTS.DANGER ? styles.variantDanger || "" : "";
  const combinedClass = `${styles.button || ""} ${sizeClass} ${variantClass} ${className}`.trim();

  return (
    <button
      type={type}
      title={title}
      aria-label={title || props["aria-label"]}
      className={combinedClass}
      {...{ [UI_DATA_ATTRS.ICON_BUTTON]: true }}
      {...props}
    >
      {icon}
    </button>
  );
};
