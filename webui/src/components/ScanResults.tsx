import React from "react";
import { useCDDMStore } from "../store/cddm-store";
import { ClonePairCard } from "./ClonePairCard";
import { Activity, Award, Copy, Clock, Layers } from "lucide-react";

/**
 * Props for ScanResults component.
 */
export interface ScanResultsProps {
  /** Optional CSS class name override */
  className?: string;
}

/**
 * Scan Results Dashboard & Metrics Grid component for CDDM WebUI.
 *
 * @param {ScanResultsProps} props - Component props
 */
export const ScanResults: React.FC<ScanResultsProps> = ({ className = "" }) => {
  const { results } = useCDDMStore();

  if (!results) return null;

  const scoreColor =
    results.dry_health_score >= 80
      ? "text-emerald-400 border-emerald-500/30 bg-emerald-950/20"
      : results.dry_health_score >= 60
      ? "text-yellow-400 border-yellow-500/30 bg-yellow-950/20"
      : "text-rose-400 border-rose-500/30 bg-rose-950/20";

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Top Metrics Banner */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        {/* DRY Health Score Card */}
        <div className={`border rounded-xl p-4 flex flex-col justify-between ${scoreColor}`}>
          <div className="flex items-center justify-between">
            <span className="text-xs font-semibold uppercase tracking-wider opacity-80">
              DRY Health Score
            </span>
            <Award className="w-5 h-5" />
          </div>
          <div className="mt-2">
            <span className="text-3xl font-extrabold font-mono">{results.dry_health_score.toFixed(1)}</span>
            <span className="text-sm opacity-60"> / 100</span>
          </div>
        </div>

        {/* Duplication Percentage */}
        <div className="bg-gray-900 border border-gray-800 rounded-xl p-4 flex flex-col justify-between">
          <div className="flex items-center justify-between text-gray-400">
            <span className="text-xs font-semibold uppercase tracking-wider">Duplication Rate</span>
            <Copy className="w-5 h-5 text-indigo-400" />
          </div>
          <div className="mt-2">
            <span className="text-3xl font-extrabold font-mono text-gray-100">
              {results.duplication_percentage.toFixed(2)}%
            </span>
          </div>
        </div>

        {/* Total Files */}
        <div className="bg-gray-900 border border-gray-800 rounded-xl p-4 flex flex-col justify-between">
          <div className="flex items-center justify-between text-gray-400">
            <span className="text-xs font-semibold uppercase tracking-wider">Files Scanned</span>
            <Layers className="w-5 h-5 text-indigo-400" />
          </div>
          <div className="mt-2">
            <span className="text-3xl font-extrabold font-mono text-gray-100">
              {results.total_files}
            </span>
          </div>
        </div>

        {/* Total Clones */}
        <div className="bg-gray-900 border border-gray-800 rounded-xl p-4 flex flex-col justify-between">
          <div className="flex items-center justify-between text-gray-400">
            <span className="text-xs font-semibold uppercase tracking-wider">Clone Pairs</span>
            <Activity className="w-5 h-5 text-indigo-400" />
          </div>
          <div className="mt-2">
            <span className="text-3xl font-extrabold font-mono text-gray-100">
              {results.total_clones}
            </span>
          </div>
        </div>

        {/* Duration */}
        <div className="bg-gray-900 border border-gray-800 rounded-xl p-4 flex flex-col justify-between">
          <div className="flex items-center justify-between text-gray-400">
            <span className="text-xs font-semibold uppercase tracking-wider">Scan Speed</span>
            <Clock className="w-5 h-5 text-indigo-400" />
          </div>
          <div className="mt-2">
            <span className="text-3xl font-extrabold font-mono text-gray-100">
              {results.duration_ms}
            </span>
            <span className="text-sm text-gray-400"> ms</span>
          </div>
        </div>
      </div>

      {/* Language Breakdown Chips */}
      {results.language_breakdown.length > 0 && (
        <div className="bg-gray-900 border border-gray-800 rounded-xl p-4">
          <h3 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3">
            Language Breakdown
          </h3>
          <div className="flex flex-wrap gap-3">
            {results.language_breakdown.map((lang) => (
              <div
                key={lang.language}
                className="bg-gray-950 border border-gray-800 rounded-lg px-3 py-1.5 flex items-center gap-2 text-xs font-mono"
              >
                <span className="font-bold text-indigo-300">{lang.language}</span>
                <span className="text-gray-500">|</span>
                <span className="text-gray-300">{lang.files} files</span>
                <span className="text-gray-500">|</span>
                <span className="text-gray-400">{lang.tokens} tokens</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Clone Pair List */}
      <div>
        <h3 className="text-lg font-bold text-gray-200 mb-4 flex items-center gap-2">
          <span>Detected Clone Pairs</span>
          <span className="text-xs bg-gray-800 text-gray-400 px-2 py-0.5 rounded-full font-mono font-normal">
            {results.clone_pairs.length} matches
          </span>
        </h3>

        {results.clone_pairs.length === 0 ? (
          <div className="bg-gray-900 border border-gray-800 rounded-xl p-8 text-center text-gray-400">
            ✔ Excellent! Zero code duplication detected in this scan.
          </div>
        ) : (
          <div className="space-y-4">
            {results.clone_pairs.slice(0, 100).map((pair, idx) => (
              <ClonePairCard key={idx} pair={pair} index={idx + 1} />
            ))}
            {results.clone_pairs.length > 100 && (
              <div className="text-center text-gray-500 py-4 font-mono text-sm">
                ... and {results.clone_pairs.length - 100} more clone pairs omitted for performance.
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};
