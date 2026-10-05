use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, pin::Pin};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub observe: Vec<String>,
    pub control: Vec<String>,
    pub features: BTreeMap<String, bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speech: Option<SpeechCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpeechMode {
    Transcription,
    Synthesis,
    VoiceTurn,
    RealtimeTranscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechVoice {
    pub id: String,
    pub name: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechCapabilities {
    pub modes: Vec<SpeechMode>,
    pub languages: Vec<String>,
    pub voices: Vec<SpeechVoice>,
    pub input_formats: Vec<String>,
    pub output_formats: Vec<String>,
    pub supports_barge_in: bool,
    pub actions_require_confirmation: bool,
    pub voice_activity_detection: VoiceActivityDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceActivityDetection {
    pub required: bool,
    pub silence_ms: u32,
    pub max_utterance_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceTurnResult {
    pub protocol_version: String,
    pub provider_id: String,
    pub transcript: String,
    pub reply: String,
    pub intent: String,
    pub action_proposed: bool,
    pub confirmation_required: bool,
    pub audio_media_type: Option<String>,
    pub audio_base64: Option<String>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    AgentHarness,
    AutomationFactory,
    CiCd,
    Research,
    Custom,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDescriptor {
    pub id: String,
    pub name: String,
    pub kind: ProviderKind,
    pub health: Health,
    pub native_url: Option<String>,
    pub capabilities: ProviderCapabilities,
    pub extensions: Option<Extensions>,
}

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

pub type EventStream = Pin<Box<dyn Stream<Item = Result<ObservatoryEvent, AdapterError>> + Send>>;
#[derive(Debug)]
pub struct AdapterError(pub String);
impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for AdapterError {}

#[async_trait]
pub trait MiiloAdapter: Send + Sync {
    async fn describe_provider(&self) -> Result<ProviderDescriptor, AdapterError>;
    async fn get_snapshot(&self, cursor: Option<&str>)
    -> Result<ObservatorySnapshot, AdapterError>;
    async fn subscribe(&self, cursor: Option<&str>) -> Result<EventStream, AdapterError>;
    async fn invoke(&self, action: ActionInvocation) -> Result<ActionResult, AdapterError>;
}
