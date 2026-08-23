import React from "react";

export type BadgeVariant = "indigo" | "emerald" | "rose" | "amber" | "slate" | "cyan";

export interface BadgeProps {
  children: React.ReactNode;
  variant?: BadgeVariant;
  size?: "sm" | "md";
  className?: string;
}

const variantStyles: Record<BadgeVariant, string> = {
  indigo: "bg-indigo-950/80 text-indigo-300 border-indigo-800/60",
  emerald: "bg-emerald-950/80 text-emerald-300 border-emerald-800/60",
  rose: "bg-rose-950/80 text-rose-300 border-rose-900/60",
  amber: "bg-amber-950/80 text-amber-300 border-amber-800/60",
  slate: "bg-slate-800/80 text-slate-300 border-slate-700/60",
  cyan: "bg-cyan-950/80 text-cyan-300 border-cyan-800/60",
};

/**
 * Universal atomic badge for counts, tags, and status indications.
 */
export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = "slate",
  size = "md",
  className = "",
}) => {
  const sizeStyle =
    size === "sm" ? "px-1.5 py-0.2 text-[10px] leading-tight" : "px-2 py-0.5 text-xs";

  return (
    <span
      className={`inline-flex items-center font-mono font-medium rounded-full border ${variantStyles[variant]} ${sizeStyle} ${className}`}
    >
      {children}
    </span>
  );
};
