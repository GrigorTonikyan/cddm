import React, { useState } from "react";
import type {
  ExtractRequest,
  ExtractResult,
  ExtractTargetKind,
  RefactorSandboxRequest,
} from "../../types/cddm-types";
import {
  AlertCircle,
  Box,
  Check,
  FileCode,
  FilePlus,
  FlaskConical,
  FolderGit2,
  Gauge,
  Play,
  RefreshCw,
  TrendingDown,
} from "lucide-react";

export interface ExtractModuleTabProps {
  sandboxRequest: RefactorSandboxRequest | null;
  extractResult: ExtractResult | null;
  isExtractLoading: boolean;
  extractError: string | null;
  onPreview: (req: ExtractRequest) => Promise<ExtractResult>;
  onApply: (req: ExtractRequest) => Promise<ExtractResult>;
}

interface FilePreviewContainerProps {
  label: string;
  icon: React.ReactNode;
  files: { file_path: string; content: string }[];
  activeTab: number;
  onSelectTab: (index: number) => void;
  borderClass: string;
  headerBg: string;
  activeBtnClass: string;
  inactiveBtnClass: string;
  codeColor: string;
}

const FilePreviewContainer: React.FC<FilePreviewContainerProps> = ({
  label,
  icon,
  files,
  activeTab,
  onSelectTab,
  borderClass,
  headerBg,
  activeBtnClass,
  inactiveBtnClass,
  codeColor,
}) => {
  if (files.length === 0) return null;
  return (
    <div className={`border rounded-xl bg-slate-950 overflow-hidden ${borderClass}`}>
      <div className={`flex items-center gap-1 px-2.5 py-1.5 border-b overflow-x-auto ${headerBg}`}>
        <span className="text-[11px] font-bold mr-2 flex items-center gap-1">
          {icon}
          {label} ({files.length}):
        </span>
        {files.map((file, idx) => (
          <button
            key={idx}
            type="button"
            onClick={() => onSelectTab(idx)}
            className={`px-2.5 py-1 rounded text-[11px] font-mono transition-colors ${
              activeTab === idx ? activeBtnClass : inactiveBtnClass
            }`}
          >
            {file.file_path}
          </button>
        ))}
      </div>
      {files[activeTab] && (
        <pre
          className={`p-3.5 text-xs font-mono overflow-x-auto max-h-[180px] leading-relaxed ${codeColor}`}
        >
          {files[activeTab].content}
        </pre>
      )}
    </div>
  );
};

export const ExtractModuleTab: React.FC<ExtractModuleTabProps> = ({
  sandboxRequest,
  extractResult,
  isExtractLoading,
  extractError,
  onPreview,
  onApply,
}) => {
  const [form, setForm] = useState({
    targetPath: "crates/shared_utils",
    customFnName: "",
    strategy: "auto" as ExtractTargetKind,
    generateTests: true,
    generateBenchmarks: true,
  });
  const [activeFileTab, setActiveFileTab] = useState<number>(0);
  const [activeTestTab, setActiveTestTab] = useState<number>(0);
  const [activeBenchTab, setActiveBenchTab] = useState<number>(0);
  const [isApplying, setIsApplying] = useState<boolean>(false);
  const [appliedSuccess, setAppliedSuccess] = useState<string | null>(null);

  const updateField = <K extends keyof typeof form>(key: K, value: (typeof form)[K]) => {
    setForm((prev) => ({ ...prev, [key]: value }));
  };

  const buildExtractRequest = (dryRun: boolean): ExtractRequest | null => {
    if (!sandboxRequest) return null;
    return {
      occurrences: sandboxRequest.occurrences,
      target_path: form.targetPath.trim() || "crates/shared_utils",
      custom_function_name: form.customFnName.trim() || undefined,
      target_kind: form.strategy,
      generate_tests: form.generateTests,
      generate_benchmarks: form.generateBenchmarks,
      dry_run: dryRun,
    };
  };

  const handleGeneratePreview = async () => {
    const req = buildExtractRequest(true);
    if (!req) return;
    setAppliedSuccess(null);
    try {
      await onPreview(req);
    } catch {
      // Handled in store
    }
  };

  const handleApplyToWorkspace = async () => {
    const req = buildExtractRequest(false);
    if (!req) return;
    setIsApplying(true);
    setAppliedSuccess(null);
    try {
      const res = await onApply(req);
      setAppliedSuccess(res.message);
    } catch {
      // Handled in store
    } finally {
      setIsApplying(false);
    }
  };

  return (
    <div className="space-y-4 font-mono text-xs text-slate-300">
      {/* Configuration Header Card */}
      <div className="p-3.5 bg-slate-900/90 border border-cyan-800/40 rounded-xl space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-cyan-300 font-bold text-xs flex items-center gap-2">
            <Box className="w-4 h-4 text-cyan-400" />
            Automated Shared Crate &amp; Module Extraction
          </span>
          <span className="text-slate-400 text-[11px]">
            Occurrences: {sandboxRequest?.occurrences.length || 0}
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-2.5 pt-1">
          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Target Path / Name</label>
            <input
              type="text"
              aria-label="Target Path"
              value={form.targetPath}
              onChange={(e) => updateField("targetPath", e.target.value)}
              placeholder="e.g. crates/shared_utils or src/common/utils.rs"
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            />
          </div>

          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Custom Function Name</label>
            <input
              type="text"
              aria-label="Function Name"
              value={form.customFnName}
              onChange={(e) => updateField("customFnName", e.target.value)}
              placeholder="e.g. compute_shared_total"
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            />
          </div>

          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Packaging Strategy</label>
            <select
              aria-label="Packaging Strategy"
              value={form.strategy}
              onChange={(e) => updateField("strategy", e.target.value as ExtractTargetKind)}
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            >
              <option value="auto">Auto-Detect</option>
              <option value="new_crate">New Standalone Crate/Package</option>
              <option value="new_module">New Shared Module File</option>
              <option value="existing_module">Existing Module</option>
            </select>
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-4 pt-0.5">
          <label className="flex items-center gap-2 text-[11px] text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              aria-label="Generate Unit Tests"
              checked={form.generateTests}
              onChange={(e) => updateField("generateTests", e.target.checked)}
              className="rounded bg-slate-950 border-slate-700 text-emerald-500 focus:ring-0 focus:outline-none"
            />
            <span className="flex items-center gap-1 text-emerald-400">
              <FlaskConical className="w-3.5 h-3.5" />
              Synthesize Idiomatic Unit Tests (*.test.ts, *_test.rs, test_*.py, etc.)
            </span>
          </label>

          <label className="flex items-center gap-2 text-[11px] text-slate-300 cursor-pointer select-none">
            <input
              type="checkbox"
              aria-label="Generate Micro-Benchmarks"
              checked={form.generateBenchmarks}
              onChange={(e) => updateField("generateBenchmarks", e.target.checked)}
              className="rounded bg-slate-950 border-slate-700 text-purple-500 focus:ring-0 focus:outline-none"
            />
            <span className="flex items-center gap-1 text-purple-400">
              <Gauge className="w-3.5 h-3.5" />
              Synthesize Performance Micro-Benchmarks (*.bench.ts, *_bench.rs, etc.)
            </span>
          </label>
        </div>

        <div className="flex items-center justify-between pt-1 border-t border-slate-800/80">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={handleGeneratePreview}
              disabled={isExtractLoading || !sandboxRequest}
              className="px-3 py-1.5 bg-cyan-900/60 hover:bg-cyan-800/70 text-cyan-200 border border-cyan-700/60 rounded-lg font-medium text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
            >
              <Play className="w-3.5 h-3.5" />
              <span>Preview Extraction Plan</span>
            </button>

            {extractResult && (
              <button
                type="button"
                onClick={handleApplyToWorkspace}
                disabled={isApplying || isExtractLoading}
                className="px-3 py-1.5 bg-emerald-900/60 hover:bg-emerald-800/70 text-emerald-200 border border-emerald-700/60 rounded-lg font-medium text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
              >
                {isApplying ? (
                  <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                ) : (
                  <Check className="w-3.5 h-3.5" />
                )}
                <span>Apply to Workspace</span>
              </button>
            )}
          </div>

          {extractResult && (
            <div className="flex items-center gap-2 text-[11px] text-slate-400 font-mono">
              <span className="px-2 py-0.5 bg-emerald-950/60 border border-emerald-800/60 text-emerald-300 rounded flex items-center gap-1">
                <TrendingDown className="w-3 h-3 text-emerald-400" />~
                {extractResult.total_lines_saved} lines saved
              </span>
              <span className="px-2 py-0.5 bg-slate-950 border border-slate-800 text-slate-300 rounded">
                Strategy: {extractResult.target_kind}
              </span>
            </div>
          )}
        </div>
      </div>

      {appliedSuccess && (
        <div className="p-3 bg-emerald-950/40 border border-emerald-800/60 rounded-xl text-emerald-300 text-xs flex items-center gap-2">
          <Check className="w-4 h-4 text-emerald-400 flex-shrink-0" />
          <span>{appliedSuccess}</span>
        </div>
      )}

      {isExtractLoading ? (
        <div className="py-16 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs bg-slate-950/60 border border-slate-800 rounded-xl">
          <RefreshCw className="w-6 h-6 animate-spin text-cyan-400" />
          <span>Synthesizing shared crate structure and manifest updates...</span>
        </div>
      ) : extractError ? (
        <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-rose-400 flex-shrink-0" />
          <span>{extractError}</span>
        </div>
      ) : extractResult ? (
        <div className="space-y-3">
          {/* Target Generated Files Tabs */}
          <FilePreviewContainer
            label="Generated Files"
            icon={<FilePlus className="w-3.5 h-3.5 text-cyan-400" />}
            files={extractResult.generated_files}
            activeTab={activeFileTab}
            onSelectTab={setActiveFileTab}
            borderClass="border-slate-800"
            headerBg="bg-slate-900/80 border-slate-800 text-slate-400"
            activeBtnClass="bg-cyan-950/80 text-cyan-300 border border-cyan-800/60 font-semibold"
            inactiveBtnClass="text-slate-400 hover:text-slate-200 hover:bg-slate-800/40"
            codeColor="text-slate-200"
          />

          {/* Synthesized Unit Tests Tabs */}
          <FilePreviewContainer
            label="Synthesized Unit Tests"
            icon={<FlaskConical className="w-3.5 h-3.5 text-emerald-400" />}
            files={extractResult.test_files || []}
            activeTab={activeTestTab}
            onSelectTab={setActiveTestTab}
            borderClass="border-emerald-900/60"
            headerBg="bg-emerald-950/40 border-emerald-900/60 text-emerald-300"
            activeBtnClass="bg-emerald-900/80 text-emerald-200 border border-emerald-700/60 font-semibold"
            inactiveBtnClass="text-slate-400 hover:text-emerald-300 hover:bg-emerald-950/30"
            codeColor="text-emerald-200"
          />

          {/* Synthesized Micro-Benchmarks Tabs */}
          <FilePreviewContainer
            label="Synthesized Micro-Benchmarks"
            icon={<Gauge className="w-3.5 h-3.5 text-purple-400" />}
            files={extractResult.benchmark_files || []}
            activeTab={activeBenchTab}
            onSelectTab={setActiveBenchTab}
            borderClass="border-purple-900/60"
            headerBg="bg-purple-950/40 border-purple-900/60 text-purple-300"
            activeBtnClass="bg-purple-900/80 text-purple-200 border border-purple-700/60 font-semibold"
            inactiveBtnClass="text-slate-400 hover:text-purple-300 hover:bg-purple-950/30"
            codeColor="text-purple-200"
          />

          {/* Manifest Updates Preview */}
          {extractResult.manifest_updates.length > 0 && (
            <div className="space-y-2">
              <span className="text-slate-300 font-semibold text-xs flex items-center gap-1.5">
                <FolderGit2 className="w-3.5 h-3.5 text-amber-400" />
                Manifest Updates ({extractResult.manifest_updates.length})
              </span>
              <div className="space-y-2">
                {extractResult.manifest_updates.map((mu, muIdx) => (
                  <div
                    key={muIdx}
                    className="p-3 bg-slate-900/60 border border-amber-900/40 rounded-xl space-y-1.5"
                  >
                    <div className="flex items-center justify-between text-slate-200 text-xs">
                      <span className="font-semibold text-amber-300">{mu.manifest_path}</span>
                      <span className="text-slate-400 text-[10px] font-mono">
                        Added Dependency: {mu.dependency_name}
                      </span>
                    </div>
                    <pre className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-[11px] font-mono text-emerald-300 overflow-x-auto">
                      {mu.diff_preview}
                    </pre>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Caller File Rewrites */}
          {extractResult.caller_rewrites.length > 0 && (
            <div className="space-y-2">
              <span className="text-slate-300 font-semibold text-xs flex items-center gap-1.5">
                <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                Occurrence Caller Rewrites ({extractResult.caller_rewrites.length})
              </span>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                {extractResult.caller_rewrites.map((cr, crIdx) => (
                  <div
                    key={crIdx}
                    className="p-2.5 bg-slate-900/60 border border-slate-800 rounded-lg space-y-1 text-[11px]"
                  >
                    <div className="flex items-center justify-between text-slate-200 font-medium">
                      <span className="truncate">{cr.file_path}</span>
                      {cr.injected_import && (
                        <span className="text-cyan-400 font-mono text-[10px] truncate max-w-[160px]">
                          {cr.injected_import}
                        </span>
                      )}
                    </div>
                    <pre className="p-2 bg-slate-950 border border-slate-800 rounded text-[10px] font-mono text-slate-300 overflow-x-auto max-h-[80px]">
                      {cr.diff_patch}
                    </pre>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      ) : (
        <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500 font-mono text-xs">
          Click &quot;Preview Extraction Plan&quot; to synthesize shared crate and manifest updates.
        </div>
      )}
    </div>
  );
};
