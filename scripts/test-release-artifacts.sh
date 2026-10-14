#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
output=$(mktemp -d "${TMPDIR:-/tmp}/miilo-sdk-release.XXXXXX")
export CARGO_HOME="${CARGO_HOME:-/tmp/miilo-sdk-cargo}"
export NPM_CONFIG_CACHE="${NPM_CONFIG_CACHE:-/tmp/miilo-sdk-npm}"
export UV_CACHE_DIR="${UV_CACHE_DIR:-/tmp/miilo-sdk-uv}"
cleanup() { rm -rf "$output"; }
trap cleanup EXIT INT TERM

cargo package --manifest-path "$root/rust/Cargo.toml" --locked --allow-dirty
(cd "$root/python" && uv build --out-dir "$output")
(cd "$root/typescript" && npm pack --pack-destination "$output" >/dev/null)
tar -czf "$output/miilo-protocol-0.1.0.tar.gz" -C "$root" protocol/schemas protocol/fixtures protocol/README.md
cp "$root/rust/target/package/miilo-adapter-sdk-0.1.0.crate" "$output/"

test -f "$output/miilo-adapter-sdk-0.1.0.crate"
test -f "$output/miilo_adapter_sdk-0.1.0-py3-none-any.whl"
test -f "$output/miilo_adapter_sdk-0.1.0.tar.gz"
test -f "$output/miilo-ai-adapter-sdk-0.1.0.tgz"
test -f "$output/miilo-protocol-0.1.0.tar.gz"

(cd "$output" && shasum -a 256 ./* > SHA256SUMS && shasum -a 256 -c SHA256SUMS)
echo "Miilo SDK release artifacts passed."
