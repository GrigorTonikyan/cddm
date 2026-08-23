import React, { useState } from "react";
import { ChevronDown } from "lucide-react";
import { Badge, BadgeVariant } from "../atoms/Badge";

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
  badgeVariant = "slate",
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

  return (
    <div
      className={`bg-slate-950/70 rounded-xl border border-slate-800/80 overflow-hidden transition-colors ${className}`}
      data-collapsible-card
    >
      {/* Header Bar */}
      <div
        onClick={handleToggle}
        className={`px-4 py-2.5 bg-slate-900/60 border-b border-slate-800/60 flex items-center justify-between cursor-pointer select-none hover:bg-slate-800/40 transition-colors ${
          !isExpanded ? "border-b-0" : ""
        } ${headerClassName}`}
      >
        <div className="flex items-center gap-2.5 min-w-0">
          <ChevronDown
            className={`w-4 h-4 text-slate-400 transition-transform duration-200 shrink-0 ${
              isExpanded ? "rotate-0" : "-rotate-90"
            }`}
          />
          {icon && <span className="text-indigo-400 shrink-0">{icon}</span>}
          <span className="text-xs font-mono font-bold text-slate-300 uppercase tracking-wider truncate">
            {title}
          </span>
          {badgeCount !== undefined && (
            <Badge variant={badgeVariant} size="sm">
              {badgeCount}
            </Badge>
          )}
        </div>

        {actions && (
          <div
            onClick={(e) => e.stopPropagation()}
            className="flex items-center gap-2 shrink-0 ml-3"
          >
            {actions}
          </div>
        )}
      </div>

      {/* Expandable Body */}
      {isExpanded && (
        <div className={`p-4 animate-in fade-in slide-in-from-top-1 duration-150 ${bodyClassName}`}>
          {children}
        </div>
      )}
    </div>
  );
};
