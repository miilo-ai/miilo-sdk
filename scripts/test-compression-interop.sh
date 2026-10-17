#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
fixture_dir=$(mktemp -d)
trap 'rm -rf "$fixture_dir"' EXIT
uv run --project python --extra compression python - "$fixture_dir" <<'PY'
import sys
from pathlib import Path
from miilo_adapter_sdk.compression import encode_bundle
p=Path(sys.argv[1]); data=('Synthetic café 🦉 task evidence\n'*1000).encode()+bytes(range(256))
(p/'payload').write_bytes(data)
for codec in ['gzip','brotli','zstd']:(p/f'python-{codec}').write_bytes(encode_bundle(codec,data))
PY
cargo run --quiet --manifest-path rust/Cargo.toml --features compression --example compression-interop -- "$fixture_dir"
uv run --project python --extra compression python - "$fixture_dir" <<'PY'
import sys
from pathlib import Path
from miilo_adapter_sdk.compression import decode_bundle
p=Path(sys.argv[1]); data=(p/'payload').read_bytes()
for codec in ['gzip','brotli','zstd']:assert decode_bundle((p/f'rust-{codec}').read_bytes())[1]==data
print('Rust/Python bundle interoperability passed for gzip, Brotli and zstd.')
PY
