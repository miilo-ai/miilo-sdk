# Candidate implementation and handoff

Candidate: 0.2.0. Responsibility: Rust runtime feature, protocol and three language contracts.

## Shared boundaries

SDK runtime is an opt-in feature in the existing Rust package (not a new native
binding forced into Python/TypeScript). CLI/CE pin SDK source commit
`d2bf4970d922048c88d673795e0887bc76b25622`. Protocol stays 0.1. Local config 2
contains mode/local, private snapshotFile, sessions/groups/project/knownSessions;
no CE URL or credential. Legacy config 1 stays explicit CE publication mode.

One owner-controlled collector per configuration. Selected transcript paths are
canonical and fixed; replacement/truncation fails closed. Native file changes
wake authoritative rereads, with coalescing and a 5-second freshness heartbeat.
Local consumers reject caches older than 15 seconds. Remote consumers subscribe
before bootstrap, read fresh state after invalidations, and acquire a new baseline
after reconnect/history loss. Do not apply arbitrary event deltas over a snapshot.

## Validation

`make test`; all-feature Rust clippy; installed crate/wheel/npm qualification.

Fixture evidence is distinct from live behavior. Current-chat Codex discovery and
import were exercised, but no natural update arrived in the 10-second window.
Omarchy native service/import/repeat-import/future-group and rendering acceptance
belongs to the separate agent on example-host. Record commands, source revision,
OS/architecture and outcome in `.specify/evidence`. Windows local transcript
observation is unsupported in this candidate; remote CLI remains supported.

## Publication

No new release/tag/registry submission is authorized by this file. Candidate
branches are `codex/standalone-runtime`. Missing optional publication credentials
must still produce “No release has been made”. AUR registration is externally
closed; do not script retries. Do not count source checks or skipped CI as a
qualified installed platform artifact. Native acceptance precedes owner release.
