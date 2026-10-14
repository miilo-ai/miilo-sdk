use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const MIILO_PROTOCOL_VERSION: &str = "0.1";
pub type Extensions = Map<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Health {
    Healthy,
    Delayed,
    Stale,
    Degraded,
    Unreachable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Lifecycle {
    Queued,
    Active,
    Waiting,
    Complete,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Planning,
    Implementing,
    Verifying,
    Reviewing,
    Promoting,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttentionLevel {
    None,
    Informational,
    Decision,
    Approval,
    Intervention,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubjectKind {
    Provider,
    Workspace,
    Initiative,
    Task,
    Attempt,
    Actor,
    Artifact,
    Evidence,
    Attention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectReference {
    pub kind: SubjectKind,
    pub id: String,
}
