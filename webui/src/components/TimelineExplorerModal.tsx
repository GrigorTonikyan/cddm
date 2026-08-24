import React, { useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  History,
  TrendingUp,
  TrendingDown,
  GitCommit,
  ShieldCheck,
  RefreshCw,
  Tag,
  Sliders,
  AlertTriangle,
} from "lucide-react";
import { TimelineSnapshot } from "../types/cddm-types";

export interface TimelineExplorerModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const TimelineExplorerModal: React.FC<TimelineExplorerModalProps> = ({
  isOpen,
  onClose,
}) => {
  const {
    timelineData,
    isTimelineLoading,
    timelineError,
    hookStatus,
    fetchTimeline,
    fetchHookStatus,
    installHook,
  } = useCDDMStore();

  const [hoveredSnapshot, setHoveredSnapshot] = useState<TimelineSnapshot | null>(null);
  const [maxSamples, setMaxSamples] = useState<number>(10);
  const [hookInstallMessage, setHookInstallMessage] = useState<string | null>(null);
  const [isInstallingHook, setIsInstallingHook] = useState<boolean>(false);

  if (!isOpen) return null;

  const snapshots = timelineData?.snapshots || [];
  const hasData = snapshots.length > 0;

  const handleRefresh = async () => {
    await fetchTimeline(undefined, maxSamples);
    await fetchHookStatus();
  };

  const handleInstallPreCommit = async () => {
    setIsInstallingHook(true);
    setHookInstallMessage(null);
    try {
      const msg = await installHook("pre-commit", 15.0);
      setHookInstallMessage(msg);
    } catch (err) {
      setHookInstallMessage(err instanceof Error ? err.message : "Failed to install hook");
    } finally {
      setIsInstallingHook(false);
    }
  };

  // SVG Chart Geometry calculations
  const svgWidth = 720;
  const svgHeight = 220;
  const padLeft = 45;
  const padRight = 30;
  const padTop = 25;
  const padBottom = 35;
  const chartW = svgWidth - padLeft - padRight;
  const chartH = svgHeight - padTop - padBottom;

  const getX = (index: number) => {
    if (snapshots.length <= 1) return padLeft + chartW / 2;
    return padLeft + (index / (snapshots.length - 1)) * chartW;
  };

  // Y for DRY Score (0 to 100)
  const getYScore = (score: number) => {
    const clamped = Math.max(0, Math.min(100, score));
    return padTop + chartH - (clamped / 100) * chartH;
  };

  // Y for Duplication (0 to 50 max scaled)
  const getYDuplication = (dup: number) => {
    const maxDupScale = 50;
    const clamped = Math.max(0, Math.min(maxDupScale, dup));
    return padTop + chartH - (clamped / maxDupScale) * chartH;
  };

  // Generate SVG path points
  const dryPoints = snapshots
    .map((s, idx) => `${getX(idx)},${getYScore(s.dry_health_score)}`)
    .join(" ");
  const dupPoints = snapshots
    .map((s, idx) => `${getX(idx)},${getYDuplication(s.duplication_percentage)}`)
    .join(" ");

  const isPositiveDelta = (timelineData?.score_delta ?? 0) >= 0;

  const footerContent = (
    <>
      <div className="flex items-center gap-3 text-xs font-mono text-slate-400">
        <div className="flex items-center gap-1.5">
          <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 inline-block" />
          <span>DRY Health Score</span>
        </div>
        <div className="flex items-center gap-1.5">
          <span className="w-2.5 h-2.5 rounded-full bg-rose-400 inline-block" />
          <span>Duplication Rate %</span>
        </div>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </>
  );

  return (
    <Win2xWindow
      id="cddm-timeline-trends-window"
      windowType="timeline-trends"
      isOpen={isOpen}
      onClose={onClose}
      title="Historical Duplication & Git Timeline Evolution"
      subtitle="Time-series DRY Health trajectory, commit checkpoints, and Git hook quality gates"
      badge={
        timelineData
          ? `${snapshots.length} Snapshots (${isPositiveDelta ? "+" : ""}${timelineData.score_delta.toFixed(1)} DRY)`
          : undefined
      }
      icon={<History className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={920}
      initialHeight={680}
    >
      <div className="space-y-5">
        {/* Controls and Summary Cards */}
        <div className="flex flex-wrap items-center justify-between gap-3 bg-slate-900/60 p-3.5 rounded-xl border border-slate-800/80">
          <div className="flex items-center gap-3">
            <label className="flex items-center gap-2 text-xs text-slate-300 font-medium">
              <Sliders className="w-3.5 h-3.5 text-indigo-400" />
              <span>Sample Points:</span>
              <select
                value={maxSamples}
                onChange={(e) => {
                  const val = Number(e.target.value);
                  setMaxSamples(val);
                  void fetchTimeline(undefined, val);
                }}
                className="bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1 text-xs text-slate-200 focus:outline-none focus:border-indigo-500 font-mono"
              >
                <option value={5}>5 Commits</option>
                <option value={10}>10 Commits</option>
                <option value={20}>20 Commits</option>
                <option value={30}>30 Commits</option>
              </select>
            </label>

            <button
              type="button"
              onClick={handleRefresh}
              disabled={isTimelineLoading}
              className="flex items-center gap-1.5 px-3 py-1 bg-slate-800 hover:bg-slate-700 disabled:opacity-50 text-slate-200 rounded-lg text-xs font-semibold transition-colors"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isTimelineLoading ? "animate-spin" : ""}`} />
              <span>{isTimelineLoading ? "Sampling..." : "Resample"}</span>
            </button>
          </div>

          {timelineData && (
            <div className="flex items-center gap-4 text-xs font-mono">
              <div className="flex items-center gap-1.5">
                <span className="text-slate-400">Baseline:</span>
                <span className="text-slate-200 font-bold">
                  {timelineData.initial_score.toFixed(1)}
                </span>
              </div>
              <div className="flex items-center gap-1.5">
                <span className="text-slate-400">Current:</span>
                <span className="text-slate-200 font-bold">
                  {timelineData.current_score.toFixed(1)}
                </span>
              </div>
              <div
                className={`flex items-center gap-1 font-bold ${isPositiveDelta ? "text-emerald-400" : "text-rose-400"}`}
              >
                {isPositiveDelta ? (
                  <TrendingUp className="w-3.5 h-3.5" />
                ) : (
                  <TrendingDown className="w-3.5 h-3.5" />
                )}
                <span>
                  {isPositiveDelta ? "+" : ""}
                  {timelineData.score_delta.toFixed(1)} DRY
                </span>
              </div>
            </div>
          )}
        </div>

        {/* Error Alert */}
        {timelineError && (
          <div className="p-3.5 bg-rose-950/40 border border-rose-800/60 rounded-xl text-rose-300 text-xs flex items-center gap-2">
            <AlertTriangle className="w-4 h-4 flex-shrink-0 text-rose-400" />
            <span>{timelineError}</span>
          </div>
        )}

        {/* Interactive SVG Chart */}
        {hasData && (
          <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl relative overflow-hidden">
            <div className="flex items-center justify-between text-xs text-slate-400 mb-2 font-mono">
              <span>DRY Health & Duplication Trajectory</span>
              {hoveredSnapshot ? (
                <span className="text-indigo-300 font-semibold flex items-center gap-1.5">
                  <GitCommit className="w-3 h-3" />
                  <span>{hoveredSnapshot.short_hash}</span>
                  <span>— Score: {hoveredSnapshot.dry_health_score.toFixed(1)}</span>
                  <span>({hoveredSnapshot.duplication_percentage.toFixed(1)}% Dup)</span>
                </span>
              ) : (
                <span className="text-slate-500 italic">Hover over data points for details</span>
              )}
            </div>

            <div className="w-full overflow-x-auto">
              <svg viewBox={`0 0 ${svgWidth} ${svgHeight}`} className="w-full h-48 select-none">
                {/* Horizontal Grid lines */}
                {[0, 25, 50, 75, 100].map((val) => {
                  const y = getYScore(val);
                  return (
                    <g key={val}>
                      <line
                        x1={padLeft}
                        y1={y}
                        x2={svgWidth - padRight}
                        y2={y}
                        stroke="#334155"
                        strokeDasharray="3 3"
                        strokeWidth={0.7}
                      />
                      <text
                        x={padLeft - 8}
                        y={y + 3}
                        fill="#64748b"
                        fontSize={9}
                        textAnchor="end"
                        fontFamily="monospace"
                      >
                        {val}
                      </text>
                    </g>
                  );
                })}

                {/* Duplication Line */}
                <polyline
                  fill="none"
                  stroke="#f43f5e"
                  strokeWidth={2}
                  strokeDasharray="4 2"
                  points={dupPoints}
                />

                {/* DRY Health Score Line */}
                <polyline fill="none" stroke="#10b981" strokeWidth={2.5} points={dryPoints} />

                {/* Interactive Points */}
                {snapshots.map((s, idx) => {
                  const cx = getX(idx);
                  const cyScore = getYScore(s.dry_health_score);
                  const cyDup = getYDuplication(s.duplication_percentage);
                  const isHovered = hoveredSnapshot?.commit_hash === s.commit_hash;

                  return (
                    <g
                      key={s.commit_hash}
                      className="cursor-pointer transition-transform"
                      onMouseEnter={() => setHoveredSnapshot(s)}
                      onMouseLeave={() => setHoveredSnapshot(null)}
                    >
                      {/* Duplication point */}
                      <circle
                        cx={cx}
                        cy={cyDup}
                        r={isHovered ? 4.5 : 3}
                        fill="#f43f5e"
                        stroke="#0f172a"
                        strokeWidth={1.5}
                      />

                      {/* DRY Health score point */}
                      <circle
                        cx={cx}
                        cy={cyScore}
                        r={isHovered ? 6 : 4}
                        fill={isHovered ? "#34d399" : "#10b981"}
                        stroke="#0f172a"
                        strokeWidth={2}
                      />

                      {/* X-axis date / short hash label */}
                      <text
                        x={cx}
                        y={svgHeight - 10}
                        fill={isHovered ? "#e2e8f0" : "#64748b"}
                        fontSize={8.5}
                        textAnchor="middle"
                        fontFamily="monospace"
                      >
                        {s.short_hash}
                      </text>
                    </g>
                  );
                })}
              </svg>
            </div>
          </div>
        )}

        {/* Snapshot History Table */}
        <div className="space-y-2">
          <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-2">
            <GitCommit className="w-3.5 h-3.5 text-indigo-400" />
            <span>Sampled Commit Checkpoints</span>
          </h3>

          <div className="overflow-x-auto rounded-xl border border-slate-800 bg-slate-900/60 max-h-60 overflow-y-auto">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-slate-950 text-slate-400 border-b border-slate-800 sticky top-0">
                <tr>
                  <th className="py-2.5 px-3">Commit</th>
                  <th className="py-2.5 px-3">Date</th>
                  <th className="py-2.5 px-3">Author</th>
                  <th className="py-2.5 px-3">Message</th>
                  <th className="py-2.5 px-3 text-right">Files</th>
                  <th className="py-2.5 px-3 text-right">Clones</th>
                  <th className="py-2.5 px-3 text-right">Dup %</th>
                  <th className="py-2.5 px-3 text-right">DRY Score</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60 text-slate-300">
                {snapshots.map((s) => {
                  const isHealthy = s.dry_health_score >= 85.0;
                  return (
                    <tr
                      key={s.commit_hash}
                      className="hover:bg-slate-800/40 transition-colors"
                      onMouseEnter={() => setHoveredSnapshot(s)}
                      onMouseLeave={() => setHoveredSnapshot(null)}
                    >
                      <td className="py-2 px-3 font-semibold text-indigo-300 flex items-center gap-1.5">
                        <span>{s.short_hash}</span>
                        {s.tag && (
                          <span className="text-[10px] bg-purple-950 text-purple-300 px-1.5 py-0.2 rounded border border-purple-800/50 flex items-center gap-0.5">
                            <Tag className="w-2.5 h-2.5" />
                            {s.tag}
                          </span>
                        )}
                      </td>
                      <td className="py-2 px-3 text-slate-400 text-[11px] whitespace-nowrap">
                        {s.formatted_date}
                      </td>
                      <td className="py-2 px-3 text-slate-300 max-w-[120px] truncate">
                        {s.author}
                      </td>
                      <td
                        className="py-2 px-3 max-w-[200px] truncate text-slate-400"
                        title={s.message}
                      >
                        {s.message}
                      </td>
                      <td className="py-2 px-3 text-right text-slate-400">{s.total_files}</td>
                      <td className="py-2 px-3 text-right text-slate-400">{s.total_clones}</td>
                      <td className="py-2 px-3 text-right text-rose-300">
                        {s.duplication_percentage.toFixed(1)}%
                      </td>
                      <td className="py-2 px-3 text-right font-bold">
                        <span className={isHealthy ? "text-emerald-400" : "text-amber-400"}>
                          {s.dry_health_score.toFixed(1)}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>

        {/* Git Hook & CI/CD Enforcer Banner */}
        <div className="p-4 bg-gradient-to-r from-indigo-950/40 via-purple-950/20 to-slate-900 border border-indigo-900/40 rounded-xl flex flex-wrap items-center justify-between gap-3">
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-xs font-bold text-indigo-300">
              <ShieldCheck className="w-4 h-4 text-indigo-400" />
              <span>Automated Git Hook Quality Gate</span>
            </div>
            <p className="text-xs text-slate-400">
              Enforce maximum 15.0% code duplication threshold before every commit or pull request.
            </p>
            {hookInstallMessage && (
              <p className="text-xs text-emerald-400 font-mono mt-1">{hookInstallMessage}</p>
            )}
          </div>

          <div className="flex items-center gap-3">
            <div className="text-xs font-mono">
              <span className="text-slate-400">Pre-Commit: </span>
              <span
                className={`font-semibold ${
                  hookStatus?.pre_commit_installed ? "text-emerald-400" : "text-slate-500"
                }`}
              >
                {hookStatus?.pre_commit_installed ? "[ACTIVE]" : "[INACTIVE]"}
              </span>
            </div>

            {!hookStatus?.pre_commit_installed && (
              <button
                type="button"
                onClick={handleInstallPreCommit}
                disabled={isInstallingHook}
                className="px-3.5 py-1.5 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white rounded-lg text-xs font-semibold transition-colors shadow-sm"
              >
                {isInstallingHook ? "Installing..." : "Install Pre-Commit Hook"}
              </button>
            )}
          </div>
        </div>
      </div>
    </Win2xWindow>
  );
};
