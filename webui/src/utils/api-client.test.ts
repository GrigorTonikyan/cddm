import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { getJson, postJson } from "./api-client";

describe("api-client", () => {
  const originalFetch = globalThis.fetch;

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  it("should perform postJson successfully", async () => {
    const mockData = { success: true };
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });

    const result = await postJson("/api/test", { a: 1 }, "Failed test");
    expect(result).toEqual(mockData);
    expect(globalThis.fetch).toHaveBeenCalledWith("/api/test", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ a: 1 }),
    });
  });

  it("should throw formatted error on postJson failure", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
      text: async () => "Internal Server Error",
    });

    await expect(postJson("/api/fail", {}, "Operation failed")).rejects.toThrow(
      "Operation failed (500): Internal Server Error",
    );
  });

  it("should perform getJson successfully", async () => {
    const mockData = { status: "ready" };
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });

    const result = await getJson("/api/status", "Failed status check");
    expect(result).toEqual(mockData);
    expect(globalThis.fetch).toHaveBeenCalledWith("/api/status");
  });

  it("should throw formatted error on getJson failure", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
      statusText: "Not Found",
      text: async () => {
        throw new Error("Cannot read text");
      },
    });

    await expect(getJson("/api/missing", "Fetch failed")).rejects.toThrow(
      "Fetch failed (404): Not Found",
    );
  });
});
