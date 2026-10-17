# Runtime extraction evidence — 2026-10-16/17

Host: macOS arm64. `make test`: protocol 17, Python 6, Rust fixtures 5 and
runtime regressions 4 passed; TypeScript type/conformance check passed.
Rust all-feature/all-target clippy passed with warnings denied.
Covered: two-provider isolation/removal, identical snapshot deduplication,
restart/future/expired cursor rejection, invalid-ingestion atomicity, exact
subject explanation and missing-evidence honesty. Tests use sanitized fixtures,
not a live provider. Installed-artifact/hosted-CI/publication gates remain open.

Actual 0.2 crate/wheel/npm/schema artifacts and checksums passed. A Rust consumer
enabled runtime from the extracted crate; installed wheel and packed npm contract
consumers passed. This extends artifact verification beyond source-only tests.
