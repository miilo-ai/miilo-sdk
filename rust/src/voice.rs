use crate::{ActionRisk, ConfirmationRequirement, Extensions, MIILO_PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understanding: Option<VoiceUnderstanding>,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceTextTurnRequest {
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceCandidate {
    pub id: String,
    pub provider_id: String,
    pub title: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceActionProposal {
    pub provider_id: String,
    pub work_item_id: String,
    pub expected_revision: String,
    pub action_id: String,
    pub action_label: String,
    pub confirmation: ConfirmationRequirement,
    pub risk: ActionRisk,
    #[serde(default)]
    pub input: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceResolutionStatus {
    Resolved,
    Ambiguous,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceUnderstanding {
    pub intent: String,
    pub status: VoiceResolutionStatus,
    pub candidates: Vec<VoiceCandidate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposed_action: Option<VoiceActionProposal>,
}

fn default_protocol_version() -> String {
    MIILO_PROTOCOL_VERSION.to_owned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum VoiceStreamEvent {
    Transcript {
        #[serde(default = "default_protocol_version")]
        protocol_version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sequence: Option<u64>,
        text: String,
    },
    TextDelta {
        #[serde(default = "default_protocol_version")]
        protocol_version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sequence: Option<u64>,
        text: String,
    },
    Audio {
        #[serde(default = "default_protocol_version")]
        protocol_version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sequence: Option<u64>,
        text: String,
        media_type: String,
        audio_base64: String,
    },
    Done {
        #[serde(default = "default_protocol_version")]
        protocol_version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sequence: Option<u64>,
        reply: String,
        intent: String,
        action_proposed: bool,
        confirmation_required: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        understanding: Option<VoiceUnderstanding>,
    },
    Error {
        #[serde(default = "default_protocol_version")]
        protocol_version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sequence: Option<u64>,
        message: String,
    },
}
