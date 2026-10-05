export const MIILO_PROTOCOL_VERSION = "0.1" as const;

export type Lifecycle = "queued" | "active" | "waiting" | "complete" | "failed" | "cancelled";
export type Phase = "planning" | "implementing" | "verifying" | "reviewing" | "promoting" | "unknown";
export type AttentionLevel = "none" | "informational" | "decision" | "approval" | "intervention";
export type Health = "healthy" | "delayed" | "stale" | "degraded" | "unreachable";
export type SubjectKind = "provider" | "workspace" | "initiative" | "task" | "attempt" | "actor" | "artifact" | "evidence" | "attention";
export type Extensions = Readonly<Record<string, unknown>>;

export interface SubjectReference { kind: SubjectKind; id: string }
export type SpeechMode = "transcription" | "synthesis" | "voice_turn" | "realtime_transcription";
export interface SpeechVoice { id: string; name: string; language: string }
export interface VoiceActivityDetection { required: boolean; silenceMs: number; maxUtteranceMs: number }
export interface SpeechCapabilities { modes: readonly SpeechMode[]; languages: readonly string[]; voices: readonly SpeechVoice[]; inputFormats: readonly string[]; outputFormats: readonly string[]; supportsBargeIn: boolean; actionsRequireConfirmation: boolean; voiceActivityDetection: VoiceActivityDetection }
export interface ProviderCapabilities { observe: readonly string[]; control: readonly string[]; features: Readonly<Record<string, boolean>>; speech?: SpeechCapabilities }
export interface ProviderDescriptor { id: string; name: string; kind: "agent_harness" | "automation_factory" | "ci_cd" | "research" | "custom"; health: Health; nativeUrl?: string; capabilities: ProviderCapabilities; extensions?: Extensions }
export interface VoiceTurnResult { protocolVersion: typeof MIILO_PROTOCOL_VERSION; providerId: string; transcript: string; reply: string; intent: string; actionProposed: boolean; confirmationRequired: boolean; audioMediaType?: string; audioBase64?: string; extensions?: Extensions }
export interface AvailableAction { id: string; label: string; risk: "read" | "bounded" | "privileged" | "destructive"; confirmation: "none" | "explicit" | "strong"; requiredScopes: readonly string[]; available: boolean; unavailableReason?: string; expiresAt?: string; idempotent: boolean; inputSchema?: Readonly<Record<string, unknown>>; nativeUrl?: string }
export interface WorkItem { id: string; providerId: string; workspaceId?: string; parentId?: string; kind: "initiative" | "task"; title: string; summary: string; lifecycle: Lifecycle; phase: Phase; attention: AttentionLevel; health: Health; providerState: string; revision: string; dependencyIds: readonly string[]; actions: readonly AvailableAction[]; nativeUrl?: string; updatedAt: string; extensions?: Extensions }
export interface Actor { id: string; providerId: string; kind: "human" | "agent" | "worker" | "verifier" | "service"; name: string; health: Health; status: string; capabilities: readonly string[]; updatedAt?: string; extensions?: Extensions }
export interface Attempt { id: string; providerId: string; taskId: string; sequence: number; lifecycle: Lifecycle; phase: Phase; actorIds: readonly string[]; startedAt: string; completedAt?: string; summary?: string; extensions?: Extensions }
export interface Artifact { id: string; providerId: string; attemptId: string; kind: "change_set" | "file" | "build" | "report" | "model" | "dataset" | "preview" | "other"; name: string; mediaType: string; digest?: string; uri?: string; createdAt: string; extensions?: Extensions }
export interface Evidence { id: string; providerId: string; attemptId: string; kind: "test" | "evaluation" | "review" | "measurement" | "attestation" | "other"; outcome: "passed" | "failed" | "warning" | "unknown"; summary: string; recordedAt: string; artifactIds?: readonly string[]; extensions?: Extensions }
export interface AttentionItem { id: string; providerId: string; subject: SubjectReference; level: AttentionLevel; reason: string; message: string; createdAt: string; actions: readonly AvailableAction[]; extensions?: Extensions }
export interface ObservatorySnapshot { protocolVersion: typeof MIILO_PROTOCOL_VERSION; snapshotId: string; observedAt: string; cursor: string; providers: readonly ProviderDescriptor[]; workItems: readonly WorkItem[]; actors: readonly Actor[]; attempts: readonly Attempt[]; artifacts: readonly Artifact[]; evidence: readonly Evidence[]; attention: readonly AttentionItem[]; extensions?: Extensions }
export interface ObservatoryEvent { protocolVersion: typeof MIILO_PROTOCOL_VERSION; eventId: string; cursor: string; providerId: string; occurredAt: string; kind: "created" | "updated" | "activity" | "artifact" | "evidence" | "attention" | "action" | "deleted" | "heartbeat"; subject: SubjectReference; summary: string; payload: Readonly<Record<string, unknown>>; extensions?: Extensions }
export interface ActionInvocation { protocolVersion: typeof MIILO_PROTOCOL_VERSION; requestId: string; providerId: string; actionId: string; subject: SubjectReference; expectedRevision: string; confirmation: { method: "none" | "manual" | "voice" | "gesture" | "strong_auth"; confirmedAt: string }; input: Readonly<Record<string, unknown>>; idempotencyKey?: string }
export interface ActionResult { protocolVersion: typeof MIILO_PROTOCOL_VERSION; requestId: string; providerId: string; actionId: string; status: "accepted" | "completed" | "rejected" | "failed" | "conflict"; recordedAt: string; providerState?: string; message?: string; nativeUrl?: string; result?: Readonly<Record<string, unknown>> }

export interface MiiloAdapter {
  describeProvider(): Promise<ProviderDescriptor>;
  getSnapshot(cursor?: string): Promise<ObservatorySnapshot>;
  subscribe(cursor?: string, signal?: AbortSignal): AsyncIterable<ObservatoryEvent>;
  invoke(action: ActionInvocation): Promise<ActionResult>;
}

export interface ReadOnlyMiiloAdapter extends Omit<MiiloAdapter, "invoke"> { invoke?: never }
