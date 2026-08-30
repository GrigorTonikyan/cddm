import { describe, expect, it } from "bun:test";
import { callMcpStdio } from "./helpers";

describe("MCP Dynamic Resource Templates (MCP 2026 Standards)", () => {
  it("should list dynamic resource templates matching RFC 6570 syntax", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 101,
      method: "resources/templates/list",
      params: {},
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();
    const result = res.result as {
      resourceTemplates: Array<{ uriTemplate: string; name: string }>;
    };
    expect(result?.resourceTemplates).toBeDefined();
    expect(result.resourceTemplates.length).toBeGreaterThanOrEqual(3);

    const templates = result.resourceTemplates.map((t) => t.uriTemplate);
    expect(templates).toContain("cddm://file/{path}/clones");
    expect(templates).toContain("cddm://cluster/{cluster_id}/details");
    expect(templates).toContain("cddm://file/{path}/tokens");
  });

  it("should read parameterized file tokens template resource", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 102,
      method: "resources/read",
      params: { uri: "cddm://file/crates%2Fcddm-core%2Fsrc%2Flib.rs/tokens" },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();
    const contents = (res.result as any)?.contents;
    expect(contents).toBeDefined();
    expect(contents.length).toBe(1);

    const data = JSON.parse(contents[0].text);
    expect(data.file).toBe("crates/cddm-core/src/lib.rs");
    expect(data.language).toBe("Rust");
    expect(data.token_count).toBeGreaterThan(0);
    expect(data.token_spans.length).toBeGreaterThan(0);
  });

  it("should read parameterized file clones template resource", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 103,
      method: "resources/read",
      params: { uri: "cddm://file/src%2Fmain.rs/clones" },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();
    const contents = (res.result as any)?.contents;
    expect(contents).toBeDefined();
    expect(contents.length).toBe(1);

    const data = JSON.parse(contents[0].text);
    expect(data.file).toBe("src/main.rs");
    expect(typeof data.total_clones).toBe("number");
  });
});
