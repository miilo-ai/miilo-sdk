# Miilo Protocol v0.1 compatibility and security

## Compatibility matrix

| Consumer or provider | Snapshot | Events | Actions | Current proof |
| --- | --- | --- | --- | --- |
| Recorded Factory fixture | v0.1 | v0.1 | representative action | JSON Schema plus Rust/Python fixture tests |
| Recorded Codex fixture | v0.1 | v0.1 | representative action | JSON Schema plus Rust/Python fixture tests |
| Rust SDK | v0.1 | v0.1 | v0.1 | Deserialization, round-trip, adapter trait, and package proof |
| Python SDK | v0.1 | v0.1 | v0.1 | Strict Pydantic fixtures, adapter protocol, and wheel/sdist proof |
| TypeScript SDK | v0.1 | v0.1 | v0.1 | Strict contract and adapter surface compilation plus npm package proof |
| Miilo CLI | v0.1 | resumable SSE | read-only | Rust and Node unit, package, mock, and CE journeys |
| Miilo CE | v0.1 | replay plus live SSE | validation only | Ingress invariants, provider isolation, explanation, and cross-repository journey |

Current consumers require the exact `0.1` wire version and strict normalized fields.
Provider-specific additive data belongs under the namespaced `extensions` object.
Adding a normalized field or enum value therefore requires a coordinated protocol
version and compatibility tests; breaking field or semantic changes require parallel
serving during migration.

## Security checklist

### Adapter boundary

- [x] Provider identity is explicit on every provider-owned object.
- [x] Cross-provider objects are rejected by aggregation.
- [x] Provider-specific data is namespaced under `extensions`.
- [x] Recorded fixtures contain synthetic hosts and no credentials.
- [x] Hidden reasoning is excluded from the public activity contract.
- [ ] Live adapters apply allowlisted field projection before normalization.
- [ ] Live adapters document provider retention and deletion behavior.

### Client boundary

- [x] CLI clients reject non-HTTP(S) endpoint schemes.
- [x] Provider credentials remain outside SDK snapshots and CLI configuration.
- [x] CE supports an optional bearer boundary for all observation routes.
- [x] Unreachable provider state is distinct from a fresh empty snapshot.
- [ ] Device identity and token rotation are configured before live deployment.
- [ ] Certificate and endpoint policy are validated on target devices.

### Action boundary

- [x] Mutating actions require explicit or strong confirmation.
- [x] Invocation carries an exact expected revision.
- [x] Idempotency behavior is advertised.
- [x] Expired event cursors require snapshot reconciliation.
- [ ] Live dispatch verifies current provider authorization and action availability.
- [ ] Conflict, replay, expiry, revocation, and audit tests run against each live provider.

### Operational boundary

- [x] Provider failure is isolated and surfaced as stale data.
- [x] Event IDs and provider IDs are explicit so consumers can deduplicate at-least-once delivery.
- [x] Multiple instances of one adapter use distinct connection identities.
- [x] Public connection inventory excludes secret references and adapter settings.
- [x] CE validates protocol, ownership, unique IDs, references, revisions, and action confirmation at ingress.
- [ ] Connection ownership and per-user visibility are enforced.
- [ ] Add/edit/test/disable/remove operations are audited and revision-bound.
- [ ] Durable cursor and deduplication storage replaces process memory before deployment.
- [ ] Metrics, bounded logs, deletion, backup, and incident procedures are configured.
