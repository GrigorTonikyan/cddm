import { describe, expect, it } from "vite-plus/test";
import { ResilientSSEClient } from "./sse-client";

describe("ResilientSSEClient", () => {
  it("should initialize with default delay values", () => {
    const client = new ResilientSSEClient({
      url: "http://localhost:5173/api/events",
      initialDelayMs: 1000,
      maxDelayMs: 16000,
      backoffMultiplier: 2.0,
    });

    expect(client.getDelay()).toBe(1000);
    client.close();
  });

  it("should cleanup properly on close", () => {
    const client = new ResilientSSEClient({
      url: "http://localhost:5173/api/events",
    });

    client.close();
    expect(client.getDelay()).toBe(1000);
  });
});
