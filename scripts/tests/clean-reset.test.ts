import { afterEach, beforeEach, describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { cleanWorkspace, findCleanableItems, formatBytes, isProtectedPath } from "../clean";

describe("Workspace Cleaner & Reset Engine", () => {
  describe("Path Protection & Safety Filter", () => {
    it("should strictly protect core repository infrastructure", () => {
      expect(isProtectedPath(".git")).toBe(true);
      expect(isProtectedPath(".git/HEAD")).toBe(true);
      expect(isProtectedPath(".agents")).toBe(true);
      expect(isProtectedPath(".agents/rules/custom.md")).toBe(true);
      expect(isProtectedPath(".github/workflows/ci.yml")).toBe(true);
      expect(isProtectedPath(".vscode/settings.json")).toBe(true);
      expect(isProtectedPath(".vite-hooks/pre-commit")).toBe(true);
      expect(isProtectedPath("crates/cddm-core/src/lib.rs")).toBe(true);
      expect(isProtectedPath("webui/src/App.tsx")).toBe(true);
      expect(isProtectedPath("scripts/clean.ts")).toBe(true);
      expect(isProtectedPath("docs/API.md")).toBe(true);
      expect(isProtectedPath("npm/cddm/bin/cddm.js")).toBe(true);
    });

    it("should correctly identify non-protected artifacts for cleanup", () => {
      expect(isProtectedPath("target/debug/cddm.exe")).toBe(false);
      expect(isProtectedPath("node_modules/react")).toBe(false);
      expect(isProtectedPath("webui/node_modules/clsx")).toBe(false);
      expect(isProtectedPath("webui/dist/assets/index.js")).toBe(false);
      expect(isProtectedPath("bun.lock")).toBe(false);
      expect(isProtectedPath("Cargo.lock")).toBe(false);
      expect(isProtectedPath("tsconfig.tsbuildinfo")).toBe(false);
      expect(isProtectedPath(".logs/test.log")).toBe(false);
    });
  });

  describe("Utility Helpers", () => {
    it("should format bytes into human-readable strings", () => {
      expect(formatBytes(0)).toBe("0 B");
      expect(formatBytes(512)).toBe("512.0 B");
      expect(formatBytes(1024)).toBe("1.0 KB");
      expect(formatBytes(1048576)).toBe("1.0 MB");
      expect(formatBytes(1073741824)).toBe("1.0 GB");
    });
  });

  describe("Artifact Discovery & Filtering", () => {
    it("should find artifacts in current repository", () => {
      const items = findCleanableItems(process.cwd());
      expect(Array.isArray(items)).toBe(true);
      for (const item of items) {
        expect(isProtectedPath(item.relPath)).toBe(false);
      }
    }, 30000);

    it("should respect keepLockfiles option", () => {
      const items = findCleanableItems(process.cwd(), { keepLockfiles: true });
      const lockfileItems = items.filter((it) => it.category === "lockfile");
      expect(lockfileItems.length).toBe(0);
    }, 30000);

    it("should respect keepNodeModules option", () => {
      const items = findCleanableItems(process.cwd(), { keepNodeModules: true });
      const nodeModulesItems = items.filter((it) => it.relPath.includes("node_modules"));
      expect(nodeModulesItems.length).toBe(0);
    }, 30000);
  });

  describe("End-to-End Cleanup Fixture Execution", () => {
    const fixtureDir = join(process.cwd(), ".tmp-test-clean-fixture");

    beforeEach(() => {
      if (existsSync(fixtureDir)) {
        rmSync(fixtureDir, { recursive: true, force: true });
      }
      mkdirSync(fixtureDir, { recursive: true });

      // Create fake build artifacts & caches
      mkdirSync(join(fixtureDir, "target/debug"), { recursive: true });
      writeFileSync(join(fixtureDir, "target/debug/cddm.exe"), "binary data");

      mkdirSync(join(fixtureDir, "node_modules/fake-pkg"), { recursive: true });
      writeFileSync(join(fixtureDir, "node_modules/fake-pkg/index.js"), "module data");

      mkdirSync(join(fixtureDir, "webui/dist"), { recursive: true });
      writeFileSync(join(fixtureDir, "webui/dist/index.html"), "<html></html>");

      mkdirSync(join(fixtureDir, "webui/coverage"), { recursive: true });
      writeFileSync(join(fixtureDir, "webui/coverage/lcov.info"), "coverage data");

      mkdirSync(join(fixtureDir, "test-results"), { recursive: true });
      writeFileSync(join(fixtureDir, "test-results/.last-run.json"), "{}");

      mkdirSync(join(fixtureDir, "playwright-report"), { recursive: true });
      writeFileSync(join(fixtureDir, "playwright-report/index.html"), "report");

      mkdirSync(join(fixtureDir, "blob-report"), { recursive: true });
      writeFileSync(join(fixtureDir, "blob-report/blob.json"), "blob");

      mkdirSync(join(fixtureDir, ".cddm"), { recursive: true });
      writeFileSync(join(fixtureDir, ".cddm/cache.db"), "cache");

      mkdirSync(join(fixtureDir, "packaging/vscode"), { recursive: true });
      writeFileSync(join(fixtureDir, "packaging/vscode/cddm-1.10.0.vsix"), "vsix");

      writeFileSync(join(fixtureDir, "bun.lock"), "lock data");
      writeFileSync(join(fixtureDir, "Cargo.lock"), "cargo lock data");
      writeFileSync(join(fixtureDir, "tsconfig.tsbuildinfo"), "buildinfo data");
      writeFileSync(join(fixtureDir, "debug.log"), "log data");
      writeFileSync(join(fixtureDir, ".env.local"), "SECRET=123");
      writeFileSync(join(fixtureDir, ".DS_Store"), "os data");

      // Create fake protected files
      mkdirSync(join(fixtureDir, ".git/hooks"), { recursive: true });
      writeFileSync(join(fixtureDir, ".git/HEAD"), "ref: refs/heads/main");

      mkdirSync(join(fixtureDir, "crates/cddm-core/src"), { recursive: true });
      writeFileSync(join(fixtureDir, "crates/cddm-core/src/lib.rs"), "pub fn test() {}");
    });

    afterEach(() => {
      if (existsSync(fixtureDir)) {
        rmSync(fixtureDir, { recursive: true, force: true });
      }
    });

    it("should perform dry-run without removing any files", async () => {
      const result = await cleanWorkspace(fixtureDir, { dryRun: true });
      expect(result.dryRun).toBe(true);
      expect(result.dirsRemoved).toBeGreaterThanOrEqual(4);
      expect(result.filesRemoved).toBeGreaterThanOrEqual(5);

      // Verify files still exist
      expect(existsSync(join(fixtureDir, "target"))).toBe(true);
      expect(existsSync(join(fixtureDir, "test-results"))).toBe(true);
      expect(existsSync(join(fixtureDir, "bun.lock"))).toBe(true);
      expect(existsSync(join(fixtureDir, "Cargo.lock"))).toBe(true);
    });

    it("should execute full cleanup on fixture and preserve protected files", async () => {
      const result = await cleanWorkspace(fixtureDir, { dryRun: false });
      expect(result.dryRun).toBe(false);
      expect(result.dirsRemoved).toBeGreaterThanOrEqual(4);
      expect(result.filesRemoved).toBeGreaterThanOrEqual(5);

      // Verify artifacts are wiped
      expect(existsSync(join(fixtureDir, "target"))).toBe(false);
      expect(existsSync(join(fixtureDir, "node_modules"))).toBe(false);
      expect(existsSync(join(fixtureDir, "webui/dist"))).toBe(false);
      expect(existsSync(join(fixtureDir, "webui/coverage"))).toBe(false);
      expect(existsSync(join(fixtureDir, "test-results"))).toBe(false);
      expect(existsSync(join(fixtureDir, "playwright-report"))).toBe(false);
      expect(existsSync(join(fixtureDir, "blob-report"))).toBe(false);
      expect(existsSync(join(fixtureDir, ".cddm"))).toBe(false);
      expect(existsSync(join(fixtureDir, "packaging/vscode/cddm-1.10.0.vsix"))).toBe(false);
      expect(existsSync(join(fixtureDir, "bun.lock"))).toBe(false);
      expect(existsSync(join(fixtureDir, "Cargo.lock"))).toBe(false);
      expect(existsSync(join(fixtureDir, "tsconfig.tsbuildinfo"))).toBe(false);
      expect(existsSync(join(fixtureDir, "debug.log"))).toBe(false);
      expect(existsSync(join(fixtureDir, ".env.local"))).toBe(false);
      expect(existsSync(join(fixtureDir, ".DS_Store"))).toBe(false);

      // Verify protected structure is untouched
      expect(existsSync(join(fixtureDir, ".git/HEAD"))).toBe(true);
      expect(existsSync(join(fixtureDir, "crates/cddm-core/src/lib.rs"))).toBe(true);
    });
  });

  describe("Workspace .cddm Isolation Verification", () => {
    it("should guarantee no .cddm directories exist in subdirectories of workspace after clean", async () => {
      await cleanWorkspace(process.cwd(), { cacheOnly: true, keepNodeModules: true });
      const glob = new Bun.Glob("**/.cddm");
      const matches = Array.from(glob.scanSync({ cwd: process.cwd(), onlyFiles: false }));
      for (const match of matches) {
        const normalized = match.replace(/\\/g, "/");
        if (normalized.startsWith(".tmp-") || normalized.includes("/.tmp-")) {
          continue;
        }
        expect(normalized).toBe(".cddm");
      }
    });
  });
});
