use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry::{global, KeyValue, trace::TracerProvider as _};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use anyhow::Result;

pub fn init_telemetry(endpoint: &str, service_name: &str) -> Result<()> {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint);

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                service_name.to_string(),
            )])),
        )
        .install_batch(runtime::Tokio)?;

    let tracer = tracer_provider.tracer(service_name.to_string());
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    Registry::default()
        .with(filter)
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_telemetry_basic() {
        // We can't easily test OTLP without a collector.
    }
}
