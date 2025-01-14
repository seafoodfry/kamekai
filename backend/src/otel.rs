use anyhow::{Context, Result};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::attribute::{SERVICE_NAME, SERVICE_VERSION};
use std::time::Duration;
use tonic::metadata::MetadataMap;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SVC_NAME: &str = "kamekai";

/// Initialize OpenTelemetry with Honeycomb OTLP exporter.
pub fn init_tracer(honeycomb_api_key: String, enable_ansi: bool) -> Result<()> {
    // Set propagator.
    global::set_text_map_propagator(opentelemetry_sdk::propagation::TraceContextPropagator::new());

    // Create resource attributes.
    let resource = Resource::new(vec![
        KeyValue::new(SERVICE_NAME, SVC_NAME),
        KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
    ]);

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
        .with_resource(resource)
        .build();

    global::set_tracer_provider(provider);

    // Set up the tracing subscriber with both fmt and opentelemetry layers
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(enable_ansi)
                .with_file(true)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
        .with(tracing_opentelemetry::layer())
        .init();

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
