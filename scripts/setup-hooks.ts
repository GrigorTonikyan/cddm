#!/usr/bin/env bun
/**
 * Cross-platform Git hooks setup for CDDM using Vite Plus (`vp config`).
 * Single source of truth across Windows, Linux, and macOS.
 */

async function setupGitHooks() {
  console.log("\x1b[36mConfiguring Vite+ git hooks (.vite-hooks)...\x1b[0m");

  const proc = Bun.spawn(["vp", "config"], {
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await proc.exited;
  if (exitCode !== 0) {
    console.error(`\x1b[31mFailed to configure Vite+ hooks (exit code ${exitCode})\x1b[0m`);
    process.exit(exitCode ?? 1);
  }

  // Attempt to set executable permissions on POSIX systems
  if (process.platform !== "win32") {
    try {
      Bun.spawnSync([
        "chmod",
        "+x",
        ".vite-hooks/pre-commit",
        ".vite-hooks/pre-push",
        ".vite-hooks/commit-msg",
      ]);
    } catch {
      // Best effort chmod
    }
  }

  console.log("\x1b[32m✔ Vite+ git hooks successfully configured to .vite-hooks!\x1b[0m");
}

setupGitHooks().catch((err) => {
  console.error("Fatal hook setup error:", err);
  process.exit(1);
});

export {};
