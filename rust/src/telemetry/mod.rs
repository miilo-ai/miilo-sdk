//! Explicit GenAI operation records; no automatic capture or global provider setup.
mod domain;
mod mapping;
mod otel;

pub use domain::{
    AttributeValue, CompletedOperation, ErrorKind, Inference, InferenceKind, MappedSpan, Operation,
    Outcome, SpanKind, TelemetryError, TokenUsage,
};
pub use otel::OpenTelemetrySink;

/// Hosts own export transport, sampling, resource identity, flush and shutdown.
pub trait SpanSink {
    type Context;
    fn record(&self, span: MappedSpan, parent: &Self::Context) -> Self::Context;
}

pub struct GenAiTelemetry<S> {
    sink: S,
    enabled: bool,
}
impl<S: SpanSink> GenAiTelemetry<S> {
    /// Supplying a sink does not enable tracing.
    pub fn new(sink: S) -> Self {
        Self {
            sink,
            enabled: false,
        }
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// None means disabled, not a successfully delivered remote span.
    pub fn record(
        &self,
        operation: &CompletedOperation,
        parent: &S::Context,
    ) -> Result<Option<S::Context>, TelemetryError> {
        if !self.enabled {
            return Ok(None);
        }
        Ok(Some(self.sink.record(mapping::map(operation)?, parent)))
    }
}
