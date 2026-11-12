import { describe, expect, it } from "bun:test";
import {
  buildCargoRegistryIndexUrl,
  formatCargoAuthToken,
  getSparseIndexBaseHttpUrl,
  getSparseIndexCratePath,
  isCrateVersionPublished,
  isCratesIoRegistry,
  normalizeCargoRegistryName,
  parseGiteaCargoIndexUrl,
  parseSparseIndexVersions,
  verifyCargoChannels,
  WORKSPACE_CRATES,
} from "./cargo-registry";

describe("cargo-registry utilities", () => {
  it("normalizes crates.io and Gitea registry names accurately", () => {
    expect(normalizeCargoRegistryName("crates.io")).toBe("crates-io");
    expect(normalizeCargoRegistryName("CRATES-IO")).toBe("crates-io");
    expect(normalizeCargoRegistryName("gitea")).toBe("gitea");
    expect(isCratesIoRegistry("crates.io")).toBe(true);
    expect(isCratesIoRegistry("crates-io")).toBe(true);
    expect(isCratesIoRegistry("gitea")).toBe(false);
  });

  it("parses host and owner from explicit Gitea sparse index URLs", () => {
    expect(
      parseGiteaCargoIndexUrl("sparse+https://git.example.com/api/packages/myorg/cargo/"),
    ).toEqual({
      host: "git.example.com",
      owner: "myorg",
    });
    expect(getSparseIndexBaseHttpUrl("crates-io", "git.example.com", "myorg")).toBe(
      "https://index.crates.io",
    );
    expect(
      getSparseIndexBaseHttpUrl(
        "gitea",
        "fallback.local",
        "fallback",
        "sparse+https://git.example.com/api/packages/myorg/cargo/",
      ),
    ).toBe("https://git.example.com/api/packages/myorg/cargo");
  });

  it("formats Bearer headers for Gitea and raw tokens for crates.io", () => {
    expect(formatCargoAuthToken("cio-secret", "crates-io")).toBe("cio-secret");
    expect(formatCargoAuthToken("Bearer cio-secret", "crates-io")).toBe("cio-secret");
    expect(formatCargoAuthToken("gitea-pat", "gitea")).toBe("Bearer gitea-pat");
    expect(buildCargoRegistryIndexUrl("git.example.com", "myorg")).toBe(
      "sparse+https://git.example.com/api/packages/myorg/cargo/",
    );
    expect(getSparseIndexCratePath("cddm-cli")).toBe("cd/dm/cddm-cli");
    expect(parseSparseIndexVersions('{"vers":"4.9.4"}\n').has("4.9.4")).toBe(true);
  });

  it("verifies Cargo channel availability strictly from HTTP 200 sparse index payloads", async () => {
    const requestedUrls: string[] = [];
    const mockFetch = (async (input: string | URL | Request, init?: RequestInit) => {
      const url = typeof input === "string" ? input : input.toString();
      requestedUrls.push(url);
      if (url.startsWith("https://index.crates.io/")) {
        expect((init?.headers as Record<string, string>)?.Authorization).toBeUndefined();
        return new Response('{"vers":"4.9.4"}\n', { status: 200 });
      }
      if (url.includes("/cddm-cli")) {
        return new Response("not found", { status: 404 });
      }
      return new Response('{"vers":"4.9.4"}\n', { status: 200 });
    }) as typeof fetch;

    const availability = await verifyCargoChannels({
      host: "git.example.com",
      owner: "myorg",
      version: "4.9.4",
      token: "pat-123",
      fetchFn: mockFetch,
    });

    expect(availability.cratesIo).toBe(true);
    expect(availability.gitea).toBe(false);
    expect(requestedUrls.length).toBe(WORKSPACE_CRATES.length * 2);

    const giteaCliOk = await isCrateVersionPublished({
      registry: "gitea",
      host: "git.example.com",
      owner: "myorg",
      crateName: "cddm-core",
      version: "4.9.4",
      fetchFn: mockFetch,
    });
    expect(giteaCliOk).toBe(true);
  });
});
