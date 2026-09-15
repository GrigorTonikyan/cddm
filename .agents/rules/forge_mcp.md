---
trigger: always_on
---

# Workspace Integration Rule: Forge MCP Infrastructure & VCS SSoT

> [!IMPORTANT]
> **STANDARDIZED INFRASTRUCTURE & VCS AUTOMATION MANDATE**:
> AI agents operating in this workspace MUST NEVER use ad-hoc SSH connections, raw `midclt` commands,
> legacy scripts, or raw curl calls to manage TrueNAS SCALE, Coolify, or Gitea.
>
> All operations MUST be performed via **Forge MCP** (`forge-mcp`) tools.

## Standard Tool Mapping

| Target System            | Deprecated / Prohibited Pattern              | Required Forge MCP Tool                                                                                       |
| :----------------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| **TrueNAS Compose Apps** | `ssh ... midclt call app.compose.*`          | `truenas_app_list`, `truenas_app_status`, `truenas_app_update`, `truenas_app_upgrade`                         |
| **Container Exec**       | `ssh ... docker exec ...`                    | `truenas_app_exec(app_name, container_name, command)`                                                         |
| **Virtual Machines**     | `ssh dev@<vm_ip>`                            | `truenas_vm_exec(vm_name, command)` (QEMU Agent, no SSH)                                                      |
| **VM Lifecycle**         | `midclt call vm.start/stop`                  | `truenas_vm_start`, `truenas_vm_stop`, `truenas_vm_status`                                                    |
| **ZFS Storage**          | `ssh ... zfs snapshot/list`                  | `truenas_storage_snapshot`, `truenas_storage_datasets`                                                        |
| **Coolify PaaS**         | Direct curl to Coolify API / `mcp-bridge.py` | `coolify_list_apps`, `coolify_get_app`, `coolify_deploy`, `coolify_control`                                   |
| **VCS Defect Reporting** | Ad-hoc local hacks / monkey-patching         | `forge_report_bug`, `forge_request_feature`, `forge_suggest_enhancement`                                      |
| **VCS Issue Tracking**   | Manual curl to `git.gt-web-dev.com`          | `gitea_list_issues`, `gitea_get_issue`, `gitea_create_issue`, `gitea_update_issue`                            |
| **VCS Pull Requests**    | Manual web UI / ad-hoc scripts               | `gitea_create_pull_request`, `gitea_get_pull_request`, `gitea_list_pull_requests`, `gitea_merge_pull_request` |
| **VCS CI Quality Gate**  | Ad-hoc polling script / manual web UI        | `gitea_get_commit_statuses`, `gitea_wait_for_ci_gate`                                                         |
| **VCS Runners & Auth**   | Custom ping scripts                          | `gitea_runner_status`, `gitea_auth_status`, `gitea_sync_mirror`                                               |

## Enforcement Directives

1. **Zero Ad-Hoc Scripts**: Never create throwaway shell, python, or bun scripts to query TrueNAS, Coolify, or Gitea.
2. **Deterministic Envelopes**: Rely on structured MCP tool responses.
3. **Safety Snapshots**: Always call `truenas_storage_snapshot` before mutating containers or VMs.
4. **Upstream Defect Escalation**: When external tools lack functionality or encounter defects, file issues upstream on Gitea SSoT (`gt-dev/forge`) using `forge_report_bug`, `forge_request_feature`, or `forge_suggest_enhancement`. NEVER monkey-patch locally.
