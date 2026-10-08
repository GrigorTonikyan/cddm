import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import App from "./App";
import { useCDDMStore } from "./store/cddm-store";
import { createMockScanResult } from "./test/test-helpers";
import { Win2xManagerProvider } from "./components/ui/win2x-manager/context/win2x-manager-context";

describe("App Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
        text: () => Promise.resolve(""),
      }),
    );
  });

  it("should render CDDM Studio header", () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("CDDM Studio")).toBeDefined();
  });

  it("should render ScanConfigPanel component", () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("Scan Configuration")).toBeDefined();
  });

  it("should show error banner when store has error", () => {
    useCDDMStore.setState({ error: "Something went wrong!" });
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("Something went wrong!")).toBeDefined();
  });

  it("should open ScanConfigModal when clicking Config Window in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const configBtn = screen.getByText("Config Window");
    fireEvent.click(configBtn);
    expect(
      await screen.findByText(
        "Scan Parameters & Centralized Configuration Studio",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open HookManagerModal when clicking Hooks in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const hooksBtn = screen.getByText("Hooks");
    fireEvent.click(hooksBtn);
    expect(
      await screen.findByText("Git Hook Manager & CI/CD Studio", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open MonorepoWorkspaceModal when clicking Monorepo in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const monorepoBtn = screen.getByText("Monorepo");
    fireEvent.click(monorepoBtn);
    expect(
      await screen.findByText(
        "Monorepo Workspace & Multi-Package Architecture",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open HealthAuditModal and ExportReportModal from header when results exist", async () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });

    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    // Test Health Audit modal
    const healthBtns = screen.getAllByText("Health Audit");
    fireEvent.click(healthBtns[0]!);
    expect(
      await screen.findByText("DRY Health Score Audit & Diagnostics", {}, { timeout: 15000 }),
    ).toBeDefined();

    // Test Reports modal
    const reportsBtns = screen.getAllByText("Reports");
    fireEvent.click(reportsBtns[0]!);
    expect(
      await screen.findByText("Report Center & SARIF Exporter", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open OverlapDetectorModal when clicking Overlap Detector in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const overlapBtn = screen.getByText("Overlap Detector");
    fireEvent.click(overlapBtn);
    expect(
      await screen.findByText(
        "Ecosystem Library Reimplementation & Overlap Detector",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open DiffScanResultsModal when clicking Diff Scan in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const diffBtn = screen.getByText("Diff Scan");
    fireEvent.click(diffBtn);
    expect(
      await screen.findByText(
        "Differential Codebase Scan & Branch Comparison",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open HubFederationModal when clicking Org Hub in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const hubBtn = screen.getByText("Org Hub");
    fireEvent.click(hubBtn);
    expect(
      await screen.findByText(
        "Organization Federation Hub (.cddmhub.toml)",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open RefactorSandboxModal when clicking Refactor Studio in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const refactorBtn = screen.getByText("Refactor Studio");
    fireEvent.click(refactorBtn);
    expect(
      await screen.findByText(
        "Interactive Auto-Refactor Sandbox & Visual Studio",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open TimelineExplorerModal when clicking Timeline Trends in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const timelineBtn = screen.getByText("Timeline Trends");
    fireEvent.click(timelineBtn);
    expect(
      await screen.findByText(
        "Historical Duplication & Git Timeline Evolution",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open SuppressionRulesModal when clicking Suppression Rules in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const suppressionBtn = screen.getByText("Suppression Rules");
    fireEvent.click(suppressionBtn);
    expect(
      await screen.findByText(
        "Intelligent AST Suppression & .cddmignore Engine",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open PolicyRulesModal when clicking Policy Studio in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const policyBtn = screen.getByText("Policy Studio");
    fireEvent.click(policyBtn);
    expect(
      await screen.findByText(
        "Architectural Boundary & Anti-Duplication Policy Studio",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open SemanticGraphModal when clicking Semantic Graph in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const semanticBtn = screen.getByText("Semantic Graph");
    fireEvent.click(semanticBtn);
    expect(
      await screen.findByText(
        "Deep Semantic Graph & Polyglot Isomorphism Engine",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open CoverageCorrelationModal when clicking Coverage in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const coverageBtn = screen.getByText("Coverage");
    fireEvent.click(coverageBtn);
    expect(
      await screen.findByText(
        "Runtime Execution & Coverage-Aware De-duplication",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open DeadCodeExplorerModal when clicking Dead Code in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const deadCodeBtn = screen.getByText("Dead Code");
    fireEvent.click(deadCodeBtn);
    expect(
      await screen.findByText("Polyglot Dead Code Explorer & Safe Pruner", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open McpAppsPreviewModal when clicking MCP Apps in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const mcpBtn = screen.getByText("MCP Apps");
    fireEvent.click(mcpBtn);
    expect(
      await screen.findByText("MCP Apps Generative UI Studio", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open CodeEditorModal when clicking Code Editor in header in default state", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const editorBtn = screen.getByText("Code Editor");
    fireEvent.click(editorBtn);
    expect(
      await screen.findByText("Integrated Code Editor & Split Diff Studio", {}, { timeout: 15000 }),
    ).toBeDefined();
  });
});
