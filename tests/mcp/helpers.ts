import { existsSync } from "node:fs";
import { join } from "node:path";
import { expect } from "bun:test";

export interface JsonRpcResponse<T = unknown> {
  jsonrpc: string;
  id?: number | string;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}

export const RPC_ERRORS = {
  PARSE_ERROR: -32700,
  INVALID_REQUEST: -32600,
  METHOD_NOT_FOUND: -32601,
  INVALID_PARAMS: -32602,
  INTERNAL_ERROR: -32603,
};

export function getMcpBinaryPath(): string {
  const exeName = process.platform === "win32" ? "cddm-mcp.exe" : "cddm-mcp";
  const releasePath = join(import.meta.dir, "../../target/release", exeName);
  const debugPath = join(import.meta.dir, "../../target/debug", exeName);
  if (existsSync(releasePath)) return releasePath;
  if (existsSync(debugPath)) return debugPath;
  throw new Error(
    `cddm-mcp binary not found at ${releasePath} or ${debugPath}. Please run 'cargo build -p cddm-mcp'.`,
  );
}

export async function callMcpStdio<T = unknown>(
  request: Record<string, unknown>,
): Promise<JsonRpcResponse<T>> {
  const binaryPath = getMcpBinaryPath();
  const proc = Bun.spawn([binaryPath], {
    stdin: "pipe",
    stdout: "pipe",
    stderr: "pipe",
  });

  const payload = JSON.stringify(request) + "\n";
  void proc.stdin.write(payload);
  void proc.stdin.flush();
  void proc.stdin.end();

  const text = await new Response(proc.stdout).text();
  await proc.exited;

  const line = text
    .split("\n")
    .map((l) => l.trim())
    .find((l) => l.startsWith("{") && l.endsWith("}"));

  if (!line) {
    throw new Error(`No JSON-RPC response received from cddm-mcp. Raw stdout:\n${text}`);
  }

  return JSON.parse(line) as JsonRpcResponse<T>;
}

export async function executeTool<T = any>(
  name: string,
  args: Record<string, unknown> = {},
): Promise<T> {
  const res = await callMcpStdio({
    jsonrpc: "2.0",
    id: Date.now(),
    method: "tools/call",
    params: { name, arguments: args },
  });

  if (res.error) {
    throw new Error(`Tool '${name}' failed [code ${res.error.code}]: ${res.error.message}`);
  }

  const content = (res.result as any)?.content;
  if (!content || !Array.isArray(content) || content.length === 0) {
    throw new Error(
      `Tool '${name}' returned invalid content payload: ${JSON.stringify(res.result)}`,
    );
  }

  const text = content[0].text;
  try {
    return JSON.parse(text) as T;
  } catch {
    return text as unknown as T;
  }
}

export async function assertToolError(
  name: string,
  args?: Record<string, unknown>,
  expectedCode: number = RPC_ERRORS.INVALID_PARAMS,
): Promise<void> {
  const res = await callMcpStdio({
    jsonrpc: "2.0",
    id: Date.now(),
    method: "tools/call",
    params: { name, arguments: args },
  });

  expect(res.error).toBeDefined();
  expect(res.error?.code).toBe(expectedCode);
}

export function assertPropertyTypes(
  obj: Record<string, unknown>,
  schema: Record<string, "string" | "number" | "boolean" | "array" | "object">,
): void {
  expect(obj).toBeDefined();
  for (const [key, type] of Object.entries(schema)) {
    if (type === "array") {
      expect(Array.isArray(obj[key])).toBe(true);
    } else {
      expect(typeof obj[key]).toBe(type);
    }
  }
}
