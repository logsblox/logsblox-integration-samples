use std::collections::HashMap;
use std::time::Duration;

use opentelemetry::logs::{LogRecord as _, Logger as _, LoggerProvider as _, Severity};
use opentelemetry::KeyValue;
use opentelemetry_otlp::{Compression, LogExporter, Protocol, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = std::env::var("LOGSBLOX_ENDPOINT").unwrap_or_default();
    let api_key = std::env::var("LOGSBLOX_API_KEY").unwrap_or_default();
    let http_client = Client::builder()
        .timeout(Duration::from_millis(15_000))
        .build()?;

    let exporter = LogExporter::builder()
        .with_http()
        .with_http_client(http_client)
        .with_endpoint(endpoint)
        .with_protocol(Protocol::HttpBinary)
        .with_timeout(Duration::from_millis(15_000))
        .with_headers(HashMap::from([("x-api-key".to_string(), api_key)]))
        .with_compression(Compression::Gzip)
        .build()?;

    let provider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder_empty()
                .with_attributes([
                    KeyValue::new("service.name", "rust-service"),
                    KeyValue::new("deployment.environment", "production"),
                ])
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    let logger = provider.logger("logsblox-logger");
    let mut record = logger.create_log_record();
    record.set_severity_text("INFO");
    record.set_severity_number(Severity::Info);
    record.set_body("Welcome to Logsblox!".into());
    logger.emit(record);
    provider.shutdown()?;
    Ok(())
}
