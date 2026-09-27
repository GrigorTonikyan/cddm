import { describe, expect, it } from "bun:test";
import { callMcpStdio, executeTool, RPC_ERRORS } from "./helpers";

describe("MCP Apps Protocol & Interactive UI Refactoring Widgets", () => {
  it("should advertise 'apps' capability in initialize handshake", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2026-07-28",
        capabilities: {},
        clientInfo: { name: "cddm-apps-test", version: "4.3.0" },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(1);
    expect(res.error).toBeUndefined();

    const result = res.result as {
      capabilities?: {
        apps?: {
          widgets?: boolean;
          render?: boolean;
        };
      };
    };

    expect(result?.capabilities?.apps?.widgets).toBe(true);
    expect(result?.capabilities?.apps?.render).toBe(true);
  });

  it("should list available generative UI widgets via apps/widgets/list", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 2,
      method: "apps/widgets/list",
      params: {},
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();

    const result = res.result as {
      widgets?: Array<{
        widgetType: string;
        title: string;
        description: string;
        supportedModes: string[];
        propsSchema?: Record<string, unknown>;
      }>;
    };

    expect(result?.widgets).toBeDefined();
    expect(Array.isArray(result?.widgets)).toBe(true);
    expect(result?.widgets?.length).toBeGreaterThanOrEqual(2);

    const widgetTypes = result?.widgets?.map((w) => w.widgetType);
    expect(widgetTypes).toContain("diff-split-view");
    expect(widgetTypes).toContain("cluster-treemap");

    const diffWidget = result?.widgets?.find((w) => w.widgetType === "diff-split-view");
    expect(diffWidget?.title).toContain("Diff");
    expect(diffWidget?.supportedModes).toContain("inline");
    expect(diffWidget?.supportedModes).toContain("modal");

    const treemapWidget = result?.widgets?.find((w) => w.widgetType === "cluster-treemap");
    expect(treemapWidget?.title).toContain("Treemap");
  });

  it("should render diff-split-view widget via apps/render", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 3,
      method: "apps/render",
      params: {
        widgetType: "diff-split-view",
        props: {
          fileA: "src/old.rs",
          fileB: "src/new.rs",
          patch: "@@ -1,3 +1,3 @@\n-old line\n+new line\n context",
          mode: "side-by-side",
        },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();

    const result = res.result as {
      widgetType?: string;
      title?: string;
      html?: string;
    };

    expect(result?.widgetType).toBe("diff-split-view");
    expect(result?.html).toBeDefined();
    expect(result?.html).toContain("diff-split-view");
    expect(result?.html).toContain("src/old.rs");
    expect(result?.html).toContain("src/new.rs");
    expect(result?.html).toContain("old line");
    expect(result?.html).toContain("new line");
  });

  it("should render cluster-treemap widget via apps/render", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 4,
      method: "apps/render",
      params: {
        widgetType: "cluster-treemap",
        props: {
          clusters: [
            {
              id: "cluster-test-1",
              name: "Auth duplication",
              instances: 4,
              totalLines: 120,
              files: ["auth/login.rs", "auth/oauth.rs"],
            },
          ],
        },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();

    const result = res.result as {
      widgetType?: string;
      title?: string;
      html?: string;
    };

    expect(result?.widgetType).toBe("cluster-treemap");
    expect(result?.html).toBeDefined();
    expect(result?.html).toContain("cluster-treemap");
    expect(result?.html).toContain("Auth duplication");
  });

  it("should return invalid params error for unknown widgetType in apps/render", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 5,
      method: "apps/render",
      params: {
        widgetType: "unknown-nonexistent-widget",
        props: {},
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeDefined();
    expect(res.error?.code).toBe(RPC_ERRORS.INVALID_PARAMS);
  });

  it("should return embedded widget when include_widget: true in cddm_suggest_refactor", async () => {
    const res = await executeTool<{
      strategy: string;
      _widget?: {
        widgetType: string;
        title: string;
        html: string;
      };
    }>("cddm_suggest_refactor", {
      file_a: "crates/cddm-cli/src/commands/diff.rs",
      start_line_a: 33,
      end_line_a: 66,
      file_b: "crates/cddm-cli/src/commands/scan.rs",
      start_line_b: 43,
      end_line_b: 76,
      include_widget: true,
    });

    expect(res).toBeDefined();
    expect(res.strategy).toBeDefined();
    expect(res._widget).toBeDefined();
    expect(res._widget?.widgetType).toBe("diff-split-view");
    expect(res._widget?.html).toBeDefined();
    expect(res._widget?.html).toContain("diff-split-view");
  });
});
