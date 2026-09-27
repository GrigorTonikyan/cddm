# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_cddm_global_optspecs
    string join \n v/verbose q/quiet log-level= log-file= h/help V/version
end

function __fish_cddm_needs_command
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s (__fish_cddm_global_optspecs) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function __fish_cddm_using_subcommand
    set -l cmd (__fish_cddm_needs_command)
    test -z "$cmd"
    and return 1
    contains -- $cmd[1] $argv
end

complete -c cddm -n "__fish_cddm_needs_command" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_needs_command" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_needs_command" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_needs_command" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c cddm -n "__fish_cddm_needs_command" -s V -l version -d 'Print version'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "scan" -d 'Scan target directory for code duplication & DRY health score'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "dead-code" -d 'Detect unreferenced functions, unreachable blocks, and dead code duplicates'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "prune" -d 'Automatically prune unreachable dead clone clusters and unreferenced code'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "diff" -d 'Differential duplication scan comparing current changes against a Git base revision'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "semantic" -d 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "refactor" -d 'Synthesize automated refactoring suggestions for duplicate clone pairs'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "extract" -d 'Extract duplicate code into a standalone shared crate or module'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "serve" -d 'Launch interactive WebUI dashboard in browser'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "watch" -d 'Watch directory and trigger continuous real-time clone analysis on file save'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "lsp" -d 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "trend" -d 'Analyze historical duplication trends across Git commit history'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "hook" -d 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "ignore" -d 'Manage .cddmignore rules and test path suppression matching'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "rules" -d 'Manage architectural policy rules (.cddmrules.toml)'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "init" -d 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "comment" -d 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "heal" -d 'Autonomous AI Code Surgeon refactoring with closed-loop test healing'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "cache" -d 'Manage persistent fingerprint cache and export/import .cddmpack archives'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "monorepo" -d 'Discover and scan monorepos with multi-workspace packages'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "tui" -d 'Launch interactive Terminal UI (TUI) Studio dashboard'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "overlap" -d 'Detect reimplemented ecosystem library algorithms and suggest standard packages'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "hub" -d 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "coverage" -d 'Dynamic runtime execution & coverage-aware de-duplication analysis'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "completions" -d 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish'
complete -c cddm -n "__fish_cddm_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s f -l format -d 'Output report format (console, json, markdown, sarif)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l fail-threshold -d 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s l -l languages -d 'Specific language(s) to scan (e.g. Rust, TypeScript, Python)' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s i -l ignore -d 'Glob patterns to ignore (e.g. node_modules, target)' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l cache-dir -d 'Custom path for persistent redb cache database (default: OS user cache)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l cddmignore -d 'Custom path to .cddmignore configuration file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l ignore-tests -d 'Automatically filter test files and test directories (default: true)' -r -f -a "true\t''
false\t''"
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l ignore-mocks -d 'Automatically filter mock and fixture files (default: true)' -r -f -a "true\t''
false\t''"
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l rules -d 'Path to custom architectural policy rules (.cddmrules.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l cross-language -d 'Detect cross-language semantic clones across different programming languages' -r -f -a "true\t''
false\t''"
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l detect-type4 -d 'Detect Type-4 semantic clones using AST/CFG Weisfeiler-Lehman graph analysis' -r -f -a "true\t''
false\t''"
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s j -l threads -d 'Maximum number of parallel worker threads to utilize (default: all logical cores)' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l summary -d 'Output compact summary report to conserve terminal/token output'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l include-ignored -d 'Include files and directories ignored by .gitignore'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l git-blame -d 'Enable in-process git blame author & line age annotation'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l in-tree-cache -d 'Store persistent redb cache database in workspace (.cddm/cache.db) instead of OS user cache'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-cache -d 'Bypass persistent disk cache and force full re-scan'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l clear-cache -d 'Clear existing persistent cache database before scanning'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-ignore-tests -d 'Include test files and test directories in scan'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-ignore-mocks -d 'Include mock and fixture files in scan'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l ignore-generated -d 'Automatically filter auto-generated files with generator headers'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l enforce-policies -d 'Enforce architectural policy rules (exit code 1 on error-level violations)'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-cross-language -d 'Disable cross-language semantic clone detection'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-type3 -d 'Disable Type-3 near-miss modified statement clone detection'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -l no-type4 -d 'Disable Type-4 semantic AST/CFG clone detection'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand scan" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s m -l min-tokens -d 'Minimum token count threshold for dead code items' -r
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s f -l format -d 'Output report format (console, json, markdown, sarif)' -r
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s c -l coverage -d 'Path to optional coverage report file (e.g. lcov.info, coverage.xml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s l -l languages -d 'Filter by target programming languages' -r
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s i -l ignore -d 'Custom file or path ignore patterns' -r
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -l summary -d 'Output compact summary report to conserve terminal/token output'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -l static-only -d 'Restrict analysis to static AST & symbol analysis only'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -l include-ignored -d 'Include files and directories ignored by .gitignore'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand dead-code" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s t -l threshold -d 'Confidence threshold for safe removal (0.0 to 1.0)' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s m -l min-tokens -d 'Minimum token count threshold for dead clone items' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s f -l format -d 'Output report format (console, json, markdown, sarif)' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s l -l languages -d 'Filter by target programming languages' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s i -l ignore -d 'Custom file or path ignore patterns' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand prune" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand prune" -l dry-run -d 'Dry run preview without modifying files on disk'
complete -c cddm -n "__fish_cddm_using_subcommand prune" -l safe-only -d 'Only prune dead clones meeting strict closed-loop safety verification (default: true)'
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand prune" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s d -l directory -d 'Directory path of the Git repository to scan (default: current directory)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s f -l format -d 'Output report format (console, json, markdown, sarif)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l fail-threshold -d 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s l -l languages -d 'Specific language(s) to scan' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s i -l ignore -d 'Glob patterns to ignore' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l cache-dir -d 'Custom path for persistent redb cache database (default: .cddm/cache.db)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l cddmignore -d 'Custom path to .cddmignore configuration file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l rules -d 'Path to custom architectural policy rules (.cddmrules.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l matrix -d 'Multi-branch clone drift matrix comparison across multiple revisions (comma-separated list)' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l git-blame -d 'Enable in-process git blame author & line age annotation'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l no-cache -d 'Bypass persistent disk cache and force full re-scan'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l include-ignored -d 'Include files and directories ignored by .gitignore'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l ignore-tests -d 'Automatically filter test files and test directories'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l ignore-mocks -d 'Automatically filter mock and fixture files'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l ignore-generated -d 'Automatically filter auto-generated files with generator headers'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l enforce-policies -d 'Enforce architectural policy rules (exit code 1 on error-level violations)'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -l cross-language -d 'Detect cross-language semantic clones across different programming languages'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand diff" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s t -l threshold -d 'Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s f -l format -d 'Output report format (console, json, markdown)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s l -l languages -d 'Specific language(s) to scan' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s i -l ignore -d 'Glob patterns to ignore' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l neural-threshold -d 'Minimum cosine similarity threshold for neural matching (default: 0.85)' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s j -l threads -d 'Maximum number of parallel worker threads to utilize (default: all logical cores)' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l neural -d 'Enable in-process dense neural code embedding equivalence scan'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l hnsw -d 'Use Hierarchical Navigable Small World (HNSW) index for sub-linear logarithmic search'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -l sq8 -d 'Use 8-bit scalar quantization (SQ8) for 4x vector memory reduction during search'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand semantic" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s p -l pair -d '1-based index of clone pair to refactor' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s c -l cluster -d '1-based index of clone cluster to refactor' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s o -l output -d 'Output file path to write patch to (default: stdout)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l fn-name -d 'Custom name for extracted function' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l target-module -d 'Target module path for extracted helper' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l apply-branch -d 'Apply refactoring to dedicated Git branch' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l test-cmd -d 'Custom test command for verification' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s l -l languages -d 'Specific language(s) to scan' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s i -l ignore -d 'Glob patterns to ignore' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l provider -d 'AI provider to use for streaming refactoring (gemini, claude, openai, ollama, custom, mock)' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l model -d 'AI model identifier to use' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l prompt -d 'Generate formatted markdown prompt for AI refactoring agents'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l ast -d 'Generate Tree-sitter AST-native code transformations'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l verify -d 'Verify refactoring against test suite'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -l stream -d 'Stream AI token diff generation directly to terminal'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand refactor" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s p -l pair -d '1-based index of clone pair to extract' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s c -l cluster -d '1-based index of clone cluster to extract' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s t -l target -d 'Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l fn-name -d 'Custom extracted helper function name' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l crate-type -d 'Packaging strategy: auto, crate, module, existing' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s m -l min-tokens -d 'Minimum token count for clone detection' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l dry-run -d 'Perform a dry-run preview without modifying files'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l apply -d 'Commit and apply generated files, manifest updates, and caller rewrites'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l generate-tests -d 'Automatically synthesize unit tests for the extracted helper'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -l generate-benchmarks -l bench -d 'Automatically synthesize performance micro-benchmarks for the extracted helper'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand extract" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand serve" -s p -l port -d 'Port to bind the WebUI HTTP and WebSocket server to' -r
complete -c cddm -n "__fish_cddm_using_subcommand serve" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand serve" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand serve" -s o -l open -d 'Automatically open WebUI in default web browser'
complete -c cddm -n "__fish_cddm_using_subcommand serve" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand serve" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand serve" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s l -l languages -d 'Specific language(s) to scan' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s i -l ignore -d 'Glob patterns to ignore' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l cache-dir -d 'Custom path for persistent redb cache database (default: .cddm/cache.db)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s d -l debounce-ms -d 'Debounce delay in milliseconds before scanning on file changes' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l fail-threshold -d 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s s -l serve -d 'Optionally start embedded WebUI Studio server on specified port (default: 3000)' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s f -l format -d 'Output report format (console, json, markdown, ndjson)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l git-blame -d 'Enable in-process git blame author & line age annotation'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l no-cache -d 'Bypass persistent disk cache and force full re-scan'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s o -l open -d 'Automatically open WebUI in browser when --serve is enabled'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -l cross-language -d 'Detect cross-language semantic clones across different programming languages'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand watch" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -s m -l min-tokens -d 'Minimum token count for clone detection' -r
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand lsp" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s s -l max-samples -d 'Maximum number of historical commit snapshots to sample (default: 10)' -r
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s f -l format -d 'Output report format (console, json, markdown)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand trend" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand trend" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand trend" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -f -a "install" -d 'Install a Git hook enforcing code duplication thresholds'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -f -a "uninstall" -d 'Uninstall an existing CDDM Git hook'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -f -a "status" -d 'Check current installation status of Git hooks'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and not __fish_seen_subcommand_from install uninstall status help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -s t -l hook-type -d 'Hook type to install (pre-commit or pre-push)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -l fail-threshold -d 'Duplication percentage threshold to fail on (default: 15.0)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -s m -l min-tokens -d 'Minimum token count for clone detection (default: 50)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from install" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -s t -l hook-type -d 'Hook type to remove (pre-commit or pre-push)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from uninstall" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from status" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from status" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from status" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from status" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from status" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from help" -f -a "install" -d 'Install a Git hook enforcing code duplication thresholds'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from help" -f -a "uninstall" -d 'Uninstall an existing CDDM Git hook'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from help" -f -a "status" -d 'Check current installation status of Git hooks'
complete -c cddm -n "__fish_cddm_using_subcommand hook; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -f -a "init" -d 'Initialize a standard, well-documented .cddmignore configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -f -a "check" -d 'Test whether a specific file path or line number is ignored by suppression rules'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and not __fish_seen_subcommand_from init check help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -s f -l force -d 'Overwrite existing .cddmignore file if present'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -s l -l line -d 'Optional 1-based line number to check for inline suppression directives' -r
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l cddmignore -d 'Path to custom .cddmignore file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l ignore-tests -d 'Check with test file suppression enabled'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l ignore-mocks -d 'Check with mock file suppression enabled'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -l ignore-generated -d 'Check with generated file suppression enabled'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from help" -f -a "init" -d 'Initialize a standard, well-documented .cddmignore configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from help" -f -a "check" -d 'Test whether a specific file path or line number is ignored by suppression rules'
complete -c cddm -n "__fish_cddm_using_subcommand ignore; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -f -a "init" -d 'Initialize a starter .cddmrules.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -f -a "check" -d 'Evaluate architectural policy rules against codebase'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and not __fish_seen_subcommand_from init check help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -s o -l output -d 'Target output file path (default: .cddmrules.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -s f -l force -d 'Overwrite existing file if present'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -l write -d 'Write directly to disk (default: true)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s r -l rules -d 'Custom path to .cddmrules.toml file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s m -l min-tokens -d 'Minimum token count for clone detection' -r
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s f -l format -d 'Output report format (console, json, markdown)' -r -f -a "console\t''
json\t''
markdown\t''
sarif\t''
ndjson\t''"
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -l enforce-policies -d 'Exit with non-zero code if any policy violations exist'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -l summary -d 'Output compact summary report to conserve terminal/token output'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from help" -f -a "init" -d 'Initialize a starter .cddmrules.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from help" -f -a "check" -d 'Evaluate architectural policy rules against codebase'
complete -c cddm -n "__fish_cddm_using_subcommand rules; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand init" -l fail-threshold -d 'Duplication percentage threshold to fail on (default: 15.0)' -r
complete -c cddm -n "__fish_cddm_using_subcommand init" -s m -l min-tokens -d 'Minimum token count for clone detection (default: 50)' -r
complete -c cddm -n "__fish_cddm_using_subcommand init" -s o -l output -d 'Output file path (defaults to standard platform config file)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand init" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand init" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand init" -s w -l write -d 'Write directly to disk (default: print to stdout unless --write or output specified)'
complete -c cddm -n "__fish_cddm_using_subcommand init" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand init" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand init" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand comment" -l fail-threshold -d 'Duplication percentage threshold to fail on (default: 15.0)' -r
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s p -l platform -d 'Target CI/CD platform format: github, gitlab, or azure' -r -f -a "gitea\t''
github\t''
gitlab\t''
azure\t''"
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s o -l output -d 'Output file path to write Markdown comment to (default: stdout)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand comment" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand comment" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand comment" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s c -l cluster -d 'Target clone cluster index to heal' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s p -l pair -d 'Target clone pair index to heal' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l provider -d 'AI Provider backend (gemini, claude, openai, ollama, mock)' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l model -d 'Model identifier name (e.g. gemini-3.8-pro, claude-3-7-sonnet, gpt-4o, qwen2.5-coder)' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l api-key -d 'Secret API key for authentication' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l endpoint -d 'Custom endpoint URL (e.g. http://localhost:11434 for Ollama)' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s i -l max-iterations -d 'Maximum healing repair iterations' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l test-cmd -d 'Custom test command (e.g. "cargo test", "bun test")' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l branch -d 'Apply passing refactoring to dedicated Git branch' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l fn-name -d 'Custom extracted function name' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l target-module -d 'Target module path for helper function' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l custom-instructions -d 'Custom instructions or architectural constraints for the AI' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s m -l min-tokens -d 'Minimum token count for clone detection' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand heal" -l verify -d 'Verify refactoring against test suite'
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand heal" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -f -a "export" -d 'Export persistent cache database to a portable .cddmpack archive'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -f -a "import" -d 'Import a portable .cddmpack archive into persistent cache database'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and not __fish_seen_subcommand_from export import help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -l cache-dir -d 'Custom path to cache database (default: .cddm/cache.db)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -s o -l output -d 'Output pack archive file path (default: cddm-cache.cddmpack)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from export" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -l target-dir -d 'Target cache directory to populate (default: .cddm)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from import" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "export" -d 'Export persistent cache database to a portable .cddmpack archive'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "import" -d 'Import a portable .cddmpack archive into persistent cache database'
complete -c cddm -n "__fish_cddm_using_subcommand cache; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand monorepo" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand tui" -l fail-threshold -d 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)' -r
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s l -l languages -d 'Specific language(s) to scan' -r
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s i -l ignore -d 'Glob patterns to ignore' -r
complete -c cddm -n "__fish_cddm_using_subcommand tui" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand tui" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s w -l watch -d 'Enable live watch mode for real-time rescanning on file changes'
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand tui" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -s t -l threshold -d 'Confidence threshold for library overlap detection (0.0 to 1.0)' -r
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -s f -l format -d 'Output format (console, json, markdown)' -r
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand overlap" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -f -a "init" -d 'Initialize a new .cddmhub.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -f -a "scan" -d 'Scan organization federation repositories for cross-repository duplication'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -f -a "extract" -d 'Extract a cross-repository duplicate cluster into a standalone shared package'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -f -a "sync" -d 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and not __fish_seen_subcommand_from init scan extract sync help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -s c -l config -d 'Custom configuration file path (default: .cddmhub.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -s n -l name -d 'Organization or hub name' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -s f -l format -d 'Output format (console, json, markdown)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -s m -l min-tokens -d 'Minimum token count' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from scan" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s C -l config -d 'Configuration file path (default: .cddmhub.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s c -l cluster -d 'Cluster index to extract' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s n -l pkg-name -d 'Target package name (e.g. @org/shared-utils or cddm-shared-common)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s t -l pkg-type -d 'Target package ecosystem (npm, cargo, pypi, go)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s d -l target-dir -d 'Destination directory path for the new standalone package' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -l dry-run -d 'Dry run preview without writing changes to disk'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from extract" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s c -l config -d 'Custom configuration file path (default: .cddmhub.toml)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s e -l endpoint -d 'Remote peering endpoint URL (HTTPS/gRPC)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s r -l repo -d 'Local repository name (defaults to directory name)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s s -l salt -d 'Cryptographic privacy salt (shared secret for organization hashing)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s f -l format -d 'Output format (console, json)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -l dry-run -d 'Dry run preview without transmitting or saving cache packs'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from sync" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from help" -f -a "init" -d 'Initialize a new .cddmhub.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from help" -f -a "scan" -d 'Scan organization federation repositories for cross-repository duplication'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from help" -f -a "extract" -d 'Extract a cross-repository duplicate cluster into a standalone shared package'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from help" -f -a "sync" -d 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
complete -c cddm -n "__fish_cddm_using_subcommand hub; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s r -l report -d 'Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s m -l min-tokens -d 'Minimum token count to consider as duplicate clone' -r
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s f -l format -d 'Output report format (console, json, markdown)' -r
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -l min-hits -d 'Filter clones by minimum combined runtime execution hits' -r
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -l risk-threshold -d 'Filter clones exceeding this risk score threshold (0-100)' -r
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -l dead-code-only -d 'Show only dead code duplicates (0 runtime executions across all sites)'
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand coverage" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand completions" -l log-level -d 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)' -r
complete -c cddm -n "__fish_cddm_using_subcommand completions" -l log-file -d 'Write structured logs to a dedicated log file' -r -F
complete -c cddm -n "__fish_cddm_using_subcommand completions" -s v -l verbose -d 'Enable verbose debug logging output (-v for debug, -vv for trace)'
complete -c cddm -n "__fish_cddm_using_subcommand completions" -s q -l quiet -d 'Suppress all non-error output and diagnostics'
complete -c cddm -n "__fish_cddm_using_subcommand completions" -s h -l help -d 'Print help'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "scan" -d 'Scan target directory for code duplication & DRY health score'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "dead-code" -d 'Detect unreferenced functions, unreachable blocks, and dead code duplicates'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "prune" -d 'Automatically prune unreachable dead clone clusters and unreferenced code'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "diff" -d 'Differential duplication scan comparing current changes against a Git base revision'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "semantic" -d 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "refactor" -d 'Synthesize automated refactoring suggestions for duplicate clone pairs'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "extract" -d 'Extract duplicate code into a standalone shared crate or module'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "serve" -d 'Launch interactive WebUI dashboard in browser'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "watch" -d 'Watch directory and trigger continuous real-time clone analysis on file save'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "lsp" -d 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "trend" -d 'Analyze historical duplication trends across Git commit history'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "hook" -d 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "ignore" -d 'Manage .cddmignore rules and test path suppression matching'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "rules" -d 'Manage architectural policy rules (.cddmrules.toml)'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "init" -d 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "comment" -d 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "heal" -d 'Autonomous AI Code Surgeon refactoring with closed-loop test healing'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "cache" -d 'Manage persistent fingerprint cache and export/import .cddmpack archives'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "monorepo" -d 'Discover and scan monorepos with multi-workspace packages'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "tui" -d 'Launch interactive Terminal UI (TUI) Studio dashboard'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "overlap" -d 'Detect reimplemented ecosystem library algorithms and suggest standard packages'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "hub" -d 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "coverage" -d 'Dynamic runtime execution & coverage-aware de-duplication analysis'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "completions" -d 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish'
complete -c cddm -n "__fish_cddm_using_subcommand help; and not __fish_seen_subcommand_from scan dead-code prune diff semantic refactor extract serve watch lsp trend hook ignore rules init comment heal cache monorepo tui overlap hub coverage completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hook" -f -a "install" -d 'Install a Git hook enforcing code duplication thresholds'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hook" -f -a "uninstall" -d 'Uninstall an existing CDDM Git hook'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hook" -f -a "status" -d 'Check current installation status of Git hooks'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from ignore" -f -a "init" -d 'Initialize a standard, well-documented .cddmignore configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from ignore" -f -a "check" -d 'Test whether a specific file path or line number is ignored by suppression rules'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from rules" -f -a "init" -d 'Initialize a starter .cddmrules.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from rules" -f -a "check" -d 'Evaluate architectural policy rules against codebase'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from cache" -f -a "export" -d 'Export persistent cache database to a portable .cddmpack archive'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from cache" -f -a "import" -d 'Import a portable .cddmpack archive into persistent cache database'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hub" -f -a "init" -d 'Initialize a new .cddmhub.toml configuration template'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hub" -f -a "scan" -d 'Scan organization federation repositories for cross-repository duplication'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hub" -f -a "extract" -d 'Extract a cross-repository duplicate cluster into a standalone shared package'
complete -c cddm -n "__fish_cddm_using_subcommand help; and __fish_seen_subcommand_from hub" -f -a "sync" -d 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
