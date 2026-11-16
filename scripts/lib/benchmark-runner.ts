/**
 * Shared subprocess execution and timing utilities for CDDM ecosystem benchmarks.
 */

import { existsSync } from "node:fs";
import { mkdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { computeMedian, type ToolBenchmarkMeasurement } from "./benchmark-corpus";

export interface ParsedToolMetrics {
  clonesDetected: number | null;
  duplicationPercent: number | null;
}

export function extractFirstJsonObject(raw: string): Record<string, unknown> | null {
  const idx = raw.indexOf("{");
  if (idx === -1) return null;
  try {
    return JSON.parse(raw.slice(idx)) as Record<string, unknown>;
  } catch {
    return null;
  }
}

export function normalizeRssMb(maxRss: number | undefined, osPlatform: string): number | null {
  if (!maxRss || maxRss <= 0) return null;
  // Darwin & Win32 report maxRSS in bytes; Linux getrusage().ru_maxrss reports KiB
  if (osPlatform === "darwin" || osPlatform === "win32") {
    return Math.round((maxRss / (1024 * 1024)) * 10) / 10;
  }
  return Math.round((maxRss / 1024) * 10) / 10;
}

export async function captureCommandVersion(command: string[]): Promise<string | null> {
  try {
    const proc = Bun.spawn(command, { stdout: "pipe", stderr: "pipe" });
    const [stdout, exitCode] = await Promise.all([new Response(proc.stdout).text(), proc.exited]);
    if (exitCode !== 0) return null;
    const firstLine = stdout.trim().split(/\r?\n/)[0]?.trim();
    return firstLine || null;
  } catch {
    return null;
  }
}

export async function runTimedCommand(
  cmd: string[],
  parseStdout: (stdout: string) => Promise<ParsedToolMetrics> | ParsedToolMetrics,
  allowExitOne = false,
): Promise<{ elapsedMs: number; rssMb: number | null; metrics: ParsedToolMetrics }> {
  const start = performance.now();
  const proc = Bun.spawn(cmd, {
    stdout: "pipe",
    stderr: "pipe",
    env: { ...process.env, JSCPD_NO_TIPS: "1" },
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  const elapsedMs = Math.round((performance.now() - start) * 100) / 100;
  if (exitCode !== 0 && !(allowExitOne && exitCode === 1)) {
    throw new Error(`Command failed (${exitCode}): ${cmd.join(" ")}\n${stderr}`);
  }
  const rssMb = normalizeRssMb(proc.resourceUsage()?.maxRSS, process.platform);
  const metrics = await parseStdout(stdout);
  return { elapsedMs, rssMb, metrics };
}

export function parseCddmJsonMetrics(stdout: string): ParsedToolMetrics {
  const parsed = extractFirstJsonObject(stdout);
  if (!parsed) return { clonesDetected: null, duplicationPercent: null };
  const clones = typeof parsed.total_clones === "number" ? parsed.total_clones : null;
  const dup =
    typeof parsed.duplication_percentage === "number"
      ? Math.round(parsed.duplication_percentage * 100) / 100
      : null;
  return { clonesDetected: clones, duplicationPercent: dup };
}

export async function parseJscpdReport(reportPath: string): Promise<ParsedToolMetrics> {
  if (!existsSync(reportPath)) return { clonesDetected: null, duplicationPercent: null };
  const raw = await Bun.file(reportPath).text();
  const parsed = JSON.parse(raw) as {
    duplicates?: unknown[];
    statistics?: { total?: { clones?: number; percentage?: number } };
  };
  const clones =
    parsed.statistics?.total?.clones ??
    (Array.isArray(parsed.duplicates) ? parsed.duplicates.length : null);
  const dup =
    typeof parsed.statistics?.total?.percentage === "number"
      ? Math.round(parsed.statistics.total.percentage * 100) / 100
      : null;
  return { clonesDetected: clones, duplicationPercent: dup };
}

export async function measureSeries(params: {
  tool: string;
  version: string;
  mode: string;
  flags: string;
  totalLines: number;
  runs: number;
  notes?: string;
  warmupCmd?: () => Promise<void>;
  executeRun: () => Promise<{
    elapsedMs: number;
    rssMb: number | null;
    metrics: ParsedToolMetrics;
  }>;
}): Promise<ToolBenchmarkMeasurement> {
  if (params.warmupCmd) {
    await params.warmupCmd();
  }
  const runsMs: number[] = [];
  let maxRssMb: number | null = null;
  let lastMetrics: ParsedToolMetrics = { clonesDetected: null, duplicationPercent: null };

  for (let i = 0; i < params.runs; i++) {
    const res = await params.executeRun();
    runsMs.push(res.elapsedMs);
    lastMetrics = res.metrics;
    if (res.rssMb !== null) {
      maxRssMb = maxRssMb === null ? res.rssMb : Math.max(maxRssMb, res.rssMb);
    }
  }

  const medianMs = computeMedian(runsMs);
  return {
    tool: params.tool,
    version: params.version,
    mode: params.mode,
    runsMs,
    medianMs,
    minMs: Math.min(...runsMs),
    maxMs: Math.max(...runsMs),
    locPerSecond: medianMs > 0 ? (params.totalLines / medianMs) * 1000 : 0,
    clonesDetected: lastMetrics.clonesDetected,
    duplicationPercent: lastMetrics.duplicationPercent,
    peakRssMb: maxRssMb,
    flags: params.flags,
    notes: params.notes,
    available: true,
  };
}

export async function measureJscpdModes(params: {
  cmd: string[];
  version: string;
  target: string;
  reportDir: string;
  minTokens: number;
  totalLines: number;
  runs: number;
  ignoreGlob?: string;
  modeLabelPrefix: string;
  flagTargetLabel: string;
}): Promise<ToolBenchmarkMeasurement[]> {
  await mkdir(params.reportDir, { recursive: true });
  const reportFile = join(params.reportDir, "jscpd-report.json");
  const ignoreArgs = params.ignoreGlob ? ["-i", params.ignoreGlob] : [];

  const modes: Array<{ mode: string; extraArgs: string[]; flagSuffix: string; notes: string }> = [
    {
      mode: `${params.modeLabelPrefix}exact clones`,
      extraArgs: [],
      flagSuffix: "-r json -s",
      notes: "Rabin-Karp exact token matching",
    },
    {
      mode: `${params.modeLabelPrefix}renamed (--ignore-identifiers)`,
      extraArgs: ["--ignore-identifiers"],
      flagSuffix: "--ignore-identifiers -r json -s",
      notes: "Type-1 + consistent identifier-normalized Type-2",
    },
  ];

  const results: ToolBenchmarkMeasurement[] = [];
  for (const variant of modes) {
    results.push(
      await measureSeries({
        tool: "jscpd",
        version: params.version,
        mode: variant.mode,
        flags: `jscpd ${params.flagTargetLabel} -k ${params.minTokens} ${variant.flagSuffix}`,
        totalLines: params.totalLines,
        runs: params.runs,
        notes: variant.notes,
        executeRun: async () => {
          await rm(reportFile, { force: true });
          return runTimedCommand(
            [
              ...params.cmd,
              params.target,
              "-k",
              String(params.minTokens),
              ...variant.extraArgs,
              ...ignoreArgs,
              "-r",
              "json",
              "-o",
              params.reportDir,
              "--report-name",
              "jscpd-report",
              "-s",
              "--no-tips",
            ],
            () => parseJscpdReport(reportFile),
          );
        },
      }),
    );
  }
  return results;
}
