import React from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Play, RotateCcw, Sliders, Folder, Shield, Code } from "lucide-react";

/**
 * Props for ScanConfigPanel component.
 */
export interface ScanConfigPanelProps {
  /** Optional CSS class name override */
  className?: string;
}

/**
 * Interactive Scan Configuration Panel component for CDDM WebUI.
 *
 * @param {ScanConfigPanelProps} props - Component props
 */
export const ScanConfigPanel: React.FC<ScanConfigPanelProps> = ({ className = "" }) => {
  const { config, setConfig, startScan, isScanning, resetScan } = useCDDMStore();

  return (
    <div className={`bg-gray-900 border border-gray-800 rounded-xl p-6 shadow-xl ${className}`}>
      <div className="flex items-center gap-2 mb-6 text-indigo-400 font-semibold text-lg border-b border-gray-800 pb-3">
        <Sliders className="w-5 h-5" />
        <span>Scan Configuration</span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Directory Input */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2 flex items-center gap-2">
            <Folder className="w-4 h-4 text-indigo-400" />
            Target Directory
          </label>
          <input
            type="text"
            value={config.directory}
            onChange={(e) => setConfig({ directory: e.target.value })}
            className="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-200 focus:outline-none focus:border-indigo-500 transition-colors"
            placeholder="e.g. ./src or /path/to/repo"
          />
        </div>

        {/* Minimum Tokens Slider */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2 flex items-center justify-between">
            <span className="flex items-center gap-2">
              <Code className="w-4 h-4 text-indigo-400" />
              Minimum Token Threshold
            </span>
            <span className="text-xs bg-indigo-950 text-indigo-300 px-2 py-0.5 rounded font-mono font-bold">
              {config.min_tokens} tokens
            </span>
          </label>
          <input
            type="range"
            min="10"
            max="200"
            step="5"
            value={config.min_tokens}
            onChange={(e) => setConfig({ min_tokens: Number(e.target.value) })}
            className="w-full h-2 bg-gray-950 rounded-lg appearance-none cursor-pointer accent-indigo-500"
          />
          <div className="flex justify-between text-xs text-gray-500 mt-1">
            <span>10 (Aggressive)</span>
            <span>50 (Recommended)</span>
            <span>200 (Strict)</span>
          </div>
        </div>

        {/* Ignore Patterns */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2 flex items-center gap-2">
            <Shield className="w-4 h-4 text-indigo-400" />
            Ignore Patterns (comma-separated)
          </label>
          <input
            type="text"
            value={config.ignore_patterns.join(", ")}
            onChange={(e) =>
              setConfig({
                ignore_patterns: e.target.value.split(",").map((s) => s.trim()).filter(Boolean),
              })
            }
            className="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-200 focus:outline-none focus:border-indigo-500 transition-colors"
            placeholder="node_modules, target, .git, dist"
          />
        </div>

        {/* Toggles */}
        <div className="flex items-center gap-6 pt-6">
          <label className="flex items-center gap-2 text-sm text-gray-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.detect_type2}
              onChange={(e) => setConfig({ detect_type2: e.target.checked })}
              className="w-4 h-4 rounded border-gray-800 bg-gray-950 text-indigo-500 focus:ring-indigo-500"
            />
            <span>Type-2 (Renamed Clones)</span>
          </label>

          <label className="flex items-center gap-2 text-sm text-gray-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.scan_self}
              onChange={(e) => setConfig({ scan_self: e.target.checked })}
              className="w-4 h-4 rounded border-gray-800 bg-gray-950 text-indigo-500 focus:ring-indigo-500"
            />
            <span>Intra-file Duplication</span>
          </label>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex items-center gap-4 mt-8 pt-4 border-t border-gray-800">
        <button
          onClick={() => void startScan()}
          disabled={isScanning}
          className="flex-1 bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white font-medium py-3 px-6 rounded-lg transition-all shadow-lg flex items-center justify-center gap-2 disabled:opacity-50"
        >
          <Play className="w-5 h-5 fill-current" />
          <span>{isScanning ? "Scanning Codebase..." : "Run Duplicate Analysis"}</span>
        </button>

        <button
          onClick={resetScan}
          disabled={isScanning}
          className="bg-gray-800 hover:bg-gray-700 text-gray-300 font-medium py-3 px-4 rounded-lg transition-colors flex items-center gap-2 disabled:opacity-50"
        >
          <RotateCcw className="w-4 h-4" />
          <span>Reset</span>
        </button>
      </div>
    </div>
  );
};
