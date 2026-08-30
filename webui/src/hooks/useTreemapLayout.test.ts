import { describe, it, expect } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { useTreemapLayout } from "./useTreemapLayout";
import { ClonePair } from "../types/cddm-types";

describe("useTreemapLayout hook", () => {
  const mockClonePairs: ClonePair[] = [
    {
      fragment_hash: "hash-1",
      file_a: "crates/cddm-core/src/parser.rs",
      start_line_a: 10,
      end_line_a: 30,
      file_b: "crates/cddm-core/src/lexer.rs",
      start_line_b: 15,
      end_line_b: 35,
      token_count: 120,
      similarity: 95.0,
      clone_type: "Renamed",
      author_a: "Alice",
      author_b: "Bob",
    },
    {
      fragment_hash: "hash-2",
      file_a: "webui/src/App.tsx",
      start_line_a: 1,
      end_line_a: 20,
      file_b: "webui/src/Main.tsx",
      start_line_b: 1,
      end_line_b: 20,
      token_count: 80,
      similarity: 100.0,
      clone_type: "Exact",
      author_a: "Dev",
      author_b: "Dev",
    },
  ];

  it("initializes with root hierarchy and computed layout rects", () => {
    const { result } = renderHook(() =>
      useTreemapLayout({ clonePairs: mockClonePairs, width: 800, height: 400 }),
    );

    expect(result.current.currentPath).toBe("");
    expect(result.current.fullHierarchy.name).toBe("root");
    expect(result.current.breadcrumbs).toEqual([{ name: "Root", path: "" }]);
    expect(result.current.layoutRects.length).toBeGreaterThan(0);
  });

  it("navigates down to subdirectories and updates breadcrumbs and layout", () => {
    const { result } = renderHook(() =>
      useTreemapLayout({ clonePairs: mockClonePairs, width: 800, height: 400 }),
    );

    act(() => {
      result.current.navigateTo("crates/cddm-core");
    });

    expect(result.current.currentPath).toBe("crates/cddm-core");
    expect(result.current.breadcrumbs).toEqual([
      { name: "Root", path: "" },
      { name: "crates", path: "crates" },
      { name: "cddm-core", path: "crates/cddm-core" },
    ]);
  });

  it("navigates up and resets to root cleanly", () => {
    const { result } = renderHook(() =>
      useTreemapLayout({ clonePairs: mockClonePairs, width: 800, height: 400 }),
    );

    act(() => {
      result.current.navigateTo("crates/cddm-core/src");
    });
    expect(result.current.currentPath).toBe("crates/cddm-core/src");

    act(() => {
      result.current.navigateUp();
    });
    expect(result.current.currentPath).toBe("crates/cddm-core");

    act(() => {
      result.current.resetToRoot();
    });
    expect(result.current.currentPath).toBe("");
    expect(result.current.breadcrumbs).toEqual([{ name: "Root", path: "" }]);
  });
});
