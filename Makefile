.PHONY: test test-protocol test-python test-rust test-typescript test-release

test: test-protocol test-python test-rust test-typescript test-compression-interop

test-protocol:
	cd protocol && uv run pytest

test-python:
	cd python && uv run --extra compression pytest

test-rust:
	cargo test --manifest-path rust/Cargo.toml --all-features

test-typescript:
	cd typescript && npm ci --ignore-scripts && npm test

test-release:
	./scripts/test-release-artifacts.sh

test-compression-interop:
	bash scripts/test-compression-interop.sh
