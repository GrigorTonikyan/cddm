import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { CachePackCard } from "./CachePackCard";
import { Archive } from "lucide-react";

describe("CachePackCard Component", () => {
  it("should render card fields and handle submit button click", () => {
    const onSubmit = vi.fn();
    const onFirstChange = vi.fn();
    const onSecondChange = vi.fn();

    render(
      <CachePackCard
        title="Export Cache Pack"
        description="Bundle analyzed tokens into portable pack"
        icon={<Archive className="w-4 h-4" />}
        firstFieldLabel="Source Cache Dir"
        firstFieldValue=".cddm-cache"
        firstFieldId="test-first-field"
        onFirstFieldChange={onFirstChange}
        secondFieldLabel="Output File"
        secondFieldValue="cache.pack"
        secondFieldId="test-second-field"
        onSecondFieldChange={onSecondChange}
        summary={null}
        summaryTheme="indigo"
        isSubmitting={false}
        submitButtonText="Export Cache Pack"
        submittingText="Exporting..."
        onSubmit={onSubmit}
        submitButtonTheme="indigo"
        submitIcon={<Archive className="w-3.5 h-3.5" />}
      />,
    );

    expect(screen.getAllByText("Export Cache Pack")).toHaveLength(2);
    expect(screen.getByText("Bundle analyzed tokens into portable pack")).toBeDefined();

    const input1 = screen.getByLabelText("Source Cache Dir") as HTMLInputElement;
    fireEvent.change(input1, { target: { value: ".custom-cache" } });
    expect(onFirstChange).toHaveBeenCalledWith(".custom-cache");

    const submitBtn = screen.getByRole("button", { name: /Export Cache Pack/i });
    fireEvent.click(submitBtn);
    expect(onSubmit).toHaveBeenCalled();
  });

  it("should display summary details when provided", () => {
    render(
      <CachePackCard
        title="Import Cache Pack"
        description="Hydrate local cache"
        icon={<Archive className="w-4 h-4" />}
        firstFieldLabel="Pack File"
        firstFieldValue="cache.pack"
        firstFieldId="test-import-field"
        onFirstFieldChange={() => {}}
        secondFieldLabel="Target Dir"
        secondFieldValue=".cddm-cache"
        secondFieldId="test-target-field"
        onSecondFieldChange={() => {}}
        summary={{
          success: true,
          entry_count: 55,
          pack_file: "cache.pack",
          checksum: "sha256xyz",
          message: "Pack imported",
        }}
        summaryTheme="purple"
        isSubmitting={false}
        submitButtonText="Import Cache Pack"
        submittingText="Importing..."
        onSubmit={() => {}}
        submitButtonTheme="purple"
        submitIcon={<Archive className="w-3.5 h-3.5" />}
      />,
    );

    expect(screen.getByText("Pack imported")).toBeDefined();
    expect(screen.getByText("Entries: 55")).toBeDefined();
    expect(screen.getByText("File: cache.pack")).toBeDefined();
  });
});
