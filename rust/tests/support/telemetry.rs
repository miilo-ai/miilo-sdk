use chrono::{DateTime, Utc};
use miilo_adapter_sdk::telemetry::{
    CompletedOperation, GenAiTelemetry, Inference, InferenceKind, OpenTelemetrySink, Operation,
    Outcome, TokenUsage,
};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};

pub fn at(second: u32) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&format!("2026-01-01T00:00:{second:02}Z"))
        .unwrap()
        .to_utc()
}
pub fn operation() -> CompletedOperation {
    CompletedOperation {
        operation: Operation::Inference {
            kind: InferenceKind::Chat,
            details: Inference {
                model: Some("sample-model".into()),
                response_model: Some("sample-model-v1".into()),
                usage: TokenUsage {
                    input: Some(12),
                    output: Some(0),
                },
                streaming: Some(true),
            },
        },
        provider: Some("example".into()),
        started_at: at(1),
        ended_at: at(2),
        outcome: Outcome::Success,
    }
}
pub fn fixture(
    enabled: bool,
) -> (
    SdkTracerProvider,
    InMemorySpanExporter,
    GenAiTelemetry<OpenTelemetrySink<opentelemetry_sdk::trace::Tracer>>,
) {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let mut telemetry =
        GenAiTelemetry::new(OpenTelemetrySink::new(provider.tracer("miilo.gen_ai")));
    telemetry.set_enabled(enabled);
    (provider, exporter, telemetry)
}
