import React from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Loader2 } from "lucide-react";

/**
 * Props for ScanProgressBar component.
 */
export interface ScanProgressBarProps {
  /** Optional CSS class name override */
  className?: string;
}

/**
 * Animated Scan Progress Bar component for CDDM WebUI.
 *
 * @param {ScanProgressBarProps} props - Component props
 */
export const ScanProgressBar: React.FC<ScanProgressBarProps> = ({ className = "" }) => {
  const { progress, isScanning } = useCDDMStore();

  if (!isScanning && !progress) return null;

  const percentage = Math.min(100, Math.max(0, Math.round((progress?.progress ?? 0) * 100)));
  const progressPercent = Math.min(100, Math.max(0, (progress?.progress ?? 0) * 100));

  return (
    <div className={`bg-gray-900 border border-gray-800 rounded-xl p-5 shadow-xl ${className}`}>
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2 text-indigo-400 font-medium text-sm">
          <Loader2 className="w-4 h-4 animate-spin text-indigo-500" />
          <span>Phase: {progress?.phase ?? "Initializing..."}</span>
        </div>
        <span className="text-xs font-mono font-bold bg-indigo-950 text-indigo-300 px-2 py-1 rounded">
          {percentage}%
        </span>
      </div>

      {/* Progress Bar Container */}
      <div className="w-full bg-gray-950 rounded-full h-2.5 overflow-hidden border border-gray-800">
        <div
          className="bg-gradient-to-r from-indigo-500 to-indigo-400 h-2.5 rounded-full transition-all duration-150 ease-out"
          style={{ width: `${progressPercent}%` }}
        />
      </div>

      <div className="flex justify-between text-xs text-gray-400 mt-2">
        <span>{progress?.message ?? "Preparing scanner engine..."}</span>
        {progress && (
          <span>
            {progress.files_processed} / {progress.total_files} files
          </span>
        )}
      </div>
    </div>
  );
};
