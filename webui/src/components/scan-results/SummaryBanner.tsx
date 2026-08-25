import React from "react";
import type { ScanResult } from "../../types/cddm-types";
import { Activity, Award, Clock, Copy, GitBranch, Layers, Maximize2 } from "lucide-react";

interface SummaryCardProps {
  title: string;
  value: React.ReactNode;
  subtitle: string;
  icon: React.ReactNode;
}

const SummaryCard: React.FC<SummaryCardProps> = ({ title, value, subtitle, icon }) => (
  <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 flex flex-col justify-between shadow-lg">
    <div className="flex items-center justify-between text-slate-400">
      <span className="text-xs font-bold uppercase tracking-wider">{title}</span>
      {icon}
    </div>
    <div className="mt-3">
      <span className="text-3xl font-extrabold font-mono text-slate-100">{value}</span>
      <p className="text-[11px] text-slate-400 mt-1">{subtitle}</p>
    </div>
  </div>
);

export interface SummaryBannerProps {
  results: ScanResult;
  onOpenHealthAudit: () => void;
}

export const SummaryBanner: React.FC<SummaryBannerProps> = ({ results, onOpenHealthAudit }) => {
  const scoreColor =
    results.dry_health_score >= 80
      ? "text-emerald-400 border-emerald-500/40 bg-emerald-950/20 shadow-emerald-950/30 hover:border-emerald-400/80"
      : results.dry_health_score >= 60
        ? "text-amber-400 border-amber-500/40 bg-amber-950/20 shadow-amber-950/30 hover:border-amber-400/80"
        : "text-rose-400 border-rose-500/40 bg-rose-950/20 shadow-rose-950/30 hover:border-rose-400/80";

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-6 gap-4">
      {/* DRY Health Score Card (Clickable to open HealthAuditModal) */}
      <div
        onClick={onOpenHealthAudit}
        className={`border rounded-xl p-4 flex flex-col justify-between shadow-lg relative overflow-hidden cursor-pointer transition-all ${scoreColor}`}
        title="Click to open full DRY Health Score Audit Window"
      >
        <div className="flex items-center justify-between">
          <span className="text-xs font-bold uppercase tracking-wider opacity-90 flex items-center gap-1.5">
            <span>DRY Health Score</span>
            <Maximize2 className="w-3 h-3 opacity-60" />
          </span>
          <Award className="w-5 h-5" />
        </div>
        <div className="mt-3">
          <div className="flex items-baseline gap-1">
            <span className="text-3xl font-extrabold font-mono tracking-tight">
              {results.dry_health_score.toFixed(1)}
            </span>
            <span className="text-sm opacity-60">/ 100</span>
          </div>
          <div className="w-full bg-slate-900/60 rounded-full h-1.5 mt-2 overflow-hidden border border-slate-700/30">
            <div
              className={`h-full transition-all duration-500 ${
                results.dry_health_score >= 80
                  ? "bg-emerald-400"
                  : results.dry_health_score >= 60
                    ? "bg-amber-400"
                    : "bg-rose-400"
              }`}
              style={{ width: `${Math.min(100, Math.max(0, results.dry_health_score))}%` }}
            />
          </div>
        </div>
      </div>

      {/* Duplication Rate */}
      <SummaryCard
        title="Duplication Rate"
        value={`${results.duplication_percentage.toFixed(2)}%`}
        subtitle="Total code redundancy"
        icon={<Copy className="w-5 h-5 text-indigo-400" />}
      />

      {/* Files Scanned */}
      <SummaryCard
        title="Files Scanned"
        value={results.total_files.toLocaleString()}
        subtitle={`${results.total_tokens.toLocaleString()} tokens indexed`}
        icon={<Layers className="w-5 h-5 text-indigo-400" />}
      />

      {/* Clone Pairs */}
      <SummaryCard
        title="Clone Pairs"
        value={results.total_clones.toLocaleString()}
        subtitle="Pairwise duplicate fragments"
        icon={<Activity className="w-5 h-5 text-indigo-400" />}
      />

      {/* Clone Clusters */}
      <SummaryCard
        title="Clone Clusters"
        value={(results.total_clusters ?? results.clone_clusters?.length ?? 0).toLocaleString()}
        subtitle="N-way equivalence classes"
        icon={<GitBranch className="w-5 h-5 text-purple-400" />}
      />

      {/* Scan Duration */}
      <SummaryCard
        title="Engine Speed"
        value={
          <>
            {results.duration_ms}
            <span className="text-xs text-slate-400 font-mono"> ms</span>
          </>
        }
        subtitle="Winnowing M61 execution"
        icon={<Clock className="w-5 h-5 text-indigo-400" />}
      />
    </div>
  );
};
