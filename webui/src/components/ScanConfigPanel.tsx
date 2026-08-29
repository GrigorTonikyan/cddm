import {
  AppWindow,
  Code,
  Folder,
  GitBranch,
  Layers,
  Play,
  RotateCcw,
  Shield,
  SlidersHorizontal,
  Sparkles,
} from "lucide-react";
import React from "react";
import { useCDDMStore } from "../store/cddm-store";
import { SUPPORTED_EDITORS, SupportedEditor } from "../utils/ide-links";

export interface ScanConfigPanelProps {
  className?: string;
}

export const ScanConfigPanel: React.FC<ScanConfigPanelProps> = ({ className = "" }) => {
  const {
    config,
    setConfig,
    preferredEditor,
    setPreferredEditor,
    startScan,
    isScanning,
    resetScan,
  } = useCDDMStore();

  return (
    <div
      className={`bg-slate-900/80 border border-slate-800/80 rounded-xl p-6 shadow-xl backdrop-blur-md ${className}`}
    >
      <div className="flex items-center justify-between mb-6 border-b border-slate-800/80 pb-4">
        <div className="flex items-center gap-2 text-indigo-400 font-bold text-lg">
          <SlidersHorizontal className="w-5 h-5" />
          <span>Scan Configuration</span>
        </div>
        <span className="text-xs font-mono text-slate-400">CDDM Polyglot Engine</span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Directory Input */}
        <div>
          <label className="text-xs font-bold uppercase tracking-wider text-slate-300 mb-2 flex items-center gap-2">
            <Folder className="w-4 h-4 text-indigo-400" />
            Target Repository Directory
          </label>
          <input
            type="text"
            value={config.directory}
            onChange={(e) => setConfig({ directory: e.target.value })}
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-4 py-2.5 text-sm font-mono text-slate-100 focus:outline-none focus:border-indigo-500 transition-colors shadow-inner"
            placeholder="e.g. ./src or /path/to/repo"
          />
        </div>

        {/* Minimum Tokens Slider */}
        <div>
          <label className="text-xs font-bold uppercase tracking-wider text-slate-300 mb-2 flex items-center justify-between">
            <span className="flex items-center gap-2">
              <Code className="w-4 h-4 text-indigo-400" />
              Minimum Token Threshold
            </span>
            <span className="text-xs bg-indigo-950 text-indigo-300 border border-indigo-800/50 px-2.5 py-0.5 rounded font-mono font-bold">
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
            className="w-full h-2 bg-slate-950 rounded-lg appearance-none cursor-pointer accent-indigo-500 border border-slate-800"
          />
          <div className="flex justify-between text-[11px] font-mono text-slate-400 mt-1">
            <span>10 (Aggressive)</span>
            <span>50 (Recommended)</span>
            <span>200 (Strict)</span>
          </div>
        </div>

        {/* Ignore Patterns */}
        <div>
          <label className="text-xs font-bold uppercase tracking-wider text-slate-300 mb-2 flex items-center gap-2">
            <Shield className="w-4 h-4 text-indigo-400" />
            Ignore Patterns (comma-separated)
          </label>
          <input
            type="text"
            value={config.ignore_patterns.join(", ")}
            onChange={(e) =>
              setConfig({
                ignore_patterns: e.target.value
                  .split(",")
                  .map((s) => s.trim())
                  .filter(Boolean),
              })
            }
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-4 py-2.5 text-sm font-mono text-slate-100 focus:outline-none focus:border-indigo-500 transition-colors shadow-inner"
            placeholder="node_modules, target, .git, dist"
          />
        </div>

        {/* Preferred IDE Editor Deeplinks */}
        <div>
          <label className="text-xs font-bold uppercase tracking-wider text-slate-300 mb-2 flex items-center gap-2">
            <AppWindow className="w-4 h-4 text-indigo-400" />
            Preferred IDE Deeplink Target
          </label>
          <select
            value={preferredEditor}
            onChange={(e) => setPreferredEditor(e.target.value as SupportedEditor)}
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-4 py-2.5 text-sm font-mono text-slate-100 focus:outline-none focus:border-indigo-500 transition-colors shadow-inner cursor-pointer"
          >
            {SUPPORTED_EDITORS.map((editor) => (
              <option key={editor.id} value={editor.id}>
                {editor.name} ({editor.scheme}://)
              </option>
            ))}
          </select>
        </div>

        {/* Checkbox Toggles */}
        <div className="flex flex-wrap items-center gap-6 pt-2 md:col-span-2">
          <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.detect_type2}
              onChange={(e) => setConfig({ detect_type2: e.target.checked })}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-slate-900"
            />
            <span>Type-2 (Renamed Clones)</span>
          </label>

          <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.detect_type3 ?? true}
              onChange={(e) => setConfig({ detect_type3: e.target.checked })}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-slate-900"
            />
            <span className="flex items-center gap-1 text-amber-300">
              <Layers className="w-3.5 h-3.5 text-amber-400" />
              Type-3 (Near-Miss Clones)
            </span>
          </label>

          <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.scan_self}
              onChange={(e) => setConfig({ scan_self: e.target.checked })}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-slate-900"
            />
            <span>Intra-file Duplication</span>
          </label>

          <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.enable_git_blame ?? false}
              onChange={(e) => setConfig({ enable_git_blame: e.target.checked })}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-slate-900"
            />
            <span className="flex items-center gap-1">
              <GitBranch className="w-3.5 h-3.5 text-indigo-400" />
              Git Blame (Authors)
            </span>
          </label>

          <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={config.cross_language ?? false}
              onChange={(e) => setConfig({ cross_language: e.target.checked })}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-slate-900"
            />
            <span className="flex items-center gap-1 text-purple-300">
              <Sparkles className="w-3.5 h-3.5 text-purple-400" />
              Cross-Language (Type-4)
            </span>
          </label>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex items-center gap-4 mt-6 pt-4 border-t border-slate-800/80">
        <button
          type="button"
          onClick={() => void startScan()}
          disabled={isScanning}
          className="flex-1 bg-linear-to-r from-indigo-600 via-indigo-500 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-semibold py-3 px-6 rounded-lg transition-all shadow-lg flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.99] cursor-pointer"
        >
          <Play className="w-4 h-4 fill-current" />
          <span>{isScanning ? "Scanning Codebase..." : "Run Duplicate Analysis"}</span>
        </button>

        <button
          type="button"
          onClick={resetScan}
          disabled={isScanning}
          className="bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 font-medium py-3 px-5 rounded-lg transition-colors flex items-center gap-2 disabled:opacity-50 cursor-pointer"
        >
          <RotateCcw className="w-4 h-4" />
          <span>Reset</span>
        </button>
      </div>
    </div>
  );
};
