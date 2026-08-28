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

  it("should generate shared module extraction plan for TypeScript files", async () => {
    const res = await executeTool("cddm_extract_shared_module", {
      occurrences: [
        { file: "webui/src/store/slices/watch-slice.ts", start_line: 1, end_line: 10 },
        { file: "webui/src/store/slices/policy-slice.ts", start_line: 1, end_line: 10 },
      ],
      target_path: "webui/src/store/shared_store_utils.ts",
      fn_name: "sharedStoreHelper",
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(res.function_name).toBe("sharedStoreHelper");
    expect(Array.isArray(res.generated_files)).toBe(true);
  });

  it("should synthesize unit test files when generate_tests is true", async () => {
    const res = await executeTool("cddm_extract_shared_module", {
      occurrences: [
        { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        { file: "crates/cddm-cli/src/tui/views/refactor.rs", start_line: 1, end_line: 14 },
      ],
      target_path: "crates/shared_sample",
      crate_type: "crate",
      fn_name: "sampleHelper",
      generate_tests: true,
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.test_files)).toBe(true);
    expect(res.test_files.length).toBeGreaterThan(0);
    expect(res.test_files[0].file_path).toContain("test");
  });

  it("should synthesize benchmark files when generate_benchmarks is true", async () => {
    const res = await executeTool("cddm_extract_shared_module", {
      occurrences: [
        { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        { file: "crates/cddm-cli/src/tui/views/refactor.rs", start_line: 1, end_line: 14 },
      ],
      target_path: "crates/shared_sample",
      crate_type: "crate",
      fn_name: "sampleHelper",
      generate_benchmarks: true,
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.benchmark_files)).toBe(true);
    expect(res.benchmark_files.length).toBeGreaterThan(0);
    expect(res.benchmark_files[0].file_path).toContain("bench");
  });

  it("should reject invocation when cluster_id does not exist", async () => {
    await assertToolError(
      "cddm_extract_shared_module",
      { cluster_id: 999999 },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
