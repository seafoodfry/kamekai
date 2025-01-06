use anyhow::{Context, Result};
use opentelemetry::global;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry_sdk::Resource;
use std::time::Duration;
use tonic::metadata::MetadataMap;

const SVC_NAME: &str = "kamekai";

/// Initialize OpenTelemetry with Honeycomb OTLP exporter.
pub fn init_tracer(honeycomb_api_key: String) -> Result<()> {
    // Create OTLP exporter for Honeycomb.
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic() // Use tonic as the gRPC layer.
        .with_endpoint("https://api.honeycomb.io:443")
        .with_timeout(Duration::from_secs(3))
        .with_metadata(create_honeycomb_headers(honeycomb_api_key)?)
        .build()?;

    // Create and set tracer provider with batch span processor.
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, Tokio)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::new(vec![opentelemetry::KeyValue::new(
            "service.name",
            SVC_NAME,
        )]))
        .build();

    global::set_tracer_provider(provider);

    Ok(())
}

fn create_honeycomb_headers(api_key: String) -> Result<MetadataMap> {
    let mut map = MetadataMap::new();
    map.insert(
        "x-honeycomb-team",
        api_key.parse().context("Invalid API key format")?,
    );
    Ok(map)
}

// Shutdown telemetry provider.
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}
