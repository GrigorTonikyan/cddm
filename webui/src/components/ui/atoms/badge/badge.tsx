import React from "react";
import {
  BADGE_SIZES,
  BADGE_VARIANTS,
  BadgeSize,
  BadgeVariant,
  UI_DATA_ATTRS,
} from "../../constants/ui-constants";
import styles from "./badge.module.css";

export interface BadgeProps {
  children: React.ReactNode;
  variant?: BadgeVariant;
  size?: BadgeSize;
  className?: string;
}

const variantClassMap: Record<BadgeVariant, string> = {
  [BADGE_VARIANTS.INDIGO]: styles.variantIndigo || "",
  [BADGE_VARIANTS.EMERALD]: styles.variantEmerald || "",
  [BADGE_VARIANTS.ROSE]: styles.variantRose || "",
  [BADGE_VARIANTS.AMBER]: styles.variantAmber || "",
  [BADGE_VARIANTS.SLATE]: styles.variantSlate || "",
  [BADGE_VARIANTS.CYAN]: styles.variantCyan || "",
};

const sizeClassMap: Record<BadgeSize, string> = {
  [BADGE_SIZES.SM]: styles.sizeSm || "",
  [BADGE_SIZES.MD]: styles.sizeMd || "",
};

/**
 * Universal atomic badge pill component with semantic color variants.
 */
export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = BADGE_VARIANTS.INDIGO,
  size = BADGE_SIZES.MD,
  className = "",
}) => {
  const combinedClass =
    `${styles.badge || ""} ${variantClassMap[variant]} ${sizeClassMap[size]} ${className}`.trim();

  return (
    <span className={combinedClass} {...{ [UI_DATA_ATTRS.BADGE]: true }}>
      {children}
    </span>
  );
};
