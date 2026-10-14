# Miilo Adapter SDK

Public contracts for connecting automation systems, agent harnesses, CI/CD systems, and research services to Miilo.

The repository is provider-neutral. It contains the versioned JSON protocol, conformance fixtures, and supported SDKs for Rust, Python, and TypeScript. Provider credentials and orchestration remain behind each adapter boundary.

Provider descriptors may advertise speech capabilities and optional logical
model routes. Model routes describe a role and modalities using a stable alias;
they never expose deployment hosts, credentials, or weight paths. This permits
custom providers to combine local, fine-tuned, and hosted models while Miilo
continues to consume one portable capability contract.

Voice-capable providers may implement either a complete `voice_turn` or the
preferred `voice_turn_stream`. The stream is newline-delimited, versioned JSON
with ordered `transcript`, `text_delta`, `audio`, and terminal `done`/`error`
events. Providers expose intent, candidates, and action proposals as data; Miilo
retains confirmation, current-revision checks, and execution authority. The
reference fixture and schema are in `protocol/fixtures/voice-stream.events.json`
and `protocol/schemas/voice-stream-event.schema.json`.

## Layout

- `protocol/`: JSON Schema contracts, recorded Factory and Codex fixtures, and schema tests.
- `rust/`: strongly typed Rust contracts and adapter trait.
- `python/`: Pydantic contracts and adapter protocol.
- `typescript/`: TypeScript contracts and adapter interface.
- `docs/`: adapter conformance and security requirements.

## Validate

Run `make test`. Each language SDK must deserialize both reference providers and the canonical protocol tests must pass.

## Compatibility

The protocol uses semantic versions independently from each language package. A
minor package release may add optional fields or capabilities. Removing fields,
changing their meaning, or making an optional field required needs a new protocol
major version. Providers must advertise capabilities instead of relying on a
Miilo product version.

## Security boundary

The SDK describes providers; it does not receive or store provider credentials.
Adapters keep orchestration, model endpoints, secrets, and authorization behind
their own boundary. See [`docs/compatibility-security.md`](docs/compatibility-security.md)
before implementing actions or voice proposals.

## Contributing

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md). Every contract change must update
the language bindings, schemas, fixtures, and conformance tests together.

Licensed under Apache-2.0. Miilo is a trademark of its respective owner; the
license does not grant trademark rights.
