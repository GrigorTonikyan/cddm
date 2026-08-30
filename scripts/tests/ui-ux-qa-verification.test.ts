import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

describe("WebUI Studio UI/UX Verification Configuration & Suite Registry", () => {
  it("should have all required Playwright E2E browser specifications in tests/e2e", () => {
    const e2eDir = join(process.cwd(), "tests", "e2e");
    const requiredSpecs = [
      "comprehensive-ui-quality.spec.ts",
      "scan-workflow.spec.ts",
      "semantic-graph-live-watch.spec.ts",
      "browser-multi-project.spec.ts",
      "win2x-manager-desktop.spec.ts",
    ];

    for (const spec of requiredSpecs) {
      const specPath = join(e2eDir, spec);
      expect(existsSync(specPath)).toBe(true);
      const content = readFileSync(specPath, "utf-8");
      expect(content).toContain("test.describe");
      expect(content).toContain("expect");
    }
  });

  it("should have valid Playwright configuration for Desktop Chrome", () => {
    const configPath = join(process.cwd(), "tests", "e2e", "playwright.config.ts");
    expect(existsSync(configPath)).toBe(true);
    const content = readFileSync(configPath, "utf-8");
    expect(content).toContain("defineConfig");
    expect(content).toContain("baseURL");
    expect(content).toContain("chromium");
  });

  it("should enforce zero console error assertions in comprehensive browser spec", () => {
    const specPath = join(process.cwd(), "tests", "e2e", "comprehensive-ui-quality.spec.ts");
    const content = readFileSync(specPath, "utf-8");
    expect(content).toContain("consoleErrors");
    expect(content).toContain("expect(consoleErrors).toEqual([])");
  });
});
