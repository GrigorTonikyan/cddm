import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { PolicyRulesModal } from "../PolicyRulesModal";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "../../store/cddm-store";
import { PolicyConfig, ScanResult } from "../../types/cddm-types";

describe("PolicyRulesModal Component", () => {
  const mockConfig: PolicyConfig = {
    boundaries: [
      {
        name: "domain-isolation",
        description: "Domain logic must not be duplicated into presentation",
        source: "src/domain/**",
        forbidden_targets: ["src/presentation/**"],
        severity: "Error",
      },
    ],
    zero_duplication: [
      {
        name: "auth-clean",
        description: "Security auth modules must have zero code duplication",
        pattern: "src/auth/**",
        severity: "Error",
      },
    ],
    limits: [
      {
        name: "api-cluster-limit",
        description: "API handlers clone limits",
        pattern: "src/api/**",
        max_tokens: 80,
        max_occurrences: 2,
        severity: "Warning",
      },
    ],
    raw_toml: "# Test rules TOML\n",
  };

  const mockScanResult: ScanResult = {
    scan_id: "test-scan",
    total_files: 5,
    total_tokens: 500,
    total_clones: 2,
    total_clusters: 1,
    duplication_percentage: 12.5,
    dry_health_score: 87.5,
    clone_pairs: [],
    clone_clusters: [],
    duration_ms: 25,
    language_breakdown: [],
    policy_violations: [
      {
        rule_name: "domain-isolation",
        rule_type: "boundary",
        severity: "Error",
        message: "Disallowed clone across boundary",
        file_a: "src/domain/entity.rs",
        start_line_a: 10,
        end_line_a: 25,
        file_b: "src/presentation/view.rs",
        start_line_b: 30,
        end_line_b: 45,
        token_count: 65,
      },
    ],
  };

  beforeEach(() => {
    useCDDMStore.setState({
      policyConfig: mockConfig,
      isPolicyLoading: false,
      policyError: null,
      results: mockScanResult,
    });
  });

  it("should return null when closed", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <PolicyRulesModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render tabs and active policies when open", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <PolicyRulesModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(
      screen.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeDefined();
    expect(screen.getByText("Active Policies (3)")).toBeDefined();
    expect(screen.getByText("Violations Inspector (1)")).toBeDefined();
    expect(screen.getByText(".cddmrules.toml Editor")).toBeDefined();

    // Verify rules content
    expect(screen.getByText("domain-isolation")).toBeDefined();
    expect(screen.getByText("src/domain/**")).toBeDefined();
    expect(screen.getByText("src/presentation/**")).toBeDefined();
    expect(screen.getByText("auth-clean")).toBeDefined();
    expect(screen.getByText("src/auth/**")).toBeDefined();
    expect(screen.getByText("api-cluster-limit")).toBeDefined();
    expect(screen.getByText("src/api/**")).toBeDefined();
  });

  it("should switch to violations tab and display violation cards", () => {
    render(
      <Win2xManagerProvider>
        <PolicyRulesModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const violationsTab = screen.getByText("Violations Inspector (1)");
    fireEvent.click(violationsTab);

    expect(screen.getByText("Total Detected Violations: 1")).toBeDefined();
    expect(screen.getByText("Disallowed clone across boundary")).toBeDefined();
    expect(screen.getByText("src/domain/entity.rs:10-25")).toBeDefined();
    expect(screen.getByText("src/presentation/view.rs:30-45")).toBeDefined();
  });

  it("should switch to editor tab and allow editing raw TOML", () => {
    render(
      <Win2xManagerProvider>
        <PolicyRulesModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const editorTab = screen.getByText(".cddmrules.toml Editor");
    fireEvent.click(editorTab);

    expect(screen.getByText(".cddmrules.toml")).toBeDefined();
    expect(screen.getByText("Reset Starter Rules")).toBeDefined();
    expect(
      screen.getByPlaceholderText("# Enter architectural policy rules in TOML format..."),
    ).toBeDefined();
  });
});
