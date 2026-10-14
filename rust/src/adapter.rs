use crate::{
    ActionInvocation, ActionResult, ObservatoryEvent, ObservatorySnapshot, ProviderDescriptor,
    VoiceStreamEvent, VoiceTextTurnRequest, VoiceTurnResult,
};
use async_trait::async_trait;
use futures_core::Stream;
use std::{fmt, pin::Pin};

pub type EventStream = Pin<Box<dyn Stream<Item = Result<ObservatoryEvent, AdapterError>> + Send>>;
pub type VoiceEventStream =
    Pin<Box<dyn Stream<Item = Result<VoiceStreamEvent, AdapterError>> + Send>>;

#[derive(Debug)]
pub struct AdapterError(pub String);

impl fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
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

    async fn voice_turn(
        &self,
        _audio: Vec<u8>,
        _filename: &str,
        _content_type: &str,
    ) -> Result<VoiceTurnResult, AdapterError> {
        Err(AdapterError("provider does not support voice turns".into()))
    }

    async fn voice_turn_stream(
        &self,
        _audio: Vec<u8>,
        _filename: &str,
        _content_type: &str,
    ) -> Result<VoiceEventStream, AdapterError> {
        Err(AdapterError(
            "provider does not support voice streaming".into(),
        ))
    }

    async fn voice_text_turn_stream(
        &self,
        _request: VoiceTextTurnRequest,
    ) -> Result<VoiceEventStream, AdapterError> {
        Err(AdapterError(
            "provider does not support text-originated voice streaming".into(),
        ))
    }
}
