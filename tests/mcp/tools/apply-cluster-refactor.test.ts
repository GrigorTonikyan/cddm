import { describe, expect, it } from "bun:test";
import { existsSync, unlinkSync } from "node:fs";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_apply_cluster_refactor", () => {
  it("should apply unified diff patch to target file", async () => {
    const tempFile = "cddm-mcp-test-patch-target.txt";
    try {
      await Bun.write(tempFile, "alpha\nbeta\ngamma\n");
      const patch = `--- a/${tempFile}\n+++ b/${tempFile}\n@@ -1,3 +1,3 @@\n alpha\n-beta\n+delta\n gamma\n`;
      const res = await executeTool("cddm_apply_cluster_refactor", {
        patch,
        create_branch: false,
      });

      expect(res).toBeDefined();
      expect(res.success).toBe(true);
      expect(res.hunks_applied).toBe(1);
      expect(Array.isArray(res.modified_files)).toBe(true);
    } finally {
      if (existsSync(tempFile)) unlinkSync(tempFile);
    }
  });

  it("should validate patch in dry_run mode without modifying file on disk", async () => {
    const tempFile = "cddm-mcp-test-patch-dryrun.txt";
    try {
      await Bun.write(tempFile, "line1\nline2\nline3\n");
      const patch = `--- a/${tempFile}\n+++ b/${tempFile}\n@@ -1,3 +1,3 @@\n line1\n-line2\n+modified\n line3\n`;
      const res = await executeTool("cddm_apply_cluster_refactor", {
        patch,
        dry_run: true,
        create_branch: "cddm/test-dryrun-branch",
      });

      expect(res).toBeDefined();
      expect(res.success).toBe(true);
      expect(res.message).toContain("[DRY RUN]");
      // File should remain untouched
      const content = await Bun.file(tempFile).text();
      expect(content).toBe("line1\nline2\nline3\n");
    } finally {
      if (existsSync(tempFile)) unlinkSync(tempFile);
    }
  });

  it("should reject invocation when patch is missing", async () => {
    await assertToolError("cddm_apply_cluster_refactor", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
