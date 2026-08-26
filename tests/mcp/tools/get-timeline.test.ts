import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_get_timeline", () => {
  it("should retrieve historical timeline snapshots", async () => {
    const res = await executeTool("cddm_get_timeline", {
      directory: ".",
      max_samples: 3,
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.snapshots)).toBe(true);
    expect(res.snapshots.length).toBeGreaterThan(0);
    expect(res.snapshots[0].commit_hash).toBeDefined();
    expect(typeof res.snapshots[0].duplication_percentage).toBe("number");
  });
});
