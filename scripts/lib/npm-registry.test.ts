import { describe, expect, it } from "bun:test";
import {
  buildGiteaNpmRegistryUrl,
  buildPlatformNpmManifest,
  getNpmPackageMetadataUrl,
  normalizeNpmRegistryName,
  verifyNpmChannels,
} from "./npm-registry";

describe("npm-registry utilities", () => {
  it("normalizes registry names and builds metadata URLs", () => {
    expect(normalizeNpmRegistryName("npmjs")).toBe("npmjs");
    expect(normalizeNpmRegistryName("gitea")).toBe("gitea");
    expect(buildGiteaNpmRegistryUrl("git.example.com", "my-org")).toBe(
      "https://git.example.com/api/packages/my-org/npm/",
    );
    expect(getNpmPackageMetadataUrl("npmjs", "git.example.com", "my-org", "cddm")).toBe(
      "https://registry.npmjs.org/cddm",
    );
    expect(getNpmPackageMetadataUrl("gitea", "git.example.com", "my-org", "@cddm/linux-x64")).toBe(
      "https://git.example.com/api/packages/my-org/npm/@cddm%2Flinux-x64",
    );
  });

  it("verifies live NPM channel availability strictly from HTTP 200 responses", async () => {
    const channels = await verifyNpmChannels({
      host: "git.example.com",
      owner: "my-org",
      version: "4.10.3",
      fetchFn: (async (input: string | URL | Request) => {
        const url = typeof input === "string" ? input : input.toString();
        if (url.includes("git.example.com")) {
          return new Response(JSON.stringify({ versions: { "4.10.3": {} } }), { status: 200 });
        }
        return new Response("Not Found", { status: 404 });
      }) as unknown as typeof fetch,
    });
    expect(channels).toEqual({ gitea: true, npmjs: false });
  });

  it("renders valid platform package.json manifests", () => {
    const manifest = JSON.parse(
      buildPlatformNpmManifest({
        packageName: "@cddm/linux-x64",
        version: "4.10.3",
        os: "linux",
        cpu: "x64",
        binaries: ["cddm", "cddm-mcp", "cddm-lsp"],
        homepage: "https://git.example.com/my-org/cddm",
        repositoryUrl: "https://git.example.com/my-org/cddm.git",
      }),
    );
    expect(manifest.name).toBe("@cddm/linux-x64");
    expect(manifest.os).toEqual(["linux"]);
    expect(manifest.cpu).toEqual(["x64"]);
    expect(manifest.files).toEqual(["cddm", "cddm-mcp", "cddm-lsp"]);
  });
});
