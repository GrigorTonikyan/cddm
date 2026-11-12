/**
 * Shared Cargo Registry & Sparse Index Utilities
 *
 * Provides registry normalization, sparse index URL/path calculation,
 * NDJSON metadata parsing, and HTTP 200 publication verification across
 * both Gitea Private Cargo Registry and public crates.io.
 */

import { extractHostFromUrl } from "./publisher-config";

export const WORKSPACE_CRATES = ["cddm-core", "cddm-lsp", "cddm-mcp", "cddm-cli"] as const;
export type WorkspaceCrate = (typeof WORKSPACE_CRATES)[number];

export const CRATES_IO_REGISTRY_NAME = "crates-io";
export const CRATES_IO_SPARSE_INDEX_BASE = "https://index.crates.io";

export interface CargoChannelAvailability {
  gitea: boolean;
  cratesIo: boolean;
}

export interface CratePublishCheckOptions {
  registry: string;
  host: string;
  owner: string;
  crateName: string;
  version: string;
  token?: string;
  indexUrl?: string;
  cratesIoBaseUrl?: string;
  fetchFn?: typeof fetch;
}

export interface VerifyCargoChannelsOptions {
  host: string;
  owner: string;
  version: string;
  token?: string;
  indexUrl?: string;
  crates?: readonly WorkspaceCrate[];
  cratesIoBaseUrl?: string;
  fetchFn?: typeof fetch;
}

export function normalizeCargoRegistryName(registry: string): string {
  const trimmed = registry.trim().toLowerCase();
  if (trimmed === "crates.io" || trimmed === CRATES_IO_REGISTRY_NAME) {
    return CRATES_IO_REGISTRY_NAME;
  }
  return registry.trim();
}

export function isCratesIoRegistry(registry: string): boolean {
  return normalizeCargoRegistryName(registry) === CRATES_IO_REGISTRY_NAME;
}

export function resolveHostFromServerUrl(serverUrl?: string): string | undefined {
  return extractHostFromUrl(serverUrl);
}

export function parseGiteaCargoIndexUrl(indexUrl?: string): { host?: string; owner?: string } {
  if (!indexUrl) return {};
  const cleaned = indexUrl.trim().replace(/^sparse\+/, "");
  try {
    const parsed = new URL(cleaned);
    const match = /\/api\/packages\/([^/]+)\/cargo\/?$/i.exec(parsed.pathname);
    return {
      host: parsed.host || undefined,
      owner: match?.[1] || undefined,
    };
  } catch {
    return {};
  }
}

export function buildCargoRegistryIndexUrl(host: string, owner: string): string {
  return `sparse+https://${host}/api/packages/${owner}/cargo/`;
}

export function getSparseIndexBaseHttpUrl(
  registry: string,
  host: string,
  owner: string,
  explicitIndexUrl?: string,
  cratesIoBaseUrl: string = CRATES_IO_SPARSE_INDEX_BASE,
): string {
  if (isCratesIoRegistry(registry)) {
    return cratesIoBaseUrl.replace(/\/+$/, "");
  }
  if (explicitIndexUrl) {
    return explicitIndexUrl
      .trim()
      .replace(/^sparse\+/, "")
      .replace(/\/+$/, "");
  }
  return `https://${host}/api/packages/${owner}/cargo`;
}

/**
 * Compute Cargo sparse index relative path for a crate name per Cargo's index layout.
 */
export function getSparseIndexCratePath(crateName: string): string {
  const lower = crateName.toLowerCase();
  if (lower.length === 1) return `1/${lower}`;
  if (lower.length === 2) return `2/${lower}`;
  if (lower.length === 3) return `3/${lower.slice(0, 1)}/${lower}`;
  return `${lower.slice(0, 2)}/${lower.slice(2, 4)}/${lower}`;
}

export function parseSparseIndexVersions(ndjson: string): Set<string> {
  const versions = new Set<string>();
  for (const line of ndjson.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    try {
      const parsed = JSON.parse(trimmed) as { vers?: unknown };
      if (typeof parsed.vers === "string") {
        versions.add(parsed.vers);
      }
    } catch {
      // Ignore malformed index lines
    }
  }
  return versions;
}

export function formatCargoAuthToken(token: string, registry: string = "gitea"): string {
  const trimmed = token.trim();
  if (isCratesIoRegistry(registry)) {
    return trimmed.replace(/^Bearer\s+/i, "");
  }
  if (trimmed.startsWith("Bearer ") || trimmed.startsWith("token ")) {
    return trimmed;
  }
  return `Bearer ${trimmed}`;
}

export function getVcsAuthHeader(token: string): string {
  if (token.startsWith("Basic ") || token.startsWith("Bearer ") || token.startsWith("token ")) {
    return token;
  }
  return token.includes(".") ? `Bearer ${token}` : `token ${token}`;
}

/**
 * Verify via HTTP 200 + sparse index NDJSON whether a crate version exists on a registry.
 */
export async function isCrateVersionPublished(options: CratePublishCheckOptions): Promise<boolean> {
  const baseHttpUrl = getSparseIndexBaseHttpUrl(
    options.registry,
    options.host,
    options.owner,
    options.indexUrl,
    options.cratesIoBaseUrl,
  );
  const indexPath = getSparseIndexCratePath(options.crateName);
  const url = `${baseHttpUrl}/${indexPath}`;
  const headers: Record<string, string> = {
    Accept: "text/plain, application/json",
    "User-Agent": "cddm-release-publisher",
  };
  if (options.token && !isCratesIoRegistry(options.registry)) {
    headers.Authorization = getVcsAuthHeader(options.token);
  }

  const doFetch = options.fetchFn ?? fetch;
  try {
    const res = await doFetch(url, { headers });
    if (res.status !== 200) return false;
    const body = await res.text();
    return parseSparseIndexVersions(body).has(options.version);
  } catch {
    return false;
  }
}

export async function areAllCratesPublished(
  registry: string,
  options: VerifyCargoChannelsOptions,
): Promise<boolean> {
  const crates = options.crates ?? WORKSPACE_CRATES;
  if (crates.length === 0) return false;
  const checks = await Promise.all(
    crates.map((crateName) =>
      isCrateVersionPublished({
        registry,
        host: options.host,
        owner: options.owner,
        crateName,
        version: options.version,
        token: options.token,
        indexUrl: isCratesIoRegistry(registry) ? undefined : options.indexUrl,
        cratesIoBaseUrl: options.cratesIoBaseUrl,
        fetchFn: options.fetchFn,
      }),
    ),
  );
  return checks.every(Boolean);
}

/**
 * Probe Gitea Cargo Registry and crates.io sparse indices to verify which Cargo
 * distribution channels actually have all CDDM crates live for the release version.
 */
export async function verifyCargoChannels(
  options: VerifyCargoChannelsOptions,
): Promise<CargoChannelAvailability> {
  const [gitea, cratesIo] = await Promise.all([
    areAllCratesPublished("gitea", options),
    areAllCratesPublished(CRATES_IO_REGISTRY_NAME, options),
  ]);
  return { gitea, cratesIo };
}
