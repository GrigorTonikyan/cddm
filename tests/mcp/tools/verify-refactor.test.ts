import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_verify_refactor", () => {
  it("should run verification test suite with timeout", async () => {
    const res = await executeTool("cddm_verify_refactor", {
      directory: ".",
      test_command: "bun --version",
      timeout_seconds: 10,
    });

    expect(res).toBeDefined();
    expect(res.success).toBe(true);
    expect(res.exit_code).toBe(0);
    expect(typeof res.stdout_snippet).toBe("string");
  });
});
