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
sh "$root/scripts/build-release-artifacts.sh" "$output"

test -f "$output/miilo-adapter-sdk-${version}.crate"
test "$(find "$output" -name '*.whl' | wc -l | tr -d ' ')" = 1
test "$(find "$output" -name 'miilo_adapter_sdk-*.tar.gz' | wc -l | tr -d ' ')" = 1
test -f "$output/miilo-ai-adapter-sdk-${version}.tgz"
test -f "$output/miilo-protocol-${version}.tar.gz"

echo "Miilo SDK release artifacts passed."
