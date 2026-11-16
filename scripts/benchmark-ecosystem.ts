#!/usr/bin/env bun
/**
 * Reproducible cross-tool ecosystem benchmark runner for CDDM.
 * Compares CDDM (cold cache, warm cache, Type-1/2/3) against jscpd and optional PMD CPD
 * across a deterministic polyglot corpus and repository self-scan.
 */

import { existsSync } from "node:fs";
import { mkdir } from "node:fs/promises";
import { arch, cpus, platform, tmpdir, totalmem } from "node:os";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import {
  formatBenchmarkMarkdown,
  generatePolyglotCorpus,
  type BenchmarkEnvironment,
  type BenchmarkReport,
  type ToolBenchmarkMeasurement,
} from "./lib/benchmark-corpus";
import {
  captureCommandVersion,
  measureJscpdModes,
  measureSeries,
  parseCddmJsonMetrics,
  runTimedCommand,
} from "./lib/benchmark-runner";
import { printScriptBanner, printScriptHelp } from "./lib/step-runner";

export { extractFirstJsonObject, normalizeRssMb } from "./lib/benchmark-runner";

export interface CliOptions {
  corpusDir: string;
  filesPerLang: number;
  minTokens: number;
  runs: number;
  withJscpd: boolean;
  withPmd: boolean;
  selfScan: boolean;
  cddmBin: string;
  write: boolean;
  json: boolean;
  help: boolean;
}

export function resolveCddmBinary(explicit?: string): string {
  if (explicit) return explicit;
  const exe = process.platform === "win32" ? "cddm.exe" : "cddm";
  const releaseCandidate = resolve("target", "release", exe);
  if (existsSync(releaseCandidate)) return releaseCandidate;
  const debugCandidate = resolve("target", "debug", exe);
  if (existsSync(debugCandidate)) return debugCandidate;
  return "cddm";
}

export function parseBenchmarkArgs(argv: string[]): CliOptions {
  const defaultCorpusDir = join(tmpdir(), "cddm-benchmark-corpus");
  const { values } = parseArgs({
    args: argv,
    options: {
      "corpus-dir": { type: "string", default: defaultCorpusDir },
      "files-per-lang": { type: "string", default: "60" },
      "min-tokens": { type: "string", default: "50" },
      runs: { type: "string", default: "3" },
      "with-jscpd": { type: "boolean", default: false },
      "with-pmd": { type: "boolean", default: false },
      "self-scan": { type: "boolean", default: false },
      "cddm-bin": { type: "string" },
      write: { type: "boolean", default: false },
      json: { type: "boolean", default: false },
      help: { type: "boolean", short: "h", default: false },
    },
  });

  const filesPerLang = Math.max(1, Number.parseInt(values["files-per-lang"] ?? "60", 10) || 60);
  const minTokens = Math.max(10, Number.parseInt(values["min-tokens"] ?? "50", 10) || 50);
  const runs = Math.max(1, Number.parseInt(values.runs ?? "3", 10) || 3);

  return {
    corpusDir: resolve(values["corpus-dir"] ?? defaultCorpusDir),
    filesPerLang,
    minTokens,
    runs,
    withJscpd: Boolean(values["with-jscpd"]),
    withPmd: Boolean(values["with-pmd"]),
    selfScan: Boolean(values["self-scan"]),
    cddmBin: resolveCddmBinary(values["cddm-bin"]),
    write: Boolean(values.write),
    json: Boolean(values.json),
    help: Boolean(values.help),
  };
}

async function countRepoLines(rootDir: string): Promise<{ files: number; lines: number }> {
  const proc = Bun.spawn(["git", "ls-files"], { cwd: rootDir, stdout: "pipe", stderr: "pipe" });
  const stdout = await new Response(proc.stdout).text();
  await proc.exited;
  const relPaths = stdout
    .split(/\r?\n/)
    .map((p) => p.trim())
    .filter((p) => /\.(rs|ts|tsx|js|jsx|css|py|go)$/i.test(p));

  let lines = 0;
  let files = 0;
  for (const rel of relPaths) {
    const abs = join(rootDir, rel);
    if (!existsSync(abs)) continue;
    const text = await Bun.file(abs).text();
    lines += text.split("\n").length;
    files++;
  }
  return { files, lines };
}

async function resolveJscpdInvocation(): Promise<{ cmd: string[]; version: string | null }> {
  const whichName = process.platform === "win32" ? "jscpd.exe" : "jscpd";
  const directVer = await captureCommandVersion([whichName, "--version"]);
  if (directVer) return { cmd: [whichName], version: directVer };
  const bunxVer = await captureCommandVersion(["bunx", "--no-install", "jscpd", "--version"]);
  if (bunxVer) return { cmd: ["bunx", "--no-install", "jscpd"], version: bunxVer };
  return { cmd: [], version: null };
}

export async function runBenchmarkSuite(options: CliOptions): Promise<BenchmarkReport> {
  const cddmVersion = (await captureCommandVersion([options.cddmBin, "--version"])) ?? "unknown";
  const jscpdProbe = options.withJscpd
    ? await resolveJscpdInvocation()
    : { cmd: [], version: null };
  const pmdVersion = options.withPmd ? await captureCommandVersion(["pmd", "--version"]) : null;

  const cpuList = cpus();
  const env: BenchmarkEnvironment = {
    os: `${platform()}`,
    arch: `${arch()}`,
    cpuModel: cpuList[0]?.model?.trim() || "Unknown CPU",
    logicalCores: cpuList.length,
    totalRamGb: totalmem() / (1024 * 1024 * 1024),
    bunVersion: Bun.version,
    cddmVersion,
    jscpdVersion: jscpdProbe.version,
    pmdVersion,
    capturedAtIso: new Date().toISOString(),
  };

  const corpus = await generatePolyglotCorpus(options.corpusDir, options.filesPerLang);
  const cacheDir = join(options.corpusDir, ".bench-cache");
  await mkdir(cacheDir, { recursive: true });

  const corpusSrc = join(corpus.corpusDir, "src");
  const baseScanFlags = [
    "scan",
    corpusSrc,
    "-m",
    String(options.minTokens),
    "--include-ignored",
    "-q",
    "-f",
    "json",
  ];

  const corpusResults: ToolBenchmarkMeasurement[] = [
    await measureSeries({
      tool: "CDDM",
      version: cddmVersion,
      mode: "Cold scan (Type-1/2/3)",
      flags: `cddm scan <corpus> -m ${options.minTokens} --no-cache -f json`,
      totalLines: corpus.totalLines,
      runs: options.runs,
      notes: "Full parse + SIMD M61 winnowing + Type-3 merge",
      executeRun: () =>
        runTimedCommand([options.cddmBin, ...baseScanFlags, "--no-cache"], parseCddmJsonMetrics),
    }),
    await measureSeries({
      tool: "CDDM",
      version: cddmVersion,
      mode: "Warm redb cache (Type-1/2/3)",
      flags: `cddm scan <corpus> -m ${options.minTokens} --cache-dir <dir> -f json`,
      totalLines: corpus.totalLines,
      runs: options.runs,
      notes: "Incremental SHA-256 + redb mmap cache hit",
      warmupCmd: async () => {
        await runTimedCommand(
          [options.cddmBin, ...baseScanFlags, "--cache-dir", cacheDir],
          parseCddmJsonMetrics,
        );
      },
      executeRun: () =>
        runTimedCommand(
          [options.cddmBin, ...baseScanFlags, "--cache-dir", cacheDir],
          parseCddmJsonMetrics,
        ),
    }),
  ];

  if (options.withJscpd) {
    if (jscpdProbe.version && jscpdProbe.cmd.length > 0) {
      corpusResults.push(
        ...(await measureJscpdModes({
          cmd: jscpdProbe.cmd,
          version: jscpdProbe.version,
          target: corpusSrc,
          reportDir: join(options.corpusDir, ".jscpd-out"),
          minTokens: options.minTokens,
          totalLines: corpus.totalLines,
          runs: options.runs,
          modeLabelPrefix: "Corpus ",
          flagTargetLabel: "<corpus>/src",
        })),
      );
    } else {
      corpusResults.push({
        tool: "jscpd",
        version: "N/A",
        mode: "Exact + Renamed",
        runsMs: [],
        medianMs: 0,
        minMs: 0,
        maxMs: 0,
        locPerSecond: 0,
        clonesDetected: null,
        duplicationPercent: null,
        peakRssMb: null,
        flags: `jscpd <corpus>/src -k ${options.minTokens}`,
        notes: "Not installed in benchmark environment",
        available: false,
      });
    }
  }

  if (options.withPmd) {
    corpusResults.push(
      pmdVersion
        ? await measureSeries({
            tool: "PMD CPD",
            version: pmdVersion,
            mode: "Minimum tokens scan",
            flags: `pmd cpd --minimum-tokens ${options.minTokens} --dir <corpus>/src`,
            totalLines: corpus.totalLines,
            runs: options.runs,
            executeRun: () =>
              runTimedCommand(
                ["pmd", "cpd", "--minimum-tokens", String(options.minTokens), "--dir", corpusSrc],
                () => ({ clonesDetected: null, duplicationPercent: null }),
                true,
              ),
          })
        : {
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
            flags: `pmd cpd --minimum-tokens ${options.minTokens}`,
            notes: "Not installed in benchmark environment",
            available: false,
          },
    );
  }

  let selfScan: BenchmarkReport["selfScan"] = null;
  if (options.selfScan) {
    const repoStats = await countRepoLines(process.cwd());
    const repoCacheDir = join(options.corpusDir, ".self-scan-cache");
    await mkdir(repoCacheDir, { recursive: true });
    const repoFlags = ["scan", ".", "-m", String(options.minTokens), "-q", "-f", "json"];

    const selfMeasurements: ToolBenchmarkMeasurement[] = [
      await measureSeries({
        tool: "CDDM",
        version: cddmVersion,
        mode: "Self-scan cold (--no-cache)",
        flags: `cddm scan . -m ${options.minTokens} --no-cache -f json`,
        totalLines: repoStats.lines,
        runs: options.runs,
        executeRun: () =>
          runTimedCommand([options.cddmBin, ...repoFlags, "--no-cache"], parseCddmJsonMetrics),
      }),
      await measureSeries({
        tool: "CDDM",
        version: cddmVersion,
        mode: "Self-scan warm (redb cache)",
        flags: `cddm scan . -m ${options.minTokens} --cache-dir <dir> -f json`,
        totalLines: repoStats.lines,
        runs: options.runs,
        warmupCmd: async () => {
          await runTimedCommand(
            [options.cddmBin, ...repoFlags, "--cache-dir", repoCacheDir],
            parseCddmJsonMetrics,
          );
        },
        executeRun: () =>
          runTimedCommand(
            [options.cddmBin, ...repoFlags, "--cache-dir", repoCacheDir],
            parseCddmJsonMetrics,
          ),
      }),
    ];

    if (options.withJscpd && jscpdProbe.version && jscpdProbe.cmd.length > 0) {
      selfMeasurements.push(
        ...(await measureJscpdModes({
          cmd: jscpdProbe.cmd,
          version: jscpdProbe.version,
          target: ".",
          reportDir: join(options.corpusDir, ".jscpd-self-out"),
          minTokens: options.minTokens,
          totalLines: repoStats.lines,
          runs: options.runs,
          ignoreGlob: "**/node_modules/**,**/dist/**,**/target/**,**/.git/**,**/coverage/**",
          modeLabelPrefix: "Repository ",
          flagTargetLabel: ". -i <ignored>",
        })),
      );
    }

    selfScan = {
      repoFiles: repoStats.files,
      repoLines: repoStats.lines,
      measurements: selfMeasurements,
    };
  }

  return { environment: env, corpus, corpusResults, selfScan };
}

async function main(): Promise<void> {
  const options = parseBenchmarkArgs(process.argv.slice(2));
  if (options.help) {
    printScriptHelp("Ecosystem Benchmark Suite", "bun scripts/benchmark-ecosystem.ts [options]", [
      ["--corpus-dir <dir>", "Target directory for generated corpus (default: OS temp dir)"],
      ["--files-per-lang <n>", "Files per language across 4 languages (default: 60)"],
      ["--min-tokens <n>", "Minimum clone token threshold (default: 50)"],
      ["--runs <n>", "Measured iterations per mode (default: 3)"],
      ["--with-jscpd", "Include jscpd comparison runs when available"],
      ["--with-pmd", "Include PMD CPD check/comparison when available"],
      ["--self-scan", "Benchmark scanning this repository cleanly"],
      ["--write", "Write Markdown results directly to docs/BENCHMARKS.md"],
      ["--json", "Output full JSON measurement payload"],
      ["-h, --help", "Show this help message"],
    ]);
    return;
  }

  if (!options.json) {
    printScriptBanner("CDDM Ecosystem Benchmark Suite");
  }

  const report = await runBenchmarkSuite(options);
  if (options.write) {
    const md = formatBenchmarkMarkdown(report);
    const outPath = resolve("docs", "BENCHMARKS.md");
    await Bun.write(outPath, md);
    if (!options.json) {
      console.log(`[OK] Wrote benchmark report to ${outPath}`);
    }
  }

  if (options.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    for (const m of report.corpusResults) {
      const status = m.available
        ? `${m.medianMs.toFixed(1)} ms (${Math.round(m.locPerSecond)} LOC/s)`
        : "N/A";
      console.log(
        `- ${m.tool} [${m.mode}]: ${status} | clones=${m.clonesDetected ?? "N/A"} | rss=${m.peakRssMb ?? "N/A"} MB`,
      );
    }
    if (report.selfScan) {
      for (const m of report.selfScan.measurements) {
        console.log(
          `- ${m.tool} [${m.mode}]: ${m.medianMs.toFixed(1)} ms (${Math.round(m.locPerSecond)} LOC/s) | clones=${m.clonesDetected ?? "N/A"} | rss=${m.peakRssMb ?? "N/A"} MB`,
        );
      }
    }
  }
}

if (import.meta.main) {
  await main();
}
