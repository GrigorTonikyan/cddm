import { describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const rootDir = resolve(import.meta.dir, "../..");
// eslint-disable-next-line @typescript-eslint/no-require-imports
const { resolveBinaryPath } = require("../../npm/cddm/lib/run-binary.js") as {
  resolveBinaryPath: (
    baseName: string,
    options?: {
      platform?: string;
      arch?: string;
      existsSync?: (p: string) => boolean;
      requireResolve?: (spec: string) => string;
      platformDirOverride?: string;
    },
  ) => string;
};

describe("NPM Wrapper Hardening (npm/cddm)", () => {
  it("exposes cddm, cddm-mcp, and cddm-lsp and lists only built optionalDependencies", () => {
    const pkgJson = JSON.parse(readFileSync(join(rootDir, "npm/cddm/package.json"), "utf-8")) as {
      bin: Record<string, string>;
      optionalDependencies: Record<string, string>;
    };

    expect(pkgJson.bin.cddm).toBe("./bin/cddm.js");
    expect(pkgJson.bin["cddm-mcp"]).toBe("./bin/cddm-mcp.js");
    expect(pkgJson.bin["cddm-lsp"]).toBe("./bin/cddm-lsp.js");
    expect(Object.keys(pkgJson.optionalDependencies).sort()).toEqual([
      "@cddm/linux-x64",
      "@cddm/win32-x64",
    ]);

    for (const rel of Object.values(pkgJson.bin)) {
      expect(existsSync(join(rootDir, "npm/cddm", rel))).toBe(true);
    }
  });

  it("never probes workspace target/release and never falls back to bare PATH command", () => {
    const libSource = readFileSync(join(rootDir, "npm/cddm/lib/run-binary.js"), "utf-8");
    expect(libSource).not.toContain("target/release");

    expect(() =>
      resolveBinaryPath("cddm", {
        platform: "linux",
        arch: "x64",
        platformDirOverride: "",
        existsSync: () => false,
        requireResolve: () => {
          throw new Error("MODULE_NOT_FOUND");
        },
      }),
    ).toThrow('Missing required platform binary package "@cddm/linux-x64"');
  });

  it("resolves installed @cddm/<platform>-<arch> binary accurately", () => {
    const tempDir = join(tmpdir(), `cddm-npm-wrapper-test-${Date.now()}`);
    mkdirSync(tempDir, { recursive: true });
    try {
      const fakeBin = join(tempDir, "cddm-mcp.exe");
      writeFileSync(fakeBin, "binary");

      const resolved = resolveBinaryPath("cddm-mcp", {
        platform: "win32",
        arch: "x64",
        platformDirOverride: "",
        requireResolve: (spec: string) => {
          expect(spec).toBe("@cddm/win32-x64/package.json");
          return join(tempDir, "package.json");
        },
      });
      expect(resolved).toBe(fakeBin);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
