# Security policy

Please do not report vulnerabilities through public issues. Use GitHub's private
security-advisory flow for `miilo-ai/miilo-sdk`.

The SDK is a contract and conformance project. Provider implementations remain
responsible for credential storage, authorization, network isolation, action
confirmation, audit retention, and safe execution. Reports are especially useful
when they demonstrate schema ambiguity, revision bypass, unsafe action defaults,
secret exposure, unbounded payloads, or cross-provider confusion.

Protocol `0.1` is pre-stable. Security fixes may require a compatibility change;
the advisory will state the affected protocol and package versions.
