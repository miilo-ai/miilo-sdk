# Miilo Protocol v0.1

This directory incubates the public, provider-neutral Miilo protocol before it
moves to its own distributable repository. The protocol describes observable
automated work and capability-gated actions. It contains no Example Factory types.

## Contracts

- `schemas/observatory-snapshot.schema.json`: provider descriptors, actors,
  initiatives, tasks, attempts, artifacts, evidence, attention, and actions.
- `schemas/observatory-event.schema.json`: ordered, resumable provider events.
- `schemas/action-invocation.schema.json`: an explicitly confirmed action request.
- `schemas/action-result.schema.json`: the provider-owned action outcome.

Provider-specific values belong in `extensions`, under a reverse-domain or otherwise
globally unique key. Normalized fields drive Miilo presentation. `providerState`
preserves the exact source state for diagnostics.

## Compatibility

- `protocolVersion` uses major/minor form. Consumers must reject an unsupported major.
- Minor versions may add optional fields and enum values.
- Identifiers are opaque within their provider and must not be parsed for meaning.
- Cursors are opaque, monotonically resumable tokens within one provider stream.
- Events are at-least-once; consumers deduplicate by `(providerId, eventId)`.
- Snapshots are authoritative replacements for the represented provider at a cursor.
- Actions are advertised capabilities. Their presence never bypasses provider authority.

## Validation

Run from this directory:

```bash
uv run python -m pytest -q
```

The conformance suite validates both Example Factory and Codex fixtures against the
same contracts. Example Factory fixture data is synthetic and reveals no private
implementation details.
