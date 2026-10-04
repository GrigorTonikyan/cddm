import { describe, expect, it } from "bun:test";
import { resolve } from "node:path";
import { DEPENDENCY_TARGET_DIRS, installWorkspaceDeps, parseArgs } from "../setup-deps";

describe("scripts/setup-deps.ts", () => {
  it("defines expected target directories", () => {
    expect(DEPENDENCY_TARGET_DIRS).toContain("webui");
    expect(DEPENDENCY_TARGET_DIRS).toContain("editors/vscode");
    expect(DEPENDENCY_TARGET_DIRS).not.toContain("tests/e2e");
    expect(DEPENDENCY_TARGET_DIRS.length).toBe(2);
  });

  it("parses CLI flags correctly", () => {
    const opts1 = parseArgs(["--dry-run", "--verbose"]);
    expect(opts1.dryRun).toBe(true);
    expect(opts1.verbose).toBe(true);
    expect(opts1.frozenLockfile).toBeUndefined();

    const opts2 = parseArgs(["-n", "-v", "--frozen-lockfile"]);
    expect(opts2.dryRun).toBe(true);
    expect(opts2.verbose).toBe(true);
    expect(opts2.frozenLockfile).toBe(true);
  });

  it("runs installWorkspaceDeps in dry-run mode without spawning child processes", async () => {
    const root = resolve(import.meta.dir, "../..");
    const results = await installWorkspaceDeps({
      workspaceRoot: root,
      dryRun: true,
      verbose: false,
    });

    expect(results.length).toBe(DEPENDENCY_TARGET_DIRS.length);
    for (const res of results) {
      expect(res.success).toBe(true);
      expect(res.skipped).toBe(false);
      expect(res.elapsedMs).toBe(0);
    }
  });

  it("skips non-existent targets gracefully", async () => {
    const root = resolve(import.meta.dir, "../..");
    const results = await installWorkspaceDeps({
      workspaceRoot: root,
      targets: ["non-existent-subpath-abc-123"],
    });

    expect(results.length).toBe(1);
    expect(results[0]?.target).toBe("non-existent-subpath-abc-123");
    expect(results[0]?.skipped).toBe(true);
    expect(results[0]?.success).toBe(true);
  });

  it("executes CLI entrypoint with --dry-run cleanly", () => {
    const proc = Bun.spawnSync(["bun", "scripts/setup-deps.ts", "--dry-run"]);
    expect(proc.exitCode).toBe(0);
    const stdout = proc.stdout.toString();
    expect(stdout).toContain("Synchronizing nested workspace dependencies");
    expect(stdout).toContain("All workspace dependencies are synchronized and ready!");
  });
});
