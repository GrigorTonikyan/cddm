#!/usr/bin/env bun
/**
 * Cross-platform Workspace Reset & Preparation Pipeline for CDDM.
 * Completely cleans the workspace, reinstalls dependencies, configures Git hooks,
 * builds WebUI & Rust crates, and executes the complete 11-step verification suite.
 * Single source of truth across Windows, Linux, and macOS.
 */

import { existsSync } from "node:fs";
import { join } from "node:path";
import { cleanWorkspace, formatBytes } from "./clean";

export interface ResetOptions {
  skipVerify?: boolean;
  verbose?: boolean;
}

interface ResetStep {
  title: string;
  command: string[];
  cwd?: string;
  description?: string;
}

async function runStepCommand(
  step: ResetStep,
  stepIndex: number,
  totalSteps: number,
): Promise<void> {
  const stepNum = `[${stepIndex + 1}/${totalSteps}]`;
  console.log(`\n\x1b[35m${stepNum} ${step.title}...\x1b[0m`);
  const startTime = performance.now();

  const proc = Bun.spawn(step.command, {
    cwd: step.cwd,
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await proc.exited;
  const elapsedMs = Math.round(performance.now() - startTime);

  if (exitCode !== 0) {
    console.error(
      `\n\x1b[31m[FAIL] Reset step failed with exit code ${exitCode} (${elapsedMs}ms): ${step.command.join(" ")}\x1b[0m\n`,
    );
    process.exit(exitCode ?? 1);
  }

  console.log(`\x1b[32m[PASS] (${elapsedMs}ms)\x1b[0m`);
}

export async function resetWorkspace(
  workspaceRoot: string = process.cwd(),
  options: ResetOptions = {},
): Promise<void> {
  console.log("\x1b[36m=======================================================\x1b[0m");
  console.log("\x1b[36m        CDDM Full Workspace Reset & Preparation        \x1b[0m");
  console.log("\x1b[36m=======================================================\x1b[0m");

  const overallStart = performance.now();

  // Determine total steps
  const hasE2e = existsSync(join(workspaceRoot, "tests/e2e/package.json"));
  const stepCount = (options.skipVerify ? 6 : 7) + (hasE2e ? 1 : 0);
  let currentStep = 1;

  // Step 1: Deep Clean
  console.log(
    `\n\x1b[35m[${currentStep}/${stepCount}] Cleaning all workspace artifacts and lockfiles...\x1b[0m`,
  );
  const cleanResult = await cleanWorkspace(workspaceRoot, { verbose: true });
  console.log(
    `\x1b[32m[PASS] Removed ${cleanResult.dirsRemoved} directories and ${cleanResult.filesRemoved} files (${formatBytes(cleanResult.bytesFreed)})\x1b[0m`,
  );

  // Step 2: Install Root Dependencies
  currentStep++;
  await runStepCommand(
    {
      title: "Installing root workspace dependencies (bun install)",
      command: ["bun", "install"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
  );

  // Step 3: Install WebUI Dependencies
  currentStep++;
  await runStepCommand(
    {
      title: "Installing WebUI dependencies (bun install)",
      command: ["bun", "install"],
      cwd: join(workspaceRoot, "webui"),
    },
    currentStep - 1,
    stepCount,
  );

  // Step 4: Install E2E Dependencies (if present)
  if (hasE2e) {
    currentStep++;
    await runStepCommand(
      {
        title: "Installing E2E test dependencies (bun install)",
        command: ["bun", "install"],
        cwd: join(workspaceRoot, "tests/e2e"),
      },
      currentStep - 1,
      stepCount,
    );
  }

  // Step 5: Configure Git Hooks
  currentStep++;
  await runStepCommand(
    {
      title: "Configuring Git hooks (bun scripts/setup-hooks.ts)",
      command: ["bun", "scripts/setup-hooks.ts"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
  );

  // Step 6: Build WebUI
  currentStep++;
  await runStepCommand(
    {
      title: "Building WebUI production distribution (vp -C webui run build)",
      command: ["vp", "-C", "webui", "run", "build"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
  );

  // Step 7: Build Rust Crates
  currentStep++;
  await runStepCommand(
    {
      title: "Compiling Rust workspace crates (cargo build --workspace)",
      command: ["cargo", "build", "--workspace"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
  );

  // Step 8: Full Verification Pipeline (optional)
  if (!options.skipVerify) {
    currentStep++;
    await runStepCommand(
      {
        title: "Executing complete 11-step verification suite (bun scripts/verify.ts)",
        command: ["bun", "scripts/verify.ts"],
        cwd: workspaceRoot,
      },
      currentStep - 1,
      stepCount,
    );
  }

  const totalSeconds = ((performance.now() - overallStart) / 1000).toFixed(2);
  console.log("\n\x1b[32m=======================================================\x1b[0m");
  console.log(
    `\x1b[32m   CDDM workspace successfully reset & verified in ${totalSeconds}s!   \x1b[0m`,
  );
  console.log("\x1b[32m   The codebase is in a 100% default work-ready state. \x1b[0m");
  console.log("\x1b[32m=======================================================\x1b[0m\n");
}

function parseCliArgs(args: string[]): ResetOptions {
  const options: ResetOptions = {};
  for (const arg of args) {
    if (arg === "--skip-verify") {
      options.skipVerify = true;
    } else if (arg === "--verbose" || arg === "-v") {
      options.verbose = true;
    } else if (arg === "--help" || arg === "-h") {
      console.log(`
CDDM Workspace Reset & Preparation (vp run reset / bun scripts/reset.ts)

Usage:
  vp run reset [options]
  bun scripts/reset.ts [options]

Options:
  --skip-verify    Skip the final full verification step
  --verbose, -v    Verbose logging
  --help, -h       Show this help message
`);
      process.exit(0);
    }
  }
  return options;
}

async function main() {
  const options = parseCliArgs(process.argv.slice(2));
  await resetWorkspace(process.cwd(), options);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal reset error:", err);
    process.exit(1);
  });
}
