/**
 * Workspace Cleanup Engine & Artifact Scanner for CDDM.
 * Core algorithms for finding cleanable items, calculating sizes, safe deletion with Windows resilience,
 * and filtering protected repository files.
 */

import { chmodSync, existsSync, lstatSync, readdirSync, rmSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import {
  type CleanItem,
  type CleanOptions,
  type CleanResult,
  KNOWN_CLEAN_DIRS,
  KNOWN_CLEAN_FILES,
  KNOWN_PACKAGE_ROOTS,
  LOCK_NAMES,
  NEVER_TRAVERSE_DIRS,
  PROTECTED_EXACT_FILES,
  PROTECTED_PREFIXES,
  type SafeRemoveResult,
} from "./clean-types";

export * from "./clean-types";

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
  if (PROTECTED_EXACT_FILES.has(normalized)) {
    return true;
  }

  // Strictly protect all source code, docs, and repository infrastructure directories first
  for (const prefix of PROTECTED_PREFIXES) {
    if (normalized === prefix || normalized.startsWith(`${prefix}/`)) {
      return true;
    }
  }

  // Known artifacts/caches are cleanable when outside protected prefixes
  const base = normalized.split("/").pop() ?? "";
  if (
    base === ".cddm" ||
    base === "node_modules" ||
    base === "target" ||
    base === "dist" ||
    base === ".cache" ||
    base === ".turbo" ||
    base === ".vite" ||
    base === "test-results" ||
    base === "playwright-report" ||
    base === "blob-report" ||
    base === "coverage" ||
    base === ".nyc_output" ||
    base === ".DS_Store" ||
    base === "Thumbs.db" ||
    base === "desktop.ini" ||
    base === ".cddmignore" ||
    base === ".cddmrules.toml" ||
    base.endsWith(".tsbuildinfo") ||
    base.endsWith(".vsix") ||
    base.endsWith(".profraw") ||
    base.endsWith(".profdata") ||
    base.endsWith(".log")
  ) {
    return false;
  }

  return false;
}

/**
 * Discover package root directories containing package.json or Cargo.toml.
 */
export function discoverPackageRoots(workspaceRoot: string = process.cwd()): string[] {
  const roots = new Set<string>(KNOWN_PACKAGE_ROOTS);
  try {
    const glob = new Bun.Glob("**/package.json");
    for (const match of glob.scanSync({ cwd: workspaceRoot })) {
      const dir = match.replace(/\\/g, "/").replace(/\/package\.json$/, "");
      if (dir !== "package.json" && !isProtectedPath(dir) && !dir.includes("node_modules")) {
        roots.add(dir);
      }
    }
  } catch {
    // Fallback to static roots
  }
  return Array.from(roots);
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

  const matchesCategoryFilter = (category: CleanItem["category"], relPath: string): boolean => {
    if (options.targetOnly) {
      return relPath === "target" || relPath.startsWith("target/");
    }
    if (options.nodeOnly) {
      return relPath === "node_modules" || relPath.includes("/node_modules");
    }
    if (options.cacheOnly && category !== "cache") return false;
    if (options.reportsOnly && category !== "test-report") return false;
    if (options.buildOnly && category !== "build") return false;
    if (options.lockfilesOnly && category !== "lockfile") return false;

    if (options.keepLockfiles && category === "lockfile") return false;
    if (
      options.keepNodeModules &&
      (relPath === "node_modules" || relPath.includes("/node_modules"))
    ) {
      return false;
    }
    if (options.keepTarget && (relPath === "target" || relPath.startsWith("target/"))) return false;
    if (options.keepBuild && category === "build") return false;
    if (options.keepReports && category === "test-report") return false;
    if (options.keepCache && category === "cache") return false;

    return true;
  };

  const addItem = (
    relPath: string,
    isDirectory: boolean,
    category: CleanItem["category"],
    sizeBytes?: number,
  ) => {
    const normalizedRel = relPath.replace(/\\/g, "/").replace(/^\/+/, "");
    if (!normalizedRel || visitedRelPaths.has(normalizedRel)) return;
    if (isProtectedPath(normalizedRel)) return;
    if (!matchesCategoryFilter(category, normalizedRel)) return;

    visitedRelPaths.add(normalizedRel);
    items.push({
      path: join(root, normalizedRel),
      relPath: normalizedRel,
      isDirectory,
      category,
      sizeBytes,
    });
  };

  // 1. Check known directories
  for (const dirDef of KNOWN_CLEAN_DIRS) {
    const fullPath = join(root, dirDef.path);
    if (existsSync(fullPath)) {
      try {
        const isDir = statSync(fullPath).isDirectory();
        if (isDir) {
          const size = calculatePathSize(fullPath);
          addItem(dirDef.path, true, dirDef.category, size);
        }
      } catch {}
    }
  }

  // 2. Check known files
  for (const fileDef of KNOWN_CLEAN_FILES) {
    const fullPath = join(root, fileDef.path);
    if (existsSync(fullPath)) {
      try {
        const isFile = statSync(fullPath).isFile();
        if (isFile) {
          const size = statSync(fullPath).size;
          addItem(fileDef.path, false, fileDef.category, size);
        }
      } catch {}
    }
  }

  // 3. Scan for dynamic pattern-matched files & nested directories
  const scanDynamicArtifacts = (dir: string) => {
    const fullDir = join(root, dir);
    let entries: string[];
    try {
      entries = readdirSync(fullDir);
    } catch {
      return;
    }

    for (const entry of entries) {
      if (NEVER_TRAVERSE_DIRS.has(entry)) {
        continue;
      }

      const relEntry = dir === "." ? entry : join(dir, entry);
      const fullEntry = join(root, relEntry);

      let isDirectory = false;
      try {
        isDirectory = statSync(fullEntry).isDirectory();
      } catch {
        continue;
      }

      if (isDirectory) {
        if (entry === "node_modules") {
          const size = calculatePathSize(fullEntry);
          addItem(relEntry, true, "cache", size);
          continue;
        }
        if (entry === ".cddm" || entry === ".turbo" || entry === ".cache" || entry === ".vite") {
          const size = calculatePathSize(fullEntry);
          addItem(relEntry, true, "cache", size);
          continue;
        }
        if (
          entry === "test-results" ||
          entry === "playwright-report" ||
          entry === "blob-report" ||
          entry === "coverage" ||
          entry === ".nyc_output"
        ) {
          const size = calculatePathSize(fullEntry);
          addItem(relEntry, true, "test-report", size);
          continue;
        }
        if (entry === "target" || entry === "dist" || entry === "out") {
          const size = calculatePathSize(fullEntry);
          addItem(relEntry, true, "build", size);
          continue;
        }
        scanDynamicArtifacts(relEntry);
      } else {
        const lower = entry.toLowerCase();
        let size = 0;
        try {
          size = statSync(fullEntry).size;
        } catch {}

        if (
          lower.endsWith(".tsbuildinfo") ||
          lower === ".eslintcache" ||
          lower === ".prettiercache"
        ) {
          addItem(relEntry, false, "cache", size);
        } else if (
          lower.endsWith(".log") ||
          lower.startsWith("npm-debug.log") ||
          lower.startsWith("yarn-debug.log") ||
          lower.startsWith("yarn-error.log") ||
          lower.startsWith("bun.log") ||
          lower.startsWith("pnpm-debug.log")
        ) {
          addItem(relEntry, false, "temp", size);
        } else if (lower.endsWith(".vsix")) {
          addItem(relEntry, false, "build", size);
        } else if (lower.endsWith(".profraw") || lower.endsWith(".profdata")) {
          addItem(relEntry, false, "test-report", size);
        } else if (
          lower === ".env.local" ||
          lower === ".env.development.local" ||
          lower === ".env.test.local" ||
          lower === ".env.production.local"
        ) {
          addItem(relEntry, false, "temp", size);
        } else if (entry === ".DS_Store" || entry === "Thumbs.db" || entry === "desktop.ini") {
          addItem(relEntry, false, "temp", size);
        } else if (LOCK_NAMES.includes(entry) || entry === "Cargo.lock") {
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
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const val = (bytes / Math.pow(k, i)).toFixed(1);
  return `${val} ${sizes[i]}`;
}

/**
 * Safely remove a file or directory with robust retry, permission reset, and Windows locking resilience.
 */
export function safeRemovePath(fullPath: string, isDirectory: boolean): SafeRemoveResult {
  const result: SafeRemoveResult = { success: false, bytesFreed: 0, lockedFiles: [] };

  if (!existsSync(fullPath)) {
    result.success = true;
    return result;
  }

  // Attempt direct removal with retry and permission unlock
  for (let attempt = 1; attempt <= 4; attempt++) {
    try {
      if (!isDirectory) {
        try {
          chmodSync(fullPath, 0o666);
        } catch {}
      } else {
        try {
          chmodSync(fullPath, 0o777);
        } catch {}
      }
      rmSync(fullPath, { recursive: true, force: true, maxRetries: 3, retryDelay: 50 });
      result.success = true;
      return result;
    } catch {
      if (attempt < 4) {
        const start = Date.now();
        while (Date.now() - start < 25 * attempt) {
          // brief pause for handle release on Windows
        }
      }
    }
  }

  // If directory removal failed, recursively purge unlocked children
  if (isDirectory && existsSync(fullPath)) {
    try {
      const entries = readdirSync(fullPath);
      let allChildrenRemoved = true;
      for (const entry of entries) {
        const childPath = join(fullPath, entry);
        try {
          const childStat = lstatSync(childPath);
          const childRes = safeRemovePath(childPath, childStat.isDirectory());
          result.bytesFreed += childRes.bytesFreed;
          if (!childRes.success) {
            allChildrenRemoved = false;
            result.lockedFiles.push(...childRes.lockedFiles);
          }
        } catch {
          allChildrenRemoved = false;
          result.lockedFiles.push(childPath);
        }
      }

      if (allChildrenRemoved) {
        try {
          rmSync(fullPath, { recursive: true, force: true });
          result.success = true;
          return result;
        } catch {
          result.success = false;
          result.lockedFiles.push(fullPath);
          return result;
        }
      }
    } catch {
      result.success = false;
      result.lockedFiles.push(fullPath);
      return result;
    }
  } else if (!isDirectory && existsSync(fullPath)) {
    result.lockedFiles.push(fullPath);
  }

  return result;
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
  const lockedFiles: string[] = [];

  for (const item of items) {
    if (options.verbose) {
      const tag = options.dryRun ? "[DRY-RUN]" : "[CLEAN]";
      console.log(
        `  \x1b[34m${tag}\x1b[0m [${item.category}] ${item.relPath} (${formatBytes(item.sizeBytes ?? 0)})`,
      );
    }

    bytesFreed += item.sizeBytes ?? 0;

    if (!options.dryRun) {
      const res = safeRemovePath(item.path, item.isDirectory);
      if (res.success) {
        if (item.isDirectory) {
          dirsRemoved++;
        } else {
          filesRemoved++;
        }
      } else {
        lockedFiles.push(...res.lockedFiles);
        if (options.verbose) {
          console.warn(
            `  \x1b[33m[WARN] Partially cleaned ${item.relPath} (${res.lockedFiles.length} locked file(s) in use by active process)\x1b[0m`,
          );
        }
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
    lockedFiles,
    dryRun: Boolean(options.dryRun),
    elapsedMs,
  };
}
