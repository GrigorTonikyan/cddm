import { describe, expect, it } from "bun:test";

describe("scripts/check-milestones.ts", () => {
  it("executes milestone check without throwing", () => {
    const proc = Bun.spawnSync(["bun", "scripts/check-milestones.ts"]);
    expect(proc.exitCode).toBe(0);
    const stdout = proc.stdout.toString();
    expect(stdout).toContain("milestone governance compliance");
  });
});
