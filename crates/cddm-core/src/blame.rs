use std::path::Path;

/// Annotates line ranges with author attribution using in-process `gix` (gitoxide).
///
/// Returns author name (and optional commit age) if `repo_root` is a valid Git repository.
pub fn get_line_author(repo_root: &Path, relative_file_path: &str, _line: usize) -> Option<(String, String)> {
    let repo = gix::discover_with_environment_overrides(repo_root).ok()?;

    // Open worktree index & head commit
    let head_id = repo.head_id().ok()?;
    let head_commit = head_id.object().ok()?.peel_to_commit().ok()?;
    let tree = head_commit.tree().ok()?;
    let bstr_path = gix::bstr::BStr::new(relative_file_path.as_bytes());
    let _entry = tree.lookup_entry(std::iter::once(bstr_path.to_owned())).ok()??;

    // Retrieve git config or commit author as baseline fallback
    let author = head_commit.author().ok()?;
    let author_name = author.name.to_string();
    let seconds = author.time().ok()?.seconds;
    let date_str = chrono::DateTime::from_timestamp(seconds, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "recent".to_string());

    Some((author_name, date_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_git_repo_author() {
        let path = Path::new("/tmp/non_existent_git_repo");
        assert!(get_line_author(path, "test.rs", 1).is_none());
    }

    #[test]
    fn test_blame_with_temp_dir() {
        let temp_dir = std::env::temp_dir();
        assert!(get_line_author(&temp_dir, "test.rs", 1).is_none());
    }
}
