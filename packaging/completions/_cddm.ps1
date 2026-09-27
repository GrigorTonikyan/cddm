
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cddm' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cddm'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cddm' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan target directory for code duplication & DRY health score')
            [CompletionResult]::new('dead-code', 'dead-code', [CompletionResultType]::ParameterValue, 'Detect unreferenced functions, unreachable blocks, and dead code duplicates')
            [CompletionResult]::new('prune', 'prune', [CompletionResultType]::ParameterValue, 'Automatically prune unreachable dead clone clusters and unreferenced code')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'Differential duplication scan comparing current changes against a Git base revision')
            [CompletionResult]::new('semantic', 'semantic', [CompletionResultType]::ParameterValue, 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms')
            [CompletionResult]::new('refactor', 'refactor', [CompletionResultType]::ParameterValue, 'Synthesize automated refactoring suggestions for duplicate clone pairs')
            [CompletionResult]::new('extract', 'extract', [CompletionResultType]::ParameterValue, 'Extract duplicate code into a standalone shared crate or module')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Launch interactive WebUI dashboard in browser')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch directory and trigger continuous real-time clone analysis on file save')
            [CompletionResult]::new('lsp', 'lsp', [CompletionResultType]::ParameterValue, 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles')
            [CompletionResult]::new('trend', 'trend', [CompletionResultType]::ParameterValue, 'Analyze historical duplication trends across Git commit history')
            [CompletionResult]::new('hook', 'hook', [CompletionResultType]::ParameterValue, 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement')
            [CompletionResult]::new('ignore', 'ignore', [CompletionResultType]::ParameterValue, 'Manage .cddmignore rules and test path suppression matching')
            [CompletionResult]::new('rules', 'rules', [CompletionResultType]::ParameterValue, 'Manage architectural policy rules (.cddmrules.toml)')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)')
            [CompletionResult]::new('comment', 'comment', [CompletionResultType]::ParameterValue, 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests')
            [CompletionResult]::new('heal', 'heal', [CompletionResultType]::ParameterValue, 'Autonomous AI Code Surgeon refactoring with closed-loop test healing')
            [CompletionResult]::new('cache', 'cache', [CompletionResultType]::ParameterValue, 'Manage persistent fingerprint cache and export/import .cddmpack archives')
            [CompletionResult]::new('monorepo', 'monorepo', [CompletionResultType]::ParameterValue, 'Discover and scan monorepos with multi-workspace packages')
            [CompletionResult]::new('tui', 'tui', [CompletionResultType]::ParameterValue, 'Launch interactive Terminal UI (TUI) Studio dashboard')
            [CompletionResult]::new('overlap', 'overlap', [CompletionResultType]::ParameterValue, 'Detect reimplemented ecosystem library algorithms and suggest standard packages')
            [CompletionResult]::new('hub', 'hub', [CompletionResultType]::ParameterValue, 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)')
            [CompletionResult]::new('coverage', 'coverage', [CompletionResultType]::ParameterValue, 'Dynamic runtime execution & coverage-aware de-duplication analysis')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;scan' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan (e.g. Rust, TypeScript, Python)')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan (e.g. Rust, TypeScript, Python)')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore (e.g. node_modules, target)')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore (e.g. node_modules, target)')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'Custom path for persistent redb cache database (default: OS user cache)')
            [CompletionResult]::new('--cddmignore', '--cddmignore', [CompletionResultType]::ParameterName, 'Custom path to .cddmignore configuration file')
            [CompletionResult]::new('--ignore-tests', '--ignore-tests', [CompletionResultType]::ParameterName, 'Automatically filter test files and test directories (default: true)')
            [CompletionResult]::new('--ignore-mocks', '--ignore-mocks', [CompletionResultType]::ParameterName, 'Automatically filter mock and fixture files (default: true)')
            [CompletionResult]::new('--rules', '--rules', [CompletionResultType]::ParameterName, 'Path to custom architectural policy rules (.cddmrules.toml)')
            [CompletionResult]::new('--cross-language', '--cross-language', [CompletionResultType]::ParameterName, 'Detect cross-language semantic clones across different programming languages')
            [CompletionResult]::new('--detect-type4', '--detect-type4', [CompletionResultType]::ParameterName, 'Detect Type-4 semantic clones using AST/CFG Weisfeiler-Lehman graph analysis')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'Maximum number of parallel worker threads to utilize (default: all logical cores)')
            [CompletionResult]::new('--threads', '--threads', [CompletionResultType]::ParameterName, 'Maximum number of parallel worker threads to utilize (default: all logical cores)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--summary', '--summary', [CompletionResultType]::ParameterName, 'Output compact summary report to conserve terminal/token output')
            [CompletionResult]::new('--include-ignored', '--include-ignored', [CompletionResultType]::ParameterName, 'Include files and directories ignored by .gitignore')
            [CompletionResult]::new('--git-blame', '--git-blame', [CompletionResultType]::ParameterName, 'Enable in-process git blame author & line age annotation')
            [CompletionResult]::new('--in-tree-cache', '--in-tree-cache', [CompletionResultType]::ParameterName, 'Store persistent redb cache database in workspace (.cddm/cache.db) instead of OS user cache')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Bypass persistent disk cache and force full re-scan')
            [CompletionResult]::new('--clear-cache', '--clear-cache', [CompletionResultType]::ParameterName, 'Clear existing persistent cache database before scanning')
            [CompletionResult]::new('--no-ignore-tests', '--no-ignore-tests', [CompletionResultType]::ParameterName, 'Include test files and test directories in scan')
            [CompletionResult]::new('--no-ignore-mocks', '--no-ignore-mocks', [CompletionResultType]::ParameterName, 'Include mock and fixture files in scan')
            [CompletionResult]::new('--ignore-generated', '--ignore-generated', [CompletionResultType]::ParameterName, 'Automatically filter auto-generated files with generator headers')
            [CompletionResult]::new('--enforce-policies', '--enforce-policies', [CompletionResultType]::ParameterName, 'Enforce architectural policy rules (exit code 1 on error-level violations)')
            [CompletionResult]::new('--no-cross-language', '--no-cross-language', [CompletionResultType]::ParameterName, 'Disable cross-language semantic clone detection')
            [CompletionResult]::new('--no-type3', '--no-type3', [CompletionResultType]::ParameterName, 'Disable Type-3 near-miss modified statement clone detection')
            [CompletionResult]::new('--no-type4', '--no-type4', [CompletionResultType]::ParameterName, 'Disable Type-4 semantic AST/CFG clone detection')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;dead-code' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count threshold for dead code items')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count threshold for dead code items')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Path to optional coverage report file (e.g. lcov.info, coverage.xml)')
            [CompletionResult]::new('--coverage', '--coverage', [CompletionResultType]::ParameterName, 'Path to optional coverage report file (e.g. lcov.info, coverage.xml)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Filter by target programming languages')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Filter by target programming languages')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Custom file or path ignore patterns')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Custom file or path ignore patterns')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--summary', '--summary', [CompletionResultType]::ParameterName, 'Output compact summary report to conserve terminal/token output')
            [CompletionResult]::new('--static-only', '--static-only', [CompletionResultType]::ParameterName, 'Restrict analysis to static AST & symbol analysis only')
            [CompletionResult]::new('--include-ignored', '--include-ignored', [CompletionResultType]::ParameterName, 'Include files and directories ignored by .gitignore')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;prune' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Confidence threshold for safe removal (0.0 to 1.0)')
            [CompletionResult]::new('--threshold', '--threshold', [CompletionResultType]::ParameterName, 'Confidence threshold for safe removal (0.0 to 1.0)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count threshold for dead clone items')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count threshold for dead clone items')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Filter by target programming languages')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Filter by target programming languages')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Custom file or path ignore patterns')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Custom file or path ignore patterns')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run preview without modifying files on disk')
            [CompletionResult]::new('--safe-only', '--safe-only', [CompletionResultType]::ParameterName, 'Only prune dead clones meeting strict closed-loop safety verification (default: true)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;diff' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Directory path of the Git repository to scan (default: current directory)')
            [CompletionResult]::new('--directory', '--directory', [CompletionResultType]::ParameterName, 'Directory path of the Git repository to scan (default: current directory)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, sarif)')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'Custom path for persistent redb cache database (default: .cddm/cache.db)')
            [CompletionResult]::new('--cddmignore', '--cddmignore', [CompletionResultType]::ParameterName, 'Custom path to .cddmignore configuration file')
            [CompletionResult]::new('--rules', '--rules', [CompletionResultType]::ParameterName, 'Path to custom architectural policy rules (.cddmrules.toml)')
            [CompletionResult]::new('--matrix', '--matrix', [CompletionResultType]::ParameterName, 'Multi-branch clone drift matrix comparison across multiple revisions (comma-separated list)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--git-blame', '--git-blame', [CompletionResultType]::ParameterName, 'Enable in-process git blame author & line age annotation')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Bypass persistent disk cache and force full re-scan')
            [CompletionResult]::new('--include-ignored', '--include-ignored', [CompletionResultType]::ParameterName, 'Include files and directories ignored by .gitignore')
            [CompletionResult]::new('--ignore-tests', '--ignore-tests', [CompletionResultType]::ParameterName, 'Automatically filter test files and test directories')
            [CompletionResult]::new('--ignore-mocks', '--ignore-mocks', [CompletionResultType]::ParameterName, 'Automatically filter mock and fixture files')
            [CompletionResult]::new('--ignore-generated', '--ignore-generated', [CompletionResultType]::ParameterName, 'Automatically filter auto-generated files with generator headers')
            [CompletionResult]::new('--enforce-policies', '--enforce-policies', [CompletionResultType]::ParameterName, 'Enforce architectural policy rules (exit code 1 on error-level violations)')
            [CompletionResult]::new('--cross-language', '--cross-language', [CompletionResultType]::ParameterName, 'Detect cross-language semantic clones across different programming languages')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;semantic' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)')
            [CompletionResult]::new('--threshold', '--threshold', [CompletionResultType]::ParameterName, 'Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--neural-threshold', '--neural-threshold', [CompletionResultType]::ParameterName, 'Minimum cosine similarity threshold for neural matching (default: 0.85)')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'Maximum number of parallel worker threads to utilize (default: all logical cores)')
            [CompletionResult]::new('--threads', '--threads', [CompletionResultType]::ParameterName, 'Maximum number of parallel worker threads to utilize (default: all logical cores)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--neural', '--neural', [CompletionResultType]::ParameterName, 'Enable in-process dense neural code embedding equivalence scan')
            [CompletionResult]::new('--hnsw', '--hnsw', [CompletionResultType]::ParameterName, 'Use Hierarchical Navigable Small World (HNSW) index for sub-linear logarithmic search')
            [CompletionResult]::new('--sq8', '--sq8', [CompletionResultType]::ParameterName, 'Use 8-bit scalar quantization (SQ8) for 4x vector memory reduction during search')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;refactor' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, '1-based index of clone pair to refactor')
            [CompletionResult]::new('--pair', '--pair', [CompletionResultType]::ParameterName, '1-based index of clone pair to refactor')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, '1-based index of clone cluster to refactor')
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, '1-based index of clone cluster to refactor')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path to write patch to (default: stdout)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file path to write patch to (default: stdout)')
            [CompletionResult]::new('--fn-name', '--fn-name', [CompletionResultType]::ParameterName, 'Custom name for extracted function')
            [CompletionResult]::new('--target-module', '--target-module', [CompletionResultType]::ParameterName, 'Target module path for extracted helper')
            [CompletionResult]::new('--apply-branch', '--apply-branch', [CompletionResultType]::ParameterName, 'Apply refactoring to dedicated Git branch')
            [CompletionResult]::new('--test-cmd', '--test-cmd', [CompletionResultType]::ParameterName, 'Custom test command for verification')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--provider', '--provider', [CompletionResultType]::ParameterName, 'AI provider to use for streaming refactoring (gemini, claude, openai, ollama, custom, mock)')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'AI model identifier to use')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--prompt', '--prompt', [CompletionResultType]::ParameterName, 'Generate formatted markdown prompt for AI refactoring agents')
            [CompletionResult]::new('--ast', '--ast', [CompletionResultType]::ParameterName, 'Generate Tree-sitter AST-native code transformations')
            [CompletionResult]::new('--verify', '--verify', [CompletionResultType]::ParameterName, 'Verify refactoring against test suite')
            [CompletionResult]::new('--stream', '--stream', [CompletionResultType]::ParameterName, 'Stream AI token diff generation directly to terminal')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;extract' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, '1-based index of clone pair to extract')
            [CompletionResult]::new('--pair', '--pair', [CompletionResultType]::ParameterName, '1-based index of clone pair to extract')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, '1-based index of clone cluster to extract')
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, '1-based index of clone cluster to extract')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)')
            [CompletionResult]::new('--target', '--target', [CompletionResultType]::ParameterName, 'Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)')
            [CompletionResult]::new('--fn-name', '--fn-name', [CompletionResultType]::ParameterName, 'Custom extracted helper function name')
            [CompletionResult]::new('--crate-type', '--crate-type', [CompletionResultType]::ParameterName, 'Packaging strategy: auto, crate, module, existing')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Perform a dry-run preview without modifying files')
            [CompletionResult]::new('--apply', '--apply', [CompletionResultType]::ParameterName, 'Commit and apply generated files, manifest updates, and caller rewrites')
            [CompletionResult]::new('--generate-tests', '--generate-tests', [CompletionResultType]::ParameterName, 'Automatically synthesize unit tests for the extracted helper')
            [CompletionResult]::new('--generate-benchmarks', '--generate-benchmarks', [CompletionResultType]::ParameterName, 'Automatically synthesize performance micro-benchmarks for the extracted helper')
            [CompletionResult]::new('--bench', '--bench', [CompletionResultType]::ParameterName, 'Automatically synthesize performance micro-benchmarks for the extracted helper')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;serve' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Port to bind the WebUI HTTP and WebSocket server to')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'Port to bind the WebUI HTTP and WebSocket server to')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Automatically open WebUI in default web browser')
            [CompletionResult]::new('--open', '--open', [CompletionResultType]::ParameterName, 'Automatically open WebUI in default web browser')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;watch' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'Custom path for persistent redb cache database (default: .cddm/cache.db)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Debounce delay in milliseconds before scanning on file changes')
            [CompletionResult]::new('--debounce-ms', '--debounce-ms', [CompletionResultType]::ParameterName, 'Debounce delay in milliseconds before scanning on file changes')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Optionally start embedded WebUI Studio server on specified port (default: 3000)')
            [CompletionResult]::new('--serve', '--serve', [CompletionResultType]::ParameterName, 'Optionally start embedded WebUI Studio server on specified port (default: 3000)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, ndjson)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown, ndjson)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--git-blame', '--git-blame', [CompletionResultType]::ParameterName, 'Enable in-process git blame author & line age annotation')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Bypass persistent disk cache and force full re-scan')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Automatically open WebUI in browser when --serve is enabled')
            [CompletionResult]::new('--open', '--open', [CompletionResultType]::ParameterName, 'Automatically open WebUI in browser when --serve is enabled')
            [CompletionResult]::new('--cross-language', '--cross-language', [CompletionResultType]::ParameterName, 'Detect cross-language semantic clones across different programming languages')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;lsp' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;trend' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Maximum number of historical commit snapshots to sample (default: 10)')
            [CompletionResult]::new('--max-samples', '--max-samples', [CompletionResultType]::ParameterName, 'Maximum number of historical commit snapshots to sample (default: 10)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hook' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install a Git hook enforcing code duplication thresholds')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Uninstall an existing CDDM Git hook')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Check current installation status of Git hooks')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;hook;install' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Hook type to install (pre-commit or pre-push)')
            [CompletionResult]::new('--hook-type', '--hook-type', [CompletionResultType]::ParameterName, 'Hook type to install (pre-commit or pre-push)')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Duplication percentage threshold to fail on (default: 15.0)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection (default: 50)')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection (default: 50)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hook;uninstall' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Hook type to remove (pre-commit or pre-push)')
            [CompletionResult]::new('--hook-type', '--hook-type', [CompletionResultType]::ParameterName, 'Hook type to remove (pre-commit or pre-push)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hook;status' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hook;help' {
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install a Git hook enforcing code duplication thresholds')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Uninstall an existing CDDM Git hook')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Check current installation status of Git hooks')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;hook;help;install' {
            break
        }
        'cddm;hook;help;uninstall' {
            break
        }
        'cddm;hook;help;status' {
            break
        }
        'cddm;hook;help;help' {
            break
        }
        'cddm;ignore' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a standard, well-documented .cddmignore configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Test whether a specific file path or line number is ignored by suppression rules')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;ignore;init' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Overwrite existing .cddmignore file if present')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Overwrite existing .cddmignore file if present')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;ignore;check' {
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Optional 1-based line number to check for inline suppression directives')
            [CompletionResult]::new('--line', '--line', [CompletionResultType]::ParameterName, 'Optional 1-based line number to check for inline suppression directives')
            [CompletionResult]::new('--cddmignore', '--cddmignore', [CompletionResultType]::ParameterName, 'Path to custom .cddmignore file')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--ignore-tests', '--ignore-tests', [CompletionResultType]::ParameterName, 'Check with test file suppression enabled')
            [CompletionResult]::new('--ignore-mocks', '--ignore-mocks', [CompletionResultType]::ParameterName, 'Check with mock file suppression enabled')
            [CompletionResult]::new('--ignore-generated', '--ignore-generated', [CompletionResultType]::ParameterName, 'Check with generated file suppression enabled')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;ignore;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a standard, well-documented .cddmignore configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Test whether a specific file path or line number is ignored by suppression rules')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;ignore;help;init' {
            break
        }
        'cddm;ignore;help;check' {
            break
        }
        'cddm;ignore;help;help' {
            break
        }
        'cddm;rules' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a starter .cddmrules.toml configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Evaluate architectural policy rules against codebase')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;rules;init' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Target output file path (default: .cddmrules.toml)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Target output file path (default: .cddmrules.toml)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Overwrite existing file if present')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Overwrite existing file if present')
            [CompletionResult]::new('--write', '--write', [CompletionResultType]::ParameterName, 'Write directly to disk (default: true)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;rules;check' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Custom path to .cddmrules.toml file')
            [CompletionResult]::new('--rules', '--rules', [CompletionResultType]::ParameterName, 'Custom path to .cddmrules.toml file')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--enforce-policies', '--enforce-policies', [CompletionResultType]::ParameterName, 'Exit with non-zero code if any policy violations exist')
            [CompletionResult]::new('--summary', '--summary', [CompletionResultType]::ParameterName, 'Output compact summary report to conserve terminal/token output')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;rules;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a starter .cddmrules.toml configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Evaluate architectural policy rules against codebase')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;rules;help;init' {
            break
        }
        'cddm;rules;help;check' {
            break
        }
        'cddm;rules;help;help' {
            break
        }
        'cddm;init' {
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Duplication percentage threshold to fail on (default: 15.0)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection (default: 50)')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection (default: 50)')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path (defaults to standard platform config file)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file path (defaults to standard platform config file)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Write directly to disk (default: print to stdout unless --write or output specified)')
            [CompletionResult]::new('--write', '--write', [CompletionResultType]::ParameterName, 'Write directly to disk (default: print to stdout unless --write or output specified)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;comment' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Duplication percentage threshold to fail on (default: 15.0)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Target CI/CD platform format: github, gitlab, or azure')
            [CompletionResult]::new('--platform', '--platform', [CompletionResultType]::ParameterName, 'Target CI/CD platform format: github, gitlab, or azure')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file path to write Markdown comment to (default: stdout)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file path to write Markdown comment to (default: stdout)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;heal' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Target clone cluster index to heal')
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, 'Target clone cluster index to heal')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Target clone pair index to heal')
            [CompletionResult]::new('--pair', '--pair', [CompletionResultType]::ParameterName, 'Target clone pair index to heal')
            [CompletionResult]::new('--provider', '--provider', [CompletionResultType]::ParameterName, 'AI Provider backend (gemini, claude, openai, ollama, mock)')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'Model identifier name (e.g. gemini-3.8-pro, claude-3-7-sonnet, gpt-4o, qwen2.5-coder)')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'Secret API key for authentication')
            [CompletionResult]::new('--endpoint', '--endpoint', [CompletionResultType]::ParameterName, 'Custom endpoint URL (e.g. http://localhost:11434 for Ollama)')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Maximum healing repair iterations')
            [CompletionResult]::new('--max-iterations', '--max-iterations', [CompletionResultType]::ParameterName, 'Maximum healing repair iterations')
            [CompletionResult]::new('--test-cmd', '--test-cmd', [CompletionResultType]::ParameterName, 'Custom test command (e.g. "cargo test", "bun test")')
            [CompletionResult]::new('--branch', '--branch', [CompletionResultType]::ParameterName, 'Apply passing refactoring to dedicated Git branch')
            [CompletionResult]::new('--fn-name', '--fn-name', [CompletionResultType]::ParameterName, 'Custom extracted function name')
            [CompletionResult]::new('--target-module', '--target-module', [CompletionResultType]::ParameterName, 'Target module path for helper function')
            [CompletionResult]::new('--custom-instructions', '--custom-instructions', [CompletionResultType]::ParameterName, 'Custom instructions or architectural constraints for the AI')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count for clone detection')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--verify', '--verify', [CompletionResultType]::ParameterName, 'Verify refactoring against test suite')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;cache' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export persistent cache database to a portable .cddmpack archive')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import a portable .cddmpack archive into persistent cache database')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;cache;export' {
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'Custom path to cache database (default: .cddm/cache.db)')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output pack archive file path (default: cddm-cache.cddmpack)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output pack archive file path (default: cddm-cache.cddmpack)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;cache;import' {
            [CompletionResult]::new('--target-dir', '--target-dir', [CompletionResultType]::ParameterName, 'Target cache directory to populate (default: .cddm)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;cache;help' {
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export persistent cache database to a portable .cddmpack archive')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import a portable .cddmpack archive into persistent cache database')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;cache;help;export' {
            break
        }
        'cddm;cache;help;import' {
            break
        }
        'cddm;cache;help;help' {
            break
        }
        'cddm;monorepo' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;tui' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--fail-threshold', '--fail-threshold', [CompletionResultType]::ParameterName, 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'Specific language(s) to scan')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--ignore', '--ignore', [CompletionResultType]::ParameterName, 'Glob patterns to ignore')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Enable live watch mode for real-time rescanning on file changes')
            [CompletionResult]::new('--watch', '--watch', [CompletionResultType]::ParameterName, 'Enable live watch mode for real-time rescanning on file changes')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;overlap' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Confidence threshold for library overlap detection (0.0 to 1.0)')
            [CompletionResult]::new('--threshold', '--threshold', [CompletionResultType]::ParameterName, 'Confidence threshold for library overlap detection (0.0 to 1.0)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format (console, json, markdown)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hub' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a new .cddmhub.toml configuration template')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan organization federation repositories for cross-repository duplication')
            [CompletionResult]::new('extract', 'extract', [CompletionResultType]::ParameterValue, 'Extract a cross-repository duplicate cluster into a standalone shared package')
            [CompletionResult]::new('sync', 'sync', [CompletionResultType]::ParameterValue, 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;hub;init' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Custom configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Custom configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Organization or hub name')
            [CompletionResult]::new('--name', '--name', [CompletionResultType]::ParameterName, 'Organization or hub name')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hub;scan' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format (console, json, markdown)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hub;extract' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Cluster index to extract')
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, 'Cluster index to extract')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Target package name (e.g. @org/shared-utils or cddm-shared-common)')
            [CompletionResult]::new('--pkg-name', '--pkg-name', [CompletionResultType]::ParameterName, 'Target package name (e.g. @org/shared-utils or cddm-shared-common)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Target package ecosystem (npm, cargo, pypi, go)')
            [CompletionResult]::new('--pkg-type', '--pkg-type', [CompletionResultType]::ParameterName, 'Target package ecosystem (npm, cargo, pypi, go)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Destination directory path for the new standalone package')
            [CompletionResult]::new('--target-dir', '--target-dir', [CompletionResultType]::ParameterName, 'Destination directory path for the new standalone package')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run preview without writing changes to disk')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hub;sync' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Custom configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Custom configuration file path (default: .cddmhub.toml)')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Remote peering endpoint URL (HTTPS/gRPC)')
            [CompletionResult]::new('--endpoint', '--endpoint', [CompletionResultType]::ParameterName, 'Remote peering endpoint URL (HTTPS/gRPC)')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Local repository name (defaults to directory name)')
            [CompletionResult]::new('--repo', '--repo', [CompletionResultType]::ParameterName, 'Local repository name (defaults to directory name)')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Cryptographic privacy salt (shared secret for organization hashing)')
            [CompletionResult]::new('--salt', '--salt', [CompletionResultType]::ParameterName, 'Cryptographic privacy salt (shared secret for organization hashing)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format (console, json)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format (console, json)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run preview without transmitting or saving cache packs')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;hub;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a new .cddmhub.toml configuration template')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan organization federation repositories for cross-repository duplication')
            [CompletionResult]::new('extract', 'extract', [CompletionResultType]::ParameterValue, 'Extract a cross-repository duplicate cluster into a standalone shared package')
            [CompletionResult]::new('sync', 'sync', [CompletionResultType]::ParameterValue, 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;hub;help;init' {
            break
        }
        'cddm;hub;help;scan' {
            break
        }
        'cddm;hub;help;extract' {
            break
        }
        'cddm;hub;help;sync' {
            break
        }
        'cddm;hub;help;help' {
            break
        }
        'cddm;coverage' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)')
            [CompletionResult]::new('--report', '--report', [CompletionResultType]::ParameterName, 'Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('--min-tokens', '--min-tokens', [CompletionResultType]::ParameterName, 'Minimum token count to consider as duplicate clone')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output report format (console, json, markdown)')
            [CompletionResult]::new('--min-hits', '--min-hits', [CompletionResultType]::ParameterName, 'Filter clones by minimum combined runtime execution hits')
            [CompletionResult]::new('--risk-threshold', '--risk-threshold', [CompletionResultType]::ParameterName, 'Filter clones exceeding this risk score threshold (0-100)')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('--dead-code-only', '--dead-code-only', [CompletionResultType]::ParameterName, 'Show only dead code duplicates (0 runtime executions across all sites)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;completions' {
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Write structured logs to a dedicated log file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debug logging output (-v for debug, -vv for trace)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress all non-error output and diagnostics')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'cddm;help' {
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan target directory for code duplication & DRY health score')
            [CompletionResult]::new('dead-code', 'dead-code', [CompletionResultType]::ParameterValue, 'Detect unreferenced functions, unreachable blocks, and dead code duplicates')
            [CompletionResult]::new('prune', 'prune', [CompletionResultType]::ParameterValue, 'Automatically prune unreachable dead clone clusters and unreferenced code')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'Differential duplication scan comparing current changes against a Git base revision')
            [CompletionResult]::new('semantic', 'semantic', [CompletionResultType]::ParameterValue, 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms')
            [CompletionResult]::new('refactor', 'refactor', [CompletionResultType]::ParameterValue, 'Synthesize automated refactoring suggestions for duplicate clone pairs')
            [CompletionResult]::new('extract', 'extract', [CompletionResultType]::ParameterValue, 'Extract duplicate code into a standalone shared crate or module')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Launch interactive WebUI dashboard in browser')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watch directory and trigger continuous real-time clone analysis on file save')
            [CompletionResult]::new('lsp', 'lsp', [CompletionResultType]::ParameterValue, 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles')
            [CompletionResult]::new('trend', 'trend', [CompletionResultType]::ParameterValue, 'Analyze historical duplication trends across Git commit history')
            [CompletionResult]::new('hook', 'hook', [CompletionResultType]::ParameterValue, 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement')
            [CompletionResult]::new('ignore', 'ignore', [CompletionResultType]::ParameterValue, 'Manage .cddmignore rules and test path suppression matching')
            [CompletionResult]::new('rules', 'rules', [CompletionResultType]::ParameterValue, 'Manage architectural policy rules (.cddmrules.toml)')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)')
            [CompletionResult]::new('comment', 'comment', [CompletionResultType]::ParameterValue, 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests')
            [CompletionResult]::new('heal', 'heal', [CompletionResultType]::ParameterValue, 'Autonomous AI Code Surgeon refactoring with closed-loop test healing')
            [CompletionResult]::new('cache', 'cache', [CompletionResultType]::ParameterValue, 'Manage persistent fingerprint cache and export/import .cddmpack archives')
            [CompletionResult]::new('monorepo', 'monorepo', [CompletionResultType]::ParameterValue, 'Discover and scan monorepos with multi-workspace packages')
            [CompletionResult]::new('tui', 'tui', [CompletionResultType]::ParameterValue, 'Launch interactive Terminal UI (TUI) Studio dashboard')
            [CompletionResult]::new('overlap', 'overlap', [CompletionResultType]::ParameterValue, 'Detect reimplemented ecosystem library algorithms and suggest standard packages')
            [CompletionResult]::new('hub', 'hub', [CompletionResultType]::ParameterValue, 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)')
            [CompletionResult]::new('coverage', 'coverage', [CompletionResultType]::ParameterValue, 'Dynamic runtime execution & coverage-aware de-duplication analysis')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cddm;help;scan' {
            break
        }
        'cddm;help;dead-code' {
            break
        }
        'cddm;help;prune' {
            break
        }
        'cddm;help;diff' {
            break
        }
        'cddm;help;semantic' {
            break
        }
        'cddm;help;refactor' {
            break
        }
        'cddm;help;extract' {
            break
        }
        'cddm;help;serve' {
            break
        }
        'cddm;help;watch' {
            break
        }
        'cddm;help;lsp' {
            break
        }
        'cddm;help;trend' {
            break
        }
        'cddm;help;hook' {
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install a Git hook enforcing code duplication thresholds')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Uninstall an existing CDDM Git hook')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Check current installation status of Git hooks')
            break
        }
        'cddm;help;hook;install' {
            break
        }
        'cddm;help;hook;uninstall' {
            break
        }
        'cddm;help;hook;status' {
            break
        }
        'cddm;help;ignore' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a standard, well-documented .cddmignore configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Test whether a specific file path or line number is ignored by suppression rules')
            break
        }
        'cddm;help;ignore;init' {
            break
        }
        'cddm;help;ignore;check' {
            break
        }
        'cddm;help;rules' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a starter .cddmrules.toml configuration template')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Evaluate architectural policy rules against codebase')
            break
        }
        'cddm;help;rules;init' {
            break
        }
        'cddm;help;rules;check' {
            break
        }
        'cddm;help;init' {
            break
        }
        'cddm;help;comment' {
            break
        }
        'cddm;help;heal' {
            break
        }
        'cddm;help;cache' {
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export persistent cache database to a portable .cddmpack archive')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Import a portable .cddmpack archive into persistent cache database')
            break
        }
        'cddm;help;cache;export' {
            break
        }
        'cddm;help;cache;import' {
            break
        }
        'cddm;help;monorepo' {
            break
        }
        'cddm;help;tui' {
            break
        }
        'cddm;help;overlap' {
            break
        }
        'cddm;help;hub' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize a new .cddmhub.toml configuration template')
            [CompletionResult]::new('scan', 'scan', [CompletionResultType]::ParameterValue, 'Scan organization federation repositories for cross-repository duplication')
            [CompletionResult]::new('extract', 'extract', [CompletionResultType]::ParameterValue, 'Extract a cross-repository duplicate cluster into a standalone shared package')
            [CompletionResult]::new('sync', 'sync', [CompletionResultType]::ParameterValue, 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers')
            break
        }
        'cddm;help;hub;init' {
            break
        }
        'cddm;help;hub;scan' {
            break
        }
        'cddm;help;hub;extract' {
            break
        }
        'cddm;help;hub;sync' {
            break
        }
        'cddm;help;coverage' {
            break
        }
        'cddm;help;completions' {
            break
        }
        'cddm;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
