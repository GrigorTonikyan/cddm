#![forbid(unsafe_code)]

use cddm_core::{export_cache_pack, import_cache_pack};
use std::path::PathBuf;

/// Executes the CLI `cddm cache export` command.
pub fn run_cache_export_command(
    cache_dir: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = cache_dir.unwrap_or_else(|| PathBuf::from(".cddm/cache.db"));
    let out_path = output.unwrap_or_else(|| PathBuf::from("cddm-cache.cddmpack"));

    println!(
        "\x1b[36m--> Exporting persistent cache database from '{}' to '{}'...\x1b[0m",
        db_path.display(),
        out_path.display()
    );

    let summary = export_cache_pack(&db_path, &out_path)?;
    println!("\x1b[32m[SUCCESS] {}\x1b[0m", summary.message);
    println!("  Entries:  {}", summary.entry_count);
    println!("  Checksum: {}", summary.checksum);

    Ok(())
}

/// Executes the CLI `cddm cache import` command.
pub fn run_cache_import_command(
    pack_file: PathBuf,
    target_cache_dir: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = target_cache_dir.unwrap_or_else(|| PathBuf::from(".cddm"));

    println!(
        "\x1b[36m--> Importing cache pack from '{}' into '{}'...\x1b[0m",
        pack_file.display(),
        target_dir.display()
    );

    let summary = import_cache_pack(&pack_file, &target_dir)?;
    println!("\x1b[32m[SUCCESS] {}\x1b[0m", summary.message);
    println!("  Entries Imported: {}", summary.entry_count);
    println!("  SHA-256 Checksum: {}", summary.checksum);

    Ok(())
}
