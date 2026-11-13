# Observation and explanations

## How capture works

Miilo uses source-specific adapters to translate agent activity into a shared
protocol. Each source still needs an adapter; the common contract lets clients
display its observations without implementing another provider-specific UI.

The SDK supplies contracts, adapter interfaces and an optional in-process Rust
runtime. It does not search the filesystem, read agent databases or start a
collector. Those local adapters live in the
[Miilo CLI](https://github.com/miilo-ai/miilo-cli).

Local collection reads only explicitly selected existing sessions. Depending on
the adapter, its source is a JSONL transcript or a read-only session database.
It extracts bounded visible assistant activity and, where supported, tool names.
It excludes hidden reasoning, user/system message bodies, tool arguments and
results, and credentials. Session titles and project paths are discovery metadata
and can still be sensitive. Consult the CLI's
[compatibility matrix](https://github.com/miilo-ai/miilo-cli/blob/main/docs/AGENT_COMPATIBILITY.md)
for exact source formats, privacy exclusions, update behavior and validation limits.

Concurrent work keeps its provider and session identity. Snapshots relate work
items to actors, attempts and evidence through those identifiers; aggregation does
not merge unrelated sessions merely because their titles match. The runtime
validates provider-scoped references and reconciles each provider's snapshot.
An event cursor permits incremental reads; an expired cursor requires a fresh
snapshot. This is observation of what a source records, not proof that an agent
is executing at this instant.

Remote providers implement the same protocol through their own adapters. They
decide which observations and permitted actions to expose. Local transcript
observation does not grant control of those agents.

## What explanations mean

The Rust runtime's `runtime::explain` returns a deterministic explanation built
from the requested provider/work item, its observed state and revision, and
evidence IDs attached to that work item's attempts. It preserves the supplied
title, summary and observation timestamp. When verification evidence is absent,
it reports that limitation explicitly.

This explains **what was observed and which evidence was supplied**. It does not
capture hidden chain-of-thought, reconstruct why a model chose an action, infer a
root cause, or independently verify the truth of a provider's summary. A tool name
alone does not establish what the tool returned or whether the task succeeded.

Provider-authored rationale, if deliberately supplied, remains that provider's
claim. A host may offer a separate model-based explanation, but it is not the
deterministic runtime result and must retain its evidence and uncertainty boundaries.
Neither kind of explanation grants permission to execute an action. See
[adapter conformance](adapter-conformance.md) for capability and authorization rules.
