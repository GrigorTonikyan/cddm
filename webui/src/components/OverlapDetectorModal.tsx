import React, { useState, useEffect } from "react";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Layers,
  Package,
  Terminal,
  Code2,
  Copy,
  Check,
  Search,
  CheckCircle2,
  Sparkles,
} from "lucide-react";
import type { OverlapMatch, OverlapScanResult, EcosystemAlgorithm } from "../types/cddm-types";
import { ModalTabs } from "./ui/ModalTabs";

export interface OverlapDetectorModalProps {
  isOpen: boolean;
  onClose: () => void;
  initialScanResult?: OverlapScanResult | null;
}

export const OverlapDetectorModal: React.FC<OverlapDetectorModalProps> = ({
  isOpen,
  onClose,
  initialScanResult = null,
}) => {
  const [scanResult, setScanResult] = useState<OverlapScanResult | null>(initialScanResult);
  const [catalog, setCatalog] = useState<EcosystemAlgorithm[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<"matches" | "catalog">("matches");
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
  const [threshold, setThreshold] = useState<number>(0.3);

  const fetchCatalog = async () => {
    try {
      const res = await fetch("/api/overlap/catalog");
      if (res.ok) {
        const data = (await res.json()) as EcosystemAlgorithm[];
        setCatalog(data);
      }
    } catch {
      // ignore network errors in mock environments
    }
  };

  const handleScan = async () => {
    setIsLoading(true);
    try {
      const res = await fetch("/api/overlap/scan", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ directory: ".", threshold }),
      });
      if (res.ok) {
        const data = (await res.json()) as OverlapScanResult;
        setScanResult(data);
      }
    } catch {
      // ignore network errors in mock environments
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (isOpen) {
      void fetchCatalog();
      if (!scanResult) {
        void handleScan();
      }
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const matches = scanResult?.matches || [];

  const handleCopy = (text: string, index: number) => {
    void navigator.clipboard?.writeText(text);
    setCopiedIndex(index);
    setTimeout(() => setCopiedIndex(null), 2000);
  };

  return (
    <Win2xWindow
      id="cddm-overlap-detector-window"
      windowType="overlap-detector"
      isOpen={isOpen}
      onClose={onClose}
      title="Ecosystem Library Reimplementation & Overlap Detector"
      subtitle="Detect custom code reimplementing well-known standard & community open-source packages"
      badge={matches.length > 0 ? `${matches.length} Library Overlaps` : "Zero Library Overlaps"}
    >
      <div className="space-y-6">
        {/* Navigation Tabs & Controls */}
        <div className="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-slate-800">
          <ModalTabs
            tabs={[
              {
                id: "matches",
                label: "Detected Matches",
                icon: <Layers className="w-3.5 h-3.5" />,
                count: matches.length,
              },
              {
                id: "catalog",
                label: "Algorithm Catalog",
                icon: <Package className="w-3.5 h-3.5" />,
                count: catalog.length,
              },
            ]}
            activeTab={activeTab}
            onTabChange={(id) => setActiveTab(id as "matches" | "catalog")}
            activeColorClass="bg-amber-950/80 text-amber-300 border border-amber-800/60"
          />

          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2 text-xs text-slate-400">
              <label htmlFor="overlap-threshold-input" className="cursor-pointer">
                Threshold:
              </label>
              <input
                id="overlap-threshold-input"
                name="overlap_threshold"
                aria-label="Overlap detection similarity threshold"
                type="range"
                min="0.1"
                max="0.9"
                step="0.05"
                value={threshold}
                onChange={(e) => setThreshold(parseFloat(e.target.value))}
                className="w-24 accent-amber-500 cursor-pointer"
              />
              <span className="font-mono text-slate-200">{(threshold * 100).toFixed(0)}%</span>
            </div>
            <button
              type="button"
              onClick={handleScan}
              disabled={isLoading}
              className="px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 text-white font-semibold text-xs flex items-center gap-1.5 shadow-md disabled:opacity-50"
            >
              <Search className="w-3.5 h-3.5" />
              {isLoading ? "Scanning..." : "Re-Scan"}
            </button>
          </div>
        </div>

        {/* Tab 1: Matches */}
        {activeTab === "matches" && (
          <div className="space-y-4">
            {matches.length === 0 ? (
              <div className="text-center py-12 bg-slate-900/40 border border-slate-800/60 rounded-xl space-y-3">
                <CheckCircle2 className="w-10 h-10 text-emerald-400 mx-auto" />
                <h3 className="text-sm font-semibold text-slate-200">
                  No Ecosystem Library Overlaps Detected
                </h3>
                <p className="text-xs text-slate-400 max-w-md mx-auto">
                  Your codebase is clean of reinvented standard algorithms or the detection
                  threshold is high.
                </p>
              </div>
            ) : (
              matches.map((m: OverlapMatch, idx: number) => (
                <div
                  key={idx}
                  className="p-4 bg-slate-900/70 border border-amber-900/40 rounded-xl space-y-3"
                >
                  <div className="flex flex-wrap items-center justify-between gap-2">
                    <div className="flex items-center gap-2">
                      <span className="font-semibold text-sm text-slate-200">
                        {m.algorithm_name}
                      </span>
                      <span className="px-2 py-0.5 rounded text-[11px] bg-slate-800 text-slate-400 font-mono">
                        {m.category}
                      </span>
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-0.5 rounded text-xs font-semibold bg-amber-950 border border-amber-800 text-amber-300">
                        {(m.confidence * 100).toFixed(0)}% Confidence
                      </span>
                    </div>
                  </div>

                  <div className="text-xs font-mono text-slate-400 flex items-center gap-2">
                    <Code2 className="w-3.5 h-3.5 text-cyan-400" />
                    <span>
                      {m.file_path}:{m.line_span[0]}-{m.line_span[1]} ({m.function_name})
                    </span>
                  </div>

                  {/* Recommendation Card */}
                  <div className="p-3 bg-slate-950/70 border border-slate-800 rounded-lg space-y-2">
                    <div className="flex items-center justify-between text-xs">
                      <span className="text-emerald-400 font-semibold flex items-center gap-1.5">
                        <Sparkles className="w-3.5 h-3.5" />
                        Recommended Replacement: {m.recommended_library.package_name} (
                        {m.recommended_library.language})
                      </span>
                    </div>

                    {m.recommended_library.install_command && (
                      <div className="flex items-center justify-between bg-slate-900 px-3 py-1.5 rounded border border-slate-800 font-mono text-xs text-slate-300">
                        <div className="flex items-center gap-2">
                          <Terminal className="w-3.5 h-3.5 text-amber-400" />
                          <span>{m.recommended_library.install_command}</span>
                        </div>
                        <button
                          type="button"
                          onClick={() => handleCopy(m.recommended_library.install_command, idx)}
                          className="text-slate-400 hover:text-slate-200"
                          title="Copy install command"
                        >
                          {copiedIndex === idx ? (
                            <Check className="w-3.5 h-3.5 text-emerald-400" />
                          ) : (
                            <Copy className="w-3.5 h-3.5" />
                          )}
                        </button>
                      </div>
                    )}

                    {m.recommended_library.replacement_snippet && (
                      <pre className="p-2 bg-slate-900 rounded font-mono text-xs text-slate-300 overflow-x-auto">
                        <code>{m.recommended_library.replacement_snippet}</code>
                      </pre>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        )}

        {/* Tab 2: Catalog */}
        {activeTab === "catalog" && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {catalog.map((algo: EcosystemAlgorithm, idx: number) => (
              <div
                key={idx}
                className="p-4 bg-slate-900/60 border border-slate-800 rounded-xl space-y-2"
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-sm text-slate-200">{algo.name}</span>
                  <span className="px-2 py-0.5 rounded text-[11px] bg-slate-800 text-slate-400 font-mono">
                    {algo.category}
                  </span>
                </div>
                <p className="text-xs text-slate-400">{algo.description}</p>
                <div className="text-xs text-slate-500 font-mono">
                  Keywords: {algo.canonical_keywords.join(", ")}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
