import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { describe, expect, it } from "bun:test";
import {
  runPowerShellInstallerExecution,
  validateWingetManifestsCli,
  verifyLocalDistributionChannels,
  verifyRemoteDistributionChannels,
} from "../verify-distribution-channels";

const ROOT = resolve(import.meta.dir, "../..");

describe("Windows & Cross-Platform Distribution Channel Verifier", () => {
  it("verifies local WinGet v1.9.0, Scoop, install.ps1, and MCP client documentation", () => {
    const res = verifyLocalDistributionChannels(ROOT);
    expect(res.passed).toBeTrue();
    expect(res.violations).toEqual([]);
  });

  it("verifies remote distribution channels against Gitea release, package repos, Cargo, and NPM", async () => {
    const mockSha = "a".repeat(64);
    const mockFetch = (async (input: string | URL | Request) => {
      const url = typeof input === "string" ? input : input.toString();
      if (url.endsWith("/SHA256SUMS.txt")) {
        return new Response(
          `${mockSha}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n${mockSha}  cddm-v1.0.0-x86_64-unknown-linux-gnu.tar.gz\n`,
          { status: 200 },
        );
      }
      if (url.includes("/cargo/")) {
        return new Response('{"name":"cddm-cli","vers":"1.0.0"}\n', { status: 200 });
      }
      if (url.includes("/npm/")) {
        return new Response(JSON.stringify({ versions: { "1.0.0": {} } }), { status: 200 });
      }
      return new Response(JSON.stringify({ sha: "abc" }), { status: 200 });
    }) as unknown as typeof fetch;

    const res = await verifyRemoteDistributionChannels({
      workspaceRoot: ROOT,
      version: "1.0.0",
      skipWingetCli: true,
      fetchFn: mockFetch,
    });
    expect(res.passed).toBeTrue();
    expect(res.violations).toEqual([]);

    const localWg = validateWingetManifestsCli(join(ROOT, "packaging", "winget"));
    if (!localWg.skipped) {
      expect(localWg.validated).toBeFalse();
      expect(localWg.output).toContain("InstallerSha256");
    }
  });

  it("executes packaging/install.ps1 end-to-end with SHA-256 verification and rejects tampered archives", async () => {
    if (process.platform !== "win32") return;

    const workDir = mkdtempSync(join(tmpdir(), "cddm-ps1-e2e-"));
    try {
      const binSourceDir = join(workDir, "bins");
      const zipPath = join(workDir, "cddm-v1.0.0-x86_64-pc-windows-msvc.zip");
      Bun.spawnSync([
        "pwsh",
        "-NoProfile",
        "-Command",
        `New-Item -ItemType Directory -Path '${binSourceDir}' -Force | Out-Null`,
      ]);
      writeFileSync(join(binSourceDir, "cddm.exe"), "MZ-mock-cddm", "utf-8");
      writeFileSync(join(binSourceDir, "cddm-mcp.exe"), "MZ-mock-cddm-mcp", "utf-8");
      writeFileSync(join(binSourceDir, "cddm-lsp.exe"), "MZ-mock-cddm-lsp", "utf-8");

      const compressProc = Bun.spawnSync([
        "pwsh",
        "-NoProfile",
        "-Command",
        `Compress-Archive -Path '${binSourceDir}\\*' -DestinationPath '${zipPath}' -Force`,
      ]);
      expect(compressProc.exitCode).toBe(0);

      const zipBytes = readFileSync(zipPath);
      const validHash = createHash("sha256").update(zipBytes).digest("hex");
      let serveCorruptedHash = false;

      const server = Bun.serve({
        port: 0,
        fetch(req) {
          const url = new URL(req.url);
          if (url.pathname.endsWith("/releases/latest")) {
            return Response.json({
              tag_name: "v1.0.0",
              assets: [{ name: "cddm-v1.0.0-x86_64-pc-windows-msvc.zip" }],
            });
          }
          if (url.pathname.endsWith("/cddm-v1.0.0-x86_64-pc-windows-msvc.zip")) {
            return new Response(zipBytes);
          }
          if (url.pathname.endsWith("/SHA256SUMS.txt")) {
            const hash = serveCorruptedHash ? "f".repeat(64) : validHash;
            return new Response(`${hash}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n`);
          }
          return new Response("not found", { status: 404 });
        },
      });

      try {
        const baseUrl = `http://127.0.0.1:${server.port}`;
        const okRun = await runPowerShellInstallerExecution({
          workspaceRoot: ROOT,
          baseUrl,
          repo: "org/cddm",
          version: "latest",
        });
        expect(okRun.exitCode).toBe(0);
        expect(okRun.installedBinaries).toEqual(["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"]);

        serveCorruptedHash = true;
        const badRun = await runPowerShellInstallerExecution({
          workspaceRoot: ROOT,
          baseUrl,
          repo: "org/cddm",
          version: "latest",
        });
        expect(badRun.exitCode).not.toBe(0);
        expect(badRun.stderr + badRun.stdout).toContain("SHA-256 checksum mismatch");
      } finally {
        server.stop(true);
      }
    } finally {
      rmSync(workDir, { recursive: true, force: true });
    }
  });
});
