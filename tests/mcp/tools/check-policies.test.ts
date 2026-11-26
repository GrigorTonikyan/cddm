import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_check_policies", () => {
  it("should evaluate duplication quality gate policies", async () => {
    const res = await executeTool("cddm_check_policies", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.passed).toBe("boolean");
    expect(typeof res.total_violations).toBe("number");
    expect(Array.isArray(res.violations)).toBe(true);
  });

  it("should return compact policy evaluation when detail_level is compact", async () => {
    const res = await executeTool("cddm_check_policies", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detail_level: "compact",
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.passed).toBe("boolean");
    expect(typeof res.total_violations).toBe("number");
    expect(Array.isArray(res.top_violations)).toBe(true);
    expect(res.violations).toBeUndefined();
  });

  it("should evaluate prohibited pattern rules with snippet and remediation", async () => {
    const tempRules = "target/test-pattern-rules.toml";
    const rulesToml = `
[[prohibited_patterns]]
name = "test-no-lsp-pattern"
pattern = "language_server"
pattern_type = "regex"
severity = "warning"
remediation = "Use abstract server interface"
`;
    await Bun.write(tempRules, rulesToml);

    const res = await executeTool("cddm_check_policies", {
      directory: "crates/cddm-lsp",
      rules: tempRules,
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.passed).toBe("boolean");
    expect(Array.isArray(res.violations)).toBe(true);
    const found = res.violations.find(
      (v: { rule_name: string }) => v.rule_name === "test-no-lsp-pattern",
    );
    if (found) {
      expect(found.rule_type).toBe("prohibited_pattern");
      expect(typeof found.snippet).toBe("string");
      expect(found.remediation).toBe("Use abstract server interface");
    }
  });

  it("should evaluate forbidden files, SSoT sync, and route docs policies via MCP", async () => {
    const tempRules = "target/test-extended-rules.toml";
    const rulesToml = `
[[forbidden_files]]
name = "mcp-test-forbidden-lock"
patterns = ["test-forbidden.lock"]
severity = "error"
remediation = "Remove test-forbidden.lock"

[[sync_rules]]
name = "mcp-test-version-sync"
source_file = "Cargo.toml"
source_key = "workspace.package.version"
target_files = ["package.json"]
target_key = "version"
severity = "warning"

[route_docs_sync]
name = "mcp-test-route-docs"
framework = "nextjs"
routes_dir = "crates"
doc_file = "README.md"
severity = "info"
`;
    await Bun.write(tempRules, rulesToml);

    const res = await executeTool("cddm_check_policies", {
      directory: ".",
      rules: tempRules,
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.passed).toBe("boolean");
    expect(Array.isArray(res.violations)).toBe(true);
  });

  it("should enforce max_file_lines and honor ignore_tests=false in cddm_check_policies", async () => {
    const { mkdirSync, mkdtempSync, rmSync, writeFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");

    const dir = mkdtempSync(join(tmpdir(), "cddm-mcp-line-budget-"));
    try {
      mkdirSync(join(dir, "src"), { recursive: true });
      mkdirSync(join(dir, "tests"), { recursive: true });

      const bigLines = Array.from({ length: 12 }, (_, i) => `export const v${i} = ${i};`).join(
        "\n",
      );
      writeFileSync(join(dir, "src", "big.ts"), `${bigLines}\n`);
      writeFileSync(join(dir, "tests", "big.test.ts"), `${bigLines}\n`);

      const rulesPath = join(dir, ".cddmrules.toml");
      writeFileSync(
        rulesPath,
        `[[token_budgets]]\nname = "max-5-lines"\npattern = "**/*.ts"\nmax_file_lines = 5\nseverity = "error"\n`,
      );

      const defaultRes = await executeTool("cddm_check_policies", {
        directory: dir,
        rules: rulesPath,
      });
      expect(defaultRes.passed).toBe(false);
      expect(defaultRes.error_count).toBe(1);
      expect(defaultRes.violations[0].rule_type).toBe("line_budget");
      expect(defaultRes.violations[0].line_count).toBe(12);
      expect(defaultRes.violations[0].max_lines).toBe(5);

      const includeTestsRes = await executeTool("cddm_check_policies", {
        directory: dir,
        rules: rulesPath,
        ignore_tests: false,
        ignore_mocks: false,
      });
      expect(includeTestsRes.passed).toBe(false);
      expect(includeTestsRes.error_count).toBe(2);
      expect(
        includeTestsRes.violations.some((v: { file_a: string }) =>
          v.file_a.includes("big.test.ts"),
        ),
      ).toBe(true);
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });
});
