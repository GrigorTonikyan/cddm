import React from "react";
import type { DeadCodeGateResult } from "../../types/dead-code-types";

export interface DeadCodeGateBadgeProps {
  gate?: DeadCodeGateResult;
}

export const DeadCodeGateBadge: React.FC<DeadCodeGateBadgeProps> = ({ gate }) => {
  if (!gate) return null;
  const statusLabel = gate.passed ? "[PASS]" : "[FAIL]";
  const colorClasses = gate.passed
    ? "bg-emerald-950/70 text-emerald-300 border-emerald-800/60"
    : "bg-rose-950/70 text-rose-300 border-rose-800/60";

  return (
    <span
      title={gate.message}
      className={`px-2 py-0.5 rounded border font-mono text-xs font-semibold ${colorClasses}`}
    >
      {statusLabel} Dead Code Gate ({gate.total_dead_items} / {gate.max_items})
    </span>
  );
};
