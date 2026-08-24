import React, { useState } from "react";
import { ChevronDown } from "lucide-react";
import { Badge } from "../../atoms/badge/badge";
import { BADGE_VARIANTS, BadgeVariant, UI_DATA_ATTRS } from "../../constants/ui-constants";
import styles from "./collapsible-card.module.css";

export interface CollapsibleCardProps {
  icon?: React.ReactNode;
  title: string;
  badgeCount?: number | string;
  badgeVariant?: BadgeVariant;
  actions?: React.ReactNode;
  children: React.ReactNode;
  defaultOpen?: boolean;
  isOpen?: boolean;
  onToggle?: (isOpen: boolean) => void;
  className?: string;
  headerClassName?: string;
  bodyClassName?: string;
}

/**
 * Universal molecular collapsible card supporting independent expand/collapse states.
 */
export const CollapsibleCard: React.FC<CollapsibleCardProps> = ({
  icon,
  title,
  badgeCount,
  badgeVariant = BADGE_VARIANTS.SLATE,
  actions,
  children,
  defaultOpen = true,
  isOpen: controlledOpen,
  onToggle,
  className = "",
  headerClassName = "",
  bodyClassName = "",
}) => {
  const [internalOpen, setInternalOpen] = useState(defaultOpen);
  const isExpanded = controlledOpen !== undefined ? controlledOpen : internalOpen;

  const handleToggle = () => {
    const next = !isExpanded;
    if (controlledOpen === undefined) {
      setInternalOpen(next);
    }
    onToggle?.(next);
  };

  const headerClosedClass = !isExpanded ? styles.headerClosed || "" : "";
  const chevronClass = `${styles.chevron || ""} ${
    isExpanded ? styles.chevronExpanded || "" : styles.chevronCollapsed || ""
  }`;

  return (
    <div
      className={`${styles.card || ""} ${className}`.trim()}
      {...{ [UI_DATA_ATTRS.COLLAPSIBLE_CARD]: true }}
    >
      {/* Header Bar */}
      <div
        onClick={handleToggle}
        className={`${styles.header || ""} ${headerClosedClass} ${headerClassName}`.trim()}
      >
        <div className={styles.headerLeft}>
          <ChevronDown className={chevronClass} />
          {icon && <span className={styles.iconWrapper}>{icon}</span>}
          <span className={styles.title}>{title}</span>
          {badgeCount !== undefined && (
            <Badge variant={badgeVariant} size="sm">
              {badgeCount}
            </Badge>
          )}
        </div>

        {actions && (
          <div onClick={(e) => e.stopPropagation()} className={styles.actions}>
            {actions}
          </div>
        )}
      </div>

      {/* Expandable Body */}
      {isExpanded && (
        <div className={`${styles.body || ""} ${bodyClassName}`.trim()}>{children}</div>
      )}
    </div>
  );
};
