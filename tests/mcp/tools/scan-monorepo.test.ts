import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_scan_monorepo", () => {
  it("should discover packages and scan monorepo boundaries", async () => {
    const res = await executeTool("cddm_scan_monorepo", {
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.workspaces)).toBe(true);
    expect(typeof res.total_workspaces).toBe("number");
    expect(typeof res.total_clones).toBe("number");
    for (const ws of res.workspaces) {
      expect(typeof ws.name).toBe("string");
      expect(typeof ws.path).toBe("string");
      expect(typeof ws.manifest_file).toBe("string");
      expect(typeof ws.package_type).toBe("string");
    }
  });

  it("should classify discovered workspaces with accurate ecosystem metadata", async () => {
    const res = await executeTool("cddm_scan_monorepo", {
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    // In CDDM repository, all 4 crates are Rust (Cargo)
    const cargoWs = res.workspaces.find((w: { name: string }) => w.name === "cddm-core");
    expect(cargoWs).toBeDefined();
    expect(cargoWs.package_type).toBe("Rust (Cargo)");
    expect(cargoWs.manifest_file).toBe("Cargo.toml");
  });

  it("should return compact monorepo summary when detail_level is compact", async () => {
    const res = await executeTool("cddm_scan_monorepo", {
      directory: ".",
      min_tokens: 50,
      detail_level: "compact",
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.total_workspaces).toBe("number");
    expect(Array.isArray(res.top_clusters)).toBe(true);
    expect(res.scan_result).toBeUndefined();
  });
});
