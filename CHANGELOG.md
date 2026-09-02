# Changelog

All notable changes to **CDDM** (_Code De-Duplication Meister_) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.2.0] - 2026-09-01

### Features

- **clean**: enhance workspace cleanup engine with Windows resilience and target discovery (`c91f0ac`)
- **ci**: implement Gitea portal content populator and complete documentation sync (`3830d9e`)
- **webui**: React 19 Feature-Sliced Studio with Monaco diff visualizer (`aeff948`)
- **mcp**: Model Context Protocol server with 27 tools and dynamic discovery (`8f08de7`)
- **ai-surgeon**: automated AST clone cluster refactoring surgeon [EP-35] (#13) (`4bdd9a4`)
- **ai-surgeon**: automated AST clone cluster refactoring surgeon [EP-35] (`2903f1d`)
- **hub**: federation hub and multi-repository clone correlation matrix [EP-36] (#14) (`65aa1c8`)
- **hub**: federation hub and multi-repository clone correlation matrix [EP-36] (`d66f5ea`)

### Bug Fixes

- **webui**: resolve form field accessibility labels and id attributes (`22d4e18`)
- **engine**: fix Type-4 semantic clone detection and 4-pillar parity (`ebff82f`)

### Documentation

- integrate planning and contribution docs into Gitea SSoT lifecycle (`64c946c`)
- **rfc**: update AI refactor surgeon RFC with issue #6 and v1.11.0 milestone (`01d1a50`)
- **rfc**: update federation hub RFC with issue #7 and v1.11.0 milestone (`ac32f2b`)

### Tooling & Maintenance

- **governance**: enforce Gitea as primary SSoT and codify issue-release lifecycle (#18) (`52d2bb7`)
- **governance**: codify issue-to-release lifecycle and API merge standards (`66f8cbe`)
- **governance**: enforce Gitea as primary SSoT and GitHub as mirror (`99e382b`)
- **release**: v1.10.0 (`e4466c4`)
- **scripts**: enforce strict TS and zero bypasses for PR 15 (`649e4bd`)
- **gitea**: automated Gitea Actions CI/CD matrix and cross-compilation (`1ba81df`)

## [1.9.0] - 2026-08-30

### Features

- **arch**: enhance core parser pool, treemap hook, ast memo and mcp (`703f98d`)
- **architecture**: integrate unified workspace service and incremental query engine (`9959bd8`)
- **timeline**: implement multi-branch clone drift matrix and webui lazy code splitting (`6bf1fa4`)
- **engine**: complete 4-pillar parity, chrome uiux verification, mcp tests, and modular refactoring (`6c934b9`)
- **detector**: add type-3 clone detection toggle and deduplicate core engine (`4ea9daa`)
- implement federation hub, coverage correlation, and neural clones (`4c18281`)
- **overlap**: add algorithm overlap detector and benchmark synthesis (`584064c`)
- **extract**: implement polyglot unit test synthesizer with 4-pillar parity [EP-34] (`83fd783`)
- **extract**: implement polyglot AST rewriters and multi-language shared module extraction (EP-33) (`cb87923`)
- **scripts**: use 100% native Bun.file, Bun.Glob, and zero node:fs in matrix generator (`e67ab73`)
- **scripts**: use native Bun.Glob and eliminate node:path in test matrix generator (`4251eef`)
- **scripts**: enforce Bun-only runtime, native APIs, and policy (`73030f6`)
- **tests**: enforce universal test architecture, 1:1 MCP suites, and parity (`3382ee3`)
- **extract**: implement automated shared module and crate extraction (EP-31) (`0804a76`)
- **watch**: implement live watch daemon and real-time studio sync (`275d744`)
- **semantic**: cross-language semantic matching, hybrid embeddings, and polyglot explorer (`2093ab7`)
- **vscode**: add embedded webview studio, activity bar dashboard and vsix packager (`847cfd0`)
- **studio**: add semantic graph visualizer, live watch sync, and polyglot ast refactor (`19e8c61`)
- **core**: implement ai surgeon, semantic graph, and cache packs (`331c493`)
- **core**: implement architectural boundary policy engine and polyglot expansion (`5fad2ba`)
- **core**: implement AST rewrite engine and test verification (`8aae9f2`)

### Bug Fixes

- **extract**: deduplicate manifest updaters and compact parity table (`8fe942f`)

### Performance Improvements

- **core**: multithread cross-language scan, add SIMD dot products and granular progress streaming (`4260e96`)

### Refactoring

- **query**: extract get_entry helper to eliminate memoization duplication (`9a7d041`)
- **core**: eliminate codebase duplication and enforce quality gate standard (`f94e68b`)
- **arch**: decompose monolithic modules and lock zero-grandfather policy (`5571ee4`)

### Tooling & Maintenance

- **scripts**: update package-vscode test and mcp timeouts (`f701296`)
- **mcp**: add 30s timeout to export-cache-pack test to prevent CI flakiness (`0a5e8fc`)
- **scripts**: add co-located tests for scripts/lib and package-distribution (`7afe805`)
- **mcp**: add multi-tool fidelity audit test suite and update feature matrix (`08d821b`)
- **e2e**: update cross-language explorer cutoff assertion in browser workflow spec (`ae0e889`)
- **engine**: deduplicate consensus helpers and enforce inline suppression in runner (`d8b28ab`)
- **e2e**: update playwright webserver cwd and conversation screenshot directory (`f9fb92f`)
- **e2e**: complete Playwright full-stack browser suite and optimize dogfooding scan (`db036e5`)

## [1.8.0] - 2026-08-30

### Features

- **arch**: enhance core parser pool, treemap hook, ast memo and mcp (`703f98d`)
- **architecture**: integrate unified workspace service and incremental query engine (`9959bd8`)
- **timeline**: implement multi-branch clone drift matrix and webui lazy code splitting (`6bf1fa4`)
- **engine**: complete 4-pillar parity, chrome uiux verification, mcp tests, and modular refactoring (`6c934b9`)
- **detector**: add type-3 clone detection toggle and deduplicate core engine (`4ea9daa`)
- implement federation hub, coverage correlation, and neural clones (`4c18281`)
- **overlap**: add algorithm overlap detector and benchmark synthesis (`584064c`)
- **extract**: implement polyglot unit test synthesizer with 4-pillar parity [EP-34] (`83fd783`)
- **extract**: implement polyglot AST rewriters and multi-language shared module extraction (EP-33) (`cb87923`)
- **scripts**: use 100% native Bun.file, Bun.Glob, and zero node:fs in matrix generator (`e67ab73`)
- **scripts**: use native Bun.Glob and eliminate node:path in test matrix generator (`4251eef`)
- **scripts**: enforce Bun-only runtime, native APIs, and policy (`73030f6`)
- **tests**: enforce universal test architecture, 1:1 MCP suites, and parity (`3382ee3`)
- **extract**: implement automated shared module and crate extraction (EP-31) (`0804a76`)
- **watch**: implement live watch daemon and real-time studio sync (`275d744`)
- **semantic**: cross-language semantic matching, hybrid embeddings, and polyglot explorer (`2093ab7`)
- **vscode**: add embedded webview studio, activity bar dashboard and vsix packager (`847cfd0`)
- **studio**: add semantic graph visualizer, live watch sync, and polyglot ast refactor (`19e8c61`)
- **core**: implement ai surgeon, semantic graph, and cache packs (`331c493`)
- **core**: implement architectural boundary policy engine and polyglot expansion (`5fad2ba`)
- **core**: implement AST rewrite engine and test verification (`8aae9f2`)

### Bug Fixes

- **extract**: deduplicate manifest updaters and compact parity table (`8fe942f`)

### Performance Improvements

- **core**: multithread cross-language scan, add SIMD dot products and granular progress streaming (`4260e96`)

### Refactoring

- **query**: extract get_entry helper to eliminate memoization duplication (`9a7d041`)
- **core**: eliminate codebase duplication and enforce quality gate standard (`f94e68b`)
- **arch**: decompose monolithic modules and lock zero-grandfather policy (`5571ee4`)

### Tooling & Maintenance

- **scripts**: add co-located tests for scripts/lib and package-distribution (`7afe805`)
- **mcp**: add multi-tool fidelity audit test suite and update feature matrix (`08d821b`)
- **e2e**: update cross-language explorer cutoff assertion in browser workflow spec (`ae0e889`)
- **engine**: deduplicate consensus helpers and enforce inline suppression in runner (`d8b28ab`)
- **e2e**: update playwright webserver cwd and conversation screenshot directory (`f9fb92f`)
- **e2e**: complete Playwright full-stack browser suite and optimize dogfooding scan (`db036e5`)

## [1.7.0] - 2026-08-24

### Features

- **policy**: implement architectural boundary isolation, zero-duplication critical zones, and clone limit policy engine via `.cddmrules.toml` (`cddm-core::policy`)
- **polyglot**: expand native Tree-sitter AST engine to 22 languages with Kotlin, Zig, Scala, Elixir, SQL, and Dockerfile parsers (`cddm-core::ast::parser`, `cddm-core::grammar`)
- **cli**: add `cddm rules init` and `cddm rules check` subcommands, plus `--rules` and `--enforce-policies` options to `scan`, `diff`, and `rules` commands
- **sarif**: map architectural policy violations to SARIF 2.1.0 rules (`CDDM_BOUNDARY`, `CDDM_ZERO_DUP`, `CDDM_LIMIT`) with counterpart `relatedLocations`
- **api**: expose `GET/POST /api/policy/rules` and `POST /api/policy/evaluate` Axum endpoints
- **lsp**: surface real-time architectural policy violation diagnostics in IDEs (`cddm-lsp::diagnostics`)
- **mcp**: add `cddm_check_policies` tool and `cddm://workspace/policies` resource to MCP server
- **webui**: integrate `PolicyRulesModal` Studio with active policy inspector, violation cards, and live `.cddmrules.toml` TOML editor
- **ci**: add Architectural Policy Violations table to PR/MR quality gate comments (`cddm-core::pr_comment`)

## [1.6.0] - 2026-08-24

### Features

- **ast-rewrite**: implement Tree-sitter AST-native parameter type inference and concrete syntax tree replacement engine (`cddm-core::ast::type_infer`, `cddm-core::ast::rewriter`)
- **import-resolver**: implement cross-module import statement synthesizer and deduplicator across 8 polyglot languages (`cddm-core::ast::import_resolver`)
- **verification**: implement closed-loop test suite verification runner with auto-detection for Cargo, Bun, NPM, Go, and Pytest (`cddm-core::refactor::verify_refactor_test_suite`)
- **cli**: add `--ast`, `--fn-name`, `--target-module`, `--apply-branch`, `--verify`, and `--test-cmd` flags to `cddm refactor`
- **api**: expose `POST /api/refactor/ast` and `POST /api/refactor/verify` Axum endpoints
- **mcp**: add `cddm_ast_refactor` and `cddm_verify_refactor` stdio JSON-RPC 2.0 tools
- **webui**: integrate AST-Native Rewrite tab and interactive test suite verification runner into `RefactorSandboxModal`

## [1.5.0] - 2026-08-24

### Features

- **polyglot**: expand native Tree-sitter AST engine from 9 to 16 languages with Ruby, PHP, Swift, Bash, Lua, JSON, and HTML parsers
- **ai**: implement AI-augmented refactoring prompt synthesizer and context exporter (`cddm refactor --prompt`, `POST /api/refactor/ai-prompt`, MCP tool `cddm_generate_ai_prompt`, WebUI Studio "Copy AI Prompt" action)
- **ci**: implement turnkey PR/MR Markdown quality gate comment generator with threshold compliance evaluation and clone rankings (`cddm comment`)
- **webui**: integrate one-click AI prompt generation into `RefactorSandboxModal` with clipboard status feedback

## [1.4.0] - 2026-08-24

### Features

- **suppression**: implement intelligent AST-aware suppression engine with `.cddmignore` glob rules, per-path threshold overrides, inline comment directives (`// cddm:ignore`, `/* cddm:ignore-start */`), and test/mock/generated file auto-detection
- **refactor**: implement interactive auto-refactor sandbox studio with customized function signatures, destination module placement, and transactional Git branch application (`gix`)
- **cli**: add `cddm ignore init` and `cddm ignore check` subcommands, plus `--cddmignore`, `--ignore-tests`, `--ignore-mocks`, and `--ignore-generated` flags
- **mcp**: add `cddm_check_suppression` and `cddm_apply_cluster_refactor` tools, and `cddm://workspace/suppressions` resource
- **webui**: add `SuppressionRulesModal` and `RefactorSandboxModal` with live parameter customization, colorized diff preview, and one-click Git branch deployment
- **timeline**: implement git duplication trends and turnkey ci generator (`da3ed61`)
- **lsp**: implement real-time language server engine and vscode extension (`14f7bdb`)
- **core**: implement live watch SSE sync, workspace patch application, and IDE deeplinks (`b917475`)
- **core**: add N-way clone graph clustering and multi-site deduplication synthesis (`b0735f9`)
- **core**: implement high-throughput zero-copy mmap and simd rolling hash vectorization (`d582f32`)

## [0.6.0] - 2026-08-24

### Features

- **core**: expand polyglot tree-sitter grammars and integrate ast merkle clone classifier (`964aeb7`)
- **core**: implement unified diff parser and atomic workspace patch applier
- **cli**: implement real-time live watch subcommand `cddm watch` with delta reporting
- **cli**: implement Server-Sent Events `/api/events` and patch application endpoint `/api/apply-patch`
- **webui**: implement live watch push sync, IDE protocol deeplinks, and direct refactor application

### Refactoring

- **webui**: implement modular atomic UI primitives and win2x window manager (`b551fd9`)

### Documentation

- update feature matrix, roadmap, and architecture for milestone v0.5.0 (`21d9de1`)

## [0.5.0] - 2026-08-23

### Features

- **webui**: implement atomic windowing system, diff viewer, duplication treemap, and refactor modal (`6142cc8`)

## [0.4.0] - 2026-08-23

### Features

- **core**: implement redb caching, git differential scans, and refactor advisor (`0d24083`)

## [0.3.0] - 2026-08-23

### Features

- **core**: implement SARIF exporter, refactor advisor, and MCP tools (`db8517d`)

### Tooling & Maintenance

- **vscode**: replace eslint extension with vite-plus extension pack (`5d2da2f`)

## [0.2.0] - 2026-08-23

### Features

- **scripts**: sync README badges and Cargo.lock on version bump (`83f324c`)
- **scripts**: add workspace clean and reset runners with full verification (`47ad3ee`)
- **tooling**: enforce zero-emoji policy and eradicate emojis across codebase (`c499090`)

### Bug Fixes

- **vscode**: remove duplicate flags from rust-analyzer extraArgs (`cdaf76f`)
- **ci**: enforce workspace-wide formatting in CI and configure markdownlint MD024 (`f300cab`)

### Documentation

- **agents**: add zero-downgrade dependency policy to prime directives (`e345352`)
- add comprehensive strategic roadmap, enhancement proposals, and active todo tracker (`27d108f`)

### Tooling & Maintenance

- **deps**: upgrade rust dependencies to latest versions preserving precision (`8c5f20a`)
- **cargo**: enforce missing_debug_implementations = deny workspace-wide (`37dfab5`)
- **deps**: update vitest to 4.1.11 in webui (`16d7dec`)
- **docs**: integrate doc integrity and roadmap sync into verification pipeline (`e333809`)

## [0.1.2] - 2026-08-23

### Features

- **mcp**: add `cddm-mcp` stdio JSON-RPC 2.0 server for AI coding agents
- **ast**: add Tree-sitter AST subtree hashing with Blake3 and filesystem watcher
- **webui**: embedded React 19 Studio WebUI served natively from Axum binary
- **blame**: in-process `gix` Git blame author annotation without subprocesses
- **core**: Winnowing M61 rolling hash clone detection engine in Rust 2024

### Bug Fixes

- merge overlapping clone pairs to prevent combinatorial explosion
- resolve Axum thread starvation and large DOM render freezing
- enforce strict typing in Zustand store and resolve floating promises

### Documentation

- comprehensive system architecture, API specifications, and feature matrix
- exhaustively verified requirements and performance benchmark tables

### Tooling & Maintenance

- unified workspace-wide toolchain with Vite Plus (`vp`) and TypeScript 7.0.2
- automated cross-platform Conventional Commits and Semantic Versioning engine
