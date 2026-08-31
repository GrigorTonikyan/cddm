# ci(gitea): automated Gitea Actions CI/CD matrix and cross-compilation

## Summary
Configures production-grade CI/CD pipelines on self-hosted Gitea Actions Linux runner.

## Capabilities
- Multi-job quality gates for Rust, WebUI, and MCP.
- Cross-compilation of standalone binaries for Linux AMD64 and Windows x86_64.
- VS Code extension VSIX packaging and SHA256 checksum publishing.

## Verification
- [x] Gitea Actions Run 63, 65, 66 verified 100% green.

Branch: `ci/gitea-actions-pipeline`
Milestone: v1.9.0
