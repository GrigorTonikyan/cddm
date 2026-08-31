#!/usr/bin/env bun
/**
 * Cross-platform Workspace Cleanup Tool for CDDM.
 * Removes all build artifacts, temporary caches, generated files, test reports, and lockfiles.
 * Single source of truth across Windows, Linux, and macOS.
 */

import {
  cleanWorkspace,
  discoverPackageRoots,
  findCleanableItems,
  formatBytes,
  isProtectedPath,
  KNOWN_CLEAN_DIRS,
  KNOWN_CLEAN_FILES,
  KNOWN_PACKAGE_ROOTS,
  LOCK_NAMES,
  PROTECTED_EXACT_FILES,
  PROTECTED_PREFIXES,
  safeRemovePath,
  type CleanItem,
  type CleanOptions,
  type CleanResult,
  type SafeRemoveResult,
} from "./lib/clean-engine";
import { printScriptBanner, printScriptHelp } from "./lib/step-runner";

// Re-export public API for backwards compatibility with tests and consumers
export {
  cleanWorkspace,
  discoverPackageRoots,
  findCleanableItems,
  formatBytes,
  isProtectedPath,
  KNOWN_CLEAN_DIRS,
  KNOWN_CLEAN_FILES,
  KNOWN_PACKAGE_ROOTS,
  LOCK_NAMES,
  PROTECTED_EXACT_FILES,
  PROTECTED_PREFIXES,
  safeRemovePath,
  type CleanItem,
  type CleanOptions,
  type CleanResult,
  type SafeRemoveResult,
};

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
    } else if (arg === "--keep-target" || arg === "--keep-cargo") {
      options.keepTarget = true;
    } else if (arg === "--keep-build") {
      options.keepBuild = true;
    } else if (arg === "--keep-reports") {
      options.keepReports = true;
    } else if (arg === "--keep-cache") {
      options.keepCache = true;
    } else if (arg === "--target-only" || arg === "--cargo-only") {
      options.targetOnly = true;
    } else if (arg === "--node-only") {
      options.nodeOnly = true;
    } else if (arg === "--cache-only") {
      options.cacheOnly = true;
    } else if (arg === "--reports-only") {
      options.reportsOnly = true;
    } else if (arg === "--build-only") {
      options.buildOnly = true;
    } else if (arg === "--lockfiles-only") {
      options.lockfilesOnly = true;
    } else if (arg === "--help" || arg === "-h") {
      printScriptHelp(
        "Workspace Cleaner",
        "vp run clean [options] / bun scripts/clean.ts [options]",
        [
          ["--dry-run, -n", "Simulate cleanup and print targets without deleting"],
          ["--verbose, -v", "Print every item being deleted"],
          ["--keep-lockfiles", "Keep lockfiles (bun.lock, Cargo.lock)"],
          ["--keep-node-modules", "Keep node_modules directories"],
          ["--keep-target, --keep-cargo", "Keep Rust target/ compilation output"],
          ["--keep-build", "Keep all build artifacts (target, dist, out, vsix)"],
          ["--keep-reports", "Keep test reports and coverage outputs"],
          ["--keep-cache", "Keep cache directories (.turbo, .cache, .vite, .cddm)"],
          ["--target-only", "Only clean Rust target/ directory"],
          ["--node-only", "Only clean node_modules across all workspaces"],
          ["--cache-only", "Only clean cache directories and buildinfo files"],
          ["--reports-only", "Only clean test reports and coverage outputs"],
          ["--build-only", "Only clean build output directories"],
          ["--lockfiles-only", "Only clean lockfiles"],
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
  let statusMsg = `\x1b[32m[PASS] ${actionText}: ${result.dirsRemoved} directories, ${result.filesRemoved} files (${formatBytes(result.bytesFreed)}) in ${result.elapsedMs}ms\x1b[0m`;
  if (!result.dryRun && result.lockedFiles.length > 0) {
    statusMsg += `\n\x1b[33m[INFO] ${result.lockedFiles.length} file(s) currently held by running processes were preserved.\x1b[0m`;
  }
  console.log(statusMsg);
  console.log("\x1b[32m=======================================================\x1b[0m\n");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal clean error:", err);
    process.exit(1);
  });
}
