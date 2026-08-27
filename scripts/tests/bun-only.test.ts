import { describe, expect, it } from "bun:test";

function getScriptFiles(dir: string): string[] {
  const glob = new Bun.Glob("**/*.{ts,js}");
  const files: string[] = [];
  for (const match of glob.scanSync({ cwd: dir })) {
    files.push(`${dir}/${match}`.replace(/\\/g, "/"));
  }
  return files;
}

describe("Bun-Only Runtime & API Mandate Scanner", () => {
  it("should enforce zero child_process imports across all scripts/", async () => {
    const scriptsDir = `${process.cwd()}/scripts`.replace(/\\/g, "/");
    const scriptFiles = getScriptFiles(scriptsDir);
    const violations: Array<{ file: string; match: string }> = [];

    const forbiddenPatterns = [
      /import\s+.*from\s+["']node:child_process["']/,
      /import\s+.*from\s+["']child_process["']/,
      /require\(["']node:child_process["']\)/,
      /require\(["']child_process["']\)/,
    ];

    for (const file of scriptFiles) {
      if (file.endsWith("bun-only.test.ts")) continue;
      const content = await Bun.file(file).text();
      for (const pattern of forbiddenPatterns) {
        const match = content.match(pattern);
        if (match) {
          violations.push({ file, match: match[0] });
        }
      }
    }

    expect(violations).toEqual([]);
    expect(violations.length).toBe(0);
  });

  it("should verify that native Bun APIs are functional in script execution context", () => {
    expect(typeof Bun.spawn).toBe("function");
    expect(typeof Bun.spawnSync).toBe("function");
    expect(typeof Bun.write).toBe("function");
    expect(typeof Bun.file).toBe("function");
  });
});
