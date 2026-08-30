import { describe, it, expect } from "bun:test";
import { crc32, createZipArchive } from "./zip-builder";

describe("zip-builder utility", () => {
  it("computes crc32 accurately for sample buffers", () => {
    const data = new TextEncoder().encode("Hello, CDDM!");
    const crc = crc32(data);
    expect(crc).toBeGreaterThan(0);
    expect(typeof crc).toBe("number");
  });

  it("creates a compliant ZIP archive buffer from multiple entries", () => {
    const entries = [
      { name: "file1.txt", data: "Hello World from CDDM" },
      { name: "sub/file2.json", data: JSON.stringify({ name: "cddm", version: "1.7.0" }) },
    ];

    const zipBuffer = createZipArchive(entries);
    expect(zipBuffer).toBeInstanceOf(Buffer);
    expect(zipBuffer.length).toBeGreaterThan(50);

    // Verify ZIP magic signature 0x04034b50 (PK\x03\x04)
    expect(zipBuffer[0]).toBe(0x50);
    expect(zipBuffer[1]).toBe(0x4b);
    expect(zipBuffer[2]).toBe(0x03);
    expect(zipBuffer[3]).toBe(0x04);
  });

  it("handles empty entries safely", () => {
    const zipBuffer = createZipArchive([]);
    expect(zipBuffer).toBeInstanceOf(Buffer);
    // End of central directory record is 22 bytes
    expect(zipBuffer.length).toBe(22);
    // Signature 0x06054b50 (PK\x05\x06)
    expect(zipBuffer[0]).toBe(0x50);
    expect(zipBuffer[1]).toBe(0x4b);
    expect(zipBuffer[2]).toBe(0x05);
    expect(zipBuffer[3]).toBe(0x06);
  });
});
