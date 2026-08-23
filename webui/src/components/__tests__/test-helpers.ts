import { ClonePair, ScanProgress, ScanResult } from "../../types/cddm-types";
import { useCDDMStore } from "../../store/cddm-store";

export function resetTestStore(): void {
  useCDDMStore.getState().resetScan();
}

export function createMockClonePair(overrides: Partial<ClonePair> = {}): ClonePair {
  return {
    file_a: "src/a.ts",
    start_line_a: 10,
    end_line_a: 20,
    file_b: "src/b.ts",
    start_line_b: 15,
    end_line_b: 25,
    token_count: 55,
    similarity: 0.95,
    fragment_hash: "hash123",
    clone_type: "Exact",
    author_a: "Grigor",
    author_b: "Grigor",
    ...overrides,
  };
}

export function createMockScanResult(overrides: Partial<ScanResult> = {}): ScanResult {
  return {
    scan_id: "demo-scan-123",
    total_files: 42,
    total_tokens: 15420,
    total_clones: 3,
    duplication_percentage: 4.85,
    dry_health_score: 92.7,
    duration_ms: 12,
    clone_pairs: [],
    language_breakdown: [{ language: "Rust", files: 10, tokens: 1000, clones: 1 }],
    ...overrides,
  };
}

export function createMockProgress(overrides: Partial<ScanProgress> = {}): ScanProgress {
  return {
    progress: 0.5,
    phase: "Tokenization",
    message: "Tokenizing files...",
    files_processed: 10,
    total_files: 20,
    scan_id: "123",
    ...overrides,
  };
}
