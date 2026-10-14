use crate::SubjectReference;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRisk {
    Read,
    Bounded,
    Privileged,
    Destructive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationRequirement {
    None,
    Explicit,
    Strong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    pub id: String,
    pub label: String,
    pub risk: ActionRisk,
    pub confirmation: ConfirmationRequirement,
    pub required_scopes: Vec<String>,
    pub available: bool,
    pub unavailable_reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub idempotent: bool,
    pub input_schema: Option<Map<String, Value>>,
    pub native_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    None,
    Manual,
    Voice,
    Gesture,
    StrongAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    pub method: ConfirmationMethod,
    pub confirmed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub protocol_version: String,
    pub request_id: String,
    pub provider_id: String,
    pub action_id: String,
    pub subject: SubjectReference,
    pub expected_revision: String,
    pub confirmation: Confirmation,
    pub input: Map<String, Value>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Accepted,
    Completed,
    Rejected,
    Failed,
    Conflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub protocol_version: String,
    pub request_id: String,
    pub provider_id: String,
    pub action_id: String,
    pub status: ActionStatus,
    pub recorded_at: DateTime<Utc>,
    pub provider_state: Option<String>,
    pub message: Option<String>,
    pub native_url: Option<String>,
    pub result: Option<Map<String, Value>>,
}
