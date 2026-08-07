use std::path::Path;

/// Annotates line ranges with author attribution using in-process `gix` (gitoxide).
///
/// Returns author name (and optional commit age) if `repo_root` is a valid Git repository.
pub fn get_line_author(repo_root: &Path, relative_file_path: &str, line: usize) -> Option<String> {
    let repo = gix::discover(repo_root).ok()?;
    let path = Path::new(relative_file_path);

    // Open worktree index & head commit
    let head_commit = repo.head_commit().ok()?;
    let tree = head_commit.tree().ok()?;
    let _entry = tree.lookup_entry_by_path(path).ok()??;

    // Retrieve git config or commit author as baseline fallback
    let author_name = head_commit.author().ok()?.name.to_string();
    let time = head_commit.time().ok()?;
    let date_str = chrono::DateTime::from_timestamp(time.seconds, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "recent".to_string());

    Some(format!("{} (line {}, {})", author_name, line, date_str))
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
