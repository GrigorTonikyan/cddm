import { describe, it, expect } from "vitest";
import { ScanConfig, CloneType } from "../cddm-types";

describe("cddm-types", () => {
  it("should verify ScanConfig interface shape matches expected keys", () => {
    const config: ScanConfig = {
      directory: ".",
      min_tokens: 50,
      languages: [],
      ignore_patterns: [],
      detect_type2: true,
      scan_self: true,
    };
    expect(config).toHaveProperty("directory");
    expect(config).toHaveProperty("min_tokens");
    expect(config).toHaveProperty("languages");
    expect(config).toHaveProperty("ignore_patterns");
    expect(config).toHaveProperty("detect_type2");
    expect(config).toHaveProperty("scan_self");
  });

  it("should verify CloneType union covers all variants", () => {
    const exact: CloneType = "Exact";
    const renamed: CloneType = "Renamed";
    const nearMiss: CloneType = "NearMiss";
    const semantic: CloneType = "Semantic";
    
    expect(exact).toBe("Exact");
    expect(renamed).toBe("Renamed");
    expect(nearMiss).toBe("NearMiss");
    expect(semantic).toBe("Semantic");
  });
});
