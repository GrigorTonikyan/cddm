#![forbid(unsafe_code)]

use crate::types::HookAction;
use cddm_core::{get_hook_status, install_git_hook, uninstall_git_hook};

pub fn run_hook_command(action: HookAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        HookAction::Install {
            hook_type,
            fail_threshold,
            min_tokens,
            directory,
        } => match install_git_hook(&directory, &hook_type, fail_threshold, min_tokens) {
            Ok(msg) => println!("[PASS] {}", msg),
            Err(err) => {
                eprintln!("[ERROR] Failed to install hook: {}", err);
                std::process::exit(1);
            }
        },
        HookAction::Uninstall {
            hook_type,
            directory,
        } => match uninstall_git_hook(&directory, &hook_type) {
            Ok(msg) => println!("[PASS] {}", msg),
            Err(err) => {
                eprintln!("[ERROR] Failed to uninstall hook: {}", err);
                std::process::exit(1);
            }
        },
        HookAction::Status { directory } => {
            let status = get_hook_status(&directory);
            println!("=== CDDM Git Hook Status ===");
            println!("Hooks Directory: {}", status.hooks_dir);
            println!(
                "Pre-Commit Hook: {}",
                if status.pre_commit_installed {
                    "[INSTALLED]"
                } else {
                    "[NOT INSTALLED]"
                }
            );
            println!(
                "Pre-Push Hook:   {}",
                if status.pre_push_installed {
                    "[INSTALLED]"
                } else {
                    "[NOT INSTALLED]"
                }
            );
        }
    }
    Ok(())
}
