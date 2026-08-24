import React, { ReactElement } from "react";
import { render, RenderResult } from "@testing-library/react";
import { CloneCluster, ClonePair, ScanProgress, ScanResult } from "../../types/cddm-types";
import { useCDDMStore } from "../../store/cddm-store";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

export function renderWithWin2x(ui: ReactElement): RenderResult {
  return render(React.createElement(Win2xManagerProvider, null, ui));
}

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

export function createMockCluster(overrides: Partial<CloneCluster> = {}): CloneCluster {
  return {
    id: 1,
    clone_type: "Exact",
    token_count: 65,
    similarity: 1.0,
    fragment_hash: "hash_cluster_abc123456",
    occurrences: [
      { file: "src/auth/login.ts", start_line: 10, end_line: 25, author: "Grigor" },
      { file: "src/auth/register.ts", start_line: 15, end_line: 30, author: "Grigor" },
      { file: "src/auth/reset.ts", start_line: 5, end_line: 20, author: "Alice" },
    ],
    ...overrides,
  };
}

export function createMockScanResult(overrides: Partial<ScanResult> = {}): ScanResult {
  return {
    scan_id: "demo-scan-123",
    total_files: 42,
    total_tokens: 15420,
    total_clones: 3,
    total_clusters: 1,
    duplication_percentage: 4.85,
    dry_health_score: 92.7,
    duration_ms: 12,
    clone_pairs: [],
    clone_clusters: [],
    language_breakdown: [{ language: "Rust", files: 10, tokens: 1000, clones: 1 }],
    ...overrides,
  };
}

export function createMockSnippet(file: string = "src/a.ts", start: number = 10, end: number = 20) {
  return {
    file,
    start_line: start,
    end_line: end,
    context_start_line: Math.max(1, start - 2),
    context_end_line: end + 2,
    lines: [
      { line_number: start, content: "const duplicateCode = true;", is_target: true },
      { line_number: start + 1, content: "console.log(duplicateCode);", is_target: true },
    ],
    total_lines: 50,
    language: "TypeScript",
  };
}

export function mockFetchSnippets(
  snippetA?: ReturnType<typeof createMockSnippet>,
  snippetB?: ReturnType<typeof createMockSnippet>,
): void {
  const sA = snippetA ?? createMockSnippet("src/a.ts", 10, 20);
  const sB = snippetB ?? createMockSnippet("src/b.ts", 15, 25);
  globalThis.fetch = ((url: string | URL | Request) => {
    const urlStr =
      typeof url === "string" ? url : url instanceof URL ? url.href : (url as Request).url;
    if (urlStr.includes("src%2Fa.ts") || urlStr.includes("src/a.ts")) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve(sA),
      } as Response);
    }
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve(sB),
    } as Response);
  }) as unknown as typeof fetch;
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
