import { describe, expect, it } from "bun:test";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

function getScriptFiles(dir: string): string[] {
  const files: string[] = [];
  const entries = readdirSync(dir);
  for (const entry of entries) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);
    if (stat.isDirectory()) {
      files.push(...getScriptFiles(fullPath));
    } else if (entry.endsWith(".ts") || entry.endsWith(".js")) {
      files.push(fullPath);
    }
  }
  return files;
}

describe("Bun-Only Runtime & API Mandate Scanner", () => {
  it("should enforce zero child_process imports across all scripts/", () => {
    const scriptsDir = join(process.cwd(), "scripts");
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
      const content = readFileSync(file, "utf8");
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
