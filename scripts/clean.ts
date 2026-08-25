#!/usr/bin/env bun
/**
 * Cross-platform Workspace Cleanup Tool for CDDM.
 * Removes all build artifacts, temporary caches, generated files, test reports, and lockfiles.
 * Single source of truth across Windows, Linux, and macOS.
 */

import { existsSync, lstatSync, readdirSync, rmSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { printScriptBanner, printScriptHelp } from "./lib/step-runner";

export interface CleanOptions {
  dryRun?: boolean;
  verbose?: boolean;
  keepLockfiles?: boolean;
  keepNodeModules?: boolean;
  cwd?: string;
}

export interface CleanItem {
  path: string;
  relPath: string;
  isDirectory: boolean;
  category: "build" | "cache" | "lockfile" | "test-report" | "temp";
  sizeBytes?: number;
}

export interface CleanResult {
  items: CleanItem[];
  dirsRemoved: number;
  filesRemoved: number;
  bytesFreed: number;
  dryRun: boolean;
  elapsedMs: number;
}

const PKG_ROOTS = ["", "webui", "tests/e2e", "editors/vscode"];
const LOCK_NAMES = ["bun.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml"];

export const KNOWN_CLEAN_DIRS: Array<{ path: string; category: CleanItem["category"] }> = [
  { path: "target", category: "build" },
  { path: "dist", category: "build" },
  { path: "coverage", category: "test-report" },
  { path: "webui/dist", category: "build" },
  { path: "webui/coverage", category: "test-report" },
  { path: "npm/cddm/dist", category: "build" },
  { path: "editors/vscode/out", category: "build" },
  { path: "packaging/vscode", category: "build" },
  ...PKG_ROOTS.map((p) => ({
    path: p ? `${p}/node_modules` : "node_modules",
    category: "cache" as const,
  })),
  { path: "tests/e2e/test-results", category: "test-report" },
  { path: "tests/e2e/playwright-report", category: "test-report" },
  { path: "tests/e2e/blob-report", category: "test-report" },
  { path: ".logs", category: "temp" },
  { path: ".turbo", category: "cache" },
  { path: ".cache", category: "cache" },
  { path: "webui/.cache", category: "cache" },
  { path: ".vite", category: "cache" },
  { path: "webui/.vite", category: "cache" },
  { path: ".nyc_output", category: "test-report" },
];

export const KNOWN_CLEAN_FILES: Array<{ path: string; category: CleanItem["category"] }> = [
  { path: "Cargo.lock", category: "lockfile" },
  ...PKG_ROOTS.flatMap((p) =>
    LOCK_NAMES.map((name) => ({
      path: p ? `${p}/${name}` : name,
      category: "lockfile" as const,
    })),
  ),
  ...PKG_ROOTS.map((p) => ({
    path: p ? `${p}/tsconfig.tsbuildinfo` : "tsconfig.tsbuildinfo",
    category: "cache" as const,
  })),
];

export const PROTECTED_PREFIXES = new Set([
  ".git",
  ".agents",
  ".github",
  ".vscode",
  ".vite-hooks",
  "crates",
  "webui/src",
  "scripts",
  "docs",
  "npm/cddm/bin",
  "editors/vscode/src",
]);

/**
 * Recursively calculate total size in bytes of a file or directory.
 */
export function calculatePathSize(fullPath: string): number {
  try {
    const stat = lstatSync(fullPath);
    if (stat.isSymbolicLink()) {
      return 0;
    }
    if (!stat.isDirectory()) {
      return stat.size;
    }
    let total = 0;
    const entries = readdirSync(fullPath);
    for (const entry of entries) {
      total += calculatePathSize(join(fullPath, entry));
    }
    return total;
  } catch {
    return 0;
  }
}

/**
 * Checks if a relative path is strictly protected from deletion.
 */
export function isProtectedPath(relPath: string): boolean {
  const normalized = relPath.replace(/\\/g, "/").replace(/^\/+/, "");
  for (const prefix of PROTECTED_PREFIXES) {
    if (normalized === prefix || normalized.startsWith(`${prefix}/`)) {
      return true;
    }
  }
  return false;
}

/**
 * Scan workspace for all cleanable items.
 */
export function findCleanableItems(
  workspaceRoot: string = process.cwd(),
  options: CleanOptions = {},
): CleanItem[] {
  const root = resolve(workspaceRoot);
  const items: CleanItem[] = [];
  const visitedRelPaths = new Set<string>();

  const addItem = (
    relPath: string,
    isDirectory: boolean,
    category: CleanItem["category"],
    sizeBytes?: number,
  ) => {
    const normalizedRel = relPath.replace(/\\/g, "/");
    if (visitedRelPaths.has(normalizedRel)) return;
    if (isProtectedPath(normalizedRel)) return;

    visitedRelPaths.add(normalizedRel);
    items.push({
      path: join(root, relPath),
      relPath: normalizedRel,
      isDirectory,
      category,
      sizeBytes,
    });
  };

  // 1. Check known directories
  for (const dirDef of KNOWN_CLEAN_DIRS) {
    if (options.keepNodeModules && dirDef.path.includes("node_modules")) {
      continue;
    }
    const fullPath = join(root, dirDef.path);
    if (existsSync(fullPath)) {
      const isDir = statSync(fullPath).isDirectory();
      if (isDir) {
        const size = calculatePathSize(fullPath);
        addItem(dirDef.path, true, dirDef.category, size);
      }
    }
  }

  // 2. Check known files
  for (const fileDef of KNOWN_CLEAN_FILES) {
    if (options.keepLockfiles && fileDef.category === "lockfile") {
      continue;
    }
    const fullPath = join(root, fileDef.path);
    if (existsSync(fullPath)) {
      const isFile = statSync(fullPath).isFile();
      if (isFile) {
        const size = statSync(fullPath).size;
        addItem(fileDef.path, false, fileDef.category, size);
      }
    }
  }

  // 3. Scan for dynamic pattern-matched files (.tsbuildinfo, .log, .DS_Store, Thumbs.db, nested node_modules)
  const scanDynamicArtifacts = (dir: string) => {
    const fullDir = join(root, dir);
    let entries: string[];
    try {
      entries = readdirSync(fullDir);
    } catch {
      return;
    }

    for (const entry of entries) {
      const relEntry = dir === "." ? entry : join(dir, entry);
      const normalizedRel = relEntry.replace(/\\/g, "/");

      if (isProtectedPath(normalizedRel)) {
        continue;
      }

      const fullEntry = join(root, relEntry);
      let isDirectory = false;
      try {
        isDirectory = statSync(fullEntry).isDirectory();
      } catch {
        continue;
      }

      if (isDirectory) {
        if (entry === "node_modules") {
          if (!options.keepNodeModules) {
            const size = calculatePathSize(fullEntry);
            addItem(relEntry, true, "cache", size);
          }
          // Do not recurse into node_modules
          continue;
        }
        if (entry === "target" || entry === "dist" || entry === "coverage") {
          // Already handled or top-level artifact
          continue;
        }
        // Recurse into non-ignored directories
        scanDynamicArtifacts(relEntry);
      } else {
        const lower = entry.toLowerCase();
        if (lower.endsWith(".tsbuildinfo")) {
          const size = statSync(fullEntry).size;
          addItem(relEntry, false, "cache", size);
        } else if (
          lower.endsWith(".log") ||
          lower.startsWith("npm-debug.log") ||
          lower.startsWith("yarn-debug.log") ||
          lower.startsWith("yarn-error.log")
        ) {
          const size = statSync(fullEntry).size;
          addItem(relEntry, false, "temp", size);
        } else if (entry === ".DS_Store" || entry === "Thumbs.db") {
          const size = statSync(fullEntry).size;
          addItem(relEntry, false, "temp", size);
        } else if (
          !options.keepLockfiles &&
          (entry === "bun.lock" ||
            entry === "package-lock.json" ||
            entry === "yarn.lock" ||
            entry === "pnpm-lock.yaml")
        ) {
          const size = statSync(fullEntry).size;
          addItem(relEntry, false, "lockfile", size);
        }
      }
    }
  };

  scanDynamicArtifacts(".");

  return items;
}

/**
 * Format bytes into human-readable representation.
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const val = (bytes / Math.pow(k, i)).toFixed(1);
  return `${val} ${sizes[i]}`;
}

/**
 * Safely remove a file or directory with robust error handling for locked entries.
 */
export function safeRemovePath(fullPath: string, isDirectory: boolean): boolean {
  try {
    rmSync(fullPath, { recursive: true, force: true, maxRetries: 3, retryDelay: 100 });
    return true;
  } catch {
    if (isDirectory && existsSync(fullPath)) {
      // If root directory removal fails (e.g. EBUSY on parent handle), purge children individually
      try {
        const entries = readdirSync(fullPath);
        for (const entry of entries) {
          const childPath = join(fullPath, entry);
          try {
            const childStat = lstatSync(childPath);
            safeRemovePath(childPath, childStat.isDirectory());
          } catch {
            // Skip unreadable child
          }
        }
        // Try removing the empty directory again
        try {
          rmSync(fullPath, { recursive: true, force: true });
          return true;
        } catch {
          return false;
        }
      } catch {
        return false;
      }
    }
    return false;
  }
}

/**
 * Clean all workspace artifacts, caches, test reports, and lockfiles.
 */
export async function cleanWorkspace(
  workspaceRoot: string = process.cwd(),
  options: CleanOptions = {},
): Promise<CleanResult> {
  const startTime = performance.now();
  const root = resolve(workspaceRoot);
  const items = findCleanableItems(root, options);

  let dirsRemoved = 0;
  let filesRemoved = 0;
  let bytesFreed = 0;

  for (const item of items) {
    if (options.verbose) {
      const tag = options.dryRun ? "[DRY-RUN]" : "[CLEAN]";
      console.log(
        `  \x1b[34m${tag}\x1b[0m [${item.category}] ${item.relPath} (${formatBytes(item.sizeBytes ?? 0)})`,
      );
    }

    bytesFreed += item.sizeBytes ?? 0;

    if (!options.dryRun) {
      const success = safeRemovePath(item.path, item.isDirectory);
      if (success) {
        if (item.isDirectory) {
          dirsRemoved++;
        } else {
          filesRemoved++;
        }
      } else {
        console.warn(`  \x1b[33m[WARN] Could not completely remove ${item.relPath}\x1b[0m`);
      }
    } else {
      if (item.isDirectory) {
        dirsRemoved++;
      } else {
        filesRemoved++;
      }
    }
  }

  const elapsedMs = Math.round(performance.now() - startTime);

  return {
    items,
    dirsRemoved,
    filesRemoved,
    bytesFreed,
    dryRun: Boolean(options.dryRun),
    elapsedMs,
  };
}

function parseCliArgs(args: string[]): CleanOptions {
  const options: CleanOptions = {};
  for (const arg of args) {
    if (arg === "--dry-run" || arg === "-n") {
      options.dryRun = true;
    } else if (arg === "--verbose" || arg === "-v") {
      options.verbose = true;
    } else if (arg === "--keep-lockfiles") {
      options.keepLockfiles = true;
    } else if (arg === "--keep-node-modules") {
      options.keepNodeModules = true;
    } else if (arg === "--help" || arg === "-h") {
      printScriptHelp(
        "Workspace Cleaner",
        "vp run clean [options] / bun scripts/clean.ts [options]",
        [
          ["--dry-run, -n", "Simulate cleanup and print targets without deleting"],
          ["--verbose, -v", "Print every item being deleted"],
          ["--keep-lockfiles", "Keep lockfiles (bun.lock, Cargo.lock)"],
          ["--keep-node-modules", "Keep node_modules directories"],
          ["--help, -h", "Show this help message"],
        ],
      );
      process.exit(0);
    }
  }
  return options;
}

async function main() {
  const options = parseCliArgs(process.argv.slice(2));
  printScriptBanner("CDDM Workspace Cleanup Tool");

  if (options.dryRun) {
    console.log("\x1b[33m[INFO] Running in DRY-RUN mode (no files will be deleted).\x1b[0m\n");
  }

  const result = await cleanWorkspace(process.cwd(), { ...options, verbose: true });

  console.log("\n\x1b[32m=======================================================\x1b[0m");
  const actionText = result.dryRun ? "Identified for removal" : "Cleaned successfully";
  console.log(
    `\x1b[32m[PASS] ${actionText}: ${result.dirsRemoved} directories, ${result.filesRemoved} files (${formatBytes(result.bytesFreed)}) in ${result.elapsedMs}ms\x1b[0m`,
  );
  console.log("\x1b[32m=======================================================\x1b[0m\n");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal clean error:", err);
    process.exit(1);
  });
}
