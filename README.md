# Miilo Adapter SDK

Public contracts for connecting automation systems, agent harnesses, CI/CD systems, and research services to Miilo.

The repository is provider-neutral. It contains the versioned JSON protocol, conformance fixtures, and supported SDKs for Rust, Python, and TypeScript. Provider credentials and orchestration remain behind each adapter boundary.

## Layout

- `protocol/`: JSON Schema contracts, recorded Factory and Codex fixtures, and schema tests.
- `rust/`: strongly typed Rust contracts and adapter trait.
- `python/`: Pydantic contracts and adapter protocol.
- `typescript/`: TypeScript contracts and adapter interface.
- `docs/`: adapter conformance and security requirements.

## Validate

Run `make test`. Each language SDK must deserialize both reference providers and the canonical protocol tests must pass.

This repository currently has no remote and no selected distribution license. Choose a license before public release.
