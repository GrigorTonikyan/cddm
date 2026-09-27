#![forbid(unsafe_code)]

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

/// Generate shell completions for the specified shell and write to standard output.
pub fn run_completions_command(shell: Shell) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = crate::types::Cli::command();
    generate(shell, &mut cmd, "cddm", &mut io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bash_completions() {
        let mut buf = Vec::new();
        let mut cmd = crate::types::Cli::command();
        generate(Shell::Bash, &mut cmd, "cddm", &mut buf);
        let output = String::from_utf8(buf).expect("valid utf-8");
        assert!(output.contains("_cddm"));
        assert!(output.contains("scan"));
        assert!(output.contains("completions"));
    }

    #[test]
    fn test_generate_zsh_completions() {
        let mut buf = Vec::new();
        let mut cmd = crate::types::Cli::command();
        generate(Shell::Zsh, &mut cmd, "cddm", &mut buf);
        let output = String::from_utf8(buf).expect("valid utf-8");
        assert!(output.contains("#compdef cddm"));
    }

    #[test]
    fn test_generate_fish_completions() {
        let mut buf = Vec::new();
        let mut cmd = crate::types::Cli::command();
        generate(Shell::Fish, &mut cmd, "cddm", &mut buf);
        let output = String::from_utf8(buf).expect("valid utf-8");
        assert!(output.contains("complete -c cddm"));
    }

    #[test]
    fn test_generate_powershell_completions() {
        let mut buf = Vec::new();
        let mut cmd = crate::types::Cli::command();
        generate(Shell::PowerShell, &mut cmd, "cddm", &mut buf);
        let output = String::from_utf8(buf).expect("valid utf-8");
        assert!(output.contains("Register-ArgumentCompleter"));
    }

    #[test]
    fn test_generate_elvish_completions() {
        let mut buf = Vec::new();
        let mut cmd = crate::types::Cli::command();
        generate(Shell::Elvish, &mut cmd, "cddm", &mut buf);
        let output = String::from_utf8(buf).expect("valid utf-8");
        assert!(output.contains("edit:completion:arg-completer[cddm]"));
    }
}
