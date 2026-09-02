import { describe, expect, it } from "bun:test";
import {
  countFileLines,
  GRANDFATHERED_LINE_CAPS,
  MAX_STANDARD_FILE_LINES,
  scanCodebaseFileLengths,
} from "../check-file-length";

describe("File Length & Modularity Policy Validator", () => {
  it("should have standard maximum file lines defined at 500", () => {
    expect(MAX_STANDARD_FILE_LINES).toBe(500);
  });

  it("should count lines accurately for files", () => {
    const packageJsonLines = countFileLines("package.json");
    expect(packageJsonLines).toBeGreaterThan(30);
    expect(packageJsonLines).toBeLessThan(100);
  });

  it("should return 0 lines for non-existent file", () => {
    expect(countFileLines("non_existent_file_12345.xyz")).toBe(0);
  });

  it("should have exactly two grandfathered exemptions for data-heavy files", () => {
    expect(GRANDFATHERED_LINE_CAPS).toBeDefined();
    expect(Object.keys(GRANDFATHERED_LINE_CAPS).length).toBe(2);
  });

  it("should verify that the entire repository currently passes file length policy", () => {
    const summary = scanCodebaseFileLengths(".");
    expect(summary.violations).toEqual([]);
    expect(summary.violations.length).toBe(0);
    expect(summary.filesChecked).toBeGreaterThan(100);
  });

  it("should identify violations when a file exceeds custom standard ceiling", () => {
    // If standard max lines was set very low (e.g., 50 lines), it should flag files
    const customSummary = scanCodebaseFileLengths(
      "scripts",
      process.cwd(),
      new Set([".git", "node_modules"]),
      new Set([".ts"]),
      {}, // no grandfathered exemptions
      50, // standard ceiling 50 lines
    );

    expect(customSummary.violations.length).toBeGreaterThan(0);
    const hasViolation = customSummary.violations.some((v) => v.actualLines > 50);
    expect(hasViolation).toBe(true);
  });

  it("should flag grandfathered files if they exceed their ratchet ceiling", () => {
    // Test with artificially lower ratchet caps
    const strictCaps: Record<string, number> = {
      "crates/cddm-cli/src/main.rs": 100, // actual is >2400
    };

    const summary = scanCodebaseFileLengths(
      "crates/cddm-cli/src",
      process.cwd(),
      new Set([".git", "node_modules", "target"]),
      new Set([".rs"]),
      strictCaps,
      500,
    );

    const mainViolation = summary.violations.find((v) =>
      v.file.includes("crates/cddm-cli/src/main.rs"),
    );
    expect(mainViolation).toBeDefined();
    expect(mainViolation?.isGrandfathered).toBe(true);
    expect(mainViolation?.maxAllowedLines).toBe(100);
  });
});
