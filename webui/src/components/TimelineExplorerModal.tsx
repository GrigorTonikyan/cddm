import React, { useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  History,
  TrendingUp,
  TrendingDown,
  GitBranch,
  ShieldCheck,
  RefreshCw,
  Sliders,
  AlertTriangle,
} from "lucide-react";
import type { TimelineSnapshot } from "../types/cddm-types";
import { CommitEvolutionChart } from "./timeline/CommitEvolutionChart";
import { CommitHistoryTable } from "./timeline/CommitHistoryTable";
import { BranchDriftMatrixSection } from "./timeline/BranchDriftMatrixSection";

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

  const [activeTab, setActiveTab] = useState<"timeline" | "matrix">("timeline");
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
      subtitle="Time-series DRY Health trajectory, commit checkpoints, and cross-branch drift matrix"
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
        {/* Sub-tab Navigation */}
        <div className="flex items-center gap-2 border-b border-slate-800 pb-2.5">
          <button
            type="button"
            onClick={() => setActiveTab("timeline")}
            className={`px-3.5 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center gap-1.5 ${
              activeTab === "timeline"
                ? "bg-indigo-600 text-white shadow-sm"
                : "bg-slate-900 text-slate-400 hover:text-slate-200"
            }`}
          >
            <History className="w-3.5 h-3.5" />
            <span>Commit Evolution Timeline</span>
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("matrix")}
            className={`px-3.5 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center gap-1.5 ${
              activeTab === "matrix"
                ? "bg-indigo-600 text-white shadow-sm"
                : "bg-slate-900 text-slate-400 hover:text-slate-200"
            }`}
          >
            <GitBranch className="w-3.5 h-3.5" />
            <span>Multi-Branch Drift Matrix</span>
          </button>
        </div>

        {activeTab === "matrix" ? (
          <BranchDriftMatrixSection />
        ) : (
          <>
            {/* Controls and Summary Cards */}
            <div className="flex flex-wrap items-center justify-between gap-3 bg-slate-900/60 p-3.5 rounded-xl border border-slate-800/80">
              <div className="flex items-center gap-3">
                <label
                  htmlFor="timeline-sample-points"
                  className="flex items-center gap-2 text-xs text-slate-300 font-medium"
                >
                  <Sliders className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Sample Points:</span>
                  <select
                    id="timeline-sample-points"
                    name="sample_points"
                    aria-label="Timeline History Sample Points"
                    value={maxSamples}
                    onChange={(e) => {
                      const val = Number(e.target.value);
                      setMaxSamples(val);
                      void fetchTimeline(undefined, val);
                    }}
                    className="bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1 text-xs text-slate-200 focus:outline-none focus:border-indigo-500 font-mono cursor-pointer"
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
                    className={`flex items-center gap-1 font-bold ${
                      isPositiveDelta ? "text-emerald-400" : "text-rose-400"
                    }`}
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
              <CommitEvolutionChart
                snapshots={snapshots}
                timelineData={timelineData}
                hoveredSnapshot={hoveredSnapshot}
                setHoveredSnapshot={setHoveredSnapshot}
              />
            )}

            {/* Snapshot History Table */}
            <CommitHistoryTable
              snapshots={snapshots}
              hoveredSnapshot={hoveredSnapshot}
              setHoveredSnapshot={setHoveredSnapshot}
            />

            {/* Git Hook & CI/CD Enforcer Banner */}
            <div className="p-4 bg-gradient-to-r from-indigo-950/40 via-purple-950/20 to-slate-900 border border-indigo-900/40 rounded-xl flex flex-wrap items-center justify-between gap-3">
              <div className="space-y-1">
                <div className="flex items-center gap-2 text-xs font-bold text-indigo-300">
                  <ShieldCheck className="w-4 h-4 text-indigo-400" />
                  <span>Automated Git Hook Quality Gate</span>
                </div>
                <p className="text-xs text-slate-400">
                  Enforce maximum 15.0% code duplication threshold before every commit or pull
                  request.
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
          </>
        )}
      </div>
    </Win2xWindow>
  );
};
