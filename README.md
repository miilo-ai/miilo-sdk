# Miilo SDK

**One protocol for observing and safely controlling automated work.**

[![CI](https://github.com/miilo-ai/miilo-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/miilo-ai/miilo-sdk/actions/workflows/ci.yml)
[![Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Miilo should understand a coding agent, private automation factory, CI pipeline,
research service, or local model without owning any of them. This repository is
the contract that makes that possible.

It gives provider authors a small, versioned vocabulary for:

- work, actors, attempts, artifacts, evidence, and attention;
- live resumable events;
- revision-bound actions with explicit risk and confirmation;
- speech, voice activity detection, and streamed voice responses;
- logical model routes that reveal capabilities without leaking infrastructure.

Credentials, orchestration, prompts, model locations, and proprietary logic stay
behind the provider boundary.

## Pick your language

| Package | Directory | What it provides |
| --- | --- | --- |
| Rust | [`rust/`](rust/) | Typed contracts, adapter trait, async streams; opt-in in-process runtime |
| Python | [`python/`](python/) | Pydantic contracts and adapter protocol |
| TypeScript | [`typescript/`](typescript/) | Browser and server contracts and adapter interface |
| JSON Schema | [`protocol/`](protocol/) | Language-neutral schemas, fixtures, and conformance tests |

All implementations describe the same protocol. A provider is conformant only
when its snapshots and events pass the shared fixtures.

## The boundary

```text
provider internals ── adapter ── Miilo protocol ── Miilo client
 secrets/models         │          public data       web/mobile/CLI
 workers/policies       └── keeps private machinery private
```

Miilo receives normalized observations and permitted actions. It does not receive
the provider's credentials or direct access to its workers. Action proposals are
data; the consuming Miilo deployment remains responsible for confirmation,
current-revision checks, authorization, and execution.

## Start an adapter

1. Read [`docs/adapter-conformance.md`](docs/adapter-conformance.md).
2. Implement `describeProvider`, `getSnapshot`, `subscribe`, and `invoke` in one
   supported language.
3. Advertise optional speech and model capabilities rather than assuming them.
4. Add sanitized fixtures for the provider.
5. Run the complete conformance suite.

```bash
make test
```

That command validates the JSON protocol plus Rust, Python, and TypeScript. A
contract change is incomplete until schemas, language bindings, fixtures, and
tests agree.

## Compatibility

The wire protocol is versioned independently from package releases. Current v0.1
consumers are strict: provider-specific additions belong under `extensions`, while
normalized field or enum changes require a coordinated protocol version and
compatibility tests. Breaking semantics require parallel serving during migration.

Read [`docs/compatibility-security.md`](docs/compatibility-security.md) before
exposing actions or voice proposals.

## Contributing and releases

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md). Pull requests run every language
suite. A `v*` tag packages the Rust crate, Python wheel and source distribution,
TypeScript tarball, protocol bundle, checksums, and provenance into a GitHub
Release. Registry publishing remains a separate, deliberate decision.

Apache-2.0. The license does not grant rights to the Miilo name or marks.

## Release automation

See [.github/RELEASING.md](.github/RELEASING.md) for private-repository rehearsals, manifest-driven tags, credential-missing skips and activation. Publication stays disabled until explicitly requested/configured.

## Standalone runtime (candidate 0.2)

Enable Rust feature `runtime` for `ObservatoryRuntime`, validated provider-scoped
ingestion, bounded resumable event history and deterministic grounded explanations.
Default imports remain contract-only. The runtime has no listener, filesystem,
account, Axum or CE dependency. Hosts supply an epoch and observation clock values.
Linux/macOS local discovery, file access and continuous collection live in the
separate public CLI repository, not this package. Python/TypeScript expose the
same protocol contracts; they do not embed another observation runtime.

Register providers, replace their validated snapshots, then read an aggregate or
events after its opaque cursor. Identical ingestions emit no duplicate event.
Foreign, future or expired cursors require a fresh snapshot. A new host process
must choose a new epoch. Disconnect removes only that provider's records.
`runtime::explain` cites supplied evidence and explicitly reports missing evidence;
it does not invent root causes or execution authority.

Run `make test` (including runtime tests) and `cargo clippy --manifest-path
rust/Cargo.toml --all-features --all-targets -- -D warnings`. Candidate source is
not an owner-approved release. Protocol remains 0.1; package version is 0.2.0.

## Architecture checks

The [engineering constitution](.specify/memory/constitution.md) defines the
boundaries. Run `python3 tools/check_architecture.py` and
`python3 tools/test_architecture.py`; CI qualification runs both. These are
development checks, not runtime dependencies.
