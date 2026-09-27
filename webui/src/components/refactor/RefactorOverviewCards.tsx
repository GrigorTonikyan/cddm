import React from "react";
import { CollapsibleCard, BADGE_VARIANTS } from "../ui";
import { GitBranch, Layers, FileCode2 } from "lucide-react";
import { ClusterRefactorSuggestion, RefactorSuggestion } from "../../types/cddm-types";

interface RefactorOverviewCardsProps {
  clusterSuggestion: ClusterRefactorSuggestion | null;
  pairSuggestion: RefactorSuggestion | null;
}

export const RefactorOverviewCards: React.FC<RefactorOverviewCardsProps> = ({
  clusterSuggestion,
  pairSuggestion,
}) => {
  const linesSaved = clusterSuggestion?.total_lines_saved ?? pairSuggestion?.lines_saved ?? 0;
  const strategy = clusterSuggestion?.strategy ?? pairSuggestion?.strategy;
  const functionName =
    clusterSuggestion?.suggested_function_name ?? pairSuggestion?.suggested_function_name;

  return (
    <CollapsibleCard
      icon={<GitBranch className="w-4 h-4" />}
      title="Strategy & Overview"
      badgeCount={`~${linesSaved} lines saved`}
      badgeVariant={BADGE_VARIANTS.EMERALD}
      defaultOpen={true}
    >
      <div className="grid grid-cols-1 md:grid-cols-3 gap-3.5">
        <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
          <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
            <GitBranch className="w-3.5 h-3.5 text-indigo-400" />
            Strategy
          </span>
          <div className="font-mono text-xs font-bold text-indigo-300">
            {strategy === "extract_function" ? "Extract Function" : "Multi-Site Extraction"}
          </div>
        </div>

        <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
          <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
            <Layers className="w-3.5 h-3.5 text-emerald-400" />
            Estimated Savings
          </span>
          <div className="font-mono text-xs font-bold text-emerald-400">
            ~{linesSaved} lines eliminated
          </div>
        </div>

        <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
          <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
            <FileCode2 className="w-3.5 h-3.5 text-amber-400" />
            Helper Name
          </span>
          <div className="font-mono text-xs font-bold text-slate-200 truncate">
            {functionName}()
          </div>
        </div>
      </div>
    </CollapsibleCard>
  );
};
