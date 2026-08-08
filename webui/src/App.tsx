import React from "react";
import { ScanConfigPanel } from "./components/ScanConfigPanel";
import { ScanProgressBar } from "./components/ScanProgressBar";
import { ScanResults } from "./components/ScanResults";
import { Scissors, Terminal, Sparkles, ShieldCheck } from "lucide-react";
import { useCDDMStore } from "./store/cddm-store";

export const App: React.FC = () => {
  const { error } = useCDDMStore();

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500/30 selection:text-indigo-200">
      {/* Radiant Glow Background Accent */}
      <div className="fixed top-0 left-1/2 -translate-x-1/2 w-full max-w-7xl h-96 bg-gradient-to-b from-indigo-950/20 via-purple-950/10 to-transparent pointer-events-none blur-3xl -z-10" />

      {/* Header Bar */}
      <header className="bg-slate-900/90 border-b border-slate-800/80 px-6 py-4 flex items-center justify-between sticky top-0 z-50 backdrop-blur-md shadow-lg">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-tr from-indigo-600 to-purple-600 rounded-xl shadow-lg shadow-indigo-900/30">
            <Scissors className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="font-extrabold text-xl tracking-tight text-white flex items-center gap-2.5">
              <span>CDDM Studio</span>
              <span className="text-xs bg-indigo-950 text-indigo-300 font-mono px-2.5 py-0.5 rounded-full font-semibold border border-indigo-800/50">
                v0.1.2
              </span>
            </h1>
            <p className="text-xs text-slate-400">Code De-Duplication Meister & Architectural Health</p>
          </div>
        </div>

        <div className="flex items-center gap-3 text-xs font-mono text-slate-400">
          <div className="hidden sm:flex items-center gap-1.5 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800/80">
            <Terminal className="w-3.5 h-3.5 text-indigo-400" />
            <span>cddm serve</span>
          </div>
          <div className="flex items-center gap-1.5 bg-indigo-950/60 text-indigo-300 px-3 py-1.5 rounded-lg border border-indigo-800/60 shadow-sm">
            <Sparkles className="w-3.5 h-3.5 text-indigo-400" />
            <span>M61 Winnowing Engine</span>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl w-full mx-auto p-4 sm:p-6 space-y-6">
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

      {/* Footer */}
      <footer className="border-t border-slate-900 py-6 text-center text-xs text-slate-500 font-mono bg-slate-950/80">
        CDDM — Code De-Duplication Meister &copy; 2026 Grigor Tonikyan. Open Source MIT / Apache-2.0.
      </footer>
    </div>
  );
};

export default App;
