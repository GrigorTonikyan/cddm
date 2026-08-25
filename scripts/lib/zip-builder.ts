import { deflateRawSync } from "node:zlib";

export interface ZipEntry {
  name: string;
  data: Uint8Array | Buffer | string;
}

// CRC-32 Lookup Table
const CRC_TABLE = new Uint32Array(256);
for (let i = 0; i < 256; i++) {
  let c = i;
  for (let k = 0; k < 8; k++) {
    c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  }
  CRC_TABLE[i] = c;
}

export function crc32(buffer: Uint8Array): number {
  let crc = 0xffffffff;
  for (let i = 0; i < buffer.length; i++) {
    const byte = buffer[i] ?? 0;
    const tableVal = CRC_TABLE[(crc ^ byte) & 0xff] ?? 0;
    crc = (crc >>> 8) ^ tableVal;
  }
  return (crc ^ 0xffffffff) >>> 0;
}

/**
 * Creates a standard, fully compliant ZIP archive buffer from a list of entries.
 */
export function createZipArchive(entries: ZipEntry[]): Buffer {
  const localHeaders: Buffer[] = [];
  const centralHeaders: Buffer[] = [];
  let currentOffset = 0;

  for (const entry of entries) {
    const rawData =
      typeof entry.data === "string" ? Buffer.from(entry.data, "utf-8") : Buffer.from(entry.data);
    const uncompressedSize = rawData.length;
    const entryCrc = crc32(rawData);

    // Compress using DEFLATE
    let compressedData = rawData;
    let compressionMethod = 0; // Stored (no compression)

    if (uncompressedSize > 0) {
      const deflated = deflateRawSync(rawData);
      if (deflated.length < uncompressedSize) {
        compressedData = deflated;
        compressionMethod = 8; // Deflated
      }
    }

    const compressedSize = compressedData.length;
    const nameBuffer = Buffer.from(entry.name.replace(/\\/g, "/"), "utf-8");
    const nameLength = nameBuffer.length;

    // 1. Local File Header (30 bytes + nameLength + compressedSize)
    const localHeader = Buffer.alloc(30 + nameLength);
    localHeader.writeUInt32LE(0x04034b50, 0); // signature
    localHeader.writeUInt16LE(20, 4); // version needed (2.0)
    localHeader.writeUInt16LE(0, 6); // general purpose bit flag
    localHeader.writeUInt16LE(compressionMethod, 8); // compression method
    localHeader.writeUInt16LE(0, 10); // file last mod time
    localHeader.writeUInt16LE(0, 12); // file last mod date
    localHeader.writeUInt32LE(entryCrc, 14); // crc-32
    localHeader.writeUInt32LE(compressedSize, 18); // compressed size
    localHeader.writeUInt32LE(uncompressedSize, 22); // uncompressed size
    localHeader.writeUInt16LE(nameLength, 26); // file name length
    localHeader.writeUInt16LE(0, 28); // extra field length
    nameBuffer.copy(localHeader, 30);

    localHeaders.push(localHeader, compressedData);

    // 2. Central Directory Header (46 bytes + nameLength)
    const centralHeader = Buffer.alloc(46 + nameLength);
    centralHeader.writeUInt32LE(0x02014b50, 0); // signature
    centralHeader.writeUInt16LE(20, 4); // version made by
    centralHeader.writeUInt16LE(20, 6); // version needed
    centralHeader.writeUInt16LE(0, 8); // general purpose bit flag
    centralHeader.writeUInt16LE(compressionMethod, 10); // compression method
    centralHeader.writeUInt16LE(0, 12); // file last mod time
    centralHeader.writeUInt16LE(0, 14); // file last mod date
    centralHeader.writeUInt32LE(entryCrc, 16); // crc-32
    centralHeader.writeUInt32LE(compressedSize, 20); // compressed size
    centralHeader.writeUInt32LE(uncompressedSize, 24); // uncompressed size
    centralHeader.writeUInt16LE(nameLength, 28); // file name length
    centralHeader.writeUInt16LE(0, 30); // extra field length
    centralHeader.writeUInt16LE(0, 32); // comment length
    centralHeader.writeUInt16LE(0, 34); // disk number start
    centralHeader.writeUInt16LE(0, 36); // internal file attributes
    centralHeader.writeUInt32LE(0, 38); // external file attributes
    centralHeader.writeUInt32LE(currentOffset, 42); // relative offset of local header
    nameBuffer.copy(centralHeader, 46);

    centralHeaders.push(centralHeader);

    currentOffset += localHeader.length + compressedData.length;
  }

  const centralDirBuffer = Buffer.concat(centralHeaders);
  const centralDirSize = centralDirBuffer.length;
  const centralDirOffset = currentOffset;

  // 3. End of Central Directory Record (22 bytes)
  const eocd = Buffer.alloc(22);
  eocd.writeUInt32LE(0x06054b50, 0); // signature
  eocd.writeUInt16LE(0, 4); // number of this disk
  eocd.writeUInt16LE(0, 6); // disk where central directory starts
  eocd.writeUInt16LE(entries.length, 8); // number of central directory records on this disk
  eocd.writeUInt16LE(entries.length, 10); // total number of central directory records
  eocd.writeUInt32LE(centralDirSize, 12); // size of central directory
  eocd.writeUInt32LE(centralDirOffset, 16); // offset of start of central directory
  eocd.writeUInt16LE(0, 20); // comment length

  return Buffer.concat([...localHeaders, centralDirBuffer, eocd]);
}
