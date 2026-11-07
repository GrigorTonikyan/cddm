import { describe, it, expect, beforeEach, beforeAll } from "vite-plus/test";
import { LocalStorageAdapter, MemoryAdapter } from "./storage-adapter";

class MockLocalStorage {
  private store = new Map<string, string>();

  get length(): number {
    return this.store.size;
  }

  clear(): void {
    this.store.clear();
  }

  getItem(key: string): string | null {
    return this.store.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.store.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  setItem(key: string, value: string): void {
    this.store.set(key, String(value));
  }
}

describe("Storage Adapters", () => {
  describe("LocalStorageAdapter", () => {
    let adapter: LocalStorageAdapter;
    const mockStorage = new MockLocalStorage();

    beforeAll(() => {
      Object.defineProperty(window, "localStorage", {
        value: mockStorage,
        writable: true,
        configurable: true,
      });
    });

    beforeEach(() => {
      window.localStorage.clear();
      adapter = new LocalStorageAdapter();
    });

    it("writes and reads JSON values safely", () => {
      adapter.setItem("test_key", { x: 100, y: 200 });
      const read = adapter.getItem<{ x: number; y: number }>("test_key");
      expect(read).toEqual({ x: 100, y: 200 });
    });

    it("returns null for nonexistent keys", () => {
      expect(adapter.getItem("nonexistent")).toBeNull();
    });

    it("handles corrupted JSON gracefully by returning null", () => {
      window.localStorage.setItem("corrupt_key", "invalid json {{{");
      expect(adapter.getItem("corrupt_key")).toBeNull();
    });

    it("removes items safely", () => {
      adapter.setItem("to_remove", "value");
      adapter.removeItem("to_remove");
      expect(adapter.getItem("to_remove")).toBeNull();
    });
  });

  describe("MemoryAdapter", () => {
    it("stores and retrieves values in memory", () => {
      const adapter = new MemoryAdapter();
      adapter.setItem("mem_key", [1, 2, 3]);
      expect(adapter.getItem<number[]>("mem_key")).toEqual([1, 2, 3]);

      adapter.removeItem("mem_key");
      expect(adapter.getItem("mem_key")).toBeNull();
    });
  });
});
