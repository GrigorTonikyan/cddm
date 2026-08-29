import { renderHook, act } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { useFormState } from "./use-form-state";

describe("useFormState", () => {
  it("initializes with initial state", () => {
    const { result } = renderHook(() => useFormState({ name: "CDDM", count: 42 }));
    expect(result.current.form.name).toBe("CDDM");
    expect(result.current.form.count).toBe(42);
  });

  it("updates individual fields properly", () => {
    const { result } = renderHook(() => useFormState({ name: "Initial", active: false }));

    act(() => {
      result.current.updateField("name", "Updated");
    });
    expect(result.current.form.name).toBe("Updated");
    expect(result.current.form.active).toBe(false);

    act(() => {
      result.current.updateField("active", true);
    });
    expect(result.current.form.active).toBe(true);
  });

  it("resets form back to initial state", () => {
    const { result } = renderHook(() => useFormState({ name: "Base", val: 1 }));

    act(() => {
      result.current.updateField("name", "Changed");
      result.current.updateField("val", 99);
    });
    expect(result.current.form.name).toBe("Changed");

    act(() => {
      result.current.resetForm();
    });
    expect(result.current.form.name).toBe("Base");
    expect(result.current.form.val).toBe(1);
  });
});
