#![forbid(unsafe_code)]

use super::patch::apply_patch_to_workspace;
use crate::types::ApplyRefactorBranchResult;
use std::path::Path;

/// Applies a refactoring patch with optional automated Git branch creation.
pub fn apply_cluster_refactor_branch(
    repo_root: &Path,
    patch: &str,
    branch_name: Option<&str>,
    create_branch: bool,
) -> Result<ApplyRefactorBranchResult, String> {
    let mut branch_created = None;

    if create_branch {
        let bname = branch_name.unwrap_or("cddm/refactor-auto");
        if let Ok(repo) = gix::discover_with_environment_overrides(repo_root)
            && let Ok(head_id) = repo.head_id()
        {
            let ref_name = format!("refs/heads/{bname}");
            let _ = repo.reference(
                ref_name,
                head_id,
                gix::refs::transaction::PreviousValue::Any,
                "cddm automated refactoring branch creation",
            );
            branch_created = Some(bname.to_string());
        }
    }

    let patch_res = apply_patch_to_workspace(patch, false)?;

    Ok(ApplyRefactorBranchResult {
        success: patch_res.success,
        branch_created,
        modified_files: patch_res.modified_files,
        hunks_applied: patch_res.hunks_applied,
        message: patch_res.message,
    })
}
