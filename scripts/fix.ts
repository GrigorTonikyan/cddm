#!/usr/bin/env bun
/**
 * Cross-platform auto-fix runner for CDDM.
 * Automatically fixes all auto-fixable formatting, linter issues, and hooks,
 * then executes the full verification pipeline to ensure 100% passing state.
 * Uses Vite Plus (`vp check --fix`) workspace-wide.
 */

import { executeStep, printScriptBanner, ScriptStep } from "./lib/step-runner";

const FIX_STEPS: ScriptStep[] = [
  {
    title: "Auto-format Rust codebase (cargo fmt)",
    command: ["cargo", "fmt"],
  },
  {
    title: "Auto-fix Rust Clippy linter warnings (cargo clippy --fix)",
    command: [
      "cargo",
      "clippy",
      "--workspace",
      "--all-targets",
      "--fix",
      "--allow-dirty",
      "--allow-staged",
      "--",
      "-D",
      "warnings",
    ],
    allowFailure: true,
  },
  {
    title: "Workspace-wide Vite Plus auto-formatting and type-aware lint fix (vp check --fix)",
    command: ["vp", "check", "--fix"],
    allowFailure: true,
  },
  {
    title: "Configure and enforce Git hooks path (.githooks)",
    command: ["bun", "scripts/setup-hooks.ts"],
  },
];

async function main() {
  printScriptBanner("CDDM Automated Codebase Fixer & Verifier", "\x1b[35m");

  const overallStart = performance.now();

  for (const [i, step] of FIX_STEPS.entries()) {
    await executeStep(step, i, FIX_STEPS.length, "\x1b[35m");
  }

  const fixTime = (performance.now() - overallStart) / 1000;
  console.log(`\n\x1b[32m[PASS] Auto-fix steps completed in ${fixTime.toFixed(2)}s.\x1b[0m`);
  console.log("\x1b[36m--> Launching complete verification pipeline...\x1b[0m\n");

  // Run verify.ts directly via bun
  const verifyProc = Bun.spawn(["bun", "scripts/verify.ts"], {
    stdout: "inherit",
    stderr: "inherit",
  });

  const verifyExitCode = await verifyProc.exited;
  if (verifyExitCode !== 0) {
    process.exit(verifyExitCode ?? 1);
  }
}

main().catch((err) => {
  console.error("Fatal fix error:", err);
  process.exit(1);
});

export {};
