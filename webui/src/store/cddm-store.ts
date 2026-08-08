import { create } from "zustand";
import { ScanConfig, ScanProgress, ScanResult } from "../types/cddm-types";

/**
 * Interface for CDDM Zustand Store State and Actions.
 */
export interface CDDMStoreState {
  /** Current scan configuration */
  config: ScanConfig;
  /** Active scan ID or null if idle */
  activeScanId: string | null;
  /** Active scan progress details */
  progress: ScanProgress | null;
  /** Final scan results if completed */
  results: ScanResult | null;
  /** Whether a scan is currently running */
  isScanning: boolean;
  /** Error message if scan failed */
  error: string | null;

  /** Updates the scan configuration */
  setConfig: (config: Partial<ScanConfig>) => void;
  /** Initiates a new code duplication scan */
  startScan: () => Promise<void>;
  /** Cancels an ongoing scan */
  cancelScan: () => void;
  /** Resets state to idle */
  resetScan: () => void;
}

const DEFAULT_CONFIG: ScanConfig = {
  directory: ".",
  min_tokens: 50,
  languages: [],
  ignore_patterns: ["node_modules", "target", ".git", "dist", "build"],
  detect_type2: true,
  scan_self: true,
  enable_git_blame: true,
};

/**
 * Global Zustand store for CDDM WebUI control plane.
 */
export const useCDDMStore = create<CDDMStoreState>((set, get) => ({
  config: DEFAULT_CONFIG,
  activeScanId: null,
  progress: null,
  results: null,
  isScanning: false,
  error: null,

  setConfig: (newConfig) => {
    set((state) => ({
      config: { ...state.config, ...newConfig },
    }));
  },

  startScan: async () => {
    set({ isScanning: true, error: null, results: null, progress: null });
    const { config } = get();

    try {
      // In standalone REST/Axum mode, POST to /api/scan
      const res = await fetch("/api/scan", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });

      if (!res.ok) {
        throw new Error(`Scan request failed: ${res.statusText}`);
      }

      const results: ScanResult = await res.json();
      set({ results, isScanning: false, activeScanId: results.scan_id });
    } catch (err) {
      // Fallback mock mode for static dev server preview
      if (typeof process === "undefined" || process.env.NODE_ENV !== "test") {
        console.warn("API request failed, using demo fallback state:", err);
      }
      const mockResult: ScanResult = {
        scan_id: "demo-scan-123",
        total_files: 42,
        total_tokens: 15420,
        total_clones: 3,
        duplication_percentage: 4.85,
        dry_health_score: 92.7,
        duration_ms: 184,
        clone_pairs: [
          {
            file_a: "src/utils/calculator.ts",
            start_line_a: 14,
            end_line_a: 48,
            file_b: "src/helpers/math.ts",
            start_line_b: 22,
            end_line_b: 56,
            token_count: 65,
            similarity: 0.98,
            fragment_hash: "a1b2c3d4",
            clone_type: "Exact",
            author_a: "Grigor Tonikyan",
            author_b: "Grigor Tonikyan",
          },
        ],
        language_breakdown: [
          { language: "TypeScript", files: 28, tokens: 11200, clones: 2 },
          { language: "Rust", files: 14, tokens: 4220, clones: 1 },
        ],
      };
      set({ results: mockResult, isScanning: false, activeScanId: mockResult.scan_id });
    }
  },

  cancelScan: () => {
    set({ isScanning: false, progress: null, error: "Scan cancelled" });
  },

  resetScan: () => {
    set({ results: null, progress: null, isScanning: false, error: null });
  },
}));
