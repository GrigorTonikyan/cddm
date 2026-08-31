#![forbid(unsafe_code)]

use cddm_core::{
    CachePackSummary, export_cache_pack, find_workspace_root, import_cache_pack,
    resolve_default_cache_path,
};
use std::path::{Path, PathBuf};

fn print_pack_summary(summary: &CachePackSummary, entry_label: &str) {
    println!("\x1b[32m[SUCCESS] {}\x1b[0m", summary.message);
    println!("  {}: {}", entry_label, summary.entry_count);
    println!("  Checksum: {}", summary.checksum);
}

/// Executes the CLI `cddm cache export` command.
pub fn run_cache_export_command(
    cache_dir: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = cache_dir.unwrap_or_else(|| resolve_default_cache_path(Path::new(".")));
    let out_path = output.unwrap_or_else(|| PathBuf::from("cddm-cache.cddmpack"));

    println!(
        "\x1b[36m--> Exporting persistent cache database from '{}' to '{}'...\x1b[0m",
        db_path.display(),
        out_path.display()
    );

    let summary = export_cache_pack(&db_path, &out_path)?;
    print_pack_summary(&summary, "Entries");
    Ok(())
}

/// Executes the CLI `cddm cache import` command.
pub fn run_cache_import_command(
    pack_file: PathBuf,
    target_cache_dir: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir =
        target_cache_dir.unwrap_or_else(|| find_workspace_root(Path::new(".")).join(".cddm"));

    println!(
        "\x1b[36m--> Importing cache pack from '{}' into '{}'...\x1b[0m",
        pack_file.display(),
        target_dir.display()
    );

    let summary = import_cache_pack(&pack_file, &target_dir)?;
    print_pack_summary(&summary, "Entries Imported");
    Ok(())
}
