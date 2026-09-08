import { describe, expect, it } from "bun:test";
import { verifyCurrentReleaseNotes } from "../check-release-notes";

describe("scripts/check-release-notes.ts", () => {
  it("verifies current workspace release notes satisfy all standard rules", () => {
    const res = verifyCurrentReleaseNotes();
    expect(res.valid).toBe(true);
    expect(res.errors).toHaveLength(0);
  });

  it("executes CLI verification without error", () => {
    const proc = Bun.spawnSync(["bun", "scripts/check-release-notes.ts"]);
    expect(proc.exitCode).toBe(0);
    const stdout = proc.stdout.toString();
    expect(stdout).toContain("Rich Release Notes & MCP Standard");
    expect(stdout).toContain("[PASS]");
  });
});
