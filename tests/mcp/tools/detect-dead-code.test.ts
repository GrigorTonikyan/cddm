import { mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_detect_dead_code", () => {
  it("should run polyglot dead code detection on workspace", async () => {
    const res = await executeTool("cddm_detect_dead_code", {
      directory: ".",
      min_tokens: 50,
      static_only: true,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_dead_items).toBe("number");
    expect(typeof res.dead_functions).toBe("number");
    expect(typeof res.unreachable_blocks).toBe("number");
    expect(typeof res.dead_clones).toBe("number");
    expect(typeof res.total_dead_lines).toBe("number");
    expect(typeof res.estimated_savings_pct).toBe("number");
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);

  it("should support running with default parameters", async () => {
    const res = await executeTool("cddm_detect_dead_code", {});

    expect(res).toBeDefined();
    expect(typeof res.total_dead_items).toBe("number");
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);

  it("should not classify closing braces as unreachable statement dead code (Gitea Issue #144)", async () => {
    const tmpDir = join(import.meta.dir, "../fixtures/tmp-issue-144-" + Date.now());
    mkdirSync(tmpDir, { recursive: true });
    try {
      const code = `function resolvePath(dir: string, file: string): string | null {
  const list = [dir, file];
  for (const item of list) {
    if (item.length > 0) {
      return item;
    }
  }
  return null;
}
`;
      await Bun.write(join(tmpDir, "sample.ts"), code);
      const res = await executeTool("cddm_detect_dead_code", {
        directory: tmpDir,
        min_tokens: 1,
        static_only: true,
      });

      expect(res).toBeDefined();
      const unreachable = (res.items as Array<{ kind: string }>).filter(
        (i) => i.kind === "unreachable_block",
      );
      expect(unreachable.length).toBe(0);
    } finally {
      rmSync(tmpDir, { recursive: true, force: true });
    }
  }, 30000);

  it("should return compact dead code summary when summary_only is true", async () => {
    const res = await executeTool("cddm_detect_dead_code", {
      directory: ".",
      min_tokens: 50,
      static_only: true,
      summary_only: true,
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.total_dead_items).toBe("number");
    expect(Array.isArray(res.top_items)).toBe(true);
    expect(res.items).toBeUndefined();
  }, 30000);
});
