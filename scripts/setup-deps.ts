#!/usr/bin/env bun
/**
 * Cross-platform workspace dependency installer for CDDM.
 * Ensures all nested package workspaces (webui, editors/vscode, tests/e2e)
 * have their dependencies installed following root package installation.
 */

import { existsSync } from "node:fs";
import { join, resolve } from "node:path";

export const DEPENDENCY_TARGET_DIRS = ["webui", "editors/vscode", "tests/e2e"] as const;

export interface SetupDepsOptions {
  workspaceRoot?: string;
  frozenLockfile?: boolean;
  verbose?: boolean;
  dryRun?: boolean;
  targets?: readonly string[];
}

export interface SetupDepsResult {
  target: string;
  skipped: boolean;
  success: boolean;
  elapsedMs: number;
  output?: string;
  error?: string;
}

export async function installWorkspaceDeps(
  options: SetupDepsOptions = {},
): Promise<SetupDepsResult[]> {
  const root = options.workspaceRoot || resolve(import.meta.dir, "..");
  const targets = options.targets || DEPENDENCY_TARGET_DIRS;
  const results: SetupDepsResult[] = [];

  for (const relDir of targets) {
    const targetDir = join(root, relDir);
    const pkgJsonPath = join(targetDir, "package.json");

    if (!existsSync(pkgJsonPath)) {
      results.push({
        target: relDir,
        skipped: true,
        success: true,
        elapsedMs: 0,
      });
      continue;
    }

    if (options.dryRun) {
      if (options.verbose) {
        console.log(`[DRY-RUN] Would install dependencies in ${relDir}`);
      }
      results.push({
        target: relDir,
        skipped: false,
        success: true,
        elapsedMs: 0,
      });
      continue;
    }

    const start = performance.now();
    const cmd = ["bun", "install"];
    if (options.frozenLockfile) {
      cmd.push("--frozen-lockfile");
    }

    if (options.verbose) {
      console.log(`--> Installing dependencies in ${relDir} (${cmd.join(" ")})...`);
    }

    const proc = Bun.spawnSync(cmd, {
      cwd: targetDir,
      stdout: "pipe",
      stderr: "pipe",
    });

    const elapsedMs = Math.round(performance.now() - start);

    if (proc.exitCode !== 0) {
      const errOut = proc.stderr.toString().trim() || proc.stdout.toString().trim();
      console.error(
        `[FAIL] Dependency installation failed in ${relDir} (exit code ${proc.exitCode}):`,
      );
      if (errOut) {
        console.error(errOut);
      }
      results.push({
        target: relDir,
        skipped: false,
        success: false,
        elapsedMs,
        error: errOut,
      });
    } else {
      const stdOut = proc.stdout.toString().trim();
      results.push({
        target: relDir,
        skipped: false,
        success: true,
        elapsedMs,
        output: stdOut,
      });
      console.log(`[OK] Installed dependencies for ${relDir} in ${elapsedMs}ms`);
    }
  }

  return results;
}

export function parseArgs(args: string[]): SetupDepsOptions {
  const options: SetupDepsOptions = {};
  for (const arg of args) {
    if (arg === "--frozen-lockfile") {
      options.frozenLockfile = true;
    } else if (arg === "--verbose" || arg === "-v") {
      options.verbose = true;
    } else if (arg === "--dry-run" || arg === "-n") {
      options.dryRun = true;
    }
  }
  return options;
}

async function main() {
  const args = process.argv.slice(2);
  const options = parseArgs(args);

  console.log(
    "\x1b[36mSynchronizing nested workspace dependencies (webui, editors/vscode, tests/e2e)...\x1b[0m",
  );

  const results = await installWorkspaceDeps(options);
  const failures = results.filter((r) => !r.success);

  if (failures.length > 0) {
    console.error(
      `\x1b[31mFailed to install dependencies for ${failures.length} workspace(s).\x1b[0m`,
    );
    process.exit(1);
  }

  console.log("\x1b[32m[OK] All workspace dependencies are synchronized and ready!\x1b[0m");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal setup-deps error:", err);
    process.exit(1);
  });
}
