import { describe, expect, it } from "bun:test";
import {
  commitFileToRepo,
  ensureSignedReleaseTag,
  getExistingFileSha,
  publishRepoPayload,
} from "./gitea-git-transport";

describe("gitea-git-transport", () => {
  it("fetches existing file blob sha and commits updates via Contents API", async () => {
    const requests: Array<{ url: string; method: string; body?: string }> = [];
    const mockFetch = (async (input: string | URL | Request, init?: RequestInit) => {
      const url = typeof input === "string" ? input : input.toString();
      const method = init?.method ?? "GET";
      requests.push({
        url,
        method,
        body: typeof init?.body === "string" ? init.body : undefined,
      });
      if (method === "GET") {
        return new Response(JSON.stringify({ sha: "deadbeef" }), { status: 200 });
      }
      return new Response(JSON.stringify({ content: { sha: "newsha" } }), { status: 200 });
    }) as unknown as typeof fetch;

    const sha = await getExistingFileSha(
      "vcs.example.com",
      "org/repo",
      "Formula/cddm.rb",
      "mock-tok", // gitleaks:allow — unit test fixture
      "main",
      mockFetch,
    );
    expect(sha).toBe("deadbeef");

    const ok = await commitFileToRepo(
      "vcs.example.com",
      "org/repo",
      "Formula/cddm.rb",
      "class Cddm < Formula\nend\n",
      "release: bump",
      "mock-tok", // gitleaks:allow — unit test fixture
      "main",
      mockFetch,
    );
    expect(ok).toBe(true);
    expect(requests.some((r) => r.method === "PUT")).toBe(true);
  });

  it("supports dry-run batch publication cleanly", async () => {
    const ok = await publishRepoPayload(
      "vcs.example.com",
      "org/winget-pkgs",
      [{ path: "manifests/o/org/cddm/1.0.0/org.cddm.yaml", content: "ManifestVersion: 1.9.0\n" }],
      "release: bump",
      { dryRun: true },
    );
    expect(ok).toBe(true);
  });

  it("supports SSH-signed release tag creation in dry-run and skips when no key is set", () => {
    const skipped = ensureSignedReleaseTag("v1.0.0", "HEAD", { signingKey: "" });
    expect(skipped).toEqual({ signed: false, created: false });

    const signed = ensureSignedReleaseTag("v1.0.0", "HEAD", {
      signingKey: "-----BEGIN OPENSSH PRIVATE KEY-----\ntest\n-----END OPENSSH PRIVATE KEY-----", // gitleaks:allow — unit test fixture
      dryRun: true,
    });
    expect(signed).toEqual({ signed: true, created: true });
  });

  it("falls back to HTTPS Contents API when SSH deploy port is unreachable in container network", async () => {
    const requests: string[] = [];
    const mockFetch = (async (input: string | URL | Request, init?: RequestInit) => {
      const url = typeof input === "string" ? input : input.toString();
      const method = init?.method ?? "GET";
      requests.push(`${method} ${url}`);
      if (method === "GET") {
        return new Response(JSON.stringify({ sha: "abc123" }), { status: 200 });
      }
      return new Response(JSON.stringify({ content: { sha: "def456" } }), { status: 200 });
    }) as unknown as typeof fetch;

    const ok = await publishRepoPayload(
      "127.0.0.1:1",
      "org/scoop-bucket",
      [{ path: "bucket/cddm.json", content: "{}" }],
      "release: bump",
      {
        dryRun: false,
        deployKey:
          "-----BEGIN OPENSSH PRIVATE KEY-----\ninvalid\n-----END OPENSSH PRIVATE KEY-----", // gitleaks:allow — unit test fixture
        token: "mock-tok", // gitleaks:allow — unit test fixture
        fetchFn: mockFetch,
      },
    );
    expect(ok).toBe(true);
    expect(requests.some((r) => r.startsWith("PUT "))).toBe(true);
  });
});
