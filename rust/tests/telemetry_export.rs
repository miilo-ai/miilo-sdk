#![cfg(feature = "telemetry")]
#[path = "support/telemetry.rs"]
mod support;
use miilo_adapter_sdk::telemetry::{
    ErrorKind, GenAiTelemetry, Inference, InferenceKind, OpenTelemetrySink, Operation, Outcome,
};
use opentelemetry::{
    Context, Value,
    trace::{SpanKind, Status, TraceContextExt, TracerProvider},
};
use opentelemetry_sdk::trace::{InMemorySpanExporter, Sampler, SdkTracerProvider};
use support::{at, fixture, operation};

fn attr<'a>(span: &'a opentelemetry_sdk::trace::SpanData, key: &str) -> Option<&'a Value> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| &kv.value)
}

#[test]
fn disabled_by_default_and_after_revocation() {
    let (_provider, exporter, mut telemetry) = fixture(false);
    assert!(
        telemetry
            .record(&operation(), &Context::new())
            .unwrap()
            .is_none()
    );
    assert!(exporter.get_finished_spans().unwrap().is_empty());
    telemetry.set_enabled(true);
    telemetry.record(&operation(), &Context::new()).unwrap();
    telemetry.set_enabled(false);
    telemetry.record(&operation(), &Context::new()).unwrap();
    assert_eq!(exporter.get_finished_spans().unwrap().len(), 1);
}

#[test]
fn real_exporter_receives_semantics_observed_times_and_known_usage_only() {
    let (provider, exporter, telemetry) = fixture(true);
    telemetry.record(&operation(), &Context::new()).unwrap();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let span = &spans[0];
    assert_eq!(span.name, "chat sample-model");
    assert_eq!(span.span_kind, SpanKind::Client);
    assert_eq!(span.start_time, at(1).into());
    assert_eq!(span.end_time, at(2).into());
    assert_eq!(
        attr(span, "gen_ai.provider.name"),
        Some(&Value::from("example"))
    );
    assert_eq!(
        attr(span, "gen_ai.request.model"),
        Some(&Value::from("sample-model"))
    );
    assert_eq!(
        attr(span, "gen_ai.response.model"),
        Some(&Value::from("sample-model-v1"))
    );
    assert_eq!(
        attr(span, "gen_ai.usage.input_tokens"),
        Some(&Value::I64(12))
    );
    assert_eq!(
        attr(span, "gen_ai.usage.output_tokens"),
        Some(&Value::I64(0))
    );
    assert_eq!(
        attr(span, "gen_ai.request.stream"),
        Some(&Value::Bool(true))
    );
    assert_eq!(span.status, Status::Unset);
    assert!(span.events.is_empty());
    let forbidden = [
        "gen_ai.input.messages",
        "gen_ai.output.messages",
        "gen_ai.system_instructions",
        "gen_ai.conversation.id",
        "gen_ai.agent.id",
        "server.address",
        "gen_ai.tool.call.arguments",
        "gen_ai.tool.call.result",
    ];
    for key in forbidden {
        assert!(attr(span, key).is_none(), "Unexpected content key {key}");
    }
    assert_eq!(span.attributes.len(), 7);
}

#[test]
fn agent_inference_and_tool_form_a_trace_with_explicit_parent_contexts() {
    let (_provider, exporter, telemetry) = fixture(true);
    let mut agent = operation();
    agent.operation = Operation::InvokeAgent {
        name: Some("assistant".into()),
        remote: false,
    };
    agent.ended_at = at(9);
    let context = telemetry.record(&agent, &Context::new()).unwrap().unwrap();
    telemetry.record(&operation(), &context).unwrap();
    let mut tool = operation();
    tool.operation = Operation::ExecuteTool {
        name: "verify".into(),
    };
    tool.provider = None;
    telemetry.record(&tool, &context).unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].name, "invoke_agent assistant");
    assert_eq!(spans[0].span_kind, SpanKind::Internal);
    assert_eq!(spans[2].name, "execute_tool verify");
    assert_eq!(spans[2].span_kind, SpanKind::Internal);
    assert_eq!(
        attr(&spans[2], "gen_ai.tool.name"),
        Some(&Value::from("verify"))
    );
    for child in &spans[1..] {
        assert_eq!(
            child.span_context.trace_id(),
            context.span().span_context().trace_id()
        );
        assert_eq!(child.parent_span_id, spans[0].span_context.span_id());
    }
    assert!(attr(&spans[0], "gen_ai.usage.input_tokens").is_none());
}

#[test]
fn unknown_optional_metadata_is_omitted_instead_of_invented() {
    let (_provider, exporter, telemetry) = fixture(true);
    for kind in [
        InferenceKind::Chat,
        InferenceKind::GenerateContent,
        InferenceKind::TextCompletion,
        InferenceKind::Embeddings,
    ] {
        let mut record = operation();
        record.operation = Operation::Inference {
            kind,
            details: Inference::default(),
        };
        telemetry.record(&record, &Context::new()).unwrap();
    }
    for span in exporter.get_finished_spans().unwrap() {
        assert_eq!(span.attributes.len(), 2);
        assert!(attr(&span, "gen_ai.request.model").is_none());
        assert!(attr(&span, "gen_ai.usage.input_tokens").is_none());
    }
}

#[test]
fn safe_error_classes_and_cancellation_do_not_emit_exception_content() {
    let (_provider, exporter, telemetry) = fixture(true);
    let mut record = operation();
    record.outcome = Outcome::Error(ErrorKind::Timeout);
    telemetry.record(&record, &Context::new()).unwrap();
    record.outcome = Outcome::Cancelled;
    telemetry.record(&record, &Context::new()).unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(attr(&spans[0], "error.type"), Some(&Value::from("timeout")));
    assert_eq!(spans[0].status, Status::error("GenAI operation failed"));
    assert_eq!(spans[1].status, Status::Unset);
    assert_eq!(
        attr(&spans[1], "miilo.operation.cancelled"),
        Some(&Value::Bool(true))
    );
    assert!(attr(&spans[1], "error.type").is_none());
    assert!(spans.iter().all(|span| span.events.is_empty()));
}

#[test]
fn host_sampling_is_respected_and_acceptance_is_not_a_delivery_receipt() {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::AlwaysOff)
        .with_simple_exporter(exporter.clone())
        .build();
    let mut telemetry = GenAiTelemetry::new(OpenTelemetrySink::new(provider.tracer("test")));
    telemetry.set_enabled(true);
    assert!(
        telemetry
            .record(&operation(), &Context::new())
            .unwrap()
            .is_some()
    );
    provider.force_flush().unwrap();
    assert!(exporter.get_finished_spans().unwrap().is_empty());
}
