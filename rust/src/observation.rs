use crate::{
    AttentionLevel, AvailableAction, Extensions, Health, Lifecycle, Phase, ProviderDescriptor,
    SubjectReference,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemKind {
    Initiative,
    Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItem {
    pub id: String,
    pub provider_id: String,
    pub workspace_id: Option<String>,
    pub parent_id: Option<String>,
    pub kind: WorkItemKind,
    pub title: String,
    pub summary: String,
    pub lifecycle: Lifecycle,
    pub phase: Phase,
    pub attention: AttentionLevel,
    pub health: Health,
    pub provider_state: String,
    pub revision: String,
    pub dependency_ids: Vec<String>,
    pub actions: Vec<AvailableAction>,
    pub native_url: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    Human,
    Agent,
    Worker,
    Verifier,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: String,
    pub provider_id: String,
    pub kind: ActorKind,
    pub name: String,
    pub health: Health,
    pub status: String,
    pub capabilities: Vec<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub id: String,
    pub provider_id: String,
    pub task_id: String,
    pub sequence: u32,
    pub lifecycle: Lifecycle,
    pub phase: Phase,
    pub actor_ids: Vec<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub summary: Option<String>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    ChangeSet,
    File,
    Build,
    Report,
    Model,
    Dataset,
    Preview,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: String,
    pub provider_id: String,
    pub attempt_id: String,
    pub kind: ArtifactKind,
    pub name: String,
    pub media_type: String,
    pub digest: Option<String>,
    pub uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Test,
    Evaluation,
    Review,
    Measurement,
    Attestation,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOutcome {
    Passed,
    Failed,
    Warning,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub id: String,
    pub provider_id: String,
    pub attempt_id: String,
    pub kind: EvidenceKind,
    pub outcome: EvidenceOutcome,
    pub summary: String,
    pub recorded_at: DateTime<Utc>,
    pub artifact_ids: Option<Vec<String>>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttentionItem {
    pub id: String,
    pub provider_id: String,
    pub subject: SubjectReference,
    pub level: AttentionLevel,
    pub reason: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub actions: Vec<AvailableAction>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservatorySnapshot {
    pub protocol_version: String,
    pub snapshot_id: String,
    pub observed_at: DateTime<Utc>,
    pub cursor: String,
    pub providers: Vec<ProviderDescriptor>,
    pub work_items: Vec<WorkItem>,
    pub actors: Vec<Actor>,
    pub attempts: Vec<Attempt>,
    pub artifacts: Vec<Artifact>,
    pub evidence: Vec<Evidence>,
    pub attention: Vec<AttentionItem>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Created,
    Updated,
    Activity,
    Artifact,
    Evidence,
    Attention,
    Action,
    Deleted,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservatoryEvent {
    pub protocol_version: String,
    pub event_id: String,
    pub cursor: String,
    pub provider_id: String,
    pub occurred_at: DateTime<Utc>,
    pub kind: EventKind,
    pub subject: SubjectReference,
    pub summary: String,
    pub payload: Map<String, Value>,
    pub extensions: Option<Extensions>,
}
