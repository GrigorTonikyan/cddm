/**
 * Gitea API Client Helper for CDDM Portal Management
 */

export const GITEA_BASE = "https://git.gt-web-dev.com";
export const GITEA_TOKEN = "006df1eddf22dbb22eb29ec461bac5be6421a673";
export const GITEA_REPO = "gt-dev/cddm";
export const GITEA_OWNER = "gt-dev";

export function getApiHeaders(useJson = true): Record<string, string> {
  const headers: Record<string, string> = {
    Authorization: `token ${GITEA_TOKEN}`,
    Accept: "application/json",
  };
  if (useJson) {
    headers["Content-Type"] = "application/json";
  }
  return headers;
}

export async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function giteaFetch<T>(
  path: string,
  options: RequestInit = {},
  retries = 3,
): Promise<{ status: number; ok: boolean; data: T }> {
  const url = path.startsWith("http") ? path : `${GITEA_BASE}/api/v1${path}`;
  const headers = {
    ...getApiHeaders(!options.body || typeof options.body === "string"),
    ...(options.headers as Record<string, string>),
  };

  await sleep(100);

  for (let attempt = 1; attempt <= retries; attempt++) {
    try {
      const res = await fetch(url, { ...options, headers });
      let data: T | null = null;
      const contentType = res.headers.get("content-type") || "";
      if (contentType.includes("application/json")) {
        try {
          data = await res.json();
        } catch {
          data = null;
        }
      } else {
        data = (await res.text()) as unknown as T;
      }

      return { status: res.status, ok: res.ok, data: data as T };
    } catch (err) {
      if (attempt === retries) {
        throw err;
      }
      console.warn(
        `    [RETRY ${attempt}/${retries}] Retrying ${path} after error: ${String(err)}`,
      );
      await sleep(500 * attempt);
    }
  }

  throw new Error(`Failed to fetch ${path} after ${retries} attempts`);
}
