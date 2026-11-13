//! Synthetic export rehearsal. No microphone, model, credentials or network.
use chrono::DateTime;
use miilo_adapter_sdk::telemetry::{
    CompletedOperation, GenAiTelemetry, Inference, InferenceKind, OpenTelemetrySink, Operation,
    Outcome,
};
use opentelemetry::{Context, trace::TracerProvider};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let mut telemetry =
        GenAiTelemetry::new(OpenTelemetrySink::new(provider.tracer("miilo.gen_ai")));
    telemetry.set_enabled(true);
    let start = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")?.to_utc();
    let mut record = CompletedOperation {
        operation: Operation::InvokeAgent {
            name: Some("assistant".into()),
            remote: false,
        },
        provider: Some("example".into()),
        started_at: start,
        ended_at: start + chrono::Duration::seconds(3),
        outcome: Outcome::Success,
    };
    let parent = telemetry
        .record(&record, &Context::new())?
        .ok_or("Tracing disabled")?;
    record.operation = Operation::ExecuteTool {
        name: "transcribe".into(),
    };
    record.provider = None;
    record.ended_at = start + chrono::Duration::seconds(1);
    telemetry.record(&record, &parent)?;
    record.operation = Operation::Inference {
        kind: InferenceKind::Chat,
        details: Inference {
            model: Some("example-model".into()),
            ..Inference::default()
        },
    };
    record.provider = Some("example".into());
    record.started_at = record.ended_at;
    record.ended_at = start + chrono::Duration::seconds(3);
    telemetry.record(&record, &parent)?;
    provider.force_flush()?;
    for span in exporter.get_finished_spans()? {
        println!(
            "{} [{:?}] {} attributes",
            span.name,
            span.span_kind,
            span.attributes.len()
        );
    }
    provider.shutdown()?;
    Ok(())
}
