.PHONY: test test-protocol test-python test-rust test-typescript test-release

test: test-protocol test-python test-rust test-typescript

test-protocol:
	cd protocol && uv run pytest

test-python:
	cd python && uv run pytest

test-rust:
	cargo test --manifest-path rust/Cargo.toml

test-typescript:
	cd typescript && npm ci --ignore-scripts && npm test

test-release:
	./scripts/test-release-artifacts.sh
