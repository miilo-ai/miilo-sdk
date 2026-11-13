<h1 align="center">Miilo Adapter SDK for Rust</h1>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-b7410e?style=flat-square&amp;logo=rust&amp;logoColor=white" />
</p>

Typed provider contracts for the [Miilo SDK](https://github.com/miilo-ai/miilo-sdk).
The language-neutral JSON Schemas and conformance fixtures live in the repository's
`protocol` directory.

```toml
[dependencies]
miilo-adapter-sdk = "0.1"
```

Implement `MiiloAdapter` to expose a provider without moving its credentials or
orchestration into Miilo.

The optional `telemetry` feature provides `telemetry::GenAiTelemetry` and
`telemetry::OpenTelemetrySink` for explicit, content-free GenAI operation records.
Tracing remains disabled until enabled by the host. The host supplies its tracer
and owns sampling/export configuration; this crate never installs a global
provider or collects prompts, audio or transcripts. See the repository's
[tracing guide](https://github.com/miilo-ai/miilo-sdk/blob/main/docs/genai-telemetry.md).
