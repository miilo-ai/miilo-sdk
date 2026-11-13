# GenAI tracing

The optional Rust `telemetry` feature emits completed operation records through
a caller-owned OpenTelemetry tracer. It is disabled by default, including after
constructing `GenAiTelemetry`. Enable it explicitly with `set_enabled(true)`.
This does not change the wire protocol or Python/TypeScript bindings.

The core maps typed records through a `SpanSink` port. `OpenTelemetrySink` is the
OpenTelemetry adapter. It does not initialize a global provider, inspect environment
variables, start a collector, choose an endpoint, install an exporter or read files.
Normal contract-only dependencies do not include OpenTelemetry. Test/example
dependencies include an in-memory exporter.

## Operation mapping

| Record | Span name | Kind |
| --- | --- | --- |
| Chat / generate content / text completion / embeddings | Operation followed by the known request model | Client |
| Local agent invocation | `invoke_agent` followed by a known stable agent name | Internal |
| Remote agent invocation | Same name rule | Client |
| Tool execution | `execute_tool` followed by the tool name | Internal |

Inference and agent records require a known provider label. Inference accepts
known request/response model names, a streaming flag and provider-reported input/output
token totals. Missing optional values are omitted; a reported zero is retained.
Token totals belong to the actual inference, not again to its parent agent/tool.
Unknown model or agent names leave the span name as the operation alone.

The mapping follows the OpenTelemetry GenAI conventions reviewed on November 12,
2026, which are still **Development**. It uses `gen_ai.provider.name`, not the older
`gen_ai.system`. Keep mapping changes explicit and test them when upstream evolves:

- [Inference conventions](https://github.com/open-telemetry/semantic-conventions-genai/blob/main/docs/gen-ai/client-inference.md)
- [Agent and tool conventions](https://github.com/open-telemetry/semantic-conventions-genai/blob/main/docs/gen-ai/gen-ai-agent-spans.md)
- [Attribute registry](https://opentelemetry.io/docs/specs/semconv/registry/attributes/gen-ai/)

## Privacy and accuracy

There is no input for prompts, messages, instructions, audio, transcripts, tool
arguments/results, user/session IDs, URLs, arbitrary attributes or raw error text.
Labels must be bounded machine identifiers, not UI titles or content. Metadata can
still be sensitive: hosts must supply approved, low-cardinality provider/model/agent
labels. Validation is a format guard, not a secret detector or anonymization claim.
Host-added resource/context attributes and exporter configuration remain host-owned.

Supply actual operation start/end timestamps. This module never guesses durations,
tokens, inference calls or tool executions from observation snapshots. A source that
does not know those boundaries must omit the trace rather than manufacture a span.
Errors carry a fixed class and generic status description; user cancellation is
marked `miilo.operation.cancelled=true` without automatically treating it as an error.
No exception events or content capture are emitted.

`record` validates before invoking the sink and returns a context for child spans.
Parents are explicit, preventing unrelated thread-local spans from becoming parents.
Sampling may discard a span, so the returned context is not a remote delivery receipt.
This stateless API records each explicit call; deduplication of replayed source records
belongs to the host. It is not a snapshot-export loop.

## Host integration

Use `OpenTelemetrySink::new(provider.tracer("miilo.gen_ai"))` with your existing
OpenTelemetry 0.33 provider, then `GenAiTelemetry::new(sink)`. The host owns the
provider's service resource, sampling, exporter, bounded batch queue, retry policy,
flush/shutdown and transport credentials. Configure an OTLP exporter or another
backend adapter there. Prefer a bounded batch processor for network exporters;
a synchronous sink can block its caller. Export failures must not interrupt agent
work. Keep endpoint selection and credentials outside observation data and UI source.

For voice integrations, only trace a real model operation as inference. A
Superwhisper subprocess with unknown model/provider is a tool operation such as
`execute_tool transcribe`; do not label wake detection or a whole dictation session
as a model call. Microphone consent and audio processing are separate from tracing.
Disabling tracing stops future records; it cannot retract spans already handed to
the host provider or its exporter queue.

## Rehearsal

```sh
make test-telemetry
cargo run --locked --manifest-path rust/Cargo.toml --features telemetry --example genai-telemetry
```

The example exports three synthetic, linked spans to the real OpenTelemetry
in-memory exporter: agent invocation, transcription tool and chat inference. No
microphone or model is used. Tests cover opt-in/disable behavior, parent identity,
timestamps, known/unknown token counts, error/cancel status, privacy exclusions,
invalid input rejection and host sampling. This verifies the SDK export boundary,
not a live OTLP collector, Superwhisper integration or installed-app acceptance.
