import { afterEach, beforeEach, describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import {
  calculatePathSize,
  cleanWorkspace,
  discoverPackageRoots,
  findCleanableItems,
  formatBytes,
  isProtectedPath,
  safeRemovePath,
} from "./clean-engine";

describe("Clean Engine Core Library", () => {
  describe("Path Protection & Safety Filter", () => {
    it("should strictly protect core repository infrastructure", () => {
      expect(isProtectedPath(".git")).toBe(true);
      expect(isProtectedPath(".git/HEAD")).toBe(true);
      expect(isProtectedPath(".agents")).toBe(true);
      expect(isProtectedPath(".agents/rules/custom.md")).toBe(true);
      expect(isProtectedPath(".github/workflows/ci.yml")).toBe(true);
      expect(isProtectedPath(".gitea/workflows/ci.yaml")).toBe(true);
      expect(isProtectedPath(".vscode/settings.json")).toBe(true);
      expect(isProtectedPath(".vite-hooks/pre-commit")).toBe(true);
      expect(isProtectedPath("crates/cddm-core/src/lib.rs")).toBe(true);
      expect(isProtectedPath("webui/src/App.tsx")).toBe(true);
      expect(isProtectedPath("webui/public/favicon.svg")).toBe(true);
      expect(isProtectedPath("scripts/clean.ts")).toBe(true);
      expect(isProtectedPath("docs/API.md")).toBe(true);
      expect(isProtectedPath("npm/cddm/bin/cddm.js")).toBe(true);
      expect(isProtectedPath("editors/vscode/src/extension.ts")).toBe(true);
      expect(isProtectedPath("editors/vscode/resources/icon.png")).toBe(true);
      expect(isProtectedPath("packaging/homebrew/cddm.rb")).toBe(true);
      expect(isProtectedPath("packaging/scoop/cddm.json")).toBe(true);
      expect(isProtectedPath("Cargo.toml")).toBe(true);
      expect(isProtectedPath("package.json")).toBe(true);
      expect(isProtectedPath(".env.example")).toBe(true);
    });

    it("should correctly identify non-protected artifacts for cleanup", () => {
      expect(isProtectedPath("target/debug/cddm.exe")).toBe(false);
      expect(isProtectedPath("node_modules/react")).toBe(false);
      expect(isProtectedPath("webui/node_modules/clsx")).toBe(false);
      expect(isProtectedPath("webui/dist/assets/index.js")).toBe(false);
      expect(isProtectedPath("test-results/.last-run.json")).toBe(false);
      expect(isProtectedPath("playwright-report/index.html")).toBe(false);
      expect(isProtectedPath("blob-report/report.json")).toBe(false);
      expect(isProtectedPath(".cddm/cache.db")).toBe(false);
      expect(isProtectedPath(".turbo/cache")).toBe(false);
      expect(isProtectedPath("bun.lock")).toBe(false);
      expect(isProtectedPath("Cargo.lock")).toBe(false);
      expect(isProtectedPath("tsconfig.tsbuildinfo")).toBe(false);
      expect(isProtectedPath(".logs/test.log")).toBe(false);
      expect(isProtectedPath(".env.local")).toBe(false);
      expect(isProtectedPath("packaging/vscode/cddm-1.10.0.vsix")).toBe(false);
    });
  });

  describe("Utility Helpers & Formatting", () => {
    it("should format bytes into human-readable representation", () => {
      expect(formatBytes(0)).toBe("0 B");
      expect(formatBytes(512)).toBe("512.0 B");
      expect(formatBytes(1024)).toBe("1.0 KB");
      expect(formatBytes(1048576)).toBe("1.0 MB");
      expect(formatBytes(1073741824)).toBe("1.0 GB");
      expect(formatBytes(1099511627776)).toBe("1.0 TB");
    });

    it("should calculate path sizes accurately", () => {
      const fixtureDir = join(process.cwd(), ".tmp-test-calc-size");
      try {
        mkdirSync(fixtureDir, { recursive: true });
        writeFileSync(join(fixtureDir, "test.txt"), "hello world");
        const size = calculatePathSize(fixtureDir);
        expect(size).toBe(11);
      } finally {
        if (existsSync(fixtureDir)) {
          rmSync(fixtureDir, { recursive: true, force: true });
        }
      }
    });

    it("should discover package roots containing package.json", () => {
      const roots = discoverPackageRoots(process.cwd());
      expect(roots).toContain("");
      expect(roots).toContain("webui");
      expect(roots).toContain("tests/e2e");
      expect(roots).toContain("editors/vscode");
    });
  });

  describe("Filtering & Granular Category Flags", () => {
    const fixtureDir = join(process.cwd(), ".tmp-test-clean-filter");

    beforeEach(() => {
      if (existsSync(fixtureDir)) {
        rmSync(fixtureDir, { recursive: true, force: true });
      }
      mkdirSync(fixtureDir, { recursive: true });
      mkdirSync(join(fixtureDir, "target/debug"), { recursive: true });
      writeFileSync(join(fixtureDir, "target/debug/cddm.exe"), "bin");
      mkdirSync(join(fixtureDir, "node_modules/fake"), { recursive: true });
      writeFileSync(join(fixtureDir, "node_modules/fake/index.js"), "mod");
      mkdirSync(join(fixtureDir, "test-results"), { recursive: true });
      writeFileSync(join(fixtureDir, "test-results/run.json"), "report");
      mkdirSync(join(fixtureDir, ".turbo"), { recursive: true });
      writeFileSync(join(fixtureDir, ".turbo/cache"), "cache");
      writeFileSync(join(fixtureDir, "bun.lock"), "lock");
      writeFileSync(join(fixtureDir, "Cargo.lock"), "cargo lock");
      writeFileSync(join(fixtureDir, "app.log"), "log");
      writeFileSync(join(fixtureDir, "extension.vsix"), "vsix");
    });

    afterEach(() => {
      if (existsSync(fixtureDir)) {
        rmSync(fixtureDir, { recursive: true, force: true });
      }
    });

    it("should filter for targetOnly mode", () => {
      const items = findCleanableItems(fixtureDir, { targetOnly: true });
      expect(items.every((it) => it.relPath === "target" || it.relPath.startsWith("target/"))).toBe(
        true,
      );
      expect(items.length).toBeGreaterThan(0);
    });

    it("should filter for nodeOnly mode", () => {
      const items = findCleanableItems(fixtureDir, { nodeOnly: true });
      expect(items.every((it) => it.relPath.includes("node_modules"))).toBe(true);
      expect(items.length).toBeGreaterThan(0);
    });

    it("should filter for reportsOnly mode", () => {
      const items = findCleanableItems(fixtureDir, { reportsOnly: true });
      expect(items.every((it) => it.category === "test-report")).toBe(true);
      expect(items.length).toBeGreaterThan(0);
    });

    it("should filter for lockfilesOnly mode", () => {
      const items = findCleanableItems(fixtureDir, { lockfilesOnly: true });
      expect(items.every((it) => it.category === "lockfile")).toBe(true);
      expect(items.length).toBeGreaterThan(0);
    });

    it("should respect keepTarget option", () => {
      const items = findCleanableItems(fixtureDir, { keepTarget: true });
      expect(items.some((it) => it.relPath === "target" || it.relPath.startsWith("target/"))).toBe(
        false,
      );
    });

    it("should respect keepReports option", () => {
      const items = findCleanableItems(fixtureDir, { keepReports: true });
      expect(items.some((it) => it.category === "test-report")).toBe(false);
    });
  });

  describe("Safe Remove Engine", () => {
    it("should safely remove files and directories with retry handling", () => {
      const tempDir = join(process.cwd(), ".tmp-test-safe-remove");
      mkdirSync(join(tempDir, "nested"), { recursive: true });
      writeFileSync(join(tempDir, "nested/file.txt"), "data");

      const res = safeRemovePath(tempDir, true);
      expect(res.success).toBe(true);
      expect(existsSync(tempDir)).toBe(false);
    });

    it("should return success for non-existent paths", () => {
      const res = safeRemovePath(join(process.cwd(), "non-existent-xyz-123"), true);
      expect(res.success).toBe(true);
    });

    it("should execute cleanWorkspace in dry-run mode", async () => {
      const fixtureDir = join(process.cwd(), ".tmp-test-clean-workspace");
      try {
        mkdirSync(join(fixtureDir, "target"), { recursive: true });
        writeFileSync(join(fixtureDir, "target/app.exe"), "exe");
        const res = await cleanWorkspace(fixtureDir, { dryRun: true });
        expect(res.dryRun).toBe(true);
        expect(res.dirsRemoved).toBeGreaterThanOrEqual(1);
        expect(existsSync(join(fixtureDir, "target"))).toBe(true);
      } finally {
        if (existsSync(fixtureDir)) {
          rmSync(fixtureDir, { recursive: true, force: true });
        }
      }
    });
  });
});
