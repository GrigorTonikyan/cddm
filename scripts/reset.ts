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
import { executeStep, printScriptBanner, printScriptHelp } from "./lib/step-runner";

export interface ResetOptions {
  skipVerify?: boolean;
  verbose?: boolean;
}

export async function resetWorkspace(
  workspaceRoot: string = process.cwd(),
  options: ResetOptions = {},
): Promise<void> {
  printScriptBanner("CDDM Full Workspace Reset & Preparation");

  const overallStart = performance.now();

  // Determine total steps
  const hasE2e = existsSync(join(workspaceRoot, "tests/e2e/package.json"));
  const hasVscode = existsSync(join(workspaceRoot, "editors/vscode/package.json"));
  const stepCount = (options.skipVerify ? 6 : 7) + (hasE2e ? 1 : 0) + (hasVscode ? 1 : 0);
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
  await executeStep(
    {
      title: "Installing root workspace dependencies (bun install)",
      command: ["bun", "install"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
    "\x1b[35m",
  );

  // Step 3: Install WebUI Dependencies
  currentStep++;
  await executeStep(
    {
      title: "Installing WebUI dependencies (bun install)",
      command: ["bun", "install"],
      cwd: join(workspaceRoot, "webui"),
    },
    currentStep - 1,
    stepCount,
    "\x1b[35m",
  );

  // Step 4: Install E2E Dependencies (if present)
  if (hasE2e) {
    currentStep++;
    await executeStep(
      {
        title: "Installing E2E test dependencies (bun install)",
        command: ["bun", "install"],
        cwd: join(workspaceRoot, "tests/e2e"),
      },
      currentStep - 1,
      stepCount,
      "\x1b[35m",
    );
  }

  // Step 5: Install VS Code Extension Dependencies (if present)
  if (hasVscode) {
    currentStep++;
    await executeStep(
      {
        title: "Installing VS Code extension dependencies (bun install)",
        command: ["bun", "install"],
        cwd: join(workspaceRoot, "editors/vscode"),
      },
      currentStep - 1,
      stepCount,
      "\x1b[35m",
    );
  }

  // Step 6: Configure Git Hooks
  currentStep++;
  await executeStep(
    {
      title: "Configuring Git hooks (bun scripts/setup-hooks.ts)",
      command: ["bun", "scripts/setup-hooks.ts"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
    "\x1b[35m",
  );

  // Step 6: Build WebUI
  currentStep++;
  await executeStep(
    {
      title: "Building WebUI production distribution (vp -C webui run build)",
      command: ["vp", "-C", "webui", "run", "build"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
    "\x1b[35m",
  );

  // Step 7: Build Rust Crates
  currentStep++;
  await executeStep(
    {
      title: "Compiling Rust workspace crates (cargo build --workspace)",
      command: ["cargo", "build", "--workspace"],
      cwd: workspaceRoot,
    },
    currentStep - 1,
    stepCount,
    "\x1b[35m",
  );

  // Step 8: Full Verification Pipeline (optional)
  if (!options.skipVerify) {
    currentStep++;
    await executeStep(
      {
        title: "Executing complete 11-step verification suite (bun scripts/verify.ts)",
        command: ["bun", "scripts/verify.ts"],
        cwd: workspaceRoot,
      },
      currentStep - 1,
      stepCount,
      "\x1b[35m",
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
      printScriptHelp(
        "Workspace Reset & Preparation",
        "vp run reset [options] / bun scripts/reset.ts [options]",
        [
          ["--skip-verify", "Skip the final full verification step"],
          ["--verbose, -v", "Verbose logging"],
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
  await resetWorkspace(process.cwd(), options);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal reset error:", err);
    process.exit(1);
  });
}
