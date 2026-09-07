import { describe, expect, it } from "bun:test";
import { existsSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

function getCddmCliBinary(): string | null {
  const exeName = process.platform === "win32" ? "cddm.exe" : "cddm";
  const debugPath = join(import.meta.dir, "../../../target/debug", exeName);
  const releasePath = join(import.meta.dir, "../../../target/release", exeName);
  if (existsSync(debugPath)) return debugPath;
  if (existsSync(releasePath)) return releasePath;
  return null;
}

describe("MCP Tool: cddm_export_cache_pack", () => {
  it("should export incremental cache pack to file", async () => {
    if (!existsSync(".cddm/cache.db")) {
      const cliBin = getCddmCliBinary();
      if (cliBin) {
        Bun.spawnSync([cliBin, "scan", "crates/cddm-lsp", "--in-tree-cache"]);
      } else {
        Bun.spawnSync([
          "cargo",
          "run",
          "-p",
          "cddm-cli",
          "--",
          "scan",
          "crates/cddm-lsp",
          "--in-tree-cache",
        ]);
      }
    }
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
  }, 30000);

  it("should reject export when cache_dir does not exist", async () => {
    await assertToolError(
      "cddm_export_cache_pack",
      { cache_dir: "non/existent/cache.db", output_pack_path: "temp.pack" },
      RPC_ERRORS.INTERNAL_ERROR,
    );
  });
});
