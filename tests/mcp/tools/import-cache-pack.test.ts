import { describe, it } from "bun:test";
import { assertToolError, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_import_cache_pack", () => {
  it("should reject import when pack_file does not exist", async () => {
    await assertToolError(
      "cddm_import_cache_pack",
      { pack_file: "non-existent-pack.cddmpack" },
      RPC_ERRORS.INTERNAL_ERROR,
    );
  });

  it("should reject import when pack_file argument is missing", async () => {
    await assertToolError("cddm_import_cache_pack", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
