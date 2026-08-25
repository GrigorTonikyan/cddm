#![forbid(unsafe_code)]

use crate::types::IgnoreAction;
use cddm_core::SuppressionEngine;
use std::fs;
use std::path::Path;

pub fn run_ignore_command(action: IgnoreAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        IgnoreAction::Init { directory, force } => {
            let out_file = directory.join(".cddmignore");
            if out_file.exists() && !force {
                eprintln!(
                    "[ERROR] '{}' already exists. Use --force to overwrite.",
                    out_file.display()
                );
                std::process::exit(1);
            }
            let template = SuppressionEngine::generate_default_cddmignore();
            fs::write(&out_file, template)?;
            println!(
                "[PASS] Generated .cddmignore suppression template at '{}'",
                out_file.display()
            );
        }
        IgnoreAction::Check {
            path,
            line,
            cddmignore,
            ignore_tests,
            ignore_mocks,
            ignore_generated,
        } => {
            let engine = if let Some(p) = cddmignore {
                SuppressionEngine::from_file(&p, ignore_tests, ignore_mocks, ignore_generated)?
            } else if Path::new(".cddmignore").exists() {
                SuppressionEngine::from_file(
                    Path::new(".cddmignore"),
                    ignore_tests,
                    ignore_mocks,
                    ignore_generated,
                )?
            } else {
                SuppressionEngine::with_options(ignore_tests, ignore_mocks, ignore_generated)
            };

            let file_content = fs::read_to_string(&path).ok();
            let path_ignored = engine.is_path_ignored(&path, file_content.as_deref());

            println!("=== CDDM Suppression Check ===");
            println!("File Path:    {}", path.display());
            println!(
                "Path Ignored: {}",
                if path_ignored { "[YES]" } else { "[NO]" }
            );

            if let Some(target_line) = line
                && let Some(ref text) = file_content
            {
                let mut eng = engine.clone();
                eng.register_file_directives(&path.to_string_lossy(), text);
                let span_ignored =
                    eng.is_span_suppressed(&path.to_string_lossy(), target_line, target_line);
                println!(
                    "Line {}:       {}",
                    target_line,
                    if span_ignored {
                        "[SUPPRESSED BY INLINE DIRECTIVE]"
                    } else {
                        "[NOT SUPPRESSED]"
                    }
                );
            }
        }
    }
    Ok(())
}
