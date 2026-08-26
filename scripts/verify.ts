#!/usr/bin/env bun
/**
 * Cross-platform verification pipeline for CDDM.
 * Single source of truth across Windows, Linux, and macOS.
 * Uses Vite Plus (`vp`) exclusively for JS/TS formatting, linting, type-checking, and building.
 */

import { executeStep, printScriptBanner, ScriptStep } from "./lib/step-runner";

const STEPS: ScriptStep[] = [
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
    title: "Repository scripts unit tests (bun test scripts/tests)",
    command: ["bun", "test", "scripts/tests"],
  },
  {
    title: "MCP Server per-tool test suites & dynamic discovery (bun test tests/mcp)",
    command: ["bun", "test", "tests/mcp"],
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
    title: "Zero-Emoji policy codebase enforcement (bun scripts/check-no-emojis.ts)",
    command: ["bun", "scripts/check-no-emojis.ts"],
  },
  {
    title: "Documentation integrity & cross-reference validation (bun scripts/check-docs.ts)",
    command: ["bun", "scripts/check-docs.ts"],
  },
  {
    title: "File length cap & modularity check (bun scripts/check-file-length.ts)",
    command: ["bun", "scripts/check-file-length.ts"],
  },
  {
    title: "4-Pillar Cross-Interface Feature Parity check (bun scripts/check-feature-parity.ts)",
    command: ["bun", "scripts/check-feature-parity.ts"],
  },
  {
    title: "Ecosystem distribution packaging validation (bun scripts/package-distribution.ts)",
    command: ["bun", "scripts/package-distribution.ts"],
  },
  {
    title: "VS Code extension TypeScript check (tsc -p editors/vscode)",
    command: ["bunx", "tsc", "-p", "editors/vscode/tsconfig.json"],
  },
  {
    title: "VS Code extension unit tests (bun test scripts/tests/vscode-extension.test.ts)",
    command: ["bun", "test", "scripts/tests/vscode-extension.test.ts"],
  },
  {
    title: "VS Code extension VSIX packaging (bun scripts/package-vscode.ts)",
    command: ["bun", "scripts/package-vscode.ts"],
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

async function main() {
  printScriptBanner("CDDM Full Repository Verification Pipeline", "\x1b[36m");

  const overallStart = performance.now();

  for (const [i, step] of STEPS.entries()) {
    await executeStep(step, i, STEPS.length, "\x1b[33m");
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
