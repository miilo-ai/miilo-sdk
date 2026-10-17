<h1 align="center">Miilo Protocol v0.1</h1>

<p align="center">
  <img alt="JSON Schema" src="https://img.shields.io/badge/JSON%20Schema-444444?style=flat-square&amp;logo=json&amp;logoColor=white" />
</p>

This directory defines the provider-neutral Miilo protocol and its conformance
fixtures. The protocol describes observable
automated work and capability-gated actions.

## Contracts

- `schemas/observatory-snapshot.schema.json`: provider descriptors, actors,
  initiatives, tasks, attempts, artifacts, evidence, attention, and actions.
- `schemas/observatory-event.schema.json`: ordered, resumable provider events.
- `schemas/action-invocation.schema.json`: an explicitly confirmed action request.
- `schemas/action-result.schema.json`: the provider-owned action outcome.
- Voice action proposals carry `expectedRevision`; clients and the Miilo control plane must bind any confirmation to that exact observed provider state.

Provider-specific values belong in `extensions`, under a reverse-domain or otherwise
globally unique key. Normalized fields drive Miilo presentation. `providerState`
preserves the exact source state for diagnostics.

Providers may advertise `capabilities.speech`. Speech remains provider-owned:
OpenAI or another connected system can supply transcription,
synthesis, and complete voice turns without exposing internal model names or
credentials to Miilo clients. Any action proposed during a voice turn still
uses the normal action contract and confirmation policy.

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

The conformance suite validates synthetic factory and Codex fixtures against the
same contracts. Fixtures contain no private implementation details.
