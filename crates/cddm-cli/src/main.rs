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
    let cli = Cli::parse();

    let mut log_config = cddm_core::logging::LogConfig::new()
        .with_verbose(cli.verbose > 0)
        .with_quiet(cli.quiet);

    if let Some(ref lvl_str) = cli.log_level {
        if let Ok(lvl) = lvl_str.parse() {
            log_config = log_config.with_level(lvl);
        }
    } else if cli.verbose >= 2 {
        log_config = log_config.with_level(cddm_core::logging::LogLevel::Trace);
    } else if cli.verbose == 1 {
        log_config = log_config.with_level(cddm_core::logging::LogLevel::Debug);
    }

    if let Some(ref log_file) = cli.log_file {
        log_config = log_config.with_log_file(log_file.clone());
    }

    let _ = cddm_core::logging::init_logging(&log_config);

    let res = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to build Tokio runtime");
            match rt.block_on(run_app(cli)) {
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

async fn run_app(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Scan(args) => {
            let enable_cross_lang = args.cross_language && !args.no_cross_language;
            let enable_type4 = args.detect_type4 && !args.no_type4;
            let effective_ignore_tests = args.ignore_tests && !args.no_ignore_tests;
            let effective_ignore_mocks = args.ignore_mocks && !args.no_ignore_mocks;
            run_scan_command(
                args.directory,
                args.min_tokens,
                args.format,
                args.fail_threshold,
                args.languages,
                args.ignore,
                args.git_blame,
                args.cache_dir,
                args.no_cache,
                args.clear_cache,
                args.cddmignore,
                effective_ignore_tests,
                effective_ignore_mocks,
                args.ignore_generated,
                args.rules,
                args.enforce_policies,
                enable_cross_lang,
                !args.no_type3,
                enable_type4,
                args.threads,
            )
            .await?;
        }

        Commands::DeadCode(args) => {
            run_dead_code_command(args).await?;
        }

        Commands::Prune(args) => {
            run_prune_command(args).await?;
        }

        Commands::Diff(args) => {
            run_diff_command(
                args.base_ref,
                args.target_ref,
                args.directory,
                args.min_tokens,
                args.format,
                args.fail_threshold,
                args.languages,
                args.ignore,
                args.git_blame,
                args.cache_dir,
                args.no_cache,
                args.cddmignore,
                args.ignore_tests,
                args.ignore_mocks,
                args.ignore_generated,
                args.rules,
                args.enforce_policies,
                args.cross_language,
                args.matrix,
            )
            .await?;
        }

        Commands::Semantic(args) => {
            run_semantic_command(
                args.directory,
                args.threshold,
                args.min_tokens,
                args.format,
                args.languages,
                args.ignore,
                args.neural,
                args.neural_threshold,
                args.threads,
            )?;
        }

        Commands::Refactor(args) => {
            run_refactor_command(
                args.pair,
                args.cluster,
                args.directory,
                args.min_tokens,
                args.output,
                args.prompt,
                args.ast,
                args.fn_name,
                args.target_module,
                args.apply_branch,
                args.verify,
                args.test_cmd,
                args.languages,
                args.ignore,
            )
            .await?;
        }

        Commands::Extract(args) => {
            run_extract_command(args).await?;
        }

        Commands::Serve(args) => {
            serve::start_server(args.port, args.open).await?;
        }

        Commands::Watch(args) => {
            run_watch_command(
                args.directory,
                args.min_tokens,
                args.languages,
                args.ignore,
                args.git_blame,
                args.cache_dir,
                args.no_cache,
                args.debounce_ms,
                args.fail_threshold,
                args.serve,
                args.open,
                args.format,
                args.cross_language,
            )
            .await?;
        }

        Commands::Lsp(args) => {
            run_lsp_command(args.directory, args.min_tokens).await?;
        }

        Commands::Trend(args) => {
            run_trend_command(
                args.directory,
                args.max_samples,
                args.min_tokens,
                args.format,
            )?;
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

        Commands::Init(args) => {
            run_init_command(
                args.platform,
                args.fail_threshold,
                args.min_tokens,
                args.output,
                args.write,
            )?;
        }

        Commands::Comment(args) => {
            run_comment_command(
                args.directory,
                args.min_tokens,
                args.fail_threshold,
                args.platform,
                args.output,
            )
            .await?;
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

        Commands::Monorepo(args) => {
            run_monorepo_command(args.directory, args.min_tokens).await?;
        }

        Commands::Tui(args) => {
            run_tui_command(
                args.directory,
                args.min_tokens,
                args.watch,
                args.fail_threshold,
                args.languages,
                args.ignore,
            )
            .await?;
        }

        Commands::Overlap(args) => {
            run_overlap_command(args)?;
        }

        Commands::Hub(args) => {
            run_hub_command(args).await?;
        }

        Commands::Coverage(args) => {
            handle_coverage_command(args).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
