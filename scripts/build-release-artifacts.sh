#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
test "$#" = 1 || { echo 'Usage: build-release-artifacts.sh OUTPUT_DIRECTORY' >&2; exit 1; }
mkdir -p "$1"
output=$(CDPATH= cd -- "$1" && pwd)
test -z "$(ls -A "$output")" || { echo 'Release output directory must be empty' >&2; exit 1; }
version=$(node -p "JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')).version" "$root/typescript/package.json")

# A fresh release job has no dependency installation from the qualification job.
(cd "$root/typescript" && npm ci --ignore-scripts --include=dev)
cargo package --manifest-path "$root/rust/Cargo.toml" --locked
cp "$root/rust/target/package/miilo-adapter-sdk-${version}.crate" "$output/"
uv build --project "$root/python" --out-dir "$output"
(cd "$root/typescript" && npm pack --pack-destination "$output")
tar -czf "$output/miilo-protocol-${version}.tar.gz" -C "$root" \
  protocol/schemas protocol/fixtures protocol/README.md docs
(cd "$output" && shasum -a 256 ./* > SHA256SUMS && shasum -a 256 -c SHA256SUMS)
