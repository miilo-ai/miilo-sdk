# Miilo adapter conformance

Conformance describes what an adapter proves. It does not require providers to share
one workflow or support every Miilo feature.

## Levels

### Level 0: recorded

- Publishes a valid provider descriptor and snapshot fixture.
- Uses normalized lifecycle, phase, attention, and health.
- Preserves source state in `providerState` or namespaced extensions.
- Passes schema and referential-integrity tests.
- Contains no credentials, private URLs, or customer data.

### Level 1: observable

- Produces authenticated live snapshots.
- Supplies freshness and an opaque resumable cursor.
- Publishes stable provider-scoped identifiers.
- Supports native deep links when the provider offers them.
- Applies bounded retry and marks stale or unreachable data honestly.

### Level 2: streaming

- Publishes at-least-once events with stable event IDs.
- Resumes from a cursor or explicitly requests snapshot reconciliation.
- Documents ordering, retention, deduplication, and deletion behavior.
- Avoids leaking raw chain-of-thought, credentials, or unrestricted tool output.

### Level 3: actionable

- Advertises actions only when the current subject and caller may use them.
- Declares scopes, risk, confirmation, expiry, input schema, and idempotency.
- Rejects stale `expectedRevision` values.
- Returns provider-owned outcomes and audit provenance.
- Passes replay, conflict, duplicate-delivery, and authorization tests.

## First-provider mapping

### Public Codex adapter

| Source concept | Miilo concept |
| --- | --- |
| Goal or group of related delegated tasks | Initiative |
| Codex task/thread | Task |
| Turn or bounded execution span | Attempt |
| Agent or environment | Actor |
| Diff, generated file, report, or build | Artifact |
| Test result or review output | Evidence |
| Waiting for input or approval | Attention request |
| Open native, guidance, interrupt | Capability-gated action |
| Task/turn status | `providerState` |

Codex messages and streamed events are summarized into activity suitable for display.
Miilo does not expose hidden reasoning. Provider-native task history remains available
through a deep link when supported.

## Required cross-provider proof

Independent provider adapters must pass the same Level 0–2 test suite.
The combined view must prove that:

- no presentation component switches on provider identity;
- optional capabilities alter controls without altering domain types;
- unsupported promotion or verification features remain absent rather than simulated;
- provider outages affect only their own health and data freshness;
- duplicate events do not duplicate visible activity;
- a full snapshot repairs a missed-event gap;
- voice and gaze resolve subjects through normalized IDs;
- actions are unavailable until their provider advertises current authority.

Level 3 begins with `open_native`, `send_guidance`, and `pause_or_interrupt`. Factory
approval and promotion remain provider-specific until the generalized authority model
passes conflict and stale-revision tests.
