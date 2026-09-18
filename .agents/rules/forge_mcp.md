# Workspace Integration Rule: Forge MCP Infrastructure, VCS SSoT & Troubleshooting Hierarchy

> [!IMPORTANT]
> **STANDARDIZED DUAL-MCP & 4-TIER BOUNDARY ENFORCEMENT MANDATE**:
> AI coding agents operating across this repository MUST NEVER use ad-hoc SSH connections, raw `midclt`,
> legacy scripts, raw curl calls, or cross architectural layers when troubleshooting defects.
>
> Operations MUST be performed via the appropriate standalone MCP server and CLI interfaces:
>
> 1. **`forge-mcp`** (`forge truenas`): TrueNAS SCALE hypervisor, storage, VMs, Netdata. (Isolated from VCS).
> 2. **`gitea-mcp`** (`forge gitea`, `forge issue`, `forge pr`): Gitea Enterprise VCS, Actions CI/CD workflows, job console failure logs, runner telemetry, webhooks, secrets, PRs, issues, milestones, releases. (Isolated from Hypervisor).
> 3. **Coolify Official MCP** (`/mcp`, `forge coolify`): Staging & production application deployments and routing.

## 4-Tier Troubleshooting Boundary Hierarchy (STRICT MANDATE)

- **Tier 1 (Local Workspace)**: Debug locally first (`bun run all:check`, `uv run pytest`, `vp fmt --check`, `vp lint`).
- **Tier 2 (VCS & CI/CD Observability — `gitea-mcp`)**: When debugging CI pipeline checks, PR merge gates, or build failures, use strictly `gitea-mcp` tools (`gitea_list_workflow_runs`, `gitea_get_job_logs`, `gitea_runner_status`, `gitea_get_runner_logs`) and `forge gitea runs|run|jobs|job-logs`.
- **Tier 3 (PaaS Runtime — `coolify-mcp`)**: Only query Coolify when specifically tasked with inspecting a live deployed service or routing issue in staging/production.
- **Tier 4 (Hypervisor — `forge-mcp`)**: **STRICT ISOLATION**: Never probe TrueNAS host containers (`truenas_app_*`, `truenas_vm_*`) when debugging repository or CI issues.

## Private Package Registry Access

- **NPM & Bun Registry**: `https://git.gt-web-dev.com/api/packages/gt-dev/npm/` (`@gt-dev/forge`, `@gt-dev/forge-mcp`, `@gt-dev/gitea-mcp`)
- **Python PyPI Registry**: `https://git.gt-web-dev.com/api/packages/gt-dev/pypi/simple`

## Standard Tool Mapping

| Target System                   | Deprecated / Prohibited Pattern              | Required Server / Tool                                                                             |
| :------------------------------ | :------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| **TrueNAS Compose Apps**        | `ssh ... midclt call app.compose.*`          | `forge-mcp`: `truenas_app_list`, `truenas_app_status`, `truenas_app_update`, `truenas_app_upgrade` |
| **Container Exec**              | `ssh ... docker exec ...`                    | `forge-mcp`: `truenas_app_exec(app_name, container_name, command)`                                 |
| **Virtual Machines**            | `ssh dev@<vm_ip>`                            | `forge-mcp`: `truenas_vm_exec(vm_name, command)` (QEMU Agent, no SSH)                              |
| **VM Lifecycle**                | `midclt call vm.start/stop`                  | `forge-mcp`: `truenas_vm_start`, `truenas_vm_stop`, `truenas_vm_status`                            |
| **ZFS Storage**                 | `ssh ... zfs snapshot/list`                  | `forge-mcp`: `truenas_storage_snapshot`, `truenas_storage_datasets`                                |
| **Coolify PaaS**                | Direct curl to Coolify API / `mcp-bridge.py` | Official Coolify MCP (`/mcp`) / `coolify_setup_gitea_webhook`                                      |
| **CI/CD Actions Observability** | Probing TrueNAS host / runner containers     | `gitea-mcp`: `gitea_list_workflow_runs`, `gitea_get_job_logs`, `gitea_runner_status`               |
| **VCS Defect Reporting**        | Ad-hoc local hacks / monkey-patching         | `gitea-mcp`: `gitea_create_issue` / `forge-mcp`: `forge_report_bug`                                |
| **VCS Issue Tracking**          | Manual curl to `git.gt-web-dev.com`          | `gitea-mcp`: `gitea_list_issues`, `gitea_get_issue` / `forge issue` CLI                            |
| **Pull Requests & CI Gates**    | Manual web UI / ad-hoc scripts               | `gitea-mcp`: `gitea_create_pr`, `gitea_merge_pr`, `gitea_wait_for_ci_gate`                         |

## Enforcement Directives

1. **Zero Ad-Hoc Scripts**: Never create throwaway shell or python scripts to query TrueNAS, Coolify, or Gitea.
2. **Issue-Driven Development**: All changes must link to issues on Gitea SSoT (`Fixes #<id>`).
3. **Deterministic Envelopes**: Rely on structured MCP tool responses and `--json` CLI output.
