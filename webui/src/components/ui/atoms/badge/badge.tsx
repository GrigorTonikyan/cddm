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

const getBadgeVariantClass = (variant: BadgeVariant): string => {
  switch (variant) {
    case BADGE_VARIANTS.EMERALD:
      return styles.variantEmerald || "";
    case BADGE_VARIANTS.ROSE:
      return styles.variantRose || "";
    case BADGE_VARIANTS.AMBER:
      return styles.variantAmber || "";
    case BADGE_VARIANTS.SLATE:
      return styles.variantSlate || "";
    case BADGE_VARIANTS.CYAN:
      return styles.variantCyan || "";
    default:
      return styles.variantIndigo || "";
  }
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
  const sizeClass = size === BADGE_SIZES.SM ? styles.sizeSm : styles.sizeMd;
  const combinedClass =
    `${styles.badge || ""} ${getBadgeVariantClass(variant)} ${sizeClass || ""} ${className}`.trim();

  return (
    <span className={combinedClass} {...{ [UI_DATA_ATTRS.BADGE]: true }}>
      {children}
    </span>
  );
};
