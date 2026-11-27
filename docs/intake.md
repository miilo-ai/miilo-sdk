# Intake interoperability

Intake uses existing Miilo WorkItem, AvailableAction, ActionInvocation, ActionResult
and ObservatoryEvent contracts. It does not require a particular issue tracker,
organization, machine, execution factory, or model provider.

A backend advertises `features["intake.v1"] = true`, projects incoming tasks as
WorkItems and puts typed IntakeDetails in `extensions["io.miilo.intake"]`.
Pending decisions use attention `decision`. Advertise only actions the backend
can authorize. Standard semantic IDs are `intake.accept`, `intake.defer` and
`intake.dismiss`. Defer input is `{ "until": "2026-12-01T12:00:00Z" }`.
All mutations use existing ActionInvocation with expectedRevision and a stable
idempotencyKey. Authentication and authorization are enforced by the backend;
a client confirmation method is not a credential or proof of identity.

Decision, execution and source synchronization are independent. Acceptance does
not imply a worker started or an external ticket changed. A receipt must describe
the outcome actually durably recorded. Dismiss is not cancellation or ticket closure.
Unknown intake versions must not enable intake actions. Existing non-intake
providers and read-only observation remain compatible.

Source adapters translate native state. Execution adapters select registered
workflows. The contract contains neither source credentials nor administrator
keys. Keep private source content out of public PRs and notification previews
unless the configured destination and policy allow it.

## Bring a backend

Implement the existing snapshot, event and action adapter contract in any host
language. The reference HTTP client uses `/v1/snapshot`, `/v1/stream` and
`POST /v1/actions`. An event invalidates a snapshot; clients reread authoritative
state rather than applying arbitrary provider payloads. A backend that cannot
replay an old event cursor returns snapshot-required behavior rather than hiding
a gap. Repeated delivery must not repeat decisions or notifications.

The CLI reference manual inbox is an optional implementation for users with no
intake service. It records incoming work and decisions; its execution capability
must remain false until an executor is actually connected. An arbitrary source
can be bridged into a submission by an adapter owned by that user. A webhook is
an observation, never implicit permission to run code.

See protocol/fixtures/intake-details.json and
protocol/schemas/intake-details.schema.json for conformance data. These are
synthetic and contain no deployment-specific information.
