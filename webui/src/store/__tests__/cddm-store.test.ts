import { describe, it, expect, beforeEach } from "vitest";
import { useCDDMStore } from "../cddm-store";

describe("useCDDMStore Zustand Store", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should initialize with default scan config", () => {
    const { config } = useCDDMStore.getState();
    expect(config.directory).toBe(".");
    expect(config.min_tokens).toBe(50);
    expect(config.detect_type2).toBe(true);
    expect(config.scan_self).toBe(true);
  });

  it("should update scan config cleanly", () => {
    useCDDMStore.getState().setConfig({ min_tokens: 100, directory: "./src" });
    const { config } = useCDDMStore.getState();
    expect(config.min_tokens).toBe(100);
    expect(config.directory).toBe("./src");
  });

  it("should handle mock scan start fallback", async () => {
    await useCDDMStore.getState().startScan();
    const { results, isScanning } = useCDDMStore.getState();
    expect(isScanning).toBe(false);
    expect(results).not.toBeNull();
    expect(results?.scan_id).toBe("demo-scan-123");
    expect(results?.total_clones).toBe(3);
  });

  it("should cancel scan and set error state", () => {
    useCDDMStore.getState().cancelScan();
    const { error, isScanning } = useCDDMStore.getState();
    expect(isScanning).toBe(false);
    expect(error).toBe("Scan cancelled");
  });
});
