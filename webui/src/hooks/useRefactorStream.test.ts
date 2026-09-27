import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { useRefactorStream } from "./useRefactorStream";

describe("useRefactorStream", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("initializes with default state", () => {
    const { result } = renderHook(() => useRefactorStream());
    expect(result.current.streamingPatch).toBe("");
    expect(result.current.isStreaming).toBe(false);
    expect(result.current.streamError).toBeNull();
  });

  it("handles successful stream response", async () => {
    const sseChunks = [
      'data: {"chunk":"--- a/src/test.rs\\n"}\n\n',
      'data: {"chunk":"+++ b/src/test.rs\\n"}\n\n',
      'data: {"chunk":"@@ -1,2 +1,2 @@\\n"}\n\n',
      'data: {"done":true}\n\n',
    ];

    const encoder = new TextEncoder();
    let chunkIndex = 0;
    const mockReader = {
      read: vi.fn().mockImplementation(() => {
        if (chunkIndex < sseChunks.length) {
          const value = encoder.encode(sseChunks[chunkIndex++]);
          return Promise.resolve({ done: false, value });
        }
        return Promise.resolve({ done: true, value: undefined });
      }),
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      body: {
        getReader: () => mockReader,
      },
    });

    const { result } = renderHook(() => useRefactorStream());

    await act(async () => {
      await result.current.startStream({
        fileA: "src/a.rs",
        startLineA: 1,
        endLineA: 10,
        fileB: "src/b.rs",
        startLineB: 1,
        endLineB: 10,
      });
    });

    expect(result.current.streamingPatch).toBe(
      "--- a/src/test.rs\n+++ b/src/test.rs\n@@ -1,2 +1,2 @@\n",
    );
    expect(result.current.isStreaming).toBe(false);
    expect(result.current.streamError).toBeNull();
  });

  it("handles stream error responses", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
      statusText: "Internal Error",
      text: () => Promise.resolve("Provider failure"),
    });

    const { result } = renderHook(() => useRefactorStream());

    await act(async () => {
      await result.current.startStream({
        fileA: "src/a.rs",
        startLineA: 1,
        endLineA: 10,
        fileB: "src/b.rs",
        startLineB: 1,
        endLineB: 10,
      });
    });

    expect(result.current.streamError).toContain("Streaming failed (500)");
    expect(result.current.isStreaming).toBe(false);
  });

  it("resets stream state", () => {
    const { result } = renderHook(() => useRefactorStream());

    act(() => {
      result.current.resetStream();
    });

    expect(result.current.streamingPatch).toBe("");
    expect(result.current.streamError).toBeNull();
    expect(result.current.isStreaming).toBe(false);
  });
});
