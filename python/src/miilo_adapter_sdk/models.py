from __future__ import annotations

from datetime import datetime
from typing import Any, Literal

from pydantic import BaseModel, ConfigDict, Field

Extensions = dict[str, Any]

class Contract(BaseModel):
    model_config = ConfigDict(alias_generator=lambda v: v.split('_')[0] + ''.join(p.title() for p in v.split('_')[1:]), populate_by_name=True, extra='forbid')

class SubjectReference(Contract):
    kind: Literal['provider','workspace','initiative','task','attempt','actor','artifact','evidence','attention']
    id: str

class VoiceActivityDetection(Contract):
    required: bool
    silence_ms: int
    max_utterance_ms: int

class SpeechVoice(Contract):
    id: str
    name: str
    language: str

class SpeechCapabilities(Contract):
    modes: list[Literal['transcription','synthesis','voice_turn','realtime_transcription']]
    languages: list[str]
    voices: list[SpeechVoice]
    input_formats: list[str]
    output_formats: list[str]
    supports_barge_in: bool
    actions_require_confirmation: bool
    voice_activity_detection: VoiceActivityDetection

class ModelRoute(Contract):
    role: Literal['transcription','reasoning','vision','synthesis','embedding','search','specialist']
    logical_name: str
    input_modalities: list[str]
    output_modalities: list[str]
    specialization: str | None = None

class VoiceCandidate(Contract):
    id: str
    provider_id: str
    title: str
    score: float

class VoiceActionProposal(Contract):
    provider_id: str
    work_item_id: str
    expected_revision: str
    action_id: str
    action_label: str
    confirmation: Literal['none','explicit','strong']
    risk: Literal['read','bounded','privileged','destructive']
    input: dict[str, Any] = Field(default_factory=dict)

class VoiceUnderstanding(Contract):
    intent: str
    status: Literal['resolved','ambiguous','unresolved']
    candidates: list[VoiceCandidate]
    proposed_action: VoiceActionProposal | None = None

class VoiceTurnResult(Contract):
    protocol_version: Literal['0.1']
    provider_id: str
    transcript: str
    reply: str
    intent: str
    action_proposed: bool
    confirmation_required: bool
    audio_media_type: str | None = None
    audio_base64: str | None = None
    understanding: VoiceUnderstanding | None = None
    extensions: Extensions | None = None

class VoiceTextTurnRequest(Contract):
    protocol_version: Literal['0.1'] = '0.1'
    text: str
    locale: str | None = None

class VoiceTranscriptEvent(Contract):
    type: Literal['transcript']
    protocol_version: Literal['0.1'] = '0.1'
    sequence: int | None = None
    text: str

class VoiceTextDeltaEvent(Contract):
    type: Literal['text_delta']
    protocol_version: Literal['0.1'] = '0.1'
    sequence: int | None = None
    text: str

class VoiceAudioEvent(Contract):
    type: Literal['audio']
    protocol_version: Literal['0.1'] = '0.1'
    sequence: int | None = None
    text: str
    media_type: str
    audio_base64: str

class VoiceDoneEvent(Contract):
    type: Literal['done']
    protocol_version: Literal['0.1'] = '0.1'
    sequence: int | None = None
    reply: str
    intent: str
    action_proposed: bool
    confirmation_required: bool
    understanding: VoiceUnderstanding | None = None

class VoiceErrorEvent(Contract):
    type: Literal['error']
    protocol_version: Literal['0.1'] = '0.1'
    sequence: int | None = None
    message: str

VoiceStreamEvent = VoiceTranscriptEvent | VoiceTextDeltaEvent | VoiceAudioEvent | VoiceDoneEvent | VoiceErrorEvent

class ProviderCapabilities(Contract):
    observe: list[str]
    control: list[str]
    features: dict[str, bool]
    speech: SpeechCapabilities | None = None
    model_routes: list[ModelRoute] = Field(default_factory=list)

class ProviderDescriptor(Contract):
    id: str
    name: str
    kind: Literal['agent_harness','automation_factory','ci_cd','research','custom']
    health: Literal['healthy','delayed','stale','degraded','unreachable']
    native_url: str | None = None
    capabilities: ProviderCapabilities
    extensions: Extensions | None = None

class AvailableAction(Contract):
    id: str
    label: str
    risk: Literal['read','bounded','privileged','destructive']
    confirmation: Literal['none','explicit','strong']
    required_scopes: list[str]
    available: bool
    unavailable_reason: str | None = None
    expires_at: datetime | None = None
    idempotent: bool
    input_schema: dict[str, Any] | None = None
    native_url: str | None = None

class WorkItem(Contract):
    id: str
    provider_id: str
    workspace_id: str | None = None
    parent_id: str | None = None
    kind: Literal['initiative','task']
    title: str
    summary: str
    lifecycle: Literal['queued','active','waiting','complete','failed','cancelled']
    phase: Literal['planning','implementing','verifying','reviewing','promoting','unknown']
    attention: Literal['none','informational','decision','approval','intervention']
    health: Literal['healthy','delayed','stale','degraded','unreachable']
    provider_state: str
    revision: str
    dependency_ids: list[str]
    actions: list[AvailableAction]
    native_url: str | None = None
    updated_at: datetime
    extensions: Extensions | None = None

class Actor(Contract):
    id: str; provider_id: str
    kind: Literal['human','agent','worker','verifier','service']
    name: str
    health: Literal['healthy','delayed','stale','degraded','unreachable']
    status: str
    capabilities: list[str]
    updated_at: datetime | None = None
    extensions: Extensions | None = None

class Attempt(Contract):
    id: str; provider_id: str; task_id: str; sequence: int
    lifecycle: Literal['queued','active','waiting','complete','failed','cancelled']
    phase: Literal['planning','implementing','verifying','reviewing','promoting','unknown']
    actor_ids: list[str]
    started_at: datetime
    completed_at: datetime | None = None
    summary: str | None = None
    extensions: Extensions | None = None

class Artifact(Contract):
    id: str; provider_id: str; attempt_id: str
    kind: Literal['change_set','file','build','report','model','dataset','preview','other']
    name: str; media_type: str
    digest: str | None = None; uri: str | None = None
    created_at: datetime
    extensions: Extensions | None = None

class Evidence(Contract):
    id: str; provider_id: str; attempt_id: str
    kind: Literal['test','evaluation','review','measurement','attestation','other']
    outcome: Literal['passed','failed','warning','unknown']
    summary: str; recorded_at: datetime
    artifact_ids: list[str] | None = None
    extensions: Extensions | None = None

class AttentionItem(Contract):
    id: str; provider_id: str; subject: SubjectReference
    level: Literal['none','informational','decision','approval','intervention']
    reason: str; message: str; created_at: datetime
    actions: list[AvailableAction]
    extensions: Extensions | None = None

class ObservatorySnapshot(Contract):
    protocol_version: Literal['0.1']
    snapshot_id: str; observed_at: datetime; cursor: str
    providers: list[ProviderDescriptor]
    work_items: list[WorkItem]
    actors: list[Actor]
    attempts: list[Attempt]
    artifacts: list[Artifact]
    evidence: list[Evidence]
    attention: list[AttentionItem]
    extensions: Extensions | None = None

class ObservatoryEvent(Contract):
    protocol_version: Literal['0.1']
    event_id: str; cursor: str; provider_id: str; occurred_at: datetime
    kind: Literal['created','updated','activity','artifact','evidence','attention','action','deleted','heartbeat']
    subject: SubjectReference; summary: str; payload: dict[str, Any]
    extensions: Extensions | None = None

class Confirmation(Contract):
    method: Literal['none','manual','voice','gesture','strong_auth']
    confirmed_at: datetime

class ActionInvocation(Contract):
    protocol_version: Literal['0.1']
    request_id: str; provider_id: str; action_id: str
    subject: SubjectReference; expected_revision: str
    confirmation: Confirmation; input: dict[str, Any]
    idempotency_key: str | None = None

class ActionResult(Contract):
    protocol_version: Literal['0.1']
    request_id: str; provider_id: str; action_id: str
    status: Literal['accepted','completed','rejected','failed','conflict']
    recorded_at: datetime
    provider_state: str | None = None; message: str | None = None
    native_url: str | None = None; result: dict[str, Any] | None = None
