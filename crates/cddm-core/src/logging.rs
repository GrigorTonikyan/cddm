#![forbid(unsafe_code)]

use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Once;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

static INIT: Once = Once::new();

/// Log verbosity level options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Off,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Off => "off",
        }
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "off" | "none" => Ok(LogLevel::Off),
            other => Err(format!("Unknown log level: '{other}'")),
        }
    }
}

/// Centralized configuration for the CDDM logging subsystem.
#[derive(Debug, Clone, Default)]
pub struct LogConfig {
    pub level: Option<LogLevel>,
    pub verbose: bool,
    pub quiet: bool,
    pub log_file: Option<PathBuf>,
    pub json_format: bool,
}

impl LogConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn with_log_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.log_file = Some(path.into());
        self
    }
}

/// Initialize the global tracing subscriber with environment variables and configuration.
///
/// Priority order:
/// 1. `CDDM_LOG` environment variable
/// 2. `RUST_LOG` environment variable
/// 3. Explicit `config.level`
/// 4. Flags (`config.quiet` -> "error", `config.verbose` -> "debug")
/// 5. Default fallback: "info"
pub fn init_logging(config: &LogConfig) -> Result<(), String> {
    let mut initialized = false;

    INIT.call_once(|| {
        let filter_directive = resolve_filter_directive(config);
        let env_filter =
            EnvFilter::try_new(&filter_directive).unwrap_or_else(|_| EnvFilter::new("info"));

        let stderr_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(true)
            .with_target(false);

        if let Some(ref file_path) = config.log_file
            && let Ok(file) = open_log_file(file_path)
        {
            let file_layer = fmt::layer()
                .with_writer(file)
                .with_ansi(false)
                .with_target(true);

            let subscriber = tracing_subscriber::registry()
                .with(env_filter)
                .with(stderr_layer)
                .with(file_layer);

            let _ = subscriber.try_init();
            initialized = true;
            return;
        }

        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(stderr_layer);

        let _ = subscriber.try_init();
        initialized = true;
    });

    if initialized {
        tracing::debug!("CDDM structured logging subsystem initialized");
    }

    Ok(())
}

/// Initialize default logging subscriber writing to stderr.
pub fn init_default_logging() {
    let config = LogConfig::default();
    let _ = init_logging(&config);
}

fn resolve_filter_directive(config: &LogConfig) -> String {
    if let Ok(env_val) = std::env::var("CDDM_LOG")
        && !env_val.trim().is_empty()
    {
        return env_val;
    }

    if let Ok(env_val) = std::env::var("RUST_LOG")
        && !env_val.trim().is_empty()
    {
        return env_val;
    }

    if let Some(level) = config.level {
        return level.as_str().to_string();
    }

    if config.quiet {
        return "error".to_string();
    }

    if config.verbose {
        return "debug".to_string();
    }

    "info".to_string()
}

fn open_log_file(path: &Path) -> Result<std::fs::File, std::io::Error> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    OpenOptions::new().create(true).append(true).open(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_parsing() {
        assert_eq!(LogLevel::from_str("trace").unwrap(), LogLevel::Trace);
        assert_eq!(LogLevel::from_str("DEBUG").unwrap(), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("Info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("warning").unwrap(), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("error").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::from_str("off").unwrap(), LogLevel::Off);
        assert!(LogLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_log_config_builder() {
        let config = LogConfig::new()
            .with_level(LogLevel::Debug)
            .with_verbose(true)
            .with_quiet(false);

        assert_eq!(config.level, Some(LogLevel::Debug));
        assert!(config.verbose);
        assert!(!config.quiet);
    }

    #[test]
    fn test_resolve_filter_directive() {
        let config_quiet = LogConfig {
            quiet: true,
            ..Default::default()
        };
        assert_eq!(resolve_filter_directive(&config_quiet), "error");

        let config_verbose = LogConfig {
            verbose: true,
            ..Default::default()
        };
        assert_eq!(resolve_filter_directive(&config_verbose), "debug");

        let config_level = LogConfig {
            level: Some(LogLevel::Trace),
            ..Default::default()
        };
        assert_eq!(resolve_filter_directive(&config_level), "trace");
    }
}
