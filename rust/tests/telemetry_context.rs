#![cfg(feature = "telemetry")]
#[path = "support/telemetry.rs"]
mod support;
use miilo_adapter_sdk::telemetry::Operation;
use opentelemetry::{
    Context,
    trace::{SpanId, TraceContextExt},
};
use support::{fixture, operation};

#[test]
fn explicit_root_does_not_inherit_an_unrelated_ambient_conversation() {
    let (_provider, exporter, telemetry) = fixture(true);
    let ambient = telemetry
        .record(&operation(), &Context::new())
        .unwrap()
        .unwrap();
    let _guard = ambient.attach();
    telemetry.record(&operation(), &Context::new()).unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans[1].parent_span_id, SpanId::INVALID);
    assert_ne!(
        spans[0].span_context.trace_id(),
        spans[1].span_context.trace_id()
    );
}

#[test]
fn remote_agent_kind_and_trace_identity_are_preserved_for_children() {
    let (_provider, exporter, telemetry) = fixture(true);
    let mut record = operation();
    record.operation = Operation::InvokeAgent {
        name: None,
        remote: true,
    };
    let parent = telemetry.record(&record, &Context::new()).unwrap().unwrap();
    telemetry.record(&operation(), &parent).unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans[0].name, "invoke_agent");
    assert_eq!(spans[0].span_kind, opentelemetry::trace::SpanKind::Client);
    assert_eq!(
        spans[1].span_context.trace_id(),
        parent.span().span_context().trace_id()
    );
    assert_eq!(
        spans[1].parent_span_id,
        parent.span().span_context().span_id()
    );
}
