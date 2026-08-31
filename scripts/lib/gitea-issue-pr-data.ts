/**
 * Gitea Issues & Pull Requests Data Definitions for CDDM
 */

export interface IssueDef {
  title: string;
  body: string;
  milestoneTitle: string;
  labels: string[];
  closed: boolean;
}

export interface PRDef {
  title: string;
  branch: string;
  body: string;
  milestoneTitle: string;
  labels: string[];
  merged: boolean;
}

export const SEED_ISSUES: IssueDef[] = [
  {
    title: "[EPIC] Universal 4-Pillar Feature Parity across CLI, WebUI, MCP, and TUI",
    body: `### Description
Enforce strict 100% feature parity across all 4 interaction surfaces:
1. CLI Engine (\`crates/cddm-cli\`)
2. WebUI Studio (\`webui/\`)
3. MCP Server (\`crates/cddm-mcp\`)
4. TUI Studio (\`crates/cddm-cli/src/tui/\`)

### Acceptance Criteria
- [x] All 27 core capabilities accessible across all 4 pillars.
- [x] Dynamic AST parity check (\`bun scripts/check-feature-parity.ts\`).
- [x] Zero interface orphans.`,
    milestoneTitle: "v1.9.0",
    labels: ["feat: 4-pillar-parity", "priority: high"],
    closed: true,
  },
  {
    title: "[CI/CD] Automated Gitea Actions CI/CD with Linux/Windows Cross-Compilation",
    body: `### Description
Configure self-hosted Gitea Actions runners to build, verify, cross-compile, and distribute CDDM artifacts:
- Linux AMD64 standalone binaries
- Windows x64 binaries via MinGW cross-compiler
- VS Code VSIX extension package
- Automated SHA256 checksum generation and Gitea release publishing

### Acceptance Criteria
- [x] Release pipeline executes on \`v*\` tag push.
- [x] Full multi-job test matrix with 100% green gates.
- [x] Automated artifact attachment to Gitea Release API.`,
    milestoneTitle: "v1.9.0",
    labels: ["ci/cd: gitea-actions", "priority: high"],
    closed: true,
  },
  {
    title: "[WebUI] React 19 Feature-Sliced Design Studio with Monaco Diff Visualizer",
    body: `### Description
Deliver full-featured visual studio in \`webui/\` utilizing React 19, Feature-Sliced Design (FSD), and Monaco code editor.

### Features
- [x] Side-by-side Monaco syntax diffing.
- [x] Treemap and Sunburst duplication density visualizer.
- [x] Live SSE watch daemon for real-time rescan.
- [x] 1-click refactoring preview and apply modal.`,
    milestoneTitle: "v1.9.0",
    labels: ["feat: webui-studio", "priority: medium"],
    closed: true,
  },
  {
    title: "[MCP] Model Context Protocol Server with 27 Dedicated Tool Handlers",
    body: `### Description
Implement JSON-RPC 2.0 Model Context Protocol server exposing all CDDM clone detection, AST refactoring, and monorepo query capabilities to AI coding agents.

### Verification
- [x] 1:1 dedicated test suite per MCP tool under \`tests/mcp/tools/\`.
- [x] Dynamic discovery verification in \`tests/mcp/discovery.test.ts\`.
- [x] Full schema validation and JSON-RPC 2.0 error handling.`,
    milestoneTitle: "v1.9.0",
    labels: ["feat: mcp-protocol", "priority: high"],
    closed: true,
  },
  {
    title: "[Performance] SIMD-Accelerated Vector Dot Products and AVX2 Rolling Hash",
    body: `### Description
Optimize Winnowing fingerprinting and neural embedding similarity calculations with AVX2 and NEON SIMD vector instructions.

### Benchmarks
- [x] Sub-10ms scans for 50k+ lines of code.
- [x] AVX2 polynomial rolling hash algorithm.
- [x] Zero-copy buffer reuse during multi-threaded AST queries.`,
    milestoneTitle: "v1.9.0",
    labels: ["perf: simd-optimization", "priority: high"],
    closed: true,
  },
  {
    title: "[RFC] AI Refactor Surgeon: Automated AST Cluster Extraction [EP-35]",
    body: `### Description
Design and implement an automated AST refactoring surgeon that extracts multi-file duplicate clone clusters into standalone shared functions or modules with guaranteed syntax preservation and automated test rollback.

### Proposed Architecture
1. AST visitor identifies parameterizable variable differences.
2. Synthesizes canonical signature and extracted function body.
3. Rewrites call sites across all cluster files.
4. Executes \`cargo test\` / \`bun test\` to verify zero regression.`,
    milestoneTitle: "v1.10.0",
    labels: ["feat: ai-surgeon", "priority: high"],
    closed: false,
  },
  {
    title: "[FEAT] Federation Hub & Cross-Repository Clone Correlation Matrix [EP-36]",
    body: `### Description
Enable cross-repository duplicate code tracking across multiple distinct Git repositories in an organization.

### Deliverables
- [ ] Central hub cache repository scanner.
- [ ] Monorepo workspace package extractor.
- [ ] Cross-repo clone drift alert notifications.`,
    milestoneTitle: "v1.10.0",
    labels: ["feat: neural-clones", "priority: high"],
    closed: false,
  },
  {
    title: "[PERF] SQ8 Vector Quantization and HNSW Search for Millions of Lines",
    body: `### Description
Implement 8-bit scalar quantization (SQ8) and Hierarchical Navigable Small World (HNSW) graph indexing for large-scale semantic clone searching across enterprise monorepos.

### Target Metrics
- 4x memory footprint reduction.
- Logarithmic sub-linear vector search latency.`,
    milestoneTitle: "v2.0.0",
    labels: ["perf: simd-optimization", "feat: neural-clones", "priority: medium"],
    closed: false,
  },
];
