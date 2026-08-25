import React from "react";
import { ScanConfigPanel } from "./components/ScanConfigPanel";
import { ScanProgressBar } from "./components/ScanProgressBar";
import { ScanResults } from "./components/ScanResults";
import { ScanConfigModal } from "./components/ScanConfigModal";
import { APP_VERSION } from "./constants/cddm-constants";
import { useCDDMStore } from "./store/cddm-store";
import { TimelineExplorerModal } from "./components/TimelineExplorerModal";
import { SuppressionRulesModal } from "./components/SuppressionRulesModal";
import { RefactorSandboxModal } from "./components/RefactorSandboxModal";
import { PolicyRulesModal } from "./components/PolicyRulesModal";
import { SemanticGraphModal } from "./components/SemanticGraphModal";
import {
  Scissors,
  Terminal,
  Sparkles,
  ShieldCheck,
  ShieldAlert,
  Scale,
  Sliders,
  Award,
  FileDown,
  Radio,
  CheckCheck,
  X,
  History,
  Network,
} from "lucide-react";

export const App: React.FC = () => {
  const {
    error,
    results,
    isLiveWatchActive,
    patchStatusMessage,
    setIsLiveWatchActive,
    setPatchStatusMessage,
    isScanConfigOpen,
    setIsScanConfigOpen,
    setIsHealthAuditOpen,
    setIsExportReportOpen,
    isTimelineModalOpen,
    setIsTimelineModalOpen,
    isSuppressionModalOpen,
    setIsSuppressionModalOpen,
    isRefactorSandboxOpen,
    setIsRefactorSandboxOpen,
    isPolicyRulesModalOpen,
    setIsPolicyRulesModalOpen,
    isSemanticGraphModalOpen,
    setIsSemanticGraphModalOpen,
    liveSyncCount,
  } = useCDDMStore();

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500/30 selection:text-indigo-200">
      {/* Radiant Glow Background Accent */}
      <div className="fixed top-0 left-1/2 -translate-x-1/2 w-full max-w-7xl h-96 bg-gradient-to-b from-indigo-950/20 via-purple-950/10 to-transparent pointer-events-none blur-3xl -z-10" />

      {/* Header Bar */}
      <header className="bg-slate-900/90 border-b border-slate-800/80 px-6 py-4 flex flex-wrap items-center justify-between gap-3 sticky top-0 z-50 backdrop-blur-md shadow-lg">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-tr from-indigo-600 to-purple-600 rounded-xl shadow-lg shadow-indigo-900/30">
            <Scissors className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="font-extrabold text-xl tracking-tight text-white flex items-center gap-2.5">
              <span>CDDM Studio</span>
              <span className="text-xs bg-indigo-950 text-indigo-300 font-mono px-2.5 py-0.5 rounded-full font-semibold border border-indigo-800/50">
                v{APP_VERSION}
              </span>
            </h1>
            <p className="text-xs text-slate-400">
              Code De-Duplication Meister & Architectural Health
            </p>
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-2.5 text-xs font-mono text-slate-400">
          {/* Live Watch Status Toggle */}
          <button
            type="button"
            onClick={() => setIsLiveWatchActive(!isLiveWatchActive)}
            title={
              isLiveWatchActive
                ? "Live Workspace Sync Active: Click to pause"
                : "Live Workspace Sync Paused: Click to resume"
            }
            className={`px-3 py-1.5 rounded-lg border flex items-center gap-1.5 transition-colors shadow-sm ${
              isLiveWatchActive
                ? "bg-emerald-950/60 border-emerald-800/60 text-emerald-300 hover:bg-emerald-900/40"
                : "bg-slate-950 border-slate-800 text-slate-400 hover:bg-slate-800"
            }`}
          >
            <Radio
              className={`w-3.5 h-3.5 ${
                isLiveWatchActive ? "text-emerald-400 animate-pulse" : "text-slate-500"
              }`}
            />
            <span>
              {isLiveWatchActive
                ? liveSyncCount > 0
                  ? `Live Watch (${liveSyncCount} syncs)`
                  : "Live Watch: ON"
                : "Live Watch: OFF"}
            </span>
          </button>

          <button
            type="button"
            onClick={() => setIsScanConfigOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Sliders className="w-3.5 h-3.5 text-indigo-400" />
            <span>Config Window</span>
          </button>

          <button
            type="button"
            onClick={() => setIsTimelineModalOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <History className="w-3.5 h-3.5 text-indigo-400" />
            <span>Timeline Trends</span>
          </button>

          <button
            type="button"
            onClick={() => setIsSuppressionModalOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <ShieldAlert className="w-3.5 h-3.5 text-amber-400" />
            <span>Suppression Rules</span>
          </button>

          <button
            type="button"
            onClick={() => setIsPolicyRulesModalOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Scale className="w-3.5 h-3.5 text-purple-400" />
            <span>Policy Studio</span>
          </button>

          <button
            type="button"
            onClick={() => setIsSemanticGraphModalOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Network className="w-3.5 h-3.5 text-cyan-400" />
            <span>Semantic Graph</span>
          </button>

          {results && (
            <>
              <button
                type="button"
                onClick={() => setIsHealthAuditOpen(true)}
                className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
              >
                <Award className="w-3.5 h-3.5 text-emerald-400" />
                <span>Health Audit</span>
              </button>
              <button
                type="button"
                onClick={() => setIsExportReportOpen(true)}
                className="px-3 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold flex items-center gap-1.5 transition-colors shadow-sm"
              >
                <FileDown className="w-3.5 h-3.5" />
                <span>Reports</span>
              </button>
            </>
          )}

          <div className="hidden sm:flex items-center gap-1.5 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800/80">
            <Terminal className="w-3.5 h-3.5 text-indigo-400" />
            <span>cddm serve</span>
          </div>
          <div className="hidden md:flex items-center gap-1.5 bg-indigo-950/60 text-indigo-300 px-3 py-1.5 rounded-lg border border-indigo-800/60 shadow-sm">
            <Sparkles className="w-3.5 h-3.5 text-indigo-400" />
            <span>M61 Winnowing</span>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl w-full mx-auto p-4 sm:p-6 space-y-6">
        {patchStatusMessage && (
          <div className="bg-emerald-950/70 border border-emerald-800 text-emerald-200 px-4 py-3 rounded-xl flex items-center justify-between gap-2 shadow-lg animate-fade-in">
            <div className="flex items-center gap-2">
              <CheckCheck className="w-5 h-5 text-emerald-400 shrink-0" />
              <span className="text-xs font-mono">{patchStatusMessage}</span>
            </div>
            <button
              type="button"
              onClick={() => setPatchStatusMessage(null)}
              className="p-1 rounded-lg hover:bg-emerald-900/50 text-emerald-400"
              title="Dismiss notification"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        )}

        {error && (
          <div className="bg-rose-950/60 border border-rose-900/80 text-rose-300 px-4 py-3 rounded-xl flex items-center gap-2 shadow-lg">
            <ShieldCheck className="w-5 h-5 text-rose-400 shrink-0" />
            <span className="text-xs font-mono">{error}</span>
          </div>
        )}
        <ScanConfigPanel />
        <ScanProgressBar />
        <ScanResults />
      </main>

      {/* Global Config Modal */}
      <ScanConfigModal isOpen={isScanConfigOpen} onClose={() => setIsScanConfigOpen(false)} />

      {/* Timeline Trends Explorer Modal */}
      <TimelineExplorerModal
        isOpen={isTimelineModalOpen}
        onClose={() => setIsTimelineModalOpen(false)}
      />

      {/* Suppression Rules Modal */}
      <SuppressionRulesModal
        isOpen={isSuppressionModalOpen}
        onClose={() => setIsSuppressionModalOpen(false)}
      />

      {/* Refactor Sandbox Studio Modal */}
      <RefactorSandboxModal
        isOpen={isRefactorSandboxOpen}
        onClose={() => setIsRefactorSandboxOpen(false)}
      />

      {/* Architectural Policy Rules Studio Modal */}
      <PolicyRulesModal
        isOpen={isPolicyRulesModalOpen}
        onClose={() => setIsPolicyRulesModalOpen(false)}
      />

      {/* Semantic Graph & CFG/PDG Explorer Modal */}
      <SemanticGraphModal
        isOpen={isSemanticGraphModalOpen}
        onClose={() => setIsSemanticGraphModalOpen(false)}
      />

      {/* Footer */}
      <footer className="border-t border-slate-900 py-6 text-center text-xs text-slate-500 font-mono bg-slate-950/80">
        CDDM — Code De-Duplication Meister &copy; 2026 Grigor Tonikyan. Open Source MIT /
        Apache-2.0.
      </footer>
    </div>
  );
};

export default App;
