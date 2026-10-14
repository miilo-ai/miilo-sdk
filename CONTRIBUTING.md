# Contributing to the Miilo SDK

Miilo SDK is the provider-neutral interoperability boundary for observing agent,
automation, CI/CD, and research work. Contributions should improve that public
contract without embedding a specific provider or commercial Miilo feature.

## Development

1. Create a focused branch.
2. Change the JSON Schema and canonical fixture first.
3. Update the Rust, Python, and TypeScript bindings in the same change.
4. Run `make test`.
5. Describe compatibility and security effects in the pull request.

Keep credentials, private endpoints, customer data, executable payloads, and
provider-specific orchestration out of fixtures. Synthetic provider extensions
must use a globally unique namespace.

## Compatibility

Additive optional fields are compatible within protocol `0.1`. Renames, changed
meaning, new required fields, and removed values require an explicit protocol
version decision. Capability discovery is preferred to product-version checks.

## Pull requests

Pull requests should be small enough to review as one protocol decision. They
must include tests that deserialize the changed fixtures in every supported
binding. By contributing, you agree that your contribution is licensed under
Apache-2.0.
