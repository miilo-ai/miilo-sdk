#![cfg(feature = "telemetry")]
#[path = "support/telemetry.rs"]
mod support;
use chrono::{DateTime, Utc};
use miilo_adapter_sdk::telemetry::{
    GenAiTelemetry, Inference, InferenceKind, MappedSpan, Operation, SpanSink,
};
use opentelemetry::Context;
use support::{fixture, operation};

#[test]
fn rejects_unbounded_or_content_shaped_labels_without_exporting_partial_spans() {
    let (_provider, exporter, telemetry) = fixture(true);
    for label in [
        "".into(),
        "x".repeat(129),
        "https://private.example/key".into(),
        "/home/person/model".into(),
        "../model".into(),
        "name\nsecret".into(),
        "prompt text".into(),
        "name@example.com".into(),
    ] {
        let mut record = operation();
        record.provider = Some(label);
        assert!(telemetry.record(&record, &Context::new()).is_err());
    }
    let mut record = operation();
    record.operation = Operation::ExecuteTool {
        name: "a private command".into(),
    };
    assert!(telemetry.record(&record, &Context::new()).is_err());
    assert!(exporter.get_finished_spans().unwrap().is_empty());
}

#[test]
fn unknown_provider_and_invalid_timing_or_token_overflow_fail_before_export() {
    let (_provider, exporter, telemetry) = fixture(true);
    let mut missing = operation();
    missing.provider = None;
    let mut backwards = operation();
    backwards.ended_at = backwards.started_at - chrono::Duration::seconds(1);
    let mut ancient = operation();
    ancient.started_at = DateTime::from_timestamp(-1, 0).unwrap();
    let mut distant = operation();
    distant.ended_at = DateTime::<Utc>::MAX_UTC;
    let mut overflow = operation();
    overflow.operation = Operation::Inference {
        kind: InferenceKind::Chat,
        details: Inference {
            usage: miilo_adapter_sdk::telemetry::TokenUsage {
                input: Some(u64::MAX),
                output: None,
            },
            ..Inference::default()
        },
    };
    for record in [missing, backwards, ancient, distant, overflow] {
        assert!(telemetry.record(&record, &Context::new()).is_err());
    }
    assert!(exporter.get_finished_spans().unwrap().is_empty());
}

#[test]
fn core_uses_a_sink_port_without_opentelemetry_runtime_or_global_state() {
    use std::{cell::RefCell, rc::Rc};
    struct Sink(Rc<RefCell<Vec<MappedSpan>>>);
    impl SpanSink for Sink {
        type Context = usize;
        fn record(&self, span: MappedSpan, parent: &usize) -> usize {
            self.0.borrow_mut().push(span);
            parent + 1
        }
    }
    let records = Rc::new(RefCell::new(Vec::new()));
    let mut telemetry = GenAiTelemetry::new(Sink(records.clone()));
    assert_eq!(telemetry.record(&operation(), &4).unwrap(), None);
    telemetry.set_enabled(true);
    assert_eq!(telemetry.record(&operation(), &4).unwrap(), Some(5));
    assert_eq!(records.borrow()[0].name, "chat sample-model");
}
