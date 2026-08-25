#![forbid(unsafe_code)]

use crate::types::VerifyRefactorResult;
use std::path::Path;

/// Executes workspace test suite command to verify that refactoring introduces no regressions.
pub fn verify_refactor_test_suite(
    directory: &Path,
    test_command: Option<&str>,
    _branch_name: Option<&str>,
    _timeout_seconds: Option<u64>,
) -> Result<VerifyRefactorResult, String> {
    let start_time = std::time::Instant::now();

    let cmd_str = if let Some(custom) = test_command {
        custom.to_string()
    } else if directory.join("Cargo.toml").exists() {
        "cargo test --workspace".to_string()
    } else if directory.join("package.json").exists() {
        "bun test".to_string()
    } else if directory.join("go.mod").exists() {
        "go test ./...".to_string()
    } else if directory.join("pyproject.toml").exists() || directory.join("pytest.ini").exists() {
        "pytest".to_string()
    } else {
        "cargo test".to_string()
    };

    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty test command specified".to_string());
    }

    let program = parts[0];
    let args = &parts[1..];

    let mut command = std::process::Command::new(program);
    command.args(args);
    command.current_dir(directory);

    let output = command.output().map_err(|e| {
        format!(
            "Failed to execute test verification command '{}': {}",
            cmd_str, e
        )
    })?;

    let duration_ms = start_time.elapsed().as_millis() as u64;
    let exit_code = output.status.code().unwrap_or(-1);
    let success = output.status.success();

    let raw_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let raw_stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let stdout_snippet = raw_stdout
        .lines()
        .rev()
        .take(50)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");

    let stderr_snippet = raw_stderr
        .lines()
        .rev()
        .take(50)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");

    let message = if success {
        format!(
            "Test suite verification PASSED in {}ms with command '{}'",
            duration_ms, cmd_str
        )
    } else {
        format!(
            "Test suite verification FAILED with exit code {} in {}ms",
            exit_code, duration_ms
        )
    };

    Ok(VerifyRefactorResult {
        success,
        exit_code,
        duration_ms,
        command_executed: cmd_str,
        stdout_snippet,
        stderr_snippet,
        message,
    })
}
