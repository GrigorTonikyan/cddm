import React from "react";
import { ScanConfigPanel } from "./components/ScanConfigPanel";
import { ScanProgressBar } from "./components/ScanProgressBar";
import { ScanResults } from "./components/ScanResults";
import { Scissors, Terminal, Sparkles } from "lucide-react";
import { useCDDMStore } from "./store/cddm-store";

/**
 * Main Application Shell component for CDDM Studio WebUI.
 */
export const App: React.FC = () => {
  const { error } = useCDDMStore();

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 flex flex-col font-sans">
      {/* Header Bar */}
      <header className="bg-gray-900 border-b border-gray-800 px-6 py-4 flex items-center justify-between sticky top-0 z-50">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-indigo-600 rounded-lg shadow-lg">
            <Scissors className="w-6 h-6 text-white" />
          </div>
          <div>
            <h1 className="font-extrabold text-xl tracking-tight text-white flex items-center gap-2">
              <span>CDDM Studio</span>
              <span className="text-xs bg-indigo-950 text-indigo-300 font-mono px-2 py-0.5 rounded font-normal border border-indigo-800/50">
                v0.1.0
              </span>
            </h1>
            <p className="text-xs text-gray-400">Code De-Duplication Meister & Architectural Health</p>
          </div>
        </div>

        <div className="flex items-center gap-3 text-xs font-mono text-gray-400">
          <div className="flex items-center gap-1.5 bg-gray-950 px-3 py-1.5 rounded-lg border border-gray-800">
            <Terminal className="w-3.5 h-3.5 text-indigo-400" />
            <span>cddm serve</span>
          </div>
          <div className="flex items-center gap-1.5 bg-indigo-950/40 text-indigo-300 px-3 py-1.5 rounded-lg border border-indigo-900/50">
            <Sparkles className="w-3.5 h-3.5" />
            <span>M61 Winnowing Engine</span>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl w-full mx-auto p-6 space-y-6">
        {error && (
          <div className="bg-rose-950/50 border border-rose-900 text-rose-400 px-4 py-3 rounded-lg flex items-center gap-2">
            <span>{error}</span>
          </div>
        )}
        <ScanConfigPanel />
        <ScanProgressBar />
        <ScanResults />
      </main>

      {/* Footer */}
      <footer className="border-t border-gray-900 py-4 text-center text-xs text-gray-500 font-mono">
        CDDM — Code De-Duplication Meister &copy; 2026 Grigor Tonikyan. Open Source MIT / Apache-2.0.
      </footer>
    </div>
  );
};

export default App;
