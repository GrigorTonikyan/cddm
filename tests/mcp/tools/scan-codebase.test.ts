import { describe, expect, it } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: scan_codebase", () => {
  it("should perform polyglot code duplication scan on directory", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_files).toBe("number");
    expect(typeof res.total_tokens).toBe("number");
    expect(typeof res.total_clones).toBe("number");
    expect(typeof res.dry_health_score).toBe("number");
    expect(res.total_files).toBeGreaterThan(0);
    expect(res.dry_health_score).toBeGreaterThanOrEqual(0);
    expect(res.dry_health_score).toBeLessThanOrEqual(100);
  }, 30000);

  it("should handle default directory and token arguments", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
    });
    expect(res).toBeDefined();
    expect(res.total_files).toBeGreaterThan(0);
  }, 30000);

  it("should support detect_type3 toggle in scan_codebase", async () => {
    const resWith = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type3: true,
    });
    const resWithout = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type3: false,
    });

    expect(resWith).toBeDefined();
    expect(resWithout).toBeDefined();
    expect(resWithout.total_files).toBe(resWith.total_files);
  }, 30000);

  it("should support detect_type4 toggle in scan_codebase", async () => {
    const resWith = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type4: true,
    });
    const resWithout = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type4: false,
    });

    expect(resWith).toBeDefined();
    expect(resWithout).toBeDefined();
    expect(resWithout.total_files).toBe(resWith.total_files);
  }, 30000);

  it("should return compact summary when summary_only is true", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      summary_only: true,
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.total_files).toBe("number");
    expect(typeof res.dry_health_score).toBe("number");
    expect(Array.isArray(res.top_clusters)).toBe(true);
    expect(res.clone_pairs).toBeUndefined();
  }, 30000);

  it("should support include_ignored toggle in scan_codebase", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      include_ignored: true,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_files).toBe("number");
    expect(res.total_files).toBeGreaterThan(0);
  }, 30000);

  it("should include tests and spec files when ignore_tests and ignore_mocks are false", async () => {
    const dir = mkdtempSync(join(tmpdir(), "cddm-mcp-tests-"));
    try {
      mkdirSync(join(dir, "src"), { recursive: true });
      mkdirSync(join(dir, "tests"), { recursive: true });
      mkdirSync(join(dir, "mocks"), { recursive: true });

      writeFileSync(join(dir, "src", "main.ts"), "export const mainValue = 1;\n");
      writeFileSync(join(dir, "src", "app.test.ts"), "export const testValue = 2;\n");
      writeFileSync(join(dir, "src", "app.spec.ts"), "export const specValue = 3;\n");
      writeFileSync(join(dir, "tests", "suite.ts"), "export const suiteValue = 4;\n");
      writeFileSync(join(dir, "mocks", "data.ts"), "export const mockValue = 5;\n");

      const defaultRes = await executeTool("scan_codebase", {
        directory: dir,
        min_tokens: 50,
      });
      expect(defaultRes.total_files).toBe(1);

      const includeTestsRes = await executeTool("scan_codebase", {
        directory: dir,
        min_tokens: 50,
        ignore_tests: false,
        ignore_mocks: false,
      });
      expect(includeTestsRes.total_files).toBe(5);
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 30000);

  it("should enforce max_file_lines rules in scan_codebase when ignore_tests=false", async () => {
    const dir = mkdtempSync(join(tmpdir(), "cddm-mcp-scan-lines-"));
    try {
      mkdirSync(join(dir, "tests"), { recursive: true });
      const bigFile = Array.from({ length: 10 }, (_, i) => `export const x${i} = ${i};`).join("\n");
      writeFileSync(join(dir, "tests", "oversized.test.ts"), `${bigFile}\n`);
      const rulesPath = join(dir, ".cddmrules.toml");
      writeFileSync(
        rulesPath,
        `[[line_budgets]]\nname = "max-5-lines"\npattern = "**/*.ts"\nmax_file_lines = 5\nseverity = "error"\n`,
      );

      const res = await executeTool("scan_codebase", {
        directory: dir,
        min_tokens: 50,
        rules: rulesPath,
        ignore_tests: false,
        ignore_mocks: false,
      });
      expect(Array.isArray(res.policy_violations)).toBe(true);
      expect(res.policy_violations.length).toBe(1);
      expect(res.policy_violations[0].rule_type).toBe("line_budget");
      expect(res.policy_violations[0].line_count).toBe(10);
      expect(res.policy_violations[0].max_lines).toBe(5);
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 30000);

  it("should enforce min_dry_score quality gate and reject out-of-range thresholds", async () => {
    const dir = mkdtempSync(join(tmpdir(), "cddm-mcp-scan-qg-"));
    try {
      mkdirSync(join(dir, "src"), { recursive: true });
      const block = Array.from({ length: 12 }, (_, i) => `  const item${i} = n + ${i};`).join("\n");
      writeFileSync(
        join(dir, "src", "a.ts"),
        `export function fnA(n: number) {\n${block}\n  return item11;\n}\n`,
      );
      writeFileSync(
        join(dir, "src", "b.ts"),
        `export function fnB(n: number) {\n${block}\n  return item11;\n}\n`,
      );

      const res = await executeTool("scan_codebase", {
        directory: dir,
        min_tokens: 10,
        min_dry_score: 100.0,
      });
      expect(Array.isArray(res.policy_violations)).toBe(true);
      expect(
        res.policy_violations.some((v: { rule_type: string }) => v.rule_type === "quality_gate"),
      ).toBe(true);

      await assertToolError(
        "scan_codebase",
        { directory: dir, min_dry_score: 150 },
        RPC_ERRORS.INVALID_PARAMS,
      );
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 30000);
});
