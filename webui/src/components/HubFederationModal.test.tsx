import { screen } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { HubFederationModal } from "./HubFederationModal";
import {
  clickElementAsync,
  expectDefinedTexts,
  expectNullWhenClosed,
  mockSuccessResponse,
  renderAsyncWithWin2x,
} from "../test/test-helpers";
import type { HubScanSummary } from "../types/cddm-types";

describe("HubFederationModal Component", () => {
  const mockSummary: HubScanSummary = {
    hub_name: "Acme Federation Hub",
    total_repos: 2,
    total_files: 80,
    total_tokens: 15000,
    organization_dry_score: 97.2,
    repos: [
      {
        name: "core-backend",
        path: "services/backend",
        tech_stack: "Rust",
        total_files: 50,
        total_tokens: 10000,
        internal_duplication_percentage: 1.2,
        cross_repo_duplication_percentage: 0.5,
        dry_health_score: 98.0,
      },
      {
        name: "web-frontend",
        path: "apps/frontend",
        tech_stack: "TypeScript",
        total_files: 30,
        total_tokens: 5000,
        internal_duplication_percentage: 2.1,
        cross_repo_duplication_percentage: 0.8,
        dry_health_score: 96.4,
      },
    ],
    duplication_matrix: [
      {
        repo_a: "core-backend",
        repo_b: "web-frontend",
        shared_clones: 1,
        shared_tokens: 65,
      },
    ],
    clusters: [
      {
        id: 1,
        repos: ["core-backend", "web-frontend"],
        occurrences: [
          {
            repo_name: "core-backend",
            file_path: "src/utils.rs",
            start_line: 1,
            end_line: 10,
          },
          {
            repo_name: "web-frontend",
            file_path: "src/helpers.ts",
            start_line: 5,
            end_line: 15,
          },
        ],
        token_count: 65,
        similarity: 1.0,
        suggested_package: "@acme/shared-utils",
      },
    ],
  };

  beforeEach(() => {
    vi.restoreAllMocks();
    globalThis.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("/api/hub/extract")) {
        return mockSuccessResponse({
          package_name: "@acme/shared-utils",
          generated_files: [
            {
              file_path: "packages/@acme/shared-utils/package.json",
              content: "{}",
              is_new: true,
            },
          ],
          repo_updates: [],
        });
      }
      return mockSuccessResponse(mockSummary);
    });
  });

  it("should not render when isOpen is false", () => {
    expectNullWhenClosed(
      <HubFederationModal isOpen={false} onClose={vi.fn()} initialSummary={null} />,
    );
  });

  it("should render modal with hub summary when open", async () => {
    await renderAsyncWithWin2x(
      <HubFederationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />,
    );

    expectDefinedTexts([
      "Acme Federation Hub",
      /2 Connected Repositories/,
      "97.2 / 100.0",
      "core-backend",
      "web-frontend",
    ]);
  });

  it("should switch between tabs and allow shared package extraction", async () => {
    await renderAsyncWithWin2x(
      <HubFederationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />,
    );

    // 1. Matrix tab
    await clickElementAsync(/Duplication Matrix/);
    expect(screen.getByText("1 Shared Clusters")).toBeDefined();
    expect(screen.getByText("65 Duplicate Tokens")).toBeDefined();

    // 2. Clusters tab
    await clickElementAsync(/Cross-Repo Extraction/);
    expect(screen.getByText("@acme/shared-utils")).toBeDefined();

    // 3. Trigger extraction
    await clickElementAsync("Extract Shared Package");
    expect(screen.getByText(/Package Extraction Synthesized/)).toBeDefined();
  });
});
