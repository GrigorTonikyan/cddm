# Security Policy & Secrets Governance Standard

## 1. Supported Versions

Security patches and dependency updates are applied exclusively to the current active release line of **CDDM**.

| Version | Supported          |
| :------ | :----------------- |
| 4.x     | :white_check_mark: |
| < 4.0.0 | :x:                |

---

## 2. Reporting a Vulnerability

If you discover a potential security vulnerability within CDDM, please report it privately:

- **Primary Contact**: gt-dev Security Team at [security@gt-web-dev.com](mailto:security@gt-web-dev.com)
- **Authoritative SSoT Forge**: [https://git.gt-web-dev.com/gt-dev/cddm](https://git.gt-web-dev.com/gt-dev/cddm)

Please do **NOT** open public issues or PRs discussing unresolved security vulnerabilities.

### Submission Guidelines

Please include:

1. Vulnerability classification and severity assessment (e.g. CVSS vector or CWE category).
2. Minimal reproduction steps, proof of concept (PoC), or code snippet.
3. Affected components (CLI, WebUI, MCP Server, TUI, or CI/CD pipelines).
4. Proposed remediations or mitigations, if available.

### Response & Advisory SLA

- **Acknowledgement**: Within 24 hours.
- **Triage & Impact Assessment**: Within 72 hours.
- **Remediation & Patch Release**: Coordinated patch release cut via automated SemVer lifecycle ASAP.

---

## 3. Secrets Management & Sensitive Data Governance Standard

CDDM enforces strict zero-trust standards regarding credentials, API tokens, cryptographic keys, and sensitive data.

### 3.1 Absolute Prohibition of Plaintext Secrets in VCS

- **Zero Secrets in Git**: No credentials, API tokens, registry keys, passwords, private keys, or certificates may ever be committed to the Git repository, branch history, issue trackers, or PR descriptions.
- **Ignored Sensitive Files**: All `.env`, `.env.*`, `*.token`, `*.secret`, `*.key`, `*.pem`, `*.p12`, `*.pfx`, and `credentials.json` files are strictly excluded via `.gitignore`.
- **Pre-Commit Enforcement**: Verification scripts and pre-commit hooks validate that no sensitive environment files or tokens are staged.

### 3.2 Authoritative CI/CD Secret Store (Gitea Actions Encrypted Secrets)

- **SSoT Secret Storage**: All CI/CD automation tokens (`GITEA_TOKEN`, `REGISTRY_TOKEN`, etc.) MUST be stored encrypted in Gitea repository or organization secrets (`/settings/actions/secrets`).
- **Encryption at Rest**: Secrets in Gitea Actions are encrypted at rest using libsodium sealed-box cryptography and decrypted only within ephemeral runner sandboxes.
- **Branch Protection Scoping**: Deployment and publishing secrets are restricted to execution on protected branches (`main`) and release tags (`v*`).

### 3.3 Safe Environment Variable Injection Protocol

- **Strict `env:` Mapping**: Secrets MUST be injected into workflow jobs strictly through `env:` mappings:
  ```yaml
  env:
    GITEA_TOKEN: ${{ secrets.GITEA_TOKEN }}
  ```
- **Prohibition of Command-Line Argument Passing**: Secrets MUST NEVER be passed as CLI arguments (e.g., `--token ${{ secrets.GITEA_TOKEN }}`), because command-line arguments are exposed in process tables (`/proc/$PID/cmdline`, Windows task managers) and system event logs.

### 3.4 Secret Masking in Tooling & Logging

- **Log Sanitization**: All repository tooling and automation scripts (such as `scripts/publish-release.ts`) MUST sanitize and mask sensitive tokens (`maskSecret`) when printing URLs, HTTP requests, or diagnostic messages.
- **Zero Token Leakage**: Scripts must never echo raw headers, authorization bearers, or environment variables to standard output or error streams.

### 3.5 External Infrastructure Secret Management

- **Centralized Vault / PaaS**: For production multi-service deployments (TrueNAS SCALE hypervisor, Coolify PaaS, container registries), secrets are managed through isolated encrypted key stores or centralized Vault instances with automated rotation policies.
- **Principle of Least Privilege (PoLP)**: Access tokens and service accounts must have minimal granular permissions required for their specific role (e.g., registry push-only, release asset upload-only).

### 3.6 Continuous Security Scanning & Auditing

- **Dependency Vulnerability Scanning**: Executed during `vp run verify` and CI via `bun pm scan` and `cargo audit`.
- **Static Analysis**: Rust memory safety is guaranteed by `#![forbid(unsafe_code)]` across all core crates.
