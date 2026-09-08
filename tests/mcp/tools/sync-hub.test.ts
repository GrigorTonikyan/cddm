import { describe, expect, it } from "bun:test";
import { assertPropertyTypes, executeTool } from "../helpers";

describe("MCP Tool: cddm_sync_hub", () => {
  it("should synchronize privacy-preserving fingerprints with federation hub", async () => {
    const res = await executeTool("cddm_sync_hub", {
      repo_name: "test-cddm-mcp",
      remote_endpoint: "https://hub.example.org/v1/peering",
      org_salt: "mcp-test-secret-salt",
      dry_run: true,
    });

    assertPropertyTypes(res, {
      status: "string",
      remote_endpoint: "string",
      peers_synced: "number",
      total_remote_tokens: "number",
      cross_repo_matches_found: "number",
      privacy_mode: "string",
      duration_ms: "number",
      sync_manifest: "array",
    });

    expect(res.status).toBe("synced_ok");
    expect(res.remote_endpoint).toBe("https://hub.example.org/v1/peering");
    expect(res.privacy_mode).toContain("BLAKE3");
  });

  it("should support default parameters and return valid schema payload", async () => {
    const res = await executeTool("cddm_sync_hub", {});

    expect(res).toBeDefined();
    expect(res.status).toBe("synced_ok");
    expect(typeof res.peers_synced).toBe("number");
    expect(Array.isArray(res.sync_manifest)).toBe(true);
  });
});
