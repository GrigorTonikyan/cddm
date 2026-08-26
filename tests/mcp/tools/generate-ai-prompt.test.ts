import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_generate_ai_prompt", () => {
  it("should generate LLM refactoring prompt with occurrence snippets", async () => {
    const res = await executeTool("cddm_generate_ai_prompt", {
      function_name: "compute_total",
      target_module: "src/calc.rs",
      occurrences: [
        { file: "crates/cddm-cli/src/commands/diff.rs", start_line: 33, end_line: 50 },
        { file: "crates/cddm-cli/src/commands/scan.rs", start_line: 43, end_line: 60 },
      ],
      target_language: "Rust",
    });

    expect(typeof res === "string" || typeof res?.prompt === "string").toBe(true);
    const text = typeof res === "string" ? res : res.prompt;
    expect(text.length).toBeGreaterThan(100);
    expect(text).toContain("compute_total");
  });

  it("should generate prompt with default values when minimal args provided", async () => {
    const res = await executeTool("cddm_generate_ai_prompt", {});
    expect(typeof res === "string" || typeof res?.prompt === "string").toBe(true);
  });
});
