# Miilo Protocol v0.1 compatibility and security

## Compatibility matrix

| Consumer or provider | Snapshot | Events | Actions | Current proof |
| --- | --- | --- | --- | --- |
| Recorded Factory adapter | v0.1 | v0.1 | synthetic contract | Conformance Level 0 |
| Recorded Codex adapter | v0.1 | v0.1 | synthetic contract | Conformance Level 0 |
| Private Factory normalizer | v0.1 | boundary defined | capability mapping | Unit tested |
| Public Codex normalizer | v0.1 | boundary defined | guidance/interrupt mapping | Unit tested |
| Aggregation service | v0.1 | per-connection cursor merge | read-only | Three-connection unit proof |
| TypeScript SDK | v0.1 | v0.1 | v0.1 | Strict compilation |
| Unity C# domain | v0.1 core | deferred | policy types | EditMode tested |

Consumers reject unsupported protocol versions. A minor release may add optional
fields and enum values, so client mappings must preserve unknown provider extensions
and degrade unknown normalized enum values to `unknown` where defined. Breaking field
or semantic changes require a new major protocol version and parallel serving during
migration.

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

- [x] Unity accepts only HTTPS service endpoints.
- [x] Provider credentials remain outside Unity.
- [x] The service supports a narrow read bearer credential.
- [x] Stale fallback removes all available actions and attention prompts.
- [x] Unreachable state is distinct from fresh empty state.
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
- [x] At-least-once events are deduplicated by provider and event ID.
- [x] Multiple instances of one adapter use distinct connection identities.
- [x] Public connection inventory excludes secret references and adapter settings.
- [x] Validation occurs at protocol ingress and aggregate egress.
- [ ] Connection ownership and per-user visibility are enforced.
- [ ] Add/edit/test/disable/remove operations are audited and revision-bound.
- [ ] Durable cursor and deduplication storage replaces process memory before deployment.
- [ ] Metrics, bounded logs, deletion, backup, and incident procedures are configured.
