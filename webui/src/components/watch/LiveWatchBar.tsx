import React, { useEffect, useState } from "react";
import { Activity, Pause, Play, RefreshCw } from "lucide-react";
import { useCDDMStore } from "../../store/cddm-store";
import styles from "./LiveWatchBar.module.css";

/**
 * Live Watch & Sync HUD component for real-time daemon control and delta display.
 */
export const LiveWatchBar: React.FC = () => {
  const {
    isLiveWatchActive,
    isScanning,
    liveSyncCount,
    lastLiveSyncTimestamp,
    lastWatchDelta,
    toggleWatch,
    triggerManualRescan,
    setIsLiveEventInspectorOpen,
    fetchWatchStatus,
  } = useCDDMStore();

  const [timeAgo, setTimeAgo] = useState<string>("");

  useEffect(() => {
    void fetchWatchStatus();
  }, [fetchWatchStatus]);

  useEffect(() => {
    if (!lastLiveSyncTimestamp) {
      setTimeAgo("");
      return;
    }

    const updateTimer = () => {
      const seconds = Math.floor((Date.now() - lastLiveSyncTimestamp) / 1000);
      if (seconds < 2) {
        setTimeAgo("just now");
      } else if (seconds < 60) {
        setTimeAgo(`${seconds}s ago`);
      } else {
        const mins = Math.floor(seconds / 60);
        setTimeAgo(`${mins}m ago`);
      }
    };

    updateTimer();
    const interval = setInterval(updateTimer, 3000);
    return () => clearInterval(interval);
  }, [lastLiveSyncTimestamp]);

  const pulseClass = isScanning
    ? styles.pulseScanning
    : isLiveWatchActive
      ? styles.pulseDot
      : styles.pulsePaused;

  return (
    <div className={styles.watchBarContainer}>
      {/* Active Watch Status Toggle */}
      <button
        type="button"
        onClick={() => void toggleWatch()}
        title={
          isLiveWatchActive
            ? "Live Watch Active: Click to pause daemon"
            : "Live Watch Paused: Click to resume daemon"
        }
        className={`${styles.statusButton} ${isLiveWatchActive ? styles.statusActive : ""}`}
      >
        <span className={pulseClass} />
        <span>
          {isScanning
            ? "Syncing..."
            : isLiveWatchActive
              ? liveSyncCount > 0
                ? `Live Sync (${liveSyncCount})`
                : "Live Watch: ON"
              : "Live Watch: OFF"}
        </span>
        {isLiveWatchActive ? (
          <Pause className="w-3 h-3 text-emerald-400/80 ml-0.5" />
        ) : (
          <Play className="w-3 h-3 text-slate-500 ml-0.5" />
        )}
      </button>

      {/* Score Delta Tag */}
      {lastWatchDelta && (
        <span
          className={
            lastWatchDelta.score_delta >= 0 ? styles.deltaBadgePositive : styles.deltaBadgeNegative
          }
          title={`Last sync scanned in ${lastWatchDelta.duration_ms}ms with ${lastWatchDelta.changed_files.length} changed file(s)`}
        >
          {lastWatchDelta.score_delta > 0
            ? `▲ +${lastWatchDelta.score_delta.toFixed(1)}%`
            : lastWatchDelta.score_delta < 0
              ? `▼ ${lastWatchDelta.score_delta.toFixed(1)}%`
              : "± 0.0%"}
        </span>
      )}

      {/* Sync Time Badge */}
      {timeAgo && isLiveWatchActive && (
        <span
          className="text-slate-400 font-mono text-[11px] hidden sm:inline"
          title="Time since last background scan"
        >
          {timeAgo}
        </span>
      )}

      {/* Manual Instant Rescan Button */}
      <button
        type="button"
        onClick={() => void triggerManualRescan()}
        disabled={isScanning}
        title="Trigger immediate workspace re-scan"
        className={styles.actionButton}
      >
        <RefreshCw className={`w-3.5 h-3.5 text-indigo-400 ${isScanning ? "animate-spin" : ""}`} />
        <span className="hidden md:inline">Sync Now</span>
      </button>

      {/* Event Inspector Modal Toggle */}
      <button
        type="button"
        onClick={() => setIsLiveEventInspectorOpen(true)}
        title="Open Live Sync Events & Timeline Inspector"
        className={styles.actionButton}
      >
        <Activity className="w-3.5 h-3.5 text-cyan-400" />
        <span className="hidden md:inline">Events</span>
      </button>
    </div>
  );
};
