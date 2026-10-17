//! Launcher chat contract. Text only; tool execution is deliberately absent.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatFocus {
    pub work_key: String,
    pub revision_key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatRequest {
    pub protocol_version: String,
    pub request_id: String,
    pub text: String,
    #[serde(default)]
    pub focus: Option<ChatFocus>,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}
impl ChatRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        let bounded = |s: &str, max: usize| !s.trim().is_empty() && s.chars().count() <= max;
        if self.protocol_version != crate::MIILO_PROTOCOL_VERSION
            || !bounded(&self.request_id, 80)
            || !bounded(&self.text, 4000)
            || self.history.len() > 8
            || self.history.iter().any(|m| !bounded(&m.text, 4000))
            || self
                .focus
                .as_ref()
                .is_some_and(|f| !bounded(&f.work_key, 128) || !bounded(&f.revision_key, 128))
        {
            return Err("Invalid or oversized chat request");
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatReference {
    pub work_key: String,
    pub revision_key: String,
    pub title: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum ChatEvent {
    Context {
        protocol_version: String,
        request_id: String,
        sequence: u64,
        status: String,
        references: Vec<ChatReference>,
        omitted: usize,
    },
    TextDelta {
        protocol_version: String,
        request_id: String,
        sequence: u64,
        text: String,
    },
    Done {
        protocol_version: String,
        request_id: String,
        sequence: u64,
    },
    Error {
        protocol_version: String,
        request_id: String,
        sequence: u64,
        code: String,
        message: String,
    },
}
