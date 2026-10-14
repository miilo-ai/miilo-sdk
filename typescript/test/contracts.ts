import type {
  ActionInvocation,
  ActionResult,
  MiiloAdapter,
  ObservatoryEvent,
  ObservatorySnapshot,
  VoiceStreamEvent,
  VoiceTextTurnRequest,
} from "../src/index.js";

const snapshot: ObservatorySnapshot = {
  protocolVersion: "0.1",
  snapshotId: "snapshot-1",
  observedAt: "2026-10-13T00:00:00Z",
  cursor: "provider:1",
  providers: [],
  workItems: [],
  actors: [],
  attempts: [],
  artifacts: [],
  evidence: [],
  attention: [],
};

const event: ObservatoryEvent = {
  protocolVersion: "0.1",
  eventId: "event-1",
  cursor: "provider:2",
  providerId: "provider",
  occurredAt: "2026-10-13T00:00:00Z",
  kind: "updated",
  subject: { kind: "provider", id: "provider" },
  summary: "Provider changed.",
  payload: {},
};

const invocation: ActionInvocation = {
  protocolVersion: "0.1",
  requestId: "request-1",
  providerId: "provider",
  actionId: "retry",
  subject: { kind: "task", id: "task-1" },
  expectedRevision: "revision-1",
  confirmation: { method: "manual", confirmedAt: "2026-10-13T00:00:00Z" },
  input: {},
};

const result: ActionResult = {
  protocolVersion: "0.1",
  requestId: invocation.requestId,
  providerId: invocation.providerId,
  actionId: invocation.actionId,
  status: "accepted",
  recordedAt: "2026-10-13T00:00:01Z",
};

const request: VoiceTextTurnRequest = {
  protocolVersion: "0.1",
  text: "What needs me?",
};
const voice: VoiceStreamEvent = {
  type: "done",
  reply: "Nothing needs you.",
  intent: "attention.list",
  actionProposed: false,
  confirmationRequired: false,
};

declare const adapter: MiiloAdapter;
void adapter.describeProvider();
void adapter.getSnapshot(snapshot.cursor);
void adapter.subscribe(event.cursor);
void adapter.invoke(invocation);
void [result, request, voice];
