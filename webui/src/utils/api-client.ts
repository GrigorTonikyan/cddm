/**
 * Universal JSON REST client helper for WebUI store slices.
 * Handles headers, serialization, error unwrapping, and response decoding.
 */

export async function postJson<T>(
  url: string,
  body: unknown,
  fallbackErrorMessage: string,
): Promise<T> {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const errorDetail = await res.text().catch(() => res.statusText);
    throw new Error(`${fallbackErrorMessage} (${res.status}): ${errorDetail}`);
  }

  return (await res.json()) as T;
}

export async function getJson<T>(url: string, fallbackErrorMessage: string): Promise<T> {
  const res = await fetch(url);

  if (!res.ok) {
    const errorDetail = await res.text().catch(() => res.statusText);
    throw new Error(`${fallbackErrorMessage} (${res.status}): ${errorDetail}`);
  }

  return (await res.json()) as T;
}
