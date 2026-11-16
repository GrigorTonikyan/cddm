import { mkdir, rm } from "node:fs/promises";
import { join } from "node:path";

export interface CorpusLanguageStats {
  language: string;
  files: number;
  lines: number;
  bytes: number;
}

export interface CorpusSummary {
  corpusDir: string;
  totalFiles: number;
  totalLines: number;
  totalBytes: number;
  byLanguage: CorpusLanguageStats[];
}

export interface ToolBenchmarkMeasurement {
  tool: string;
  version: string;
  mode: string;
  runsMs: number[];
  medianMs: number;
  minMs: number;
  maxMs: number;
  locPerSecond: number;
  clonesDetected: number | null;
  duplicationPercent: number | null;
  peakRssMb: number | null;
  flags: string;
  notes?: string;
  available: boolean;
}

export interface BenchmarkEnvironment {
  os: string;
  arch: string;
  cpuModel: string;
  logicalCores: number;
  totalRamGb: number;
  bunVersion: string;
  cddmVersion: string;
  jscpdVersion: string | null;
  pmdVersion: string | null;
  capturedAtIso: string;
}

export interface BenchmarkReport {
  environment: BenchmarkEnvironment;
  corpus: CorpusSummary;
  corpusResults: ToolBenchmarkMeasurement[];
  selfScan: {
    repoFiles: number;
    repoLines: number;
    measurements: ToolBenchmarkMeasurement[];
  } | null;
}

interface LanguageTemplateSpec {
  id: string;
  label: string;
  ext: string;
  renderFile: (index: number) => string;
}

function mixBit(seed: number, step: number): number {
  let x = (Math.imul(seed + 1, 0x9e3779b1) ^ Math.imul(step + 1, 0x85ebca6b)) >>> 0;
  x ^= x >>> 13;
  x = Math.imul(x, 0xc2b2ae35) >>> 0;
  x ^= x >>> 16;
  return x >>> 0;
}

function buildCompactBlock(
  seed: number,
  stmtCount: number,
  factory: (stmtSeed: number, step: number) => string,
): string {
  const out: string[] = [];
  for (let s = 0; s < stmtCount; s++) {
    out.push(factory(mixBit(seed, s * 29 + 3), s));
  }
  return out.join("\n");
}

function renameIdentifiers(code: string, replacements: Array<[string, string]>): string {
  let updated = code;
  for (const [from, to] of replacements) {
    updated = updated.replaceAll(from, to);
  }
  return updated;
}

const SHORT_OPS = ["^=", "+=", "-=", "|=", "&=", "*="] as const;

function pickShortOp(stmtSeed: number): string {
  return SHORT_OPS[stmtSeed % SHORT_OPS.length] ?? "^=";
}

function rustCompactStmt(stmtSeed: number, step: number): string {
  const op = pickShortOp(stmtSeed);
  const c = (stmtSeed % 89) + 3;
  switch (stmtSeed % 10) {
    case 0:
      return `    acc ${op} ${c};`;
    case 1:
      return `    if acc > ${c} { acc ${op} 2; }`;
    case 2:
      return `    match acc & 1 { 0 => acc ${op} ${c}, _ => acc ^= 1 }`;
    case 3:
      return `    for k in 0..2 { acc ${op} k + 1; }`;
    case 4:
      return `    while acc == 0 { break; }`;
    case 5:
      return `    let t_${step} = acc ${op.slice(0, 1)} ${c}; acc ^= t_${step};`;
    case 6:
      return `    if false { return 0; } else { acc ^= 1; }`;
    case 7:
      return `    acc = !acc ${op.slice(0, 1)} ${c};`;
    case 8:
      return `    let mut v_${step} = input[0] ^ ${c}; v_${step} += 1; acc ^= v_${step};`;
    default:
      return `    acc <<= ${stmtSeed % 3}; acc ${op} seed;`;
  }
}

function tsCompactStmt(stmtSeed: number, step: number): string {
  const op = pickShortOp(stmtSeed);
  const c = (stmtSeed % 89) + 3;
  switch (stmtSeed % 10) {
    case 0:
      return `  checksum ${op} ${c};`;
    case 1:
      return `  if (checksum > ${c}) { checksum ${op} 2; }`;
    case 2:
      return `  switch (checksum & 1) { case 0: checksum ${op} ${c}; break; default: checksum ^= 1; }`;
    case 3:
      return `  for (let k = 0; k < 2; k++) { checksum ${op} k + 1; }`;
    case 4:
      return `  while (checksum === 0) { break; }`;
    case 5:
      return `  const t${step} = checksum ${op.slice(0, 1)} ${c}; checksum ^= t${step};`;
    case 6:
      return `  if (false) { return 0; } else { checksum ^= 1; }`;
    case 7:
      return `  checksum = (~checksum ${op.slice(0, 1)} ${c}) >>> 0;`;
    case 8:
      return `  let v${step} = (payload[0] ?? 0) ^ ${c}; v${step}++; checksum ^= v${step};`;
    default:
      return `  checksum >>>= ${stmtSeed % 3}; checksum ${op} seed;`;
  }
}

function pyCompactStmt(stmtSeed: number, step: number): string {
  const op = pickShortOp(stmtSeed);
  const c = (stmtSeed % 89) + 3;
  switch (stmtSeed % 10) {
    case 0:
      return `    state ${op} ${c}`;
    case 1:
      return `    if state > ${c}: state ${op} 2`;
    case 2:
      return `    if (state & 1) == 0: state ${op} ${c}\n    else: state ^= 1`;
    case 3:
      return `    for k in range(2): state ${op} k + 1`;
    case 4:
      return `    while state == 0:\n        break`;
    case 5:
      return `    t_${step} = state ${op.slice(0, 1)} ${c}; state ^= t_${step}`;
    case 6:
      return `    if False: return 0\n    else: state ^= 1`;
    case 7:
      return `    state = (~state ${op.slice(0, 1)} ${c}) & 0xFFFFFFFF`;
    case 8:
      return `    v_${step} = items[0] ^ ${c}; v_${step} += 1; state ^= v_${step}`;
    default:
      return `    state >>= ${stmtSeed % 3}; state ${op} seed`;
  }
}

function goCompactStmt(stmtSeed: number, step: number): string {
  const op = pickShortOp(stmtSeed);
  const c = (stmtSeed % 89) + 3;
  switch (stmtSeed % 10) {
    case 0:
      return `\tstate ${op} ${c}`;
    case 1:
      return `\tif state > ${c} { state ${op} 2 }`;
    case 2:
      return `\tswitch state & 1 { case 0: state ${op} ${c}; default: state ^= 1 }`;
    case 3:
      return `\tfor k := uint64(0); k < 2; k++ { state ${op} k + 1 }`;
    case 4:
      return `\tfor state == 0 { break }`;
    case 5:
      return `\tt${step} := state ${op.slice(0, 1)} ${c}; state ^= t${step}`;
    case 6:
      return `\tif false { return 0 } else { state ^= 1 }`;
    case 7:
      return `\tstate = (^state) ${op.slice(0, 1)} ${c}`;
    case 8:
      return `\tv${step} := input[0] ^ ${c}; v${step}++; state ^= v${step}`;
    default:
      return `\tstate >>= ${stmtSeed % 3}; state ${op} seed`;
  }
}

function renderRustModule(index: number): string {
  const uniqueBody = buildCompactBlock(index * 7919 + 11, 26, rustCompactStmt);
  const familyId = Math.floor(index / 3);
  const role = index % 3;
  if (role === 2) {
    return `pub fn process_shard_${index}(input: &[u64; 4], seed: u64) -> u64 {\n    let mut acc = seed ^ ${index + 17}u64;\n${uniqueBody}\n    acc ^ ${index + 1}u64\n}\n`;
  }
  const sharedBody = buildCompactBlock(familyId * 10007 + 70001, 16, rustCompactStmt);
  const baseClone = `pub fn normalize_bucket_${familyId}(input: &[u64; 4], seed: u64) -> u64 {\n    let mut acc = seed ^ ${familyId + 19}u64;\n${sharedBody}\n    acc\n}`;
  const cloneFn =
    role === 1
      ? renameIdentifiers(baseClone, [
          [`normalize_bucket_${familyId}`, `normalize_bucket_${familyId}_alt`],
          ["acc", "rolling"],
          ["input", "records"],
          ["seed", "anchor"],
        ])
      : baseClone;
  return `pub fn process_shard_${index}(input: &[u64; 4], seed: u64) -> u64 {\n    let mut acc = seed ^ ${index + 17}u64;\n${uniqueBody}\n    acc ^ ${index + 1}u64\n}\n\n${cloneFn}\n`;
}

function renderTypeScriptModule(index: number): string {
  const uniqueBody = buildCompactBlock(index * 7927 + 20000, 26, tsCompactStmt);
  const familyId = Math.floor(index / 3);
  const role = index % 3;
  if (role === 2) {
    return `export function computeSignature${index}(payload: number[], seed: number): number {\n  let checksum = (seed ^ ${index + 31}) >>> 0;\n${uniqueBody}\n  return checksum >>> 0;\n}\n`;
  }
  const sharedBody = buildCompactBlock(familyId * 10009 + 80001, 16, tsCompactStmt);
  const baseClone = `export function foldWindowMetrics${familyId}(payload: number[], seed: number): number {\n  let checksum = (seed ^ ${familyId + 37}) >>> 0;\n${sharedBody}\n  return checksum >>> 0;\n}`;
  const cloneFn =
    role === 1
      ? renameIdentifiers(baseClone, [
          [`foldWindowMetrics${familyId}`, `foldWindowMetrics${familyId}Alt`],
          ["checksum", "tracker"],
          ["payload", "records"],
          ["seed", "anchor"],
        ])
      : baseClone;
  return `export function computeSignature${index}(payload: number[], seed: number): number {\n  let checksum = (seed ^ ${index + 31}) >>> 0;\n${uniqueBody}\n  return checksum >>> 0;\n}\n\n${cloneFn}\n`;
}

function renderPythonModule(index: number): string {
  const uniqueBody = buildCompactBlock(index * 7933 + 40000, 24, pyCompactStmt);
  const familyId = Math.floor(index / 3);
  const role = index % 3;
  if (role === 2) {
    return `def digest_partition_${index}(items: list[int], seed: int) -> int:\n    state = (seed ^ ${index + 41}) & 0xFFFFFFFF\n${uniqueBody}\n    return state\n`;
  }
  const sharedBody = buildCompactBlock(familyId * 10037 + 90001, 16, pyCompactStmt);
  const baseClone = `def distill_series_${familyId}(items: list[int], seed: int) -> int:\n    state = (seed ^ ${familyId + 43}) & 0xFFFFFFFF\n${sharedBody}\n    return state`;
  const cloneFn =
    role === 1
      ? renameIdentifiers(baseClone, [
          [`distill_series_${familyId}`, `distill_series_${familyId}_alt`],
          ["state", "tracker"],
          ["items", "series"],
          ["seed", "anchor"],
        ])
      : baseClone;
  return `def digest_partition_${index}(items: list[int], seed: int) -> int:\n    state = (seed ^ ${index + 41}) & 0xFFFFFFFF\n${uniqueBody}\n    return state\n\n${cloneFn}\n`;
}

function renderGoModule(index: number): string {
  const uniqueBody = buildCompactBlock(index * 7937 + 60000, 24, goCompactStmt);
  const familyId = Math.floor(index / 3);
  const role = index % 3;
  if (role === 2) {
    return `package worker\n\nfunc ComputeLane_${index}(input [4]uint64, seed uint64) uint64 {\n\tstate := seed ^ ${index + 53}\n${uniqueBody}\n\treturn state\n}\n`;
  }
  const sharedBody = buildCompactBlock(familyId * 10039 + 100001, 16, goCompactStmt);
  const baseClone = `func FoldTelemetry_${familyId}(input [4]uint64, seed uint64) uint64 {\n\tstate := seed ^ ${familyId + 59}\n${sharedBody}\n\treturn state\n}`;
  const cloneFn =
    role === 1
      ? renameIdentifiers(baseClone, [
          [`FoldTelemetry_${familyId}`, `FoldTelemetry_${familyId}_Alt`],
          ["state", "tracker"],
          ["input", "records"],
          ["seed", "base"],
        ])
      : baseClone;
  return `package worker\n\nfunc ComputeLane_${index}(input [4]uint64, seed uint64) uint64 {\n\tstate := seed ^ ${index + 53}\n${uniqueBody}\n\treturn state\n}\n\n${cloneFn}\n`;
}

const LANGUAGE_TEMPLATES: LanguageTemplateSpec[] = [
  { id: "rust", label: "Rust", ext: "rs", renderFile: renderRustModule },
  { id: "typescript", label: "TypeScript", ext: "ts", renderFile: renderTypeScriptModule },
  { id: "python", label: "Python", ext: "py", renderFile: renderPythonModule },
  { id: "go", label: "Go", ext: "go", renderFile: renderGoModule },
];

export async function generatePolyglotCorpus(
  corpusDir: string,
  filesPerLanguage: number,
): Promise<CorpusSummary> {
  await rm(corpusDir, { recursive: true, force: true });
  const byLanguage: CorpusLanguageStats[] = [];
  let totalFiles = 0;
  let totalLines = 0;
  let totalBytes = 0;

  for (const lang of LANGUAGE_TEMPLATES) {
    const langDir = join(corpusDir, "src", lang.id);
    await mkdir(langDir, { recursive: true });
    let langLines = 0;
    let langBytes = 0;

    for (let i = 0; i < filesPerLanguage; i++) {
      const content = lang.renderFile(i);
      const filePath = join(langDir, `module_${String(i).padStart(4, "0")}.${lang.ext}`);
      await Bun.write(filePath, content);
      langLines += content.split("\n").length;
      langBytes += Buffer.byteLength(content, "utf-8");
    }

    byLanguage.push({
      language: lang.label,
      files: filesPerLanguage,
      lines: langLines,
      bytes: langBytes,
    });
    totalFiles += filesPerLanguage;
    totalLines += langLines;
    totalBytes += langBytes;
  }

  return { corpusDir, totalFiles, totalLines, totalBytes, byLanguage };
}

export function computeMedian(values: number[]): number {
  if (values.length === 0) return 0;
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  if (sorted.length % 2 === 1) return sorted[mid] ?? 0;
  return Math.round((((sorted[mid - 1] ?? 0) + (sorted[mid] ?? 0)) / 2) * 100) / 100;
}

function formatMeasurementRow(m: ToolBenchmarkMeasurement, includeNotes: boolean): string {
  if (!m.available) {
    const notesCol = includeNotes ? ` ${m.notes ?? "Not installed"} |` : "";
    return `| ${m.tool} | ${m.version} | ${m.mode} | N/A | N/A | N/A | N/A | \`${m.flags}\` |${notesCol}`;
  }
  const rss = m.peakRssMb !== null ? `${m.peakRssMb.toFixed(1)} MB` : "N/A";
  const clones = m.clonesDetected !== null ? String(m.clonesDetected) : "N/A";
  const notesCol = includeNotes ? ` ${m.notes ?? ""} |` : "";
  return `| ${m.tool} | ${m.version} | ${m.mode} | ${m.medianMs.toFixed(1)} ms | ${Math.round(m.locPerSecond).toLocaleString("en-US")} LOC/s | ${clones} | ${rss} | \`${m.flags}\` |${notesCol}`;
}

export function formatBenchmarkMarkdown(report: BenchmarkReport): string {
  const { environment: env, corpus, corpusResults, selfScan } = report;
  const langBreakdown = corpus.byLanguage
    .map((l) => `- **${l.language}**: ${l.files} files (${l.lines.toLocaleString("en-US")} lines)`)
    .join("\n");

  const corpusRows = corpusResults.map((m) => formatMeasurementRow(m, true)).join("\n");
  const selfScanSection = selfScan
    ? [
        "",
        `## 3. Repository Self-Scan Benchmark (\`gt-dev/cddm\`, ${selfScan.repoFiles.toLocaleString("en-US")} files / ${selfScan.repoLines.toLocaleString("en-US")} lines)`,
        "",
        "| Tool | Version | Mode | Median Latency | Throughput | Clones Found | Peak RSS | Exact CLI Flags |",
        "| :--- | :--- | :--- | ---: | ---: | ---: | ---: | :--- |",
        ...selfScan.measurements.map((m) => formatMeasurementRow(m, false)),
      ].join("\n")
    : "";

  return `# CDDM Reproducible Ecosystem Benchmarks

All figures below were measured directly by [\`scripts/benchmark-ecosystem.ts\`](../scripts/benchmark-ecosystem.ts) on real local executions — no synthetic or estimated timings are included.

> **Captured**: \`${env.capturedAtIso}\`

## 1. Benchmark Environment & Tool Versions

- **Host OS / Arch**: \`${env.os}\` (\`${env.arch}\`)
- **CPU**: ${env.cpuModel} (${env.logicalCores} logical cores)
- **System RAM**: ${env.totalRamGb.toFixed(1)} GB
- **Runtime**: Bun \`${env.bunVersion}\`
- **CDDM Binary**: \`${env.cddmVersion}\`
- **jscpd**: ${env.jscpdVersion ? `\`${env.jscpdVersion}\`` : "_not installed in environment_"}
- **PMD CPD**: ${env.pmdVersion ? `\`${env.pmdVersion}\`` : "_not installed in environment_"}

## 2. Controlled Polyglot Corpus (${corpus.totalFiles} files, ${corpus.totalLines.toLocaleString("en-US")} LOC)

Generated deterministically via [\`scripts/lib/benchmark-corpus.ts\`](../scripts/lib/benchmark-corpus.ts) with 10-way keyword/operator combinatorial AST statements plus planted per-family Type-1/Type-2 clones across 4 languages:

${langBreakdown}

| Tool | Version | Mode | Median Latency | Throughput | Clones Found | Peak RSS | Exact CLI Flags | Notes |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | :--- | :--- |
${corpusRows}
${selfScanSection}

## 4. Reproduce Locally

\`\`\`bash
# Build release binary with embedded WebUI
vp run build:release

# Regenerate benchmarks against the deterministic corpus, jscpd, and gt-dev/cddm self-scan
bun scripts/benchmark-ecosystem.ts --with-jscpd --with-pmd --self-scan --write
\`\`\`
`;
}
