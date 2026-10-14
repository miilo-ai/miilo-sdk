# Miilo Adapter SDK for Rust

Typed provider contracts for the [Miilo SDK](https://github.com/miilo-ai/miilo-sdk).
The language-neutral JSON Schemas and conformance fixtures live in the repository's
`protocol` directory.

```toml
[dependencies]
miilo-adapter-sdk = "0.1"
```

Implement `MiiloAdapter` to expose a provider without moving its credentials or
orchestration into Miilo.
