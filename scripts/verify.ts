#!/usr/bin/env bun
/**
 * Cross-platform verification pipeline for CDDM.
 * Single source of truth across Windows, Linux, and macOS.
 * Uses Vite Plus (`vp`) exclusively for JS/TS formatting, linting, type-checking, and building.
 */

interface Step {
  title: string;
  command: string[];
  cwd?: string;
}

const STEPS: Step[] = [
  {
    title: "Rust code formatting check (cargo fmt --check)",
    command: ["cargo", "fmt", "--check"],
  },
  {
    title: "Rust Clippy zero-warning linter (cargo clippy)",
    command: ["cargo", "clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
  },
  {
    title: "Rust unit & integration test suite (cargo test)",
    command: ["cargo", "test", "--workspace"],
  },
  {
    title: "Repository scripts TypeScript type check (tsc -p tsconfig.json)",
    command: ["bunx", "tsc", "-p", "tsconfig.json"],
  },
  {
    title: "Repository scripts unit tests (bun test scripts/__tests__)",
    command: ["bun", "test", "scripts/__tests__"],
  },
  {
    title: "Workspace-wide Vite Plus type-aware verification (vp check)",
    command: ["vp", "check"],
  },
  {
    title: "WebUI Vitest test suite (vp -C webui run test)",
    command: ["vp", "-C", "webui", "run", "test"],
  },
  {
    title: "WebUI production bundle build (vp -C webui run build)",
    command: ["vp", "-C", "webui", "run", "build"],
  },
  {
    title: "CDDM Dogfooding Self-Scan (cddm scan .)",
    command: [
      "cargo",
      "run",
      "-p",
      "cddm-cli",
      "--",
      "scan",
      ".",
      "--min-tokens",
      "50",
      "--fail-threshold",
      "15.0",
    ],
  },
];

async function runStep(step: Step, index: number, total: number): Promise<void> {
  const stepNum = `[${index + 1}/${total}]`;
  console.log(`\n\x1b[33m${stepNum} ${step.title}...\x1b[0m`);
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
      `\n\x1b[31m✖ Step failed with exit code ${exitCode} (${elapsedMs}ms): ${step.command.join(" ")}\x1b[0m\n`,
    );
    process.exit(exitCode ?? 1);
  }

  console.log(`\x1b[32m✔ Passed (${elapsedMs}ms)\x1b[0m`);
}

async function main() {
  console.log("\x1b[36m=======================================================\x1b[0m");
  console.log("\x1b[36m       CDDM Full Repository Verification Pipeline      \x1b[0m");
  console.log("\x1b[36m=======================================================\x1b[0m");

  const overallStart = performance.now();

  for (const [i, step] of STEPS.entries()) {
    await runStep(step, i, STEPS.length);
  }

  const totalTime = (performance.now() - overallStart) / 1000;
  console.log("\n\x1b[32m=======================================================\x1b[0m");
  console.log(
    `\x1b[32m   All ${STEPS.length} quality checks passed cleanly in ${totalTime.toFixed(2)}s!   \x1b[0m`,
  );
  console.log("\x1b[32m=======================================================\x1b[0m\n");
}

main().catch((err) => {
  console.error("Fatal verification error:", err);
  process.exit(1);
});

export {};
