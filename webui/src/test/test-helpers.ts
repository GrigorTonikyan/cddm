import React, { ReactElement } from "react";
import { expect } from "vite-plus/test";
import { act, fireEvent, render, RenderResult, screen } from "@testing-library/react";
import {
  CloneCluster,
  ClonePair,
  ControlFlowGraph,
  ScanProgress,
  ScanResult,
  SuppressionConfig,
} from "../types/cddm-types";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xManagerProvider } from "../components/ui/win2x-manager/context/win2x-manager-context";

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

export function createMockSuppressionConfig(): SuppressionConfig {
  return {
    rules: [
      {
        pattern: "**/tests/**",
        comment: "Ignore test fixtures",
        min_tokens_override: undefined,
        ignored_clone_types: ["Exact"],
      },
      {
        pattern: "legacy/services/**",
        comment: "Legacy service threshold",
        min_tokens_override: 120,
        ignored_clone_types: [],
      },
    ],
    ignore_tests: true,
    ignore_mocks: false,
    ignore_generated: true,
    raw_cddmignore: "**/tests/**\nlegacy/services/**\n",
  };
}

export function createMockSemanticResponse() {
  return {
    cfg_a: {
      function_name: "compute_a",
      file_path: "src/a.ts",
      language: "ts",
      nodes: [
        { id: 1, node_type: "Entry", code_snippet: "function a() {", line_number: 1 },
        { id: 2, node_type: "Branch", code_snippet: "if (x > 0)", line_number: 2 },
        { id: 3, node_type: "Return", code_snippet: "return x;", line_number: 3 },
      ],
      edges: [
        { from: 1, to: 2, edge_type: "Sequential" },
        { from: 2, to: 3, edge_type: "ConditionalTrue" },
      ],
      cyclomatic_complexity: 2,
    },
    cfg_b: {
      function_name: "compute_b",
      file_path: "src/b.ts",
      language: "ts",
      nodes: [
        { id: 1, node_type: "Entry", code_snippet: "function b() {", line_number: 1 },
        { id: 2, node_type: "Branch", code_snippet: "if (val > 0)", line_number: 2 },
        { id: 3, node_type: "Return", code_snippet: "return val;", line_number: 3 },
      ],
      edges: [
        { from: 1, to: 2, edge_type: "Sequential" },
        { from: 2, to: 3, edge_type: "ConditionalTrue" },
      ],
      cyclomatic_complexity: 2,
    },
    comparison: {
      similarity: 0.95,
      is_semantic_clone: true,
      wl_hash_a: 0x12345678,
      wl_hash_b: 0x12345678,
    },
  };
}

export function expectDefinedTexts(texts: (string | RegExp)[]): void {
  for (const text of texts) {
    expect(screen.getByText(text)).toBeDefined();
  }
}

export function createMockControlFlowGraph(): ControlFlowGraph {
  return {
    file_path: "src/calc.rs",
    function_name: "test_fn",
    line_start: 1,
    line_end: 10,
    nodes: [
      { id: 0, node_type: "Entry", label: "entry", statement_count: 1, line_start: 1, line_end: 1 },
      {
        id: 1,
        node_type: "Branch",
        label: "if x > 0",
        statement_count: 1,
        line_start: 2,
        line_end: 2,
      },
      {
        id: 2,
        node_type: "Return",
        label: "return x",
        statement_count: 1,
        line_start: 3,
        line_end: 3,
      },
    ],
    edges: [
      { from: 0, to: 1, edge_type: "Sequential" },
      { from: 1, to: 2, edge_type: "TrueBranch" },
    ],
    wl_hash: 123456789,
  };
}

export const DEFAULT_TEST_CLONE_PAIR_PROPS = {
  fileA: "src/a.ts",
  startLineA: 10,
  endLineA: 12,
  fileB: "src/b.ts",
  startLineB: 20,
  endLineB: 22,
};

export function assertModalClosesOnButtonClick(
  onClose: (...args: unknown[]) => void,
  btnLabel = "Close",
): void {
  fireEvent.click(screen.getByText(btnLabel));
  expect(onClose).toHaveBeenCalled();
}

export async function renderAsyncWithWin2x(ui: ReactElement): Promise<RenderResult> {
  let res!: RenderResult;
  await act(async () => {
    res = renderWithWin2x(ui);
  });
  return res;
}

export async function clickElementAsync(matcher: string | RegExp): Promise<void> {
  const el = screen.getByText(matcher);
  await act(async () => {
    fireEvent.click(el);
  });
}

export function mockSuccessResponse<T>(data: T): Promise<{ ok: boolean; json: () => Promise<T> }> {
  return Promise.resolve({
    ok: true,
    json: () => Promise.resolve(data),
  });
}

export function expectNullWhenClosed(ui: ReactElement): void {
  const { container } = renderWithWin2x(ui);
  expect(container.firstChild).toBeNull();
}
