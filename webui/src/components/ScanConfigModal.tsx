import React, { useState } from "react";
import { ScanConfigPanel } from "./ScanConfigPanel";
import { Win2xWindow } from "./ui/win2x-manager";
import { useCDDMStore } from "../store/cddm-store";
import type { CachePackSummary } from "../types/scan-types";
import { CachePackCard } from "./config/CachePackCard";
import {
  Sliders,
  Zap,
  Archive,
  Download,
  Upload,
  HardDrive,
  ShieldCheck,
  Boxes,
  FileCheck,
  AlertCircle,
  Network,
  ArrowRight,
  Database,
  Filter,
} from "lucide-react";

export interface ScanConfigModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const ScanConfigModal: React.FC<ScanConfigModalProps> = ({ isOpen, onClose }) => {
  const {
    config,
    setConfig,
    exportCachePack,
    importCachePack,
    setIsHookManagerModalOpen,
    setIsMonorepoModalOpen,
    setIsPolicyRulesModalOpen,
    setIsHubModalOpen,
  } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"tuning" | "cache" | "integrations">("tuning");

  // Cache pack export state
  const [exportCacheDir, setExportCacheDir] = useState<string>(config.cache_dir || ".cddm-cache");
  const [exportOutputPath, setExportOutputPath] = useState<string>("cddm-cache.pack");
  const [isExporting, setIsExporting] = useState<boolean>(false);
  const [exportSummary, setExportSummary] = useState<CachePackSummary | null>(null);

  // Cache pack import state
  const [importPackFile, setImportPackFile] = useState<string>("cddm-cache.pack");
  const [importTargetDir, setImportTargetDir] = useState<string>(config.cache_dir || ".cddm-cache");
  const [isImporting, setIsImporting] = useState<boolean>(false);
  const [importSummary, setImportSummary] = useState<CachePackSummary | null>(null);

  const [cacheError, setCacheError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleExportPack = async () => {
    setIsExporting(true);
    setCacheError(null);
    try {
      const summary = await exportCachePack(exportCacheDir, exportOutputPath);
      setExportSummary(summary);
    } catch (err) {
      setCacheError(err instanceof Error ? err.message : "Cache pack export failed");
    } finally {
      setIsExporting(false);
    }
  };

  const handleImportPack = async () => {
    setIsImporting(true);
    setCacheError(null);
    try {
      const summary = await importCachePack(importPackFile, importTargetDir);
      setImportSummary(summary);
    } catch (err) {
      setCacheError(err instanceof Error ? err.message : "Cache pack import failed");
    } finally {
      setIsImporting(false);
    }
  };

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        <Zap className="w-3.5 h-3.5 text-indigo-400" />
        <span>Winnowing M61 Token Algorithm & SIMD Parallelism</span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </div>
  );

  return (
    <Win2xWindow
      id="cddm-scan-config-window"
      windowType="scan-config"
      isOpen={isOpen}
      onClose={onClose}
      title="Scan Parameters & Centralized Configuration Studio"
      subtitle="Fine-tune token thresholds, engine flags, persistent cache packs, and ecosystem integrations"
      badge="Studio Mode"
      icon={<Sliders className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={920}
      initialHeight={680}
    >
      <div className="space-y-6">
        {/* Studio Tabs Header */}
        <div className="flex items-center gap-2 border-b border-slate-800 pb-2">
          <button
            type="button"
            onClick={() => setActiveTab("tuning")}
            className={`flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              activeTab === "tuning"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <Sliders className="w-3.5 h-3.5" />
            <span>Engine Parameters & Tuning</span>
          </button>

          <button
            type="button"
            onClick={() => setActiveTab("cache")}
            className={`flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              activeTab === "cache"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <HardDrive className="w-3.5 h-3.5" />
            <span>Persistent Cache Pack</span>
          </button>

          <button
            type="button"
            onClick={() => setActiveTab("integrations")}
            className={`flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              activeTab === "integrations"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <Network className="w-3.5 h-3.5" />
            <span>CI/CD & Integrations Hub</span>
          </button>
        </div>

        {/* Tab 1: Engine Parameters & Tuning */}
        {activeTab === "tuning" && (
          <div className="space-y-6">
            <ScanConfigPanel />

            {/* Advanced Filtering & Baseline Flags */}
            <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-4 space-y-4">
              <div className="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-slate-300">
                <Filter className="w-4 h-4 text-indigo-400" />
                <span>Advanced Filtering & Baseline</span>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label htmlFor="scan-cache-dir" className="block text-xs text-slate-400 mb-1">
                    Cache Storage Directory:
                  </label>
                  <input
                    id="scan-cache-dir"
                    type="text"
                    value={config.cache_dir || ""}
                    onChange={(e) => setConfig({ cache_dir: e.target.value })}
                    placeholder=".cddm-cache"
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
                  />
                </div>

                <div>
                  <label htmlFor="scan-baseline-file" className="block text-xs text-slate-400 mb-1">
                    Baseline Comparison File:
                  </label>
                  <input
                    id="scan-baseline-file"
                    type="text"
                    value={config.baseline || ""}
                    onChange={(e) => setConfig({ baseline: e.target.value })}
                    placeholder="e.g. cddm-baseline.json"
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
                  />
                </div>
              </div>

              <div className="flex flex-wrap items-center gap-6 pt-2">
                <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
                  <input
                    type="checkbox"
                    checked={config.enable_cache ?? true}
                    onChange={(e) => setConfig({ enable_cache: e.target.checked })}
                    className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500"
                  />
                  <span>Enable AST Cache</span>
                </label>

                <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
                  <input
                    type="checkbox"
                    checked={config.in_tree_cache ?? false}
                    onChange={(e) => setConfig({ in_tree_cache: e.target.checked })}
                    className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500"
                  />
                  <span>In-Tree Cache (.cddm)</span>
                </label>

                <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
                  <input
                    type="checkbox"
                    checked={config.ignore_tests ?? false}
                    onChange={(e) => setConfig({ ignore_tests: e.target.checked })}
                    className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500"
                  />
                  <span>Ignore Test Files</span>
                </label>

                <label className="flex items-center gap-2 text-xs font-medium text-slate-300 cursor-pointer select-none">
                  <input
                    type="checkbox"
                    checked={config.ignore_mocks ?? false}
                    onChange={(e) => setConfig({ ignore_mocks: e.target.checked })}
                    className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-500 focus:ring-indigo-500"
                  />
                  <span>Ignore Mock Files</span>
                </label>
              </div>
            </div>
          </div>
        )}

        {/* Tab 2: Persistent Cache Pack */}
        {activeTab === "cache" && (
          <div className="space-y-6">
            {cacheError && (
              <div className="p-3 bg-rose-950/60 border border-rose-500/40 rounded-lg text-xs text-rose-200 flex items-center gap-2">
                <AlertCircle className="w-4 h-4 text-rose-400 shrink-0" />
                <span>{cacheError}</span>
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <CachePackCard
                title="Export Cache Pack"
                description="Bundle analyzed token trees and hashes into a portable, compressed cache pack file for CI/CD runners and teammates."
                icon={<Download className="w-4 h-4 text-indigo-400" />}
                firstFieldLabel="Source Cache Directory:"
                firstFieldValue={exportCacheDir}
                firstFieldId="export-cache-dir"
                onFirstFieldChange={setExportCacheDir}
                secondFieldLabel="Output Pack File Path:"
                secondFieldValue={exportOutputPath}
                secondFieldId="export-output-path"
                onSecondFieldChange={setExportOutputPath}
                summary={exportSummary}
                summaryTheme="indigo"
                isSubmitting={isExporting}
                submitButtonText="Export Cache Pack"
                submittingText="Exporting Pack..."
                onSubmit={() => void handleExportPack()}
                submitButtonTheme="indigo"
                submitIcon={<Archive className="w-3.5 h-3.5" />}
              />

              <CachePackCard
                title="Import Cache Pack"
                description="Hydrate local cache store from a pre-built pack file to accelerate first-run scans on clean checkouts."
                icon={<Upload className="w-4 h-4 text-purple-400" />}
                firstFieldLabel="Pack File to Import:"
                firstFieldValue={importPackFile}
                firstFieldId="import-pack-file"
                onFirstFieldChange={setImportPackFile}
                secondFieldLabel="Target Cache Directory:"
                secondFieldValue={importTargetDir}
                secondFieldId="import-target-dir"
                onSecondFieldChange={setImportTargetDir}
                summary={importSummary}
                summaryTheme="purple"
                isSubmitting={isImporting}
                submitButtonText="Import Cache Pack"
                submittingText="Importing Pack..."
                onSubmit={() => void handleImportPack()}
                submitButtonTheme="purple"
                submitIcon={<Database className="w-3.5 h-3.5" />}
              />
            </div>
          </div>
        )}

        {/* Tab 3: CI/CD & Integrations Hub */}
        {activeTab === "integrations" && (
          <div className="space-y-4">
            <p className="text-xs text-slate-400">
              Quick access to CDDM configuration studios, quality gates, and ecosystem integrations:
            </p>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {[
                {
                  title: "Git Hook Manager & CI/CD Studio",
                  desc: "Install pre-commit and pre-push hooks; copy Gitea/GitHub Actions workflows.",
                  icon: ShieldCheck,
                  iconColor: "text-emerald-400",
                  onSelect: () => {
                    onClose();
                    setIsHookManagerModalOpen(true);
                  },
                },
                {
                  title: "Monorepo Workspace Studio",
                  desc: "Analyze multi-package architectures, package boundaries, and cross-package clones.",
                  icon: Boxes,
                  iconColor: "text-purple-400",
                  onSelect: () => {
                    onClose();
                    setIsMonorepoModalOpen(true);
                  },
                },
                {
                  title: "Policy Rules & Governance",
                  desc: "Configure architectural thresholds, maximum clones, and severity gates.",
                  icon: FileCheck,
                  iconColor: "text-cyan-400",
                  onSelect: () => {
                    onClose();
                    setIsPolicyRulesModalOpen(true);
                  },
                },
                {
                  title: "Organization Federation Hub",
                  desc: "Manage cross-repository peering, shared module extraction, and central registries.",
                  icon: Network,
                  iconColor: "text-amber-400",
                  onSelect: () => {
                    onClose();
                    setIsHubModalOpen(true);
                  },
                },
              ].map((item) => (
                <button
                  key={item.title}
                  type="button"
                  onClick={item.onSelect}
                  className="bg-slate-900/70 border border-slate-800 hover:border-indigo-500 rounded-xl p-4 text-left transition-colors flex items-start justify-between group"
                >
                  <div>
                    <div className="flex items-center gap-2 font-bold text-sm text-slate-200 mb-1">
                      <item.icon className={`w-4 h-4 ${item.iconColor}`} />
                      <span>{item.title}</span>
                    </div>
                    <p className="text-xs text-slate-400">{item.desc}</p>
                  </div>
                  <ArrowRight className="w-4 h-4 text-slate-600 group-hover:text-indigo-400 transition-colors shrink-0 mt-1" />
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
