import { describe, expect, it } from "bun:test";
import { formatStatusBadge } from "../monitor-ci";

describe("Gitea Actions CI/CD Monitor Script", () => {
  it("should format status badges correctly", () => {
    expect(formatStatusBadge("in_progress", null)).toBe("[RUNNING]");
    expect(formatStatusBadge("queued", null)).toBe("[QUEUED]");
    expect(formatStatusBadge("waiting", null)).toBe("[WAITING]");
    expect(formatStatusBadge("completed", "success")).toBe("[SUCCESS]");
    expect(formatStatusBadge("completed", "failure")).toBe("[FAILED]");
    expect(formatStatusBadge("completed", "cancelled")).toBe("[CANCELLED]");
    expect(formatStatusBadge("other", null)).toBe("[OTHER]");
  });

  it("should execute CLI help successfully", () => {
    const proc = Bun.spawnSync(["bun", "scripts/monitor-ci.ts", "--help"], {
      cwd: process.cwd(),
    });
    expect(proc.exitCode).toBe(0);
    const stdout = proc.stdout.toString();
    expect(stdout).toContain("Gitea CI Monitor");
    expect(stdout).toContain("--watch");
    expect(stdout).toContain("--limit");
  });
});
