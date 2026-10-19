# Architecture boundary enforcement

> Retained design record. Unchecked historical milestones are not a current release checklist. See docs/release-status.md for current scope and outstanding validation.


Domain/application owns ports and policy; infrastructure implements those ports.
Executable/runtime composition chooses concrete implementations. Core use cases
must run with fake ports. Compatibility entry points remain explicit at the edge.
The constitution and architecture policy describe the enforced source scope.

Observation::at takes explicit time and epoch. Observation::new is an outer convenience adapter. Runtime errors are a focused module; all checked runtime files are below 200 lines. Ten Rust tests and warnings-denied lint passed.
