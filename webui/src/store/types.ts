import type {
  AiRefactorPromptRequest,
  ApplyPatchResult,
  ApplyRefactorBranchResult,
  AstRewriteResult,
  CloneCluster,
  ExtractRequest,
  ExtractResult,
  HookStatus,
  PolicyConfig,
  PolicyEvaluationResult,
  RefactorSandboxRequest,
  RefactorSandboxResult,
  ScanConfig,
  ScanProgress,
  ScanResult,
  SemanticGraphRequest,
  SemanticGraphResponse,
  SuppressionConfig,
  TimelineTrend,
  VerifyRefactorRequest,
  VerifyRefactorResult,
} from "../types/cddm-types";
import type { SupportedEditor } from "../utils/ide-links";

/**
 * Interface for CDDM Zustand Store State and Actions.
 */
export interface CDDMStoreState {
  /** Current scan configuration */
  config: ScanConfig;
  /** Active scan ID or null if idle */
  activeScanId: string | null;
  /** Active scan progress details */
  progress: ScanProgress | null;
  /** Final scan results if completed */
  results: ScanResult | null;
  /** Whether a scan is currently running */
  isScanning: boolean;
  /** Error message if scan failed */
  error: string | null;

  /** Active view mode for results list (pairwise vs n-way clusters) */
  viewMode: "pairs" | "clusters";
  /** Currently selected cluster for inspection or refactoring */
  selectedCluster: CloneCluster | null;

  /** Real-time live watch & push sync status */
  isLiveWatchActive: boolean;
  /** Preferred IDE editor for protocol deeplinks */
  preferredEditor: SupportedEditor;
  /** Timestamp of the most recent live push synchronization */
  lastLiveSyncTimestamp: number | null;
  /** Whether a patch is currently being applied to workspace */
  isPatching: boolean;
  /** Status notification message for patch operations */
  patchStatusMessage: string | null;

  /** Global window modal visibility states */
  isScanConfigOpen: boolean;
  isHealthAuditOpen: boolean;
  isExportReportOpen: boolean;
  isTreemapModalOpen: boolean;
  isLanguageModalOpen: boolean;
  isClusterRefactorModalOpen: boolean;
  isTimelineModalOpen: boolean;
  isSuppressionModalOpen: boolean;
  isRefactorSandboxOpen: boolean;
  isPolicyRulesModalOpen: boolean;
  isSemanticGraphModalOpen: boolean;
  isOverlapDetectorOpen: boolean;
  isHubModalOpen: boolean;

  /** Organization Federation Hub state */
  hubConfig: import("../types/cddm-types").HubConfig | null;
  hubSummary: import("../types/cddm-types").HubScanSummary | null;
  isHubLoading: boolean;
  hubError: string | null;

  /** Live watch sync counter and last sync events */
  liveSyncCount: number;

  /** Semantic graph inspection & comparison state */
  semanticGraphRequest: SemanticGraphRequest | null;
  semanticGraphResponse: SemanticGraphResponse | null;
  isSemanticGraphLoading: boolean;
  semanticGraphError: string | null;
  crossLanguageClones: import("../types/cddm-types").CrossLanguageClonePair[];
  isCrossLanguageLoading: boolean;
  neuralResult: import("../types/cddm-types").NeuralScanResult | null;
  isNeuralLoading: boolean;

  /** Historical timeline data and loading state */
  timelineData: TimelineTrend | null;
  isTimelineLoading: boolean;
  timelineError: string | null;
  hookStatus: HookStatus | null;

  /** Suppression rules state */
  suppressionConfig: SuppressionConfig | null;
  isSuppressionLoading: boolean;
  suppressionError: string | null;

  /** Policy rules state */
  policyConfig: PolicyConfig | null;
  isPolicyLoading: boolean;
  policyError: string | null;

  /** Refactor sandbox state */
  sandboxRequest: RefactorSandboxRequest | null;
  sandboxResult: RefactorSandboxResult | null;
  isSandboxLoading: boolean;
  sandboxError: string | null;

  /** AST rewrite preview state */
  astRewriteResult: AstRewriteResult | null;
  isAstLoading: boolean;
  astError: string | null;

  /** Test verification state */
  verifyResult: VerifyRefactorResult | null;
  isVerifying: boolean;
  verifyError: string | null;

  /** Shared crate / module extraction state */
  extractResult: ExtractResult | null;
  isExtractLoading: boolean;
  extractError: string | null;

  /** Updates the scan configuration */
  setConfig: (config: Partial<ScanConfig>) => void;
  /** Initiates a new code duplication scan */
  startScan: () => Promise<void>;
  /** Cancels an ongoing scan */
  cancelScan: () => void;
  /** Resets state to idle */
  resetScan: () => void;

  /** View mode and cluster setters */
  setViewMode: (viewMode: "pairs" | "clusters") => void;
  setSelectedCluster: (selectedCluster: CloneCluster | null) => void;

  /** Live watch & IDE preferences setters */
  setIsLiveWatchActive: (active: boolean) => void;
  setPreferredEditor: (editor: SupportedEditor) => void;
  setPatchStatusMessage: (msg: string | null) => void;
  /** Applies synthesized refactoring patch directly to workspace */
  applyPatch: (patch: string, dryRun?: boolean) => Promise<ApplyPatchResult>;

  /** Historical timeline & git hooks */
  fetchTimeline: (directory?: string, maxSamples?: number, minTokens?: number) => Promise<void>;
  fetchHookStatus: (directory?: string) => Promise<void>;
  installHook: (hookType: string, failThreshold?: number, minTokens?: number) => Promise<string>;

  /** Suppression rules management */
  fetchSuppressionRules: () => Promise<void>;
  saveSuppressionRules: (config: SuppressionConfig) => Promise<void>;

  /** Policy rules management */
  fetchPolicyRules: () => Promise<void>;
  savePolicyRules: (config: PolicyConfig) => Promise<void>;
  evaluatePolicyRules: (directory?: string) => Promise<PolicyEvaluationResult>;

  /** Refactor sandbox management */
  openRefactorSandbox: (req: RefactorSandboxRequest) => Promise<void>;
  previewRefactorSandbox: (req: RefactorSandboxRequest) => Promise<RefactorSandboxResult>;
  previewAstRefactor: (req: RefactorSandboxRequest) => Promise<AstRewriteResult>;
  /** Runs closed-loop test suite verification */
  verifyRefactorTestSuite: (req: VerifyRefactorRequest) => Promise<VerifyRefactorResult>;
  /** Applies refactoring patch to workspace or dedicated Git branch */
  applyRefactorBranch: (
    patch: string,
    branchName?: string,
    createBranch?: boolean,
  ) => Promise<ApplyRefactorBranchResult>;
  /** Synthesizes an LLM AI refactoring prompt specification */
  generateAiPrompt: (req: AiRefactorPromptRequest) => Promise<string>;
  /** Generates a preview plan for extracting shared module/crate */
  previewExtractModule: (req: ExtractRequest) => Promise<ExtractResult>;
  /** Commits and applies extraction to workspace disk */
  applyExtractModule: (req: ExtractRequest) => Promise<ExtractResult>;

  /** Modal visibility setters */
  setIsScanConfigOpen: (open: boolean) => void;
  setIsHealthAuditOpen: (open: boolean) => void;
  setIsExportReportOpen: (open: boolean) => void;
  setIsTreemapModalOpen: (open: boolean) => void;
  setIsLanguageModalOpen: (open: boolean) => void;
  setIsClusterRefactorModalOpen: (open: boolean) => void;
  setIsTimelineModalOpen: (open: boolean) => void;
  setIsSuppressionModalOpen: (open: boolean) => void;
  setIsRefactorSandboxOpen: (open: boolean) => void;
  setIsPolicyRulesModalOpen: (open: boolean) => void;
  setIsSemanticGraphModalOpen: (open: boolean) => void;
  setIsOverlapDetectorOpen: (open: boolean) => void;
  setIsLiveEventInspectorOpen: (open: boolean) => void;

  /** Live Watch Daemon state and actions */
  isLiveEventInspectorOpen: boolean;
  watchEventsLog: import("../types/cddm-types").WatchDeltaReport[];
  lastWatchDelta: import("../types/cddm-types").WatchDeltaReport | null;
  recentModifiedFiles: string[];
  fetchWatchStatus: () => Promise<void>;
  toggleWatch: (active?: boolean) => Promise<void>;
  triggerManualRescan: () => Promise<void>;
  clearWatchEventsLog: () => void;

  /** Semantic graph actions */
  fetchSemanticGraph: (req: SemanticGraphRequest) => Promise<SemanticGraphResponse>;
  scanCrossLanguageClones: (
    threshold?: number,
    directory?: string,
  ) => Promise<import("../types/cddm-types").CrossLanguageClonePair[]>;
  scanNeuralClones: (
    req?: import("../types/cddm-types").SemanticNeuralRequest,
  ) => Promise<import("../types/cddm-types").NeuralScanResult>;
  openSemanticGraphModal: (req?: SemanticGraphRequest) => Promise<void>;

  /** Organization Federation Hub actions */
  setIsHubModalOpen: (open: boolean) => void;
  fetchHubConfig: () => Promise<void>;
  saveHubConfig: (config: import("../types/cddm-types").HubConfig) => Promise<void>;
  runHubScan: (
    config?: import("../types/cddm-types").HubConfig,
  ) => Promise<import("../types/cddm-types").HubScanSummary>;
  extractHubPackage: (
    req: import("../types/cddm-types").HubExtractRequest,
  ) => Promise<import("../types/cddm-types").HubExtractResult>;

  /** Runtime Execution & Coverage correlation */
  isCoverageModalOpen: boolean;
  coverageSummary: import("../types/cddm-types").CoverageCorrelationSummary | null;
  isCoverageLoading: boolean;
  coverageError: string | null;
  setIsCoverageModalOpen: (open: boolean) => void;
  ingestCoverageReport: (req: import("../types/cddm-types").CoverageIngestRequest) => Promise<void>;
  correlateCoverage: (
    req?: import("../types/cddm-types").CoverageCorrelateRequest,
  ) => Promise<import("../types/cddm-types").CoverageCorrelationSummary>;

  /** Polyglot Dead Code Detection & Safe Deletion Synthesizer */
  isDeadCodeModalOpen: boolean;
  deadCodeSummary: import("../types/dead-code-types").DeadCodeSummary | null;
  isDeadCodeLoading: boolean;
  deadCodeError: string | null;
  isDeadCodePruning: boolean;
  lastPruneResult: import("../types/dead-code-types").DeadClonePruneResult | null;
  deadCodePruneError: string | null;
  setIsDeadCodeModalOpen: (open: boolean) => void;
  scanDeadCode: (
    req?: import("../types/dead-code-types").DeadCodeScanRequest,
  ) => Promise<import("../types/dead-code-types").DeadCodeSummary>;
  pruneDeadCode: (
    req?: import("../types/dead-code-types").DeadClonePruneRequest,
  ) => Promise<import("../types/dead-code-types").DeadClonePruneResult>;
}
