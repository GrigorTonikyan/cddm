#![forbid(unsafe_code)]

use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing_opentelemetry::OpenTelemetryLayer;

pub const DEFAULT_SERVICE_NAME: &str = "cddm";
pub const ENV_OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
pub const ENV_OTEL_SERVICE_NAME: &str = "OTEL_SERVICE_NAME";

static TRACER_PROVIDER: RwLock<Option<SdkTracerProvider>> = RwLock::new(None);
static TELEMETRY_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// OpenTelemetry configuration for distributed tracing in CI pipelines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryConfig {
    pub endpoint: Option<String>,
    pub service_name: String,
    pub enabled: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl TelemetryConfig {
    /// Discovers telemetry configuration from standard environment variables:
    /// - `OTEL_EXPORTER_OTLP_ENDPOINT`
    /// - `OTEL_SERVICE_NAME`
    pub fn from_env() -> Self {
        let endpoint = std::env::var(ENV_OTEL_EXPORTER_OTLP_ENDPOINT)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let service_name = std::env::var(ENV_OTEL_SERVICE_NAME)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_SERVICE_NAME.to_string());

        let enabled = endpoint.is_some();

        Self {
            endpoint,
            service_name,
            enabled,
        }
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        let ep = endpoint.into();
        self.enabled = !ep.trim().is_empty();
        self.endpoint = Some(ep);
        self
    }

    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.service_name = name.into();
        self
    }
}

/// Initialize the OpenTelemetry tracer provider if an OTLP endpoint is configured.
pub fn init_telemetry(config: &TelemetryConfig) -> Result<Option<SdkTracerProvider>, String> {
    if !config.enabled {
        return Ok(None);
    }

    let Some(ref endpoint) = config.endpoint else {
        return Ok(None);
    };

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .build()
        .map_err(|e| format!("Failed to create OTLP span exporter: {e}"))?;

    let resource = Resource::builder_empty()
        .with_attributes(vec![KeyValue::new(
            "service.name",
            config.service_name.clone(),
        )])
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    if let Ok(mut lock) = TRACER_PROVIDER.write() {
        *lock = Some(provider.clone());
    }
    TELEMETRY_INITIALIZED.store(true, Ordering::SeqCst);

    tracing::info!(
        service = %config.service_name,
        endpoint = %endpoint,
        "Initialized OpenTelemetry distributed tracing exporter"
    );

    Ok(Some(provider))
}

/// Create an OpenTelemetry tracing-subscriber layer from an initialized provider.
pub fn create_telemetry_layer<S>(
    provider: &SdkTracerProvider,
) -> OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>
where
    S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    let tracer = provider.tracer("cddm");
    OpenTelemetryLayer::new(tracer)
}

/// Flush in-flight spans and cleanly shut down the OpenTelemetry tracer provider.
pub fn shutdown_telemetry() {
    if TELEMETRY_INITIALIZED.swap(false, Ordering::SeqCst)
        && let Ok(mut lock) = TRACER_PROVIDER.write()
        && let Some(provider) = lock.take()
    {
        if let Err(e) = provider.shutdown() {
            tracing::warn!("Failed to cleanly shut down OpenTelemetry tracer provider: {e}");
        } else {
            tracing::debug!("OpenTelemetry tracer provider shut down successfully");
        }
    }
}

/// Check if distributed telemetry is currently active.
pub fn is_telemetry_active() -> bool {
    TELEMETRY_INITIALIZED.load(Ordering::Relaxed)
}

/// Strongly-typed structured span constructors for core deduplication pipeline phases.
pub mod spans {
    use tracing::Span;

    /// Creates a structured span for candidate file discovery.
    pub fn discovery_span(directory: &str, min_tokens: usize) -> Span {
        tracing::info_span!(
            "cddm.discovery",
            otel.name = "cddm.discovery",
            directory = %directory,
            min_tokens = min_tokens,
            total_files = tracing::field::Empty,
        )
    }

    /// Creates a structured span for source tokenization and fingerprinting.
    pub fn tokenization_span(total_files: usize) -> Span {
        tracing::info_span!(
            "cddm.tokenization",
            otel.name = "cddm.tokenization",
            total_files = total_files,
            cached_files = tracing::field::Empty,
            newly_tokenized = tracing::field::Empty,
        )
    }

    /// Creates a structured span for AST parsing of candidate files.
    pub fn ast_parsing_span(file_path: &str, language: &str) -> Span {
        tracing::debug_span!(
            "cddm.ast_parsing",
            otel.name = "cddm.ast_parsing",
            file = %file_path,
            language = %language,
            token_count = tracing::field::Empty,
        )
    }

    /// Creates a structured span for winnowing fingerprint generation.
    pub fn winnowing_span(k_gram: usize, window: usize) -> Span {
        tracing::debug_span!(
            "cddm.winnowing",
            otel.name = "cddm.winnowing",
            k_gram = k_gram,
            window = window,
            fingerprints_count = tracing::field::Empty,
        )
    }

    /// Creates a structured span for clone clustering and scoring.
    pub fn clustering_span(total_pairs: usize) -> Span {
        tracing::info_span!(
            "cddm.clustering",
            otel.name = "cddm.clustering",
            total_pairs = total_pairs,
            total_clusters = tracing::field::Empty,
        )
    }
}

pub use spans::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn test_telemetry_config_defaults() {
        let config = TelemetryConfig {
            endpoint: None,
            service_name: DEFAULT_SERVICE_NAME.to_string(),
            enabled: false,
        };
        assert!(!config.enabled);
        assert_eq!(config.service_name, "cddm");
        assert!(config.endpoint.is_none());
    }

    #[test]
    fn test_telemetry_config_builder() {
        let config = TelemetryConfig::default()
            .with_endpoint("http://localhost:4318")
            .with_service_name("cddm-ci");

        assert!(config.enabled);
        assert_eq!(config.endpoint, Some("http://localhost:4318".to_string()));
        assert_eq!(config.service_name, "cddm-ci");
    }

    #[test]
    fn test_init_telemetry_disabled() {
        let config = TelemetryConfig {
            endpoint: None,
            service_name: "test".to_string(),
            enabled: false,
        };
        let result = init_telemetry(&config);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_spans_creation() {
        let _guard = tracing_subscriber::registry().set_default();

        let disc = discovery_span(".", 50);
        assert_eq!(disc.metadata().unwrap().name(), "cddm.discovery");

        let tok = tokenization_span(10);
        assert_eq!(tok.metadata().unwrap().name(), "cddm.tokenization");

        let ast = ast_parsing_span("main.rs", "Rust");
        assert_eq!(ast.metadata().unwrap().name(), "cddm.ast_parsing");

        let win = winnowing_span(25, 29);
        assert_eq!(win.metadata().unwrap().name(), "cddm.winnowing");

        let clus = clustering_span(5);
        assert_eq!(clus.metadata().unwrap().name(), "cddm.clustering");
    }

    #[test]
    fn test_shutdown_telemetry_when_not_active() {
        shutdown_telemetry();
        assert!(!is_telemetry_active());
    }
}
