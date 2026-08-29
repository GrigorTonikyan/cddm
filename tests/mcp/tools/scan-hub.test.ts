import { describe, expect, it } from "bun:test";
import { assertPropertyTypes, assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_scan_hub", () => {
  it("should scan organization repositories in federation hub", async () => {
    const res = await executeTool("cddm_scan_hub", {
      repositories: ["."],
      min_tokens: 50,
    });

    assertPropertyTypes(res, {
      hub_name: "string",
      total_repos: "number",
      total_files: "number",
      organization_dry_score: "number",
      clusters: "array",
      duplication_matrix: "array",
    });
  });

  it("should scan with default arguments", async () => {
    const res = await executeTool("cddm_scan_hub", {});

    expect(res).toBeDefined();
    expect(typeof res.organization_dry_score).toBe("number");
  });

  it("should reject invocation when non-existent config file is provided", async () => {
    await assertToolError(
      "cddm_scan_hub",
      { config_path: "non/existent/.cddmhub.toml" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
