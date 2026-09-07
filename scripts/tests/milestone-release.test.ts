import { describe, expect, it } from "bun:test";

describe("CLI: scripts/milestone-release.ts", () => {
  it("should run with --check flag without error", () => {
    const proc = Bun.spawnSync(["bun", "scripts/milestone-release.ts", "--check"]);
    expect(proc.exitCode).toBe(0);
    const output = proc.stdout.toString();
    expect(output).toContain("Checking Gitea milestones");
  });

  it("should run with --dry-run flag without error", () => {
    const proc = Bun.spawnSync(["bun", "scripts/milestone-release.ts", "--dry-run"]);
    expect(proc.exitCode).toBe(0);
  });
});
