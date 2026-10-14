use crate::{Extensions, Health, SpeechCapabilities};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    pub observe: Vec<String>,
    pub control: Vec<String>,
    pub features: BTreeMap<String, bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speech: Option<SpeechCapabilities>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_routes: Vec<ModelRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelRole {
    Transcription,
    Reasoning,
    Vision,
    Synthesis,
    Embedding,
    Search,
    Specialist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRoute {
    pub role: ModelRole,
    pub logical_name: String,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
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
