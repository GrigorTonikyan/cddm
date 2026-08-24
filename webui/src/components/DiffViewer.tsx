import React, { useEffect, useState, useRef, useCallback } from "react";
import { API_ROUTES } from "../constants/cddm-constants";
import { SnippetResponse } from "../types/cddm-types";
import { parsePath } from "../utils/path-utils";
import { getIdeDeeplink, getEditorDisplayName } from "../utils/ide-links";
import { useCDDMStore } from "../store/cddm-store";
import {
  Columns2,
  FileCode,
  Copy,
  Check,
  RefreshCw,
  AlertCircle,
  Code2,
  ExternalLink,
} from "lucide-react";

export interface DiffViewerProps {
  fileA: string;
  startLineA: number;
  endLineA: number;
  fileB: string;
  startLineB: number;
  endLineB: number;
  tokenCount?: number;
}

// Tokenize a code line into syntax-highlighted spans
const renderHighlightedLine = (text: string) => {
  if (!text) return <span>&nbsp;</span>;

  // Simple token regex matching strings, comments, numbers, keywords
  const tokenRegex =
    /("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\/\/[^\n]*|#[^\n]*|\b(?:fn|function|def|const|let|var|return|if|else|for|while|import|from|export|class|struct|enum|pub|impl|type|interface|async|await|true|false|null|None)\b|\b\d+\b|[a-zA-Z_]\w*|[^\s\w])/g;

  const parts: React.ReactNode[] = [];
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  while ((match = tokenRegex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push(text.slice(lastIndex, match.index));
    }
    const token = match[0];
    if (token.startsWith("//") || token.startsWith("#")) {
      parts.push(
        <span key={match.index} className="text-slate-500 italic">
          {token}
        </span>,
      );
    } else if (
      (token.startsWith('"') && token.endsWith('"')) ||
      (token.startsWith("'") && token.endsWith("'"))
    ) {
      parts.push(
        <span key={match.index} className="text-emerald-400">
          {token}
        </span>,
      );
    } else if (/^\d+$/.test(token)) {
      parts.push(
        <span key={match.index} className="text-amber-400">
          {token}
        </span>,
      );
    } else if (
      /^(?:fn|function|def|const|let|var|return|if|else|for|while|import|from|export|class|struct|enum|pub|impl|type|interface|async|await|true|false|null|None)$/.test(
        token,
      )
    ) {
      parts.push(
        <span key={match.index} className="text-indigo-400 font-semibold">
          {token}
        </span>,
      );
    } else {
      parts.push(
        <span key={match.index} className="text-slate-200">
          {token}
        </span>,
      );
    }
    lastIndex = match.index + token.length;
  }

  if (lastIndex < text.length) {
    parts.push(text.slice(lastIndex));
  }

  return <>{parts}</>;
};

export const DiffViewer: React.FC<DiffViewerProps> = ({
  fileA,
  startLineA,
  endLineA,
  fileB,
  startLineB,
  endLineB,
}) => {
  const { preferredEditor } = useCDDMStore();
  const [snippetA, setSnippetA] = useState<SnippetResponse | null>(null);
  const [snippetB, setSnippetB] = useState<SnippetResponse | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<"split" | "unified">("split");
  const [copiedA, setCopiedA] = useState(false);
  const [copiedB, setCopiedB] = useState(false);

  const ideLinkA = getIdeDeeplink(fileA, startLineA, preferredEditor);
  const ideLinkB = getIdeDeeplink(fileB, startLineB, preferredEditor);

  const scrollRefA = useRef<HTMLDivElement>(null);
  const scrollRefB = useRef<HTMLDivElement>(null);
  const isSyncing = useRef<boolean>(false);

  const fetchSnippets = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [resA, resB] = await Promise.all([
        fetch(
          `${API_ROUTES.SNIPPET}?file=${encodeURIComponent(fileA)}&start=${startLineA}&end=${endLineA}&context=4`,
        ),
        fetch(
          `${API_ROUTES.SNIPPET}?file=${encodeURIComponent(fileB)}&start=${startLineB}&end=${endLineB}&context=4`,
        ),
      ]);

      if (!resA.ok) {
        const errorTextA = await resA.text().catch(() => resA.statusText);
        throw new Error(
          `Failed to load Fragment A snippet (${resA.status}): ${errorTextA || resA.statusText}`,
        );
      }
      if (!resB.ok) {
        const errorTextB = await resB.text().catch(() => resB.statusText);
        throw new Error(
          `Failed to load Fragment B snippet (${resB.status}): ${errorTextB || resB.statusText}`,
        );
      }

      const dataA = (await resA.json()) as SnippetResponse;
      const dataB = (await resB.json()) as SnippetResponse;
      setSnippetA(dataA);
      setSnippetB(dataB);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "Failed to load code snippets");
    } finally {
      setLoading(false);
    }
  }, [fileA, startLineA, endLineA, fileB, startLineB, endLineB]);

  useEffect(() => {
    void fetchSnippets();
  }, [fetchSnippets]);

  // Synchronized scrolling handlers
  const handleScrollA = () => {
    if (isSyncing.current) return;
    if (scrollRefA.current && scrollRefB.current) {
      isSyncing.current = true;
      scrollRefB.current.scrollTop = scrollRefA.current.scrollTop;
      scrollRefB.current.scrollLeft = scrollRefA.current.scrollLeft;
      requestAnimationFrame(() => {
        isSyncing.current = false;
      });
    }
  };

  const handleScrollB = () => {
    if (isSyncing.current) return;
    if (scrollRefA.current && scrollRefB.current) {
      isSyncing.current = true;
      scrollRefA.current.scrollTop = scrollRefB.current.scrollTop;
      scrollRefA.current.scrollLeft = scrollRefB.current.scrollLeft;
      requestAnimationFrame(() => {
        isSyncing.current = false;
      });
    }
  };

  const handleCopyCode = (snippet: SnippetResponse | null, isA: boolean) => {
    if (!snippet) return;
    const targetCode = snippet.lines
      .filter((l) => l.is_target)
      .map((l) => l.content)
      .join("\n");

    void navigator.clipboard.writeText(targetCode);
    if (isA) {
      setCopiedA(true);
      setTimeout(() => setCopiedA(false), 2000);
    } else {
      setCopiedB(true);
      setTimeout(() => setCopiedB(false), 2000);
    }
  };

  const parsedA = parsePath(fileA);
  const parsedB = parsePath(fileB);

  if (loading) {
    return (
      <div className="bg-slate-950/90 rounded-xl border border-slate-800 p-8 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs">
        <RefreshCw className="w-5 h-5 animate-spin text-indigo-400" />
        <span>Loading synchronized code diff...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-slate-950/90 rounded-xl border border-rose-900/50 p-6 text-xs font-mono space-y-3">
        <div className="flex items-center gap-2 text-rose-400 font-semibold">
          <AlertCircle className="w-4 h-4 shrink-0" />
          <span>Snippet Retrieval Notice</span>
        </div>
        <p className="text-slate-400">{error}</p>
        <button
          type="button"
          onClick={() => void fetchSnippets()}
          className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200 transition-colors flex items-center gap-1.5"
        >
          <RefreshCw className="w-3.5 h-3.5" />
          Retry Snippet Load
        </button>
      </div>
    );
  }

  return (
    <div className="bg-slate-950 rounded-xl border border-slate-800/90 overflow-hidden shadow-2xl">
      {/* Diff Controls Header */}
      <div className="px-4 py-2.5 bg-slate-900/80 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-3 text-xs font-mono">
        <div className="flex items-center gap-2 text-slate-300 font-semibold">
          <Code2 className="w-4 h-4 text-indigo-400" />
          <span>Interactive Code Diff Visualizer</span>
          <span className="text-[11px] font-normal px-2 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/40">
            {snippetA?.language || "Source"}
          </span>
        </div>

        {/* View Mode Toggle */}
        <div className="flex items-center gap-1 bg-slate-950 p-0.5 rounded-lg border border-slate-800">
          <button
            type="button"
            onClick={() => setViewMode("split")}
            className={`px-2.5 py-1 rounded text-[11px] font-semibold flex items-center gap-1.5 transition-all ${
              viewMode === "split"
                ? "bg-indigo-600 text-white shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <Columns2 className="w-3 h-3" />
            Side-by-Side
          </button>
          <button
            type="button"
            onClick={() => setViewMode("unified")}
            className={`px-2.5 py-1 rounded text-[11px] font-semibold flex items-center gap-1.5 transition-all ${
              viewMode === "unified"
                ? "bg-indigo-600 text-white shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <FileCode className="w-3 h-3" />
            Unified
          </button>
        </div>
      </div>

      {/* Code Container */}
      {viewMode === "split" ? (
        <div className="grid grid-cols-1 lg:grid-cols-2 divide-y lg:divide-y-0 lg:divide-x divide-slate-800">
          {/* Panel Fragment A */}
          <div className="flex flex-col min-w-0">
            <div className="px-3 py-1.5 bg-slate-900/60 border-b border-slate-800/60 flex items-center justify-between text-xs font-mono">
              <span className="text-slate-300 truncate" title={fileA}>
                <span className="text-slate-500">{parsedA.directory}</span>
                <span className="font-bold text-indigo-300">{parsedA.filename}</span>
                <span className="text-slate-500 ml-1.5">
                  (L{startLineA}–{endLineA})
                </span>
              </span>
              <div className="flex items-center gap-1">
                <a
                  href={ideLinkA}
                  title={`Open in ${getEditorDisplayName(preferredEditor)} at line ${startLineA}`}
                  className="p-1 hover:bg-slate-800 text-slate-400 hover:text-indigo-300 rounded transition-colors"
                >
                  <ExternalLink className="w-3.5 h-3.5" />
                </a>
                <button
                  type="button"
                  onClick={() => handleCopyCode(snippetA, true)}
                  className="p-1 hover:bg-slate-800 text-slate-400 hover:text-slate-200 rounded transition-colors"
                  title="Copy duplicate code"
                >
                  {copiedA ? (
                    <Check className="w-3.5 h-3.5 text-emerald-400" />
                  ) : (
                    <Copy className="w-3.5 h-3.5" />
                  )}
                </button>
              </div>
            </div>
            <div
              ref={scrollRefA}
              onScroll={handleScrollA}
              className="max-h-80 overflow-x-auto overflow-y-auto p-2 font-mono text-xs leading-relaxed select-text"
            >
              {snippetA?.lines.map((line) => (
                <div
                  key={`a-${line.line_number}`}
                  className={`flex items-start gap-3 px-2 py-0.5 rounded transition-colors min-w-fit ${
                    line.is_target
                      ? "bg-indigo-950/40 text-indigo-100 border-l-2 border-indigo-500"
                      : "text-slate-400 hover:bg-slate-900/40"
                  }`}
                >
                  <span className="w-8 shrink-0 text-right select-none opacity-40 font-mono text-[11px]">
                    {line.line_number}
                  </span>
                  <pre className="whitespace-pre font-mono overflow-visible">
                    {renderHighlightedLine(line.content)}
                  </pre>
                </div>
              ))}
            </div>
          </div>

          {/* Panel Fragment B */}
          <div className="flex flex-col min-w-0">
            <div className="px-3 py-1.5 bg-slate-900/60 border-b border-slate-800/60 flex items-center justify-between text-xs font-mono">
              <span className="text-slate-300 truncate" title={fileB}>
                <span className="text-slate-500">{parsedB.directory}</span>
                <span className="font-bold text-indigo-300">{parsedB.filename}</span>
                <span className="text-slate-500 ml-1.5">
                  (L{startLineB}–{endLineB})
                </span>
              </span>
              <div className="flex items-center gap-1">
                <a
                  href={ideLinkB}
                  title={`Open in ${getEditorDisplayName(preferredEditor)} at line ${startLineB}`}
                  className="p-1 hover:bg-slate-800 text-slate-400 hover:text-indigo-300 rounded transition-colors"
                >
                  <ExternalLink className="w-3.5 h-3.5" />
                </a>
                <button
                  type="button"
                  onClick={() => handleCopyCode(snippetB, false)}
                  className="p-1 hover:bg-slate-800 text-slate-400 hover:text-slate-200 rounded transition-colors"
                  title="Copy duplicate code"
                >
                  {copiedB ? (
                    <Check className="w-3.5 h-3.5 text-emerald-400" />
                  ) : (
                    <Copy className="w-3.5 h-3.5" />
                  )}
                </button>
              </div>
            </div>
            <div
              ref={scrollRefB}
              onScroll={handleScrollB}
              className="max-h-80 overflow-x-auto overflow-y-auto p-2 font-mono text-xs leading-relaxed select-text"
            >
              {snippetB?.lines.map((line) => (
                <div
                  key={`b-${line.line_number}`}
                  className={`flex items-start gap-3 px-2 py-0.5 rounded transition-colors min-w-fit ${
                    line.is_target
                      ? "bg-indigo-950/40 text-indigo-100 border-l-2 border-indigo-500"
                      : "text-slate-400 hover:bg-slate-900/40"
                  }`}
                >
                  <span className="w-8 shrink-0 text-right select-none opacity-40 font-mono text-[11px]">
                    {line.line_number}
                  </span>
                  <pre className="whitespace-pre font-mono overflow-visible">
                    {renderHighlightedLine(line.content)}
                  </pre>
                </div>
              ))}
            </div>
          </div>
        </div>
      ) : (
        /* Unified View */
        <div className="divide-y divide-slate-800">
          <div className="p-3 bg-slate-900/40">
            <div className="text-xs font-mono text-indigo-400 font-semibold pb-2 flex items-center justify-between">
              <span>
                Fragment A: {parsedA.filename} (L{startLineA}–{endLineA})
              </span>
              <button
                type="button"
                onClick={() => handleCopyCode(snippetA, true)}
                className="text-slate-400 hover:text-slate-200 text-[11px] flex items-center gap-1"
              >
                <Copy className="w-3 h-3" />
                Copy
              </button>
            </div>
            <div className="max-h-60 overflow-x-auto overflow-y-auto p-2 font-mono text-xs leading-relaxed bg-slate-950 rounded-lg border border-slate-800/80">
              {snippetA?.lines.map((line) => (
                <div
                  key={`ua-${line.line_number}`}
                  className={`flex items-start gap-3 px-2 py-0.5 rounded min-w-fit ${
                    line.is_target
                      ? "bg-indigo-950/40 text-indigo-100 border-l-2 border-indigo-500"
                      : "text-slate-400"
                  }`}
                >
                  <span className="w-8 shrink-0 text-right select-none opacity-40 font-mono text-[11px]">
                    {line.line_number}
                  </span>
                  <pre className="whitespace-pre font-mono overflow-visible">
                    {renderHighlightedLine(line.content)}
                  </pre>
                </div>
              ))}
            </div>
          </div>

          <div className="p-3 bg-slate-900/40">
            <div className="text-xs font-mono text-indigo-400 font-semibold pb-2 flex items-center justify-between">
              <span>
                Fragment B: {parsedB.filename} (L{startLineB}–{endLineB})
              </span>
              <button
                type="button"
                onClick={() => handleCopyCode(snippetB, false)}
                className="text-slate-400 hover:text-slate-200 text-[11px] flex items-center gap-1"
              >
                <Copy className="w-3 h-3" />
                Copy
              </button>
            </div>
            <div className="max-h-60 overflow-x-auto overflow-y-auto p-2 font-mono text-xs leading-relaxed bg-slate-950 rounded-lg border border-slate-800/80">
              {snippetB?.lines.map((line) => (
                <div
                  key={`ub-${line.line_number}`}
                  className={`flex items-start gap-3 px-2 py-0.5 rounded min-w-fit ${
                    line.is_target
                      ? "bg-indigo-950/40 text-indigo-100 border-l-2 border-indigo-500"
                      : "text-slate-400"
                  }`}
                >
                  <span className="w-8 shrink-0 text-right select-none opacity-40 font-mono text-[11px]">
                    {line.line_number}
                  </span>
                  <pre className="whitespace-pre font-mono overflow-visible">
                    {renderHighlightedLine(line.content)}
                  </pre>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
