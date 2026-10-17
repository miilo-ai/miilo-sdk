# Architecture boundary checkpoint

Domain/application code depends on explicit ports; executable/runtime adapters
compose concrete infrastructure. `.specify/architecture.json` and its CI workflow
check inward dependencies and module sizes. Negative tests prove reversed imports
and silent size growth are rejected. Empty layer matches fail instead of passing.

Behavioral tests and native consumer qualification remain separate from these
lexical source checks. This checkpoint does not certify release, native UI, hosted
identity or every uninspected file. No release has been made.

## Implemented boundary

Observation::at takes explicit time and epoch. Observation::new is an outer convenience adapter. Runtime errors are a focused module; all checked runtime files are below 200 lines. Ten Rust tests and warnings-denied lint passed.
