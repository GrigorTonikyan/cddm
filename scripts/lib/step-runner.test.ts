import { describe, it, expect } from "bun:test";
import { printScriptBanner, printScriptHelp, executeStep } from "./step-runner";

describe("step-runner utility", () => {
  it("prints script banner without crashing", () => {
    let captured = "";
    const origLog = console.log;
    console.log = (...args: unknown[]) => {
      captured += args.join(" ") + "\n";
    };

    try {
      printScriptBanner("TEST BANNER");
      expect(captured).toContain("TEST BANNER");
      expect(captured).toContain("=======================================================");
    } finally {
      console.log = origLog;
    }
  });

  it("prints script help options properly", () => {
    let captured = "";
    const origLog = console.log;
    console.log = (...args: unknown[]) => {
      captured += args.join(" ") + "\n";
    };

    try {
      printScriptHelp("TestTool", "bun scripts/test.ts [options]", [
        ["--flag", "Sample flag description"],
        ["--help", "Show help"],
      ]);
      expect(captured).toContain("CDDM TestTool");
      expect(captured).toContain("bun scripts/test.ts [options]");
      expect(captured).toContain("--flag");
      expect(captured).toContain("Sample flag description");
    } finally {
      console.log = origLog;
    }
  });

  it("executes a passing command step successfully", async () => {
    const result = await executeStep(
      {
        title: "Echo Test",
        command: ["bun", "-e", "process.exit(0)"],
        allowFailure: false,
      },
      0,
      1,
    );

    expect(result.passed).toBe(true);
    expect(result.exitCode).toBe(0);
    expect(result.elapsedMs).toBeGreaterThanOrEqual(0);
  });

  it("handles an allowed failure command step cleanly", async () => {
    const result = await executeStep(
      {
        title: "Allowed Failure Test",
        command: ["bun", "-e", "process.exit(2)"],
        allowFailure: true,
      },
      0,
      1,
    );

    expect(result.passed).toBe(false);
    expect(result.exitCode).toBe(2);
  });
});
