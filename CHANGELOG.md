# Changelog

All notable changes to **CDDM** (_Code De-Duplication Meister_) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-08-23

### 🚀 Features

- **mcp**: add `cddm-mcp` stdio JSON-RPC 2.0 server for AI coding agents
- **ast**: add Tree-sitter AST subtree hashing with Blake3 and filesystem watcher
- **webui**: embedded React 19 Studio WebUI served natively from Axum binary
- **blame**: in-process `gix` Git blame author annotation without subprocesses
- **core**: Winnowing M61 rolling hash clone detection engine in Rust 2024

### 🐛 Bug Fixes

- merge overlapping clone pairs to prevent combinatorial explosion
- resolve Axum thread starvation and large DOM render freezing
- enforce strict typing in Zustand store and resolve floating promises

### 📚 Documentation

- comprehensive system architecture, API specifications, and feature matrix
- exhaustively verified requirements and performance benchmark tables

### 🛠️ Tooling & Maintenance

- unified workspace-wide toolchain with Vite Plus (`vp`) and TypeScript 7.0.2
- automated cross-platform Conventional Commits and Semantic Versioning engine
