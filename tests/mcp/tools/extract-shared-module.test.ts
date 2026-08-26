import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_extract_shared_module", () => {
  it("should generate shared module extraction plan in dry-run mode", async () => {
    const res = await executeTool("cddm_extract_shared_module", {
      occurrences: [
        { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        { file: "crates/cddm-cli/src/tui/views/refactor.rs", start_line: 1, end_line: 14 },
      ],
      target_path: "crates/cddm-core/src/shared_preview.rs",
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(res.target_path).toBeDefined();
    expect(Array.isArray(res.generated_files)).toBe(true);
  });

  it("should reject invocation when cluster_id does not exist", async () => {
    await assertToolError(
      "cddm_extract_shared_module",
      { cluster_id: 999999 },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
