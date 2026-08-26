#![forbid(unsafe_code)]

use clap::Parser;

mod commands;
mod formatters;
mod serve;
mod tui;
mod types;

use commands::*;
use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let res = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to build Tokio runtime");
            match rt.block_on(run_app()) {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })?
        .join()
        .unwrap_or_else(|e| std::panic::resume_unwind(e));

    match res {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            directory,
            min_tokens,
            format,
            fail_threshold,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
            clear_cache,
            cddmignore,
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            rules,
            enforce_policies,
            cross_language,
        } => {
            run_scan_command(
                directory,
                min_tokens,
                format,
                fail_threshold,
                languages,
                ignore,
                git_blame,
                cache_dir,
                no_cache,
                clear_cache,
                cddmignore,
                ignore_tests,
                ignore_mocks,
                ignore_generated,
                rules,
                enforce_policies,
                cross_language,
            )
            .await?;
        }

        Commands::Diff {
            base_ref,
            target_ref,
            directory,
            min_tokens,
            format,
            fail_threshold,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
            cddmignore,
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            rules,
            enforce_policies,
            cross_language,
        } => {
            run_diff_command(
                base_ref,
                target_ref,
                directory,
                min_tokens,
                format,
                fail_threshold,
                languages,
                ignore,
                git_blame,
                cache_dir,
                no_cache,
                cddmignore,
                ignore_tests,
                ignore_mocks,
                ignore_generated,
                rules,
                enforce_policies,
                cross_language,
            )
            .await?;
        }

        Commands::Semantic {
            directory,
            threshold,
            min_tokens,
            format,
            languages,
            ignore,
        } => {
            run_semantic_command(directory, threshold, min_tokens, format, languages, ignore)?;
        }

        Commands::Refactor {
            pair,
            cluster,
            directory,
            min_tokens,
            output,
            prompt,
            ast,
            fn_name,
            target_module,
            apply_branch,
            verify,
            test_cmd,
            languages,
            ignore,
        } => {
            run_refactor_command(
                pair,
                cluster,
                directory,
                min_tokens,
                output,
                prompt,
                ast,
                fn_name,
                target_module,
                apply_branch,
                verify,
                test_cmd,
                languages,
                ignore,
            )
            .await?;
        }

        Commands::Extract(args) => {
            run_extract_command(args).await?;
        }

        Commands::Serve { port, open } => {
            serve::start_server(port, open).await?;
        }

        Commands::Watch {
            directory,
            min_tokens,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
            debounce_ms,
            fail_threshold,
            serve,
            open,
            format,
            cross_language,
        } => {
            run_watch_command(
                directory,
                min_tokens,
                languages,
                ignore,
                git_blame,
                cache_dir,
                no_cache,
                debounce_ms,
                fail_threshold,
                serve,
                open,
                format,
                cross_language,
            )
            .await?;
        }

        Commands::Lsp {
            directory,
            min_tokens,
        } => {
            run_lsp_command(directory, min_tokens).await?;
        }

        Commands::Trend {
            directory,
            max_samples,
            min_tokens,
            format,
        } => {
            run_trend_command(directory, max_samples, min_tokens, format)?;
        }

        Commands::Hook { action } => {
            run_hook_command(action)?;
        }

        Commands::Ignore { action } => {
            run_ignore_command(action)?;
        }

        Commands::Rules { action } => {
            run_rules_command(action).await?;
        }

        Commands::Init {
            platform,
            fail_threshold,
            min_tokens,
            output,
            write,
        } => {
            run_init_command(platform, fail_threshold, min_tokens, output, write)?;
        }

        Commands::Comment {
            directory,
            min_tokens,
            fail_threshold,
            platform,
            output,
        } => {
            run_comment_command(directory, min_tokens, fail_threshold, platform, output).await?;
        }

        Commands::Heal(args) => {
            run_heal_command(HealCliArgs {
                directory: args.directory,
                cluster: args.cluster,
                pair: args.pair,
                provider_str: args.provider,
                model: args.model,
                api_key: args.api_key,
                endpoint: args.endpoint,
                max_iterations: args.max_iterations,
                verify: args.verify,
                test_cmd: args.test_cmd,
                branch: args.branch,
                fn_name: args.fn_name,
                target_module: args.target_module,
                custom_instructions: args.custom_instructions,
                min_tokens: args.min_tokens,
            })
            .await?;
        }

        Commands::Cache { action } => match action {
            CacheAction::Export { cache_dir, output } => {
                run_cache_export_command(cache_dir, output)?;
            }
            CacheAction::Import {
                pack_file,
                target_dir,
            } => {
                run_cache_import_command(pack_file, target_dir)?;
            }
        },

        Commands::Monorepo {
            directory,
            min_tokens,
        } => {
            run_monorepo_command(directory, min_tokens).await?;
        }

        Commands::Tui {
            directory,
            min_tokens,
            watch,
            fail_threshold,
            languages,
            ignore,
        } => {
            run_tui_command(
                directory,
                min_tokens,
                watch,
                fail_threshold,
                languages,
                ignore,
            )
            .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
