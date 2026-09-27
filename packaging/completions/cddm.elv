
use builtin;
use str;

set edit:completion:arg-completer[cddm] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'cddm'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'cddm'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand scan 'Scan target directory for code duplication & DRY health score'
            cand dead-code 'Detect unreferenced functions, unreachable blocks, and dead code duplicates'
            cand prune 'Automatically prune unreachable dead clone clusters and unreferenced code'
            cand diff 'Differential duplication scan comparing current changes against a Git base revision'
            cand semantic 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms'
            cand refactor 'Synthesize automated refactoring suggestions for duplicate clone pairs'
            cand extract 'Extract duplicate code into a standalone shared crate or module'
            cand serve 'Launch interactive WebUI dashboard in browser'
            cand watch 'Watch directory and trigger continuous real-time clone analysis on file save'
            cand lsp 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles'
            cand trend 'Analyze historical duplication trends across Git commit history'
            cand hook 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement'
            cand ignore 'Manage .cddmignore rules and test path suppression matching'
            cand rules 'Manage architectural policy rules (.cddmrules.toml)'
            cand init 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)'
            cand comment 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests'
            cand heal 'Autonomous AI Code Surgeon refactoring with closed-loop test healing'
            cand cache 'Manage persistent fingerprint cache and export/import .cddmpack archives'
            cand monorepo 'Discover and scan monorepos with multi-workspace packages'
            cand tui 'Launch interactive Terminal UI (TUI) Studio dashboard'
            cand overlap 'Detect reimplemented ecosystem library algorithms and suggest standard packages'
            cand hub 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)'
            cand coverage 'Dynamic runtime execution & coverage-aware de-duplication analysis'
            cand completions 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;scan'= {
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -f 'Output report format (console, json, markdown, sarif)'
            cand --format 'Output report format (console, json, markdown, sarif)'
            cand --fail-threshold 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)'
            cand -l 'Specific language(s) to scan (e.g. Rust, TypeScript, Python)'
            cand --languages 'Specific language(s) to scan (e.g. Rust, TypeScript, Python)'
            cand -i 'Glob patterns to ignore (e.g. node_modules, target)'
            cand --ignore 'Glob patterns to ignore (e.g. node_modules, target)'
            cand --cache-dir 'Custom path for persistent redb cache database (default: OS user cache)'
            cand --cddmignore 'Custom path to .cddmignore configuration file'
            cand --ignore-tests 'Automatically filter test files and test directories (default: true)'
            cand --ignore-mocks 'Automatically filter mock and fixture files (default: true)'
            cand --rules 'Path to custom architectural policy rules (.cddmrules.toml)'
            cand --cross-language 'Detect cross-language semantic clones across different programming languages'
            cand --detect-type4 'Detect Type-4 semantic clones using AST/CFG Weisfeiler-Lehman graph analysis'
            cand -j 'Maximum number of parallel worker threads to utilize (default: all logical cores)'
            cand --threads 'Maximum number of parallel worker threads to utilize (default: all logical cores)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --summary 'Output compact summary report to conserve terminal/token output'
            cand --include-ignored 'Include files and directories ignored by .gitignore'
            cand --git-blame 'Enable in-process git blame author & line age annotation'
            cand --in-tree-cache 'Store persistent redb cache database in workspace (.cddm/cache.db) instead of OS user cache'
            cand --no-cache 'Bypass persistent disk cache and force full re-scan'
            cand --clear-cache 'Clear existing persistent cache database before scanning'
            cand --no-ignore-tests 'Include test files and test directories in scan'
            cand --no-ignore-mocks 'Include mock and fixture files in scan'
            cand --ignore-generated 'Automatically filter auto-generated files with generator headers'
            cand --enforce-policies 'Enforce architectural policy rules (exit code 1 on error-level violations)'
            cand --no-cross-language 'Disable cross-language semantic clone detection'
            cand --no-type3 'Disable Type-3 near-miss modified statement clone detection'
            cand --no-type4 'Disable Type-4 semantic AST/CFG clone detection'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;dead-code'= {
            cand -m 'Minimum token count threshold for dead code items'
            cand --min-tokens 'Minimum token count threshold for dead code items'
            cand -f 'Output report format (console, json, markdown, sarif)'
            cand --format 'Output report format (console, json, markdown, sarif)'
            cand -c 'Path to optional coverage report file (e.g. lcov.info, coverage.xml)'
            cand --coverage 'Path to optional coverage report file (e.g. lcov.info, coverage.xml)'
            cand -l 'Filter by target programming languages'
            cand --languages 'Filter by target programming languages'
            cand -i 'Custom file or path ignore patterns'
            cand --ignore 'Custom file or path ignore patterns'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --summary 'Output compact summary report to conserve terminal/token output'
            cand --static-only 'Restrict analysis to static AST & symbol analysis only'
            cand --include-ignored 'Include files and directories ignored by .gitignore'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;prune'= {
            cand -t 'Confidence threshold for safe removal (0.0 to 1.0)'
            cand --threshold 'Confidence threshold for safe removal (0.0 to 1.0)'
            cand -m 'Minimum token count threshold for dead clone items'
            cand --min-tokens 'Minimum token count threshold for dead clone items'
            cand -f 'Output report format (console, json, markdown, sarif)'
            cand --format 'Output report format (console, json, markdown, sarif)'
            cand -l 'Filter by target programming languages'
            cand --languages 'Filter by target programming languages'
            cand -i 'Custom file or path ignore patterns'
            cand --ignore 'Custom file or path ignore patterns'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --dry-run 'Dry run preview without modifying files on disk'
            cand --safe-only 'Only prune dead clones meeting strict closed-loop safety verification (default: true)'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;diff'= {
            cand -d 'Directory path of the Git repository to scan (default: current directory)'
            cand --directory 'Directory path of the Git repository to scan (default: current directory)'
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -f 'Output report format (console, json, markdown, sarif)'
            cand --format 'Output report format (console, json, markdown, sarif)'
            cand --fail-threshold 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)'
            cand -l 'Specific language(s) to scan'
            cand --languages 'Specific language(s) to scan'
            cand -i 'Glob patterns to ignore'
            cand --ignore 'Glob patterns to ignore'
            cand --cache-dir 'Custom path for persistent redb cache database (default: .cddm/cache.db)'
            cand --cddmignore 'Custom path to .cddmignore configuration file'
            cand --rules 'Path to custom architectural policy rules (.cddmrules.toml)'
            cand --matrix 'Multi-branch clone drift matrix comparison across multiple revisions (comma-separated list)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --git-blame 'Enable in-process git blame author & line age annotation'
            cand --no-cache 'Bypass persistent disk cache and force full re-scan'
            cand --include-ignored 'Include files and directories ignored by .gitignore'
            cand --ignore-tests 'Automatically filter test files and test directories'
            cand --ignore-mocks 'Automatically filter mock and fixture files'
            cand --ignore-generated 'Automatically filter auto-generated files with generator headers'
            cand --enforce-policies 'Enforce architectural policy rules (exit code 1 on error-level violations)'
            cand --cross-language 'Detect cross-language semantic clones across different programming languages'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;semantic'= {
            cand -t 'Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)'
            cand --threshold 'Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)'
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -f 'Output report format (console, json, markdown)'
            cand --format 'Output report format (console, json, markdown)'
            cand -l 'Specific language(s) to scan'
            cand --languages 'Specific language(s) to scan'
            cand -i 'Glob patterns to ignore'
            cand --ignore 'Glob patterns to ignore'
            cand --neural-threshold 'Minimum cosine similarity threshold for neural matching (default: 0.85)'
            cand -j 'Maximum number of parallel worker threads to utilize (default: all logical cores)'
            cand --threads 'Maximum number of parallel worker threads to utilize (default: all logical cores)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --neural 'Enable in-process dense neural code embedding equivalence scan'
            cand --hnsw 'Use Hierarchical Navigable Small World (HNSW) index for sub-linear logarithmic search'
            cand --sq8 'Use 8-bit scalar quantization (SQ8) for 4x vector memory reduction during search'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;refactor'= {
            cand -p '1-based index of clone pair to refactor'
            cand --pair '1-based index of clone pair to refactor'
            cand -c '1-based index of clone cluster to refactor'
            cand --cluster '1-based index of clone cluster to refactor'
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -o 'Output file path to write patch to (default: stdout)'
            cand --output 'Output file path to write patch to (default: stdout)'
            cand --fn-name 'Custom name for extracted function'
            cand --target-module 'Target module path for extracted helper'
            cand --apply-branch 'Apply refactoring to dedicated Git branch'
            cand --test-cmd 'Custom test command for verification'
            cand -l 'Specific language(s) to scan'
            cand --languages 'Specific language(s) to scan'
            cand -i 'Glob patterns to ignore'
            cand --ignore 'Glob patterns to ignore'
            cand --provider 'AI provider to use for streaming refactoring (gemini, claude, openai, ollama, custom, mock)'
            cand --model 'AI model identifier to use'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --prompt 'Generate formatted markdown prompt for AI refactoring agents'
            cand --ast 'Generate Tree-sitter AST-native code transformations'
            cand --verify 'Verify refactoring against test suite'
            cand --stream 'Stream AI token diff generation directly to terminal'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;extract'= {
            cand -p '1-based index of clone pair to extract'
            cand --pair '1-based index of clone pair to extract'
            cand -c '1-based index of clone cluster to extract'
            cand --cluster '1-based index of clone cluster to extract'
            cand -t 'Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)'
            cand --target 'Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)'
            cand --fn-name 'Custom extracted helper function name'
            cand --crate-type 'Packaging strategy: auto, crate, module, existing'
            cand -m 'Minimum token count for clone detection'
            cand --min-tokens 'Minimum token count for clone detection'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --dry-run 'Perform a dry-run preview without modifying files'
            cand --apply 'Commit and apply generated files, manifest updates, and caller rewrites'
            cand --generate-tests 'Automatically synthesize unit tests for the extracted helper'
            cand --generate-benchmarks 'Automatically synthesize performance micro-benchmarks for the extracted helper'
            cand --bench 'Automatically synthesize performance micro-benchmarks for the extracted helper'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;serve'= {
            cand -p 'Port to bind the WebUI HTTP and WebSocket server to'
            cand --port 'Port to bind the WebUI HTTP and WebSocket server to'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -o 'Automatically open WebUI in default web browser'
            cand --open 'Automatically open WebUI in default web browser'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;watch'= {
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -l 'Specific language(s) to scan'
            cand --languages 'Specific language(s) to scan'
            cand -i 'Glob patterns to ignore'
            cand --ignore 'Glob patterns to ignore'
            cand --cache-dir 'Custom path for persistent redb cache database (default: .cddm/cache.db)'
            cand -d 'Debounce delay in milliseconds before scanning on file changes'
            cand --debounce-ms 'Debounce delay in milliseconds before scanning on file changes'
            cand --fail-threshold 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)'
            cand -s 'Optionally start embedded WebUI Studio server on specified port (default: 3000)'
            cand --serve 'Optionally start embedded WebUI Studio server on specified port (default: 3000)'
            cand -f 'Output report format (console, json, markdown, ndjson)'
            cand --format 'Output report format (console, json, markdown, ndjson)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --git-blame 'Enable in-process git blame author & line age annotation'
            cand --no-cache 'Bypass persistent disk cache and force full re-scan'
            cand -o 'Automatically open WebUI in browser when --serve is enabled'
            cand --open 'Automatically open WebUI in browser when --serve is enabled'
            cand --cross-language 'Detect cross-language semantic clones across different programming languages'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;lsp'= {
            cand -m 'Minimum token count for clone detection'
            cand --min-tokens 'Minimum token count for clone detection'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;trend'= {
            cand -s 'Maximum number of historical commit snapshots to sample (default: 10)'
            cand --max-samples 'Maximum number of historical commit snapshots to sample (default: 10)'
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -f 'Output report format (console, json, markdown)'
            cand --format 'Output report format (console, json, markdown)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hook'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
            cand install 'Install a Git hook enforcing code duplication thresholds'
            cand uninstall 'Uninstall an existing CDDM Git hook'
            cand status 'Check current installation status of Git hooks'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;hook;install'= {
            cand -t 'Hook type to install (pre-commit or pre-push)'
            cand --hook-type 'Hook type to install (pre-commit or pre-push)'
            cand --fail-threshold 'Duplication percentage threshold to fail on (default: 15.0)'
            cand -m 'Minimum token count for clone detection (default: 50)'
            cand --min-tokens 'Minimum token count for clone detection (default: 50)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hook;uninstall'= {
            cand -t 'Hook type to remove (pre-commit or pre-push)'
            cand --hook-type 'Hook type to remove (pre-commit or pre-push)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hook;status'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hook;help'= {
            cand install 'Install a Git hook enforcing code duplication thresholds'
            cand uninstall 'Uninstall an existing CDDM Git hook'
            cand status 'Check current installation status of Git hooks'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;hook;help;install'= {
        }
        &'cddm;hook;help;uninstall'= {
        }
        &'cddm;hook;help;status'= {
        }
        &'cddm;hook;help;help'= {
        }
        &'cddm;ignore'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
            cand init 'Initialize a standard, well-documented .cddmignore configuration template'
            cand check 'Test whether a specific file path or line number is ignored by suppression rules'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;ignore;init'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -f 'Overwrite existing .cddmignore file if present'
            cand --force 'Overwrite existing .cddmignore file if present'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;ignore;check'= {
            cand -l 'Optional 1-based line number to check for inline suppression directives'
            cand --line 'Optional 1-based line number to check for inline suppression directives'
            cand --cddmignore 'Path to custom .cddmignore file'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --ignore-tests 'Check with test file suppression enabled'
            cand --ignore-mocks 'Check with mock file suppression enabled'
            cand --ignore-generated 'Check with generated file suppression enabled'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;ignore;help'= {
            cand init 'Initialize a standard, well-documented .cddmignore configuration template'
            cand check 'Test whether a specific file path or line number is ignored by suppression rules'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;ignore;help;init'= {
        }
        &'cddm;ignore;help;check'= {
        }
        &'cddm;ignore;help;help'= {
        }
        &'cddm;rules'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
            cand init 'Initialize a starter .cddmrules.toml configuration template'
            cand check 'Evaluate architectural policy rules against codebase'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;rules;init'= {
            cand -o 'Target output file path (default: .cddmrules.toml)'
            cand --output 'Target output file path (default: .cddmrules.toml)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -f 'Overwrite existing file if present'
            cand --force 'Overwrite existing file if present'
            cand --write 'Write directly to disk (default: true)'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;rules;check'= {
            cand -r 'Custom path to .cddmrules.toml file'
            cand --rules 'Custom path to .cddmrules.toml file'
            cand -m 'Minimum token count for clone detection'
            cand --min-tokens 'Minimum token count for clone detection'
            cand -f 'Output report format (console, json, markdown)'
            cand --format 'Output report format (console, json, markdown)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --enforce-policies 'Exit with non-zero code if any policy violations exist'
            cand --summary 'Output compact summary report to conserve terminal/token output'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;rules;help'= {
            cand init 'Initialize a starter .cddmrules.toml configuration template'
            cand check 'Evaluate architectural policy rules against codebase'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;rules;help;init'= {
        }
        &'cddm;rules;help;check'= {
        }
        &'cddm;rules;help;help'= {
        }
        &'cddm;init'= {
            cand --fail-threshold 'Duplication percentage threshold to fail on (default: 15.0)'
            cand -m 'Minimum token count for clone detection (default: 50)'
            cand --min-tokens 'Minimum token count for clone detection (default: 50)'
            cand -o 'Output file path (defaults to standard platform config file)'
            cand --output 'Output file path (defaults to standard platform config file)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -w 'Write directly to disk (default: print to stdout unless --write or output specified)'
            cand --write 'Write directly to disk (default: print to stdout unless --write or output specified)'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;comment'= {
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand --fail-threshold 'Duplication percentage threshold to fail on (default: 15.0)'
            cand -p 'Target CI/CD platform format: github, gitlab, or azure'
            cand --platform 'Target CI/CD platform format: github, gitlab, or azure'
            cand -o 'Output file path to write Markdown comment to (default: stdout)'
            cand --output 'Output file path to write Markdown comment to (default: stdout)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;heal'= {
            cand -c 'Target clone cluster index to heal'
            cand --cluster 'Target clone cluster index to heal'
            cand -p 'Target clone pair index to heal'
            cand --pair 'Target clone pair index to heal'
            cand --provider 'AI Provider backend (gemini, claude, openai, ollama, mock)'
            cand --model 'Model identifier name (e.g. gemini-3.8-pro, claude-3-7-sonnet, gpt-4o, qwen2.5-coder)'
            cand --api-key 'Secret API key for authentication'
            cand --endpoint 'Custom endpoint URL (e.g. http://localhost:11434 for Ollama)'
            cand -i 'Maximum healing repair iterations'
            cand --max-iterations 'Maximum healing repair iterations'
            cand --test-cmd 'Custom test command (e.g. "cargo test", "bun test")'
            cand --branch 'Apply passing refactoring to dedicated Git branch'
            cand --fn-name 'Custom extracted function name'
            cand --target-module 'Target module path for helper function'
            cand --custom-instructions 'Custom instructions or architectural constraints for the AI'
            cand -m 'Minimum token count for clone detection'
            cand --min-tokens 'Minimum token count for clone detection'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --verify 'Verify refactoring against test suite'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;cache'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
            cand export 'Export persistent cache database to a portable .cddmpack archive'
            cand import 'Import a portable .cddmpack archive into persistent cache database'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;cache;export'= {
            cand --cache-dir 'Custom path to cache database (default: .cddm/cache.db)'
            cand -o 'Output pack archive file path (default: cddm-cache.cddmpack)'
            cand --output 'Output pack archive file path (default: cddm-cache.cddmpack)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;cache;import'= {
            cand --target-dir 'Target cache directory to populate (default: .cddm)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;cache;help'= {
            cand export 'Export persistent cache database to a portable .cddmpack archive'
            cand import 'Import a portable .cddmpack archive into persistent cache database'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;cache;help;export'= {
        }
        &'cddm;cache;help;import'= {
        }
        &'cddm;cache;help;help'= {
        }
        &'cddm;monorepo'= {
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;tui'= {
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand --fail-threshold 'Exit with non-zero status code if duplication percentage exceeds threshold (0-100)'
            cand -l 'Specific language(s) to scan'
            cand --languages 'Specific language(s) to scan'
            cand -i 'Glob patterns to ignore'
            cand --ignore 'Glob patterns to ignore'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -w 'Enable live watch mode for real-time rescanning on file changes'
            cand --watch 'Enable live watch mode for real-time rescanning on file changes'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;overlap'= {
            cand -t 'Confidence threshold for library overlap detection (0.0 to 1.0)'
            cand --threshold 'Confidence threshold for library overlap detection (0.0 to 1.0)'
            cand -f 'Output format (console, json, markdown)'
            cand --format 'Output format (console, json, markdown)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hub'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
            cand init 'Initialize a new .cddmhub.toml configuration template'
            cand scan 'Scan organization federation repositories for cross-repository duplication'
            cand extract 'Extract a cross-repository duplicate cluster into a standalone shared package'
            cand sync 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;hub;init'= {
            cand -c 'Custom configuration file path (default: .cddmhub.toml)'
            cand --config 'Custom configuration file path (default: .cddmhub.toml)'
            cand -n 'Organization or hub name'
            cand --name 'Organization or hub name'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hub;scan'= {
            cand -f 'Output format (console, json, markdown)'
            cand --format 'Output format (console, json, markdown)'
            cand -m 'Minimum token count'
            cand --min-tokens 'Minimum token count'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hub;extract'= {
            cand -C 'Configuration file path (default: .cddmhub.toml)'
            cand --config 'Configuration file path (default: .cddmhub.toml)'
            cand -c 'Cluster index to extract'
            cand --cluster 'Cluster index to extract'
            cand -n 'Target package name (e.g. @org/shared-utils or cddm-shared-common)'
            cand --pkg-name 'Target package name (e.g. @org/shared-utils or cddm-shared-common)'
            cand -t 'Target package ecosystem (npm, cargo, pypi, go)'
            cand --pkg-type 'Target package ecosystem (npm, cargo, pypi, go)'
            cand -d 'Destination directory path for the new standalone package'
            cand --target-dir 'Destination directory path for the new standalone package'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --dry-run 'Dry run preview without writing changes to disk'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hub;sync'= {
            cand -c 'Custom configuration file path (default: .cddmhub.toml)'
            cand --config 'Custom configuration file path (default: .cddmhub.toml)'
            cand -e 'Remote peering endpoint URL (HTTPS/gRPC)'
            cand --endpoint 'Remote peering endpoint URL (HTTPS/gRPC)'
            cand -r 'Local repository name (defaults to directory name)'
            cand --repo 'Local repository name (defaults to directory name)'
            cand -s 'Cryptographic privacy salt (shared secret for organization hashing)'
            cand --salt 'Cryptographic privacy salt (shared secret for organization hashing)'
            cand -f 'Output format (console, json)'
            cand --format 'Output format (console, json)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --dry-run 'Dry run preview without transmitting or saving cache packs'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;hub;help'= {
            cand init 'Initialize a new .cddmhub.toml configuration template'
            cand scan 'Scan organization federation repositories for cross-repository duplication'
            cand extract 'Extract a cross-repository duplicate cluster into a standalone shared package'
            cand sync 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;hub;help;init'= {
        }
        &'cddm;hub;help;scan'= {
        }
        &'cddm;hub;help;extract'= {
        }
        &'cddm;hub;help;sync'= {
        }
        &'cddm;hub;help;help'= {
        }
        &'cddm;coverage'= {
            cand -r 'Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)'
            cand --report 'Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)'
            cand -m 'Minimum token count to consider as duplicate clone'
            cand --min-tokens 'Minimum token count to consider as duplicate clone'
            cand -f 'Output report format (console, json, markdown)'
            cand --format 'Output report format (console, json, markdown)'
            cand --min-hits 'Filter clones by minimum combined runtime execution hits'
            cand --risk-threshold 'Filter clones exceeding this risk score threshold (0-100)'
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand --dead-code-only 'Show only dead code duplicates (0 runtime executions across all sites)'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;completions'= {
            cand --log-level 'Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)'
            cand --log-file 'Write structured logs to a dedicated log file'
            cand -v 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand --verbose 'Enable verbose debug logging output (-v for debug, -vv for trace)'
            cand -q 'Suppress all non-error output and diagnostics'
            cand --quiet 'Suppress all non-error output and diagnostics'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'cddm;help'= {
            cand scan 'Scan target directory for code duplication & DRY health score'
            cand dead-code 'Detect unreferenced functions, unreachable blocks, and dead code duplicates'
            cand prune 'Automatically prune unreachable dead clone clusters and unreferenced code'
            cand diff 'Differential duplication scan comparing current changes against a Git base revision'
            cand semantic 'Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms'
            cand refactor 'Synthesize automated refactoring suggestions for duplicate clone pairs'
            cand extract 'Extract duplicate code into a standalone shared crate or module'
            cand serve 'Launch interactive WebUI dashboard in browser'
            cand watch 'Watch directory and trigger continuous real-time clone analysis on file save'
            cand lsp 'Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles'
            cand trend 'Analyze historical duplication trends across Git commit history'
            cand hook 'Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement'
            cand ignore 'Manage .cddmignore rules and test path suppression matching'
            cand rules 'Manage architectural policy rules (.cddmrules.toml)'
            cand init 'Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)'
            cand comment 'Generate formatted Markdown summary comment for Pull Requests / Merge Requests'
            cand heal 'Autonomous AI Code Surgeon refactoring with closed-loop test healing'
            cand cache 'Manage persistent fingerprint cache and export/import .cddmpack archives'
            cand monorepo 'Discover and scan monorepos with multi-workspace packages'
            cand tui 'Launch interactive Terminal UI (TUI) Studio dashboard'
            cand overlap 'Detect reimplemented ecosystem library algorithms and suggest standard packages'
            cand hub 'Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)'
            cand coverage 'Dynamic runtime execution & coverage-aware de-duplication analysis'
            cand completions 'Generate shell completion scripts for Bash, Zsh, Fish, PowerShell, or Elvish'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cddm;help;scan'= {
        }
        &'cddm;help;dead-code'= {
        }
        &'cddm;help;prune'= {
        }
        &'cddm;help;diff'= {
        }
        &'cddm;help;semantic'= {
        }
        &'cddm;help;refactor'= {
        }
        &'cddm;help;extract'= {
        }
        &'cddm;help;serve'= {
        }
        &'cddm;help;watch'= {
        }
        &'cddm;help;lsp'= {
        }
        &'cddm;help;trend'= {
        }
        &'cddm;help;hook'= {
            cand install 'Install a Git hook enforcing code duplication thresholds'
            cand uninstall 'Uninstall an existing CDDM Git hook'
            cand status 'Check current installation status of Git hooks'
        }
        &'cddm;help;hook;install'= {
        }
        &'cddm;help;hook;uninstall'= {
        }
        &'cddm;help;hook;status'= {
        }
        &'cddm;help;ignore'= {
            cand init 'Initialize a standard, well-documented .cddmignore configuration template'
            cand check 'Test whether a specific file path or line number is ignored by suppression rules'
        }
        &'cddm;help;ignore;init'= {
        }
        &'cddm;help;ignore;check'= {
        }
        &'cddm;help;rules'= {
            cand init 'Initialize a starter .cddmrules.toml configuration template'
            cand check 'Evaluate architectural policy rules against codebase'
        }
        &'cddm;help;rules;init'= {
        }
        &'cddm;help;rules;check'= {
        }
        &'cddm;help;init'= {
        }
        &'cddm;help;comment'= {
        }
        &'cddm;help;heal'= {
        }
        &'cddm;help;cache'= {
            cand export 'Export persistent cache database to a portable .cddmpack archive'
            cand import 'Import a portable .cddmpack archive into persistent cache database'
        }
        &'cddm;help;cache;export'= {
        }
        &'cddm;help;cache;import'= {
        }
        &'cddm;help;monorepo'= {
        }
        &'cddm;help;tui'= {
        }
        &'cddm;help;overlap'= {
        }
        &'cddm;help;hub'= {
            cand init 'Initialize a new .cddmhub.toml configuration template'
            cand scan 'Scan organization federation repositories for cross-repository duplication'
            cand extract 'Extract a cross-repository duplicate cluster into a standalone shared package'
            cand sync 'Synchronize privacy-preserving fingerprint caches with remote Federation Hub peers'
        }
        &'cddm;help;hub;init'= {
        }
        &'cddm;help;hub;scan'= {
        }
        &'cddm;help;hub;extract'= {
        }
        &'cddm;help;hub;sync'= {
        }
        &'cddm;help;coverage'= {
        }
        &'cddm;help;completions'= {
        }
        &'cddm;help;help'= {
        }
    ]
    $completions[$command]
}
