import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_detect_overlap", () => {
  it("should scan workspace for ecosystem library overlap", async () => {
    const res = await executeTool("cddm_detect_overlap", {
      directory: ".",
      threshold: 0.1,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.matches)).toBe(true);
    expect(typeof res.total_files_scanned).toBe("number");
    expect(typeof res.scanned_functions).toBe("number");
    expect(typeof res.summary).toBe("string");
  });

  it("should scan with default arguments", async () => {
    const res = await executeTool("cddm_detect_overlap", {});

    expect(res).toBeDefined();
    expect(Array.isArray(res.matches)).toBe(true);
  });

  it("should reject invocation when target directory does not exist", async () => {
    await assertToolError(
      "cddm_detect_overlap",
      { directory: "non/existent/directory/12345" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
