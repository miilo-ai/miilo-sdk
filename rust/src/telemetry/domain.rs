use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceKind {
    Chat,
    GenerateContent,
    TextCompletion,
    Embeddings,
}
impl InferenceKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::GenerateContent => "generate_content",
            Self::TextCompletion => "text_completion",
            Self::Embeddings => "embeddings",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TokenUsage {
    pub input: Option<u64>,
    pub output: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inference {
    pub model: Option<String>,
    pub response_model: Option<String>,
    pub usage: TokenUsage,
    pub streaming: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Inference {
        kind: InferenceKind,
        details: Inference,
    },
    InvokeAgent {
        name: Option<String>,
        remote: bool,
    },
    ExecuteTool {
        name: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Timeout,
    Provider,
    InvalidResponse,
    Other,
}
impl ErrorKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::Provider => "provider_error",
            Self::InvalidResponse => "invalid_response",
            Self::Other => "_OTHER",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Success,
    Error(ErrorKind),
    Cancelled,
}

/// Actual operation metadata, never a transcript, prompt or snapshot projection.
#[derive(Clone, Debug)]
pub struct CompletedOperation {
    pub operation: Operation,
    pub provider: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub outcome: Outcome,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanKind {
    Client,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeValue {
    Text(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Clone, Debug)]
pub struct MappedSpan {
    pub name: String,
    pub kind: SpanKind,
    pub attributes: Vec<(&'static str, AttributeValue)>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelemetryError(pub &'static str);
impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for TelemetryError {}
