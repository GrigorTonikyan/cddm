import { describe, expect, test } from "bun:test";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { extractFirstJsonObject, normalizeRssMb, parseBenchmarkArgs } from "../benchmark-ecosystem";
import {
  computeMedian,
  formatBenchmarkMarkdown,
  generatePolyglotCorpus,
  type BenchmarkReport,
} from "../lib/benchmark-corpus";

describe("ecosystem benchmark suite", () => {
  test("parses CLI options deterministically with safe bounds", () => {
    const parsed = parseBenchmarkArgs([
      "--files-per-lang",
      "12",
      "--min-tokens",
      "65",
      "--runs",
      "5",
      "--with-jscpd",
      "--self-scan",
      "--write",
    ]);

    expect(parsed.filesPerLang).toBe(12);
    expect(parsed.minTokens).toBe(65);
    expect(parsed.runs).toBe(5);
    expect(parsed.withJscpd).toBe(true);
    expect(parsed.withPmd).toBe(false);
    expect(parsed.selfScan).toBe(true);
    expect(parsed.write).toBe(true);
  });

  test("generates deterministic polyglot corpus across 4 languages", async () => {
    const sandbox = mkdtempSync(join(tmpdir(), "cddm-bench-test-"));
    try {
      const summary = await generatePolyglotCorpus(sandbox, 6);
      expect(summary.totalFiles).toBe(24);
      expect(summary.byLanguage).toHaveLength(4);
      expect(summary.totalLines).toBeGreaterThan(500);
      expect(summary.totalBytes).toBeGreaterThan(10000);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("computes median and extracts JSON payloads cleanly", () => {
    expect(computeMedian([30, 10, 20])).toBe(20);
    expect(computeMedian([40, 10, 20, 30])).toBe(25);
    expect(computeMedian([])).toBe(0);

    const parsed = extractFirstJsonObject(
      'INFO booting...\n{"total_clones": 14, "dry_health_score": 91.2}',
    );
    expect(parsed?.total_clones).toBe(14);
    expect(extractFirstJsonObject("no json here")).toBeNull();

    expect(normalizeRssMb(undefined, "linux")).toBeNull();
    expect(normalizeRssMb(0, "win32")).toBeNull();
    expect(normalizeRssMb(1024 * 64, "linux")).toBe(64);
    expect(normalizeRssMb(64 * 1024 * 1024, "darwin")).toBe(64);
    expect(normalizeRssMb(12 * 1024 * 1024, "win32")).toBe(12);
  });

  test("formats Markdown benchmark report cleanly with N/A handling", () => {
    const report: BenchmarkReport = {
      environment: {
        os: "linux",
        arch: "x64",
        cpuModel: "Reference CPU",
        logicalCores: 16,
        totalRamGb: 64,
        bunVersion: "1.4.4",
        cddmVersion: "cddm 4.11.0",
        jscpdVersion: "5.4.3",
        pmdVersion: null,
        capturedAtIso: "2026-11-16T00:00:00.000Z",
      },
      corpus: {
        corpusDir: "target/benchmark-corpus",
        totalFiles: 16,
        totalLines: 1200,
        totalBytes: 42000,
        byLanguage: [
          { language: "Rust", files: 4, lines: 300, bytes: 11000 },
          { language: "TypeScript", files: 4, lines: 300, bytes: 10000 },
        ],
      },
      corpusResults: [
        {
          tool: "CDDM",
          version: "cddm 4.11.0",
          mode: "Cold scan",
          runsMs: [18.2, 19.0, 21.1],
          medianMs: 19.0,
          minMs: 18.2,
          maxMs: 21.1,
          locPerSecond: 63158,
          clonesDetected: 4,
          duplicationPercent: 18.5,
          peakRssMb: 42.3,
          flags: "cddm scan <corpus> --no-cache",
          notes: "Full parse",
          available: true,
        },
        {
          tool: "PMD CPD",
          version: "N/A",
          mode: "CPD",
          runsMs: [],
          medianMs: 0,
          minMs: 0,
          maxMs: 0,
          locPerSecond: 0,
          clonesDetected: null,
          duplicationPercent: null,
          peakRssMb: null,
          flags: "pmd cpd --minimum-tokens 50",
          notes: "Not installed in benchmark environment",
          available: false,
        },
      ],
      selfScan: null,
    };

    const md = formatBenchmarkMarkdown(report);
    expect(md).toContain("# CDDM Reproducible Ecosystem Benchmarks");
    expect(md).toContain("_not installed in environment_");
    expect(md).toContain("19.0 ms");
    expect(md).toContain("42.3 MB");
    expect(md).toContain("| PMD CPD | N/A | CPD | N/A |");
  });
});
