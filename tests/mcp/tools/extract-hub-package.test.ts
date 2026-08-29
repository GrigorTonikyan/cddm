import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_extract_hub_package", () => {
  it("should generate shared package extraction plan for federation hub", async () => {
    const res = await executeTool("cddm_extract_hub_package", {
      cluster_id: 1,
      package_name: "@org/shared-utils",
      package_type: "npm",
      target_dir: "packages/shared-utils",
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(res.package_name).toBe("@org/shared-utils");
    expect(res.package_type).toBe("npm");
    expect(Array.isArray(res.generated_files)).toBe(true);
    expect(Array.isArray(res.repo_updates)).toBe(true);
    expect(typeof res.lines_saved).toBe("number");
  });

  it("should generate cargo crate extraction plan", async () => {
    const res = await executeTool("cddm_extract_hub_package", {
      cluster_id: 1,
      package_name: "cddm-shared-common",
      package_type: "cargo",
      target_dir: "crates/cddm-shared-common",
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(res.package_name).toBe("cddm-shared-common");
    expect(res.package_type).toBe("cargo");
    expect(Array.isArray(res.generated_files)).toBe(true);
  });

  it("should reject invocation when cluster_id is missing", async () => {
    await assertToolError("cddm_extract_hub_package", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
