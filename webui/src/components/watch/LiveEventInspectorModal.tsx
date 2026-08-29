import React from "react";
import {
  Activity,
  CheckCircle2,
  Clock,
  FileCode,
  FolderSync,
  Pause,
  Play,
  RefreshCw,
  Trash2,
  TrendingDown,
  TrendingUp,
} from "lucide-react";
import { useCDDMStore } from "../../store/cddm-store";
import { Win2xWindow } from "../ui/win2x-manager";

export interface LiveEventInspectorModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const LiveEventInspectorModal: React.FC<LiveEventInspectorModalProps> = ({
  isOpen,
  onClose,
}) => {
  const {
    isLiveWatchActive,
    isScanning,
    liveSyncCount,
    lastLiveSyncTimestamp,
    watchEventsLog,
    toggleWatch,
    triggerManualRescan,
    clearWatchEventsLog,
    config,
  } = useCDDMStore();

  if (!isOpen) return null;

  return (
    <Win2xWindow
      id="cddm-live-event-inspector-modal"
      windowType="live-watch-inspector"
      title="Live Watch & Real-Time Sync Inspector"
      icon={<Activity className="w-4 h-4 text-cyan-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={780}
      initialHeight={580}
      minWidth={540}
      minHeight={400}
    >
      <div className="p-5 flex flex-col h-full gap-4 text-slate-200 overflow-hidden font-sans">
        {/* Top Header Metrics Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-4 gap-3 text-xs font-mono">
          <div className="p-3 bg-slate-900/90 border border-slate-800 rounded-xl flex flex-col justify-between">
            <span className="text-slate-400 font-semibold flex items-center gap-1.5">
              <FolderSync className="w-3.5 h-3.5 text-indigo-400" />
              Daemon State
            </span>
            <div className="mt-2 flex items-center justify-between">
              <span
                className={`px-2 py-0.5 rounded text-xs font-bold ${
                  isLiveWatchActive
                    ? "bg-emerald-950/80 text-emerald-300 border border-emerald-800/60"
                    : "bg-slate-950 text-slate-400 border border-slate-800"
                }`}
              >
                {isLiveWatchActive ? "Active" : "Paused"}
              </span>
              <button
                type="button"
                onClick={() => void toggleWatch()}
                className="p-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 transition-colors"
                title={isLiveWatchActive ? "Pause watch daemon" : "Resume watch daemon"}
              >
                {isLiveWatchActive ? <Pause className="w-3 h-3" /> : <Play className="w-3 h-3" />}
              </button>
            </div>
          </div>

          <div className="p-3 bg-slate-900/90 border border-slate-800 rounded-xl flex flex-col justify-between">
            <span className="text-slate-400 font-semibold flex items-center gap-1.5">
              <Clock className="w-3.5 h-3.5 text-cyan-400" />
              Total Syncs
            </span>
            <span className="text-lg font-extrabold text-white mt-1">{liveSyncCount}</span>
          </div>

          <div className="p-3 bg-slate-900/90 border border-slate-800 rounded-xl flex flex-col justify-between">
            <span className="text-slate-400 font-semibold flex items-center gap-1.5">
              <Activity className="w-3.5 h-3.5 text-purple-400" />
              Watch Directory
            </span>
            <span
              className="text-xs font-mono text-slate-300 truncate mt-1"
              title={config.directory}
            >
              {config.directory || "."}
            </span>
          </div>

          <div className="p-3 bg-slate-900/90 border border-slate-800 rounded-xl flex flex-col justify-between">
            <span className="text-slate-400 font-semibold flex items-center gap-1.5">
              <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
              Last Sync
            </span>
            <span className="text-xs text-slate-300 mt-1">
              {lastLiveSyncTimestamp
                ? new Date(lastLiveSyncTimestamp).toLocaleTimeString()
                : "None"}
            </span>
          </div>
        </div>

        {/* Action Controls Toolbar */}
        <div className="flex items-center justify-between gap-2 pt-1 border-b border-slate-800 pb-3">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => void triggerManualRescan()}
              disabled={isScanning}
              className="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white rounded-lg text-xs font-medium flex items-center gap-1.5 transition-colors shadow-sm"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isScanning ? "animate-spin" : ""}`} />
              <span>{isScanning ? "Scanning Workspace..." : "Trigger Manual Sync"}</span>
            </button>

            <button
              type="button"
              onClick={clearWatchEventsLog}
              disabled={watchEventsLog.length === 0}
              className="px-3 py-1.5 bg-slate-900 hover:bg-slate-800 disabled:opacity-40 text-slate-300 border border-slate-800 rounded-lg text-xs font-medium flex items-center gap-1.5 transition-colors"
            >
              <Trash2 className="w-3.5 h-3.5 text-slate-400" />
              <span>Clear History</span>
            </button>
          </div>

          <span className="text-xs text-slate-400 font-mono">
            {watchEventsLog.length} event(s) recorded
          </span>
        </div>

        {/* Event Timeline List */}
        <div className="flex-1 overflow-y-auto space-y-2 pr-1 custom-scrollbar">
          {watchEventsLog.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-48 text-slate-500 gap-2 border border-dashed border-slate-800/80 rounded-xl bg-slate-900/30">
              <FolderSync className="w-8 h-8 text-slate-600 animate-pulse" />
              <p className="text-sm font-medium">Listening for workspace file changes...</p>
              <p className="text-xs text-slate-500">
                Save any code file in your IDE to observe instantaneous incremental re-scans.
              </p>
            </div>
          ) : (
            watchEventsLog.map((event, idx) => (
              <div
                key={`${event.timestamp_millis}-${idx}`}
                className="p-3.5 bg-slate-900/70 hover:bg-slate-900 border border-slate-800/80 rounded-xl flex flex-col gap-2 transition-colors"
              >
                <div className="flex items-center justify-between gap-2 text-xs">
                  <div className="flex items-center gap-2 font-mono">
                    <span className="px-2 py-0.5 bg-slate-800 text-slate-300 rounded font-semibold">
                      {new Date(event.timestamp_millis).toLocaleTimeString()}
                    </span>
                    <span className="text-slate-400">
                      Scanned in <strong className="text-slate-200">{event.duration_ms}ms</strong>
                    </span>
                  </div>

                  <div className="flex items-center gap-2 font-mono font-semibold">
                    <span
                      className={`px-2 py-0.5 rounded flex items-center gap-1 ${
                        event.score_delta > 0
                          ? "bg-emerald-950/80 text-emerald-300 border border-emerald-800/60"
                          : event.score_delta < 0
                            ? "bg-rose-950/80 text-rose-300 border border-rose-800/60"
                            : "bg-slate-800 text-slate-400"
                      }`}
                    >
                      {event.score_delta > 0 ? (
                        <TrendingUp className="w-3 h-3" />
                      ) : event.score_delta < 0 ? (
                        <TrendingDown className="w-3 h-3" />
                      ) : null}
                      DRY: {event.new_health_score.toFixed(1)}% (
                      {event.score_delta > 0
                        ? `+${event.score_delta.toFixed(1)}%`
                        : `${event.score_delta.toFixed(1)}%`}
                      )
                    </span>

                    <span className="px-2 py-0.5 bg-slate-800 text-slate-300 rounded">
                      Clones: {event.new_clones} (
                      {event.clone_count_delta >= 0
                        ? `+${event.clone_count_delta}`
                        : event.clone_count_delta}
                      )
                    </span>
                  </div>
                </div>

                {/* Changed Files List */}
                {event.changed_files.length > 0 && (
                  <div className="flex flex-wrap items-center gap-1.5 pt-1">
                    {event.changed_files.map((file, fIdx) => (
                      <span
                        key={`${file}-${fIdx}`}
                        className="inline-flex items-center gap-1 px-2 py-0.5 bg-slate-950/80 border border-slate-800 text-slate-300 rounded text-xs font-mono"
                        title={file}
                      >
                        <FileCode className="w-3 h-3 text-indigo-400" />
                        <span className="truncate max-w-70">{file}</span>
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))
          )}
        </div>
      </div>
    </Win2xWindow>
  );
};
