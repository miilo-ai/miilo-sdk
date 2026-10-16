#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
output=$(mktemp -d "${TMPDIR:-/tmp}/miilo-sdk-release.XXXXXX")
export CARGO_HOME="${CARGO_HOME:-/tmp/miilo-sdk-cargo}"
export NPM_CONFIG_CACHE="${NPM_CONFIG_CACHE:-/tmp/miilo-sdk-npm}"
export UV_CACHE_DIR="${UV_CACHE_DIR:-/tmp/miilo-sdk-uv}"
cleanup() { rm -rf "$output"; }
trap cleanup EXIT INT TERM

version=$(node -p "JSON.parse(require('fs').readFileSync('$root/typescript/package.json', 'utf8')).version")
cargo package --manifest-path "$root/rust/Cargo.toml" --locked --allow-dirty
(cd "$root/python" && uv build --out-dir "$output")
(cd "$root/typescript" && npm pack --pack-destination "$output" >/dev/null)
tar -czf "$output/miilo-protocol-${version}.tar.gz" -C "$root" protocol/schemas protocol/fixtures protocol/README.md
cp "$root/rust/target/package/miilo-adapter-sdk-${version}.crate" "$output/"

test -f "$output/miilo-adapter-sdk-${version}.crate"
test "$(find "$output" -name '*.whl' | wc -l | tr -d ' ')" = 1
test "$(find "$output" -name 'miilo_adapter_sdk-*.tar.gz' | wc -l | tr -d ' ')" = 1
test -f "$output/miilo-ai-adapter-sdk-${version}.tgz"
test -f "$output/miilo-protocol-${version}.tar.gz"

(cd "$output" && shasum -a 256 ./* > SHA256SUMS && shasum -a 256 -c SHA256SUMS)
echo "Miilo SDK release artifacts passed."
