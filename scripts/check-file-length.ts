#!/usr/bin/env bun
/**
 * Cross-platform File Length & Modularity Policy Validator for CDDM.
 * Enforces:
 * 1. Standard maximum limit of 500 lines per file for all source code (.rs, .ts, .tsx, .js, .jsx, .css).
 * 2. Ratcheted baseline ceilings for grandfathered legacy files to prevent growth and mandate incremental modular refactoring.
 * 3. Zero emojis across all log and diagnostic output.
 */

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

export const MAX_STANDARD_FILE_LINES = 500;

export interface FileLengthViolation {
  file: string;
  actualLines: number;
  maxAllowedLines: number;
  isGrandfathered: boolean;
}

export interface FileLengthSummary {
  filesChecked: number;
  violations: FileLengthViolation[];
  totalCodeLines: number;
}

/**
 * Baseline ratcheted ceilings for legacy files exceeding 500 lines.
 * (Currently 100% of codebase files are strictly <= 500 LOC).
 */
export const GRANDFATHERED_LINE_CAPS: Record<string, number> = {};

export const DEFAULT_IGNORED_DIRS = new Set([
  ".git",
  "node_modules",
  "target",
  "dist",
  "out",
  "coverage",
  "brain",
  ".vite-hooks",
  ".cddm",
  ".vscode",
]);

export const DEFAULT_CODE_EXTENSIONS = new Set([".rs", ".ts", ".tsx", ".js", ".jsx", ".css"]);

/**
 * Count the total number of lines in a file.
 */
export function countFileLines(filePath: string, workspaceRoot: string = process.cwd()): number {
  const fullPath = join(workspaceRoot, filePath);
  try {
    const content = readFileSync(fullPath, "utf-8");
    if (content.length === 0) return 0;
    return content.split("\n").length;
  } catch {
    return 0;
  }
}

/**
 * Recursively scans directory for code files and validates their line counts against policy.
 */
export function scanCodebaseFileLengths(
  dir: string = ".",
  workspaceRoot: string = process.cwd(),
  ignoredDirs: Set<string> = DEFAULT_IGNORED_DIRS,
  codeExtensions: Set<string> = DEFAULT_CODE_EXTENSIONS,
  grandfatherCaps: Record<string, number> = GRANDFATHERED_LINE_CAPS,
  standardMaxLines: number = MAX_STANDARD_FILE_LINES,
): FileLengthSummary {
  const fullDirPath = join(workspaceRoot, dir);
  let filesChecked = 0;
  let totalCodeLines = 0;
  const violations: FileLengthViolation[] = [];

  let entries: string[];
  try {
    entries = readdirSync(fullDirPath);
  } catch {
    return { filesChecked, violations, totalCodeLines };
  }

  for (const entry of entries) {
    if (ignoredDirs.has(entry)) continue;
    const relEntryPath = dir === "." ? entry : join(dir, entry);
    const fullEntryPath = join(workspaceRoot, relEntryPath);

    try {
      const stat = statSync(fullEntryPath);
      if (stat.isDirectory()) {
        const subSummary = scanCodebaseFileLengths(
          relEntryPath,
          workspaceRoot,
          ignoredDirs,
          codeExtensions,
          grandfatherCaps,
          standardMaxLines,
        );
        filesChecked += subSummary.filesChecked;
        totalCodeLines += subSummary.totalCodeLines;
        violations.push(...subSummary.violations);
      } else if (stat.isFile()) {
        const dotIdx = entry.lastIndexOf(".");
        const ext = dotIdx !== -1 ? entry.slice(dotIdx).toLowerCase() : "";
        if (!codeExtensions.has(ext)) continue;

        const normalizedRelPath = relEntryPath.replace(/\\/g, "/");
        const lines = countFileLines(normalizedRelPath, workspaceRoot);

        filesChecked++;
        totalCodeLines += lines;

        const grandfatheredCap = grandfatherCaps[normalizedRelPath];
        const isGrandfathered = grandfatheredCap !== undefined;
        const maxAllowed = isGrandfathered ? grandfatheredCap : standardMaxLines;

        if (lines > maxAllowed) {
          violations.push({
            file: normalizedRelPath,
            actualLines: lines,
            maxAllowedLines: maxAllowed,
            isGrandfathered,
          });
        }
      }
    } catch {
      // Ignore unreadable files
    }
  }

  return { filesChecked, violations, totalCodeLines };
}

async function main() {
  console.log("\x1b[36m--> Scanning codebase for file length & modularity violations...\x1b[0m");

  const summary = scanCodebaseFileLengths(".");
  console.log(
    `\x1b[35mAnalyzed ${summary.filesChecked} code files (${summary.totalCodeLines.toLocaleString()} total LOC). Max line ceiling: ${MAX_STANDARD_FILE_LINES} lines.\x1b[0m`,
  );

  if (summary.violations.length > 0) {
    console.error(
      `\n\x1b[31m[ERROR] Found ${summary.violations.length} file length policy violation(s):\x1b[0m\n`,
    );

    for (const v of summary.violations) {
      if (v.isGrandfathered) {
        console.error(
          `  \x1b[31m[FAIL] [${v.file}]\x1b[0m Exceeds grandfathered ceiling: ${v.actualLines} lines (ratchet cap: ${v.maxAllowedLines}). Legacy files must not expand!`,
        );
      } else {
        console.error(
          `  \x1b[31m[FAIL] [${v.file}]\x1b[0m Monolithic file exceeds standard ${MAX_STANDARD_FILE_LINES}-line limit: ${v.actualLines} lines! Decompose into modular submodules/components.`,
        );
      }
    }

    console.error(
      "\n\x1b[31mPlease decompose oversized files into clean, focused submodules conforming to the Modularity Standard.\x1b[0m\n",
    );
    process.exit(1);
  }

  console.log(
    "\x1b[32m[PASS] All code files adhere strictly to the Modularity Standard & line limits!\x1b[0m\n",
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal file length scan error:", err);
    process.exit(1);
  });
}
