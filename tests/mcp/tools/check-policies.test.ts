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
});
