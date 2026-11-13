use super::{AttributeValue, MappedSpan, Outcome, SpanKind, SpanSink};
use opentelemetry::{
    Context, KeyValue,
    trace::{Span, SpanKind as OtelKind, Status, TraceContextExt, Tracer},
};

pub struct OpenTelemetrySink<T> {
    tracer: T,
}
impl<T: Tracer> OpenTelemetrySink<T> {
    pub fn new(tracer: T) -> Self {
        Self { tracer }
    }
}
impl<T: Tracer> SpanSink for OpenTelemetrySink<T> {
    type Context = Context;

    fn record(&self, record: MappedSpan, parent: &Context) -> Context {
        let attributes = record
            .attributes
            .into_iter()
            .map(|(key, value)| match value {
                AttributeValue::Text(value) => KeyValue::new(key, value),
                AttributeValue::Integer(value) => KeyValue::new(key, value),
                AttributeValue::Boolean(value) => KeyValue::new(key, value),
            });
        let builder = self
            .tracer
            .span_builder(record.name)
            .with_kind(match record.kind {
                SpanKind::Client => OtelKind::Client,
                SpanKind::Internal => OtelKind::Internal,
            })
            .with_start_time(std::time::SystemTime::from(record.started_at))
            .with_attributes(attributes);
        let mut span = self.tracer.build_with_context(builder, parent);
        if let Outcome::Error(_) = record.outcome {
            // Raw provider errors can contain credentials or user text.
            span.set_status(Status::error("GenAI operation failed"));
        }
        let context = Context::new().with_remote_span_context(span.span_context().clone());
        span.end_with_timestamp(record.ended_at.into());
        context
    }
}
