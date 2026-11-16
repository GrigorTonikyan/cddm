/**
 * Shared NPM Registry, OIDC Provenance & Platform Package Utilities
 *
 * Provides registry URL calculation, HTTP 200 publication verification across
 * Gitea Private NPM Registry and public registry.npmjs.org, and platform
 * package manifest generation (@cddm/linux-x64, @cddm/win32-x64).
 */

export const NPMJS_REGISTRY_NAME = "npmjs";
export const NPMJS_REGISTRY_URL = "https://registry.npmjs.org";

export const SUPPORTED_NPM_TARGETS = [
  {
    packageName: "@cddm/linux-x64",
    os: "linux",
    cpu: "x64",
    rustTarget: "x86_64-unknown-linux-gnu",
    archiveExt: ".tar.gz",
    binaries: ["cddm", "cddm-mcp", "cddm-lsp"],
  },
  {
    packageName: "@cddm/win32-x64",
    os: "win32",
    cpu: "x64",
    rustTarget: "x86_64-pc-windows-msvc",
    archiveExt: ".zip",
    binaries: ["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"],
  },
] as const;

export interface NpmChannelAvailability {
  gitea: boolean;
  npmjs: boolean;
}

export interface VerifyNpmChannelsOptions {
  host: string;
  owner: string;
  packageName?: string;
  version: string;
  token?: string;
  npmjsBaseUrl?: string;
  fetchFn?: typeof fetch;
}

export function normalizeNpmRegistryName(registry: string): "gitea" | "npmjs" {
  const lower = registry.trim().toLowerCase();
  if (lower === "npmjs" || lower === "npm" || lower === "registry.npmjs.org") {
    return "npmjs";
  }
  return "gitea";
}

export function buildGiteaNpmRegistryUrl(host: string, owner: string): string {
  return `https://${host.replace(/\/+$/, "")}/api/packages/${owner}/npm/`;
}

export function getNpmPackageMetadataUrl(
  registry: "gitea" | "npmjs",
  host: string,
  owner: string,
  packageName: string,
  npmjsBaseUrl: string = NPMJS_REGISTRY_URL,
): string {
  const encodedPkg = packageName.startsWith("@")
    ? `@${encodeURIComponent(packageName.slice(1))}`
    : encodeURIComponent(packageName);
  if (registry === "npmjs") {
    return `${npmjsBaseUrl.replace(/\/+$/, "")}/${encodedPkg}`;
  }
  return `${buildGiteaNpmRegistryUrl(host, owner)}${encodedPkg}`;
}

/**
 * Checks whether a specific package version is published and reachable (HTTP 200) on the target NPM registry.
 */
export async function isNpmVersionPublished(options: {
  registry: "gitea" | "npmjs";
  host: string;
  owner: string;
  packageName: string;
  version: string;
  token?: string;
  npmjsBaseUrl?: string;
  fetchFn?: typeof fetch;
}): Promise<boolean> {
  const fetchImpl = options.fetchFn ?? fetch;
  const url = getNpmPackageMetadataUrl(
    options.registry,
    options.host,
    options.owner,
    options.packageName,
    options.npmjsBaseUrl,
  );
  const headers: Record<string, string> = { Accept: "application/json" };
  if (options.registry === "gitea" && options.token) {
    headers.Authorization = options.token.includes(".")
      ? `Bearer ${options.token}`
      : `token ${options.token}`;
  }

  try {
    const res = await fetchImpl(url, { headers });
    if (res.status !== 200) return false;
    const data = (await res.json()) as {
      versions?: Record<string, unknown>;
      "dist-tags"?: Record<string, string>;
    };
    if (data.versions && typeof data.versions === "object") {
      return Object.prototype.hasOwnProperty.call(data.versions, options.version);
    }
    return data["dist-tags"]?.latest === options.version;
  } catch {
    return false;
  }
}

/**
 * Verifies live HTTP 200 availability of the main `cddm` package across Gitea NPM and public npmjs.
 */
export async function verifyNpmChannels(
  options: VerifyNpmChannelsOptions,
): Promise<NpmChannelAvailability> {
  const packageName = options.packageName ?? "cddm";
  const [gitea, npmjs] = await Promise.all([
    isNpmVersionPublished({
      registry: "gitea",
      host: options.host,
      owner: options.owner,
      packageName,
      version: options.version,
      token: options.token,
      fetchFn: options.fetchFn,
    }),
    isNpmVersionPublished({
      registry: "npmjs",
      host: options.host,
      owner: options.owner,
      packageName,
      version: options.version,
      npmjsBaseUrl: options.npmjsBaseUrl,
      fetchFn: options.fetchFn,
    }),
  ]);
  return { gitea, npmjs };
}

/**
 * Generates a platform-specific package.json manifest (e.g., @cddm/linux-x64).
 */
export function buildPlatformNpmManifest(spec: {
  packageName: string;
  version: string;
  os: string;
  cpu: string;
  binaries: readonly string[];
  homepage: string;
  repositoryUrl: string;
}): string {
  const pkg = {
    name: spec.packageName,
    version: spec.version,
    description: `Prebuilt ${spec.os}-${spec.cpu} native binaries for CDDM (cddm, cddm-mcp, cddm-lsp)`,
    os: [spec.os],
    cpu: [spec.cpu],
    files: [...spec.binaries],
    homepage: spec.homepage,
    license: "MIT OR Apache-2.0",
    repository: {
      type: "git",
      url: spec.repositoryUrl,
    },
  };
  return `${JSON.stringify(pkg, null, 2)}\n`;
}
