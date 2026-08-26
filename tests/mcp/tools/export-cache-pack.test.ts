import { describe, expect, it } from "bun:test";
import { existsSync, unlinkSync } from "node:fs";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_export_cache_pack", () => {
  it("should export incremental cache pack to file", async () => {
    const tempPack = "cddm-test-export.cddmpack";
    try {
      const res = await executeTool("cddm_export_cache_pack", {
        cache_dir: ".cddm/cache.db",
        output_pack_path: tempPack,
      });

      expect(res).toBeDefined();
      expect(res.pack_file).toBeDefined();
    } finally {
      if (existsSync(tempPack)) unlinkSync(tempPack);
    }
  });

  it("should reject export when cache_dir does not exist", async () => {
    await assertToolError(
      "cddm_export_cache_pack",
      { cache_dir: "non/existent/cache.db", output_pack_path: "temp.pack" },
      RPC_ERRORS.INTERNAL_ERROR,
    );
  });
});
