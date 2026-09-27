import { useState, useCallback, useRef } from "react";
import { API_ROUTES } from "../constants/cddm-constants";

export interface RefactorStreamParams {
  fileA: string;
  startLineA: number;
  endLineA: number;
  fileB: string;
  startLineB: number;
  endLineB: number;
  provider?: string;
  model?: string;
}

export function useRefactorStream() {
  const [streamingPatch, setStreamingPatch] = useState<string>("");
  const [isStreaming, setIsStreaming] = useState<boolean>(false);
  const [streamError, setStreamError] = useState<string | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const startStream = useCallback(async (params: RefactorStreamParams) => {
    setIsStreaming(true);
    setStreamError(null);
    setStreamingPatch("");

    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }
    const controller = new AbortController();
    abortControllerRef.current = controller;

    try {
      const res = await fetch(API_ROUTES.REFACTOR_STREAM, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        signal: controller.signal,
        body: JSON.stringify({
          file_a: params.fileA,
          start_line_a: params.startLineA,
          end_line_a: params.endLineA,
          file_b: params.fileB,
          start_line_b: params.startLineB,
          end_line_b: params.endLineB,
          provider: params.provider,
          model: params.model,
        }),
      });

      if (!res.ok) {
        const errText = await res.text().catch(() => res.statusText);
        throw new Error(`Streaming failed (${res.status}): ${errText}`);
      }

      const reader = res.body?.getReader();
      if (!reader) {
        throw new Error("Response body is not readable");
      }

      const decoder = new TextDecoder();
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split("\n");
        buffer = lines.pop() ?? "";

        for (const line of lines) {
          const trimmed = line.trim();
          if (trimmed.startsWith("data:")) {
            const dataStr = trimmed.slice(5).trim();
            if (!dataStr) continue;
            try {
              const parsed = JSON.parse(dataStr) as {
                chunk?: string;
                error?: string;
                done?: boolean;
              };
              if (parsed.chunk) {
                setStreamingPatch((prev) => prev + parsed.chunk);
              } else if (parsed.error) {
                setStreamError(parsed.error);
              }
            } catch {
              // Ignore non-json chunks
            }
          }
        }
      }
    } catch (err: unknown) {
      if ((err as Error).name !== "AbortError") {
        setStreamError(err instanceof Error ? err.message : "Stream error");
      }
    } finally {
      setIsStreaming(false);
    }
  }, []);

  const stopStream = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
    setIsStreaming(false);
  }, []);

  const resetStream = useCallback(() => {
    stopStream();
    setStreamingPatch("");
    setStreamError(null);
  }, [stopStream]);

  return {
    streamingPatch,
    isStreaming,
    streamError,
    startStream,
    stopStream,
    resetStream,
  };
}
