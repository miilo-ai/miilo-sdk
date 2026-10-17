#!/usr/bin/env bash
set -euo pipefail
root=$(pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
uv build --project python --out-dir "$scratch"
uv venv "$scratch/venv"
uv pip install --python "$scratch/venv/bin/python" "$scratch"/*.whl
(cd "$scratch" && SDK_FIXTURE="$root/protocol/fixtures/factory.snapshot.json" "$scratch/venv/bin/python" -c 'import json, os; from pathlib import Path; from pydantic import TypeAdapter; from miilo_adapter_sdk import ObservatorySnapshot; TypeAdapter(ObservatorySnapshot).validate_python(json.loads(Path(os.environ["SDK_FIXTURE"]).read_text()))')
(cd typescript && npm pack --pack-destination "$scratch" >/dev/null)
npm install --prefix "$scratch/node" "$scratch"/*.tgz typescript@5.9.3
printf 'import type { ObservatorySnapshot } from "@miilo-ai/adapter-sdk";\nconst providers = (snapshot: ObservatorySnapshot) => snapshot.providers;\n' > "$scratch/node/consumer.mts"
(cd "$scratch/node" && ./node_modules/.bin/tsc --noEmit --module nodenext --target ES2022 consumer.mts)
cargo package --manifest-path rust/Cargo.toml --locked --allow-dirty
version=$(node -p "JSON.parse(require('fs').readFileSync('typescript/package.json','utf8')).version")
mkdir -p "$scratch/rust"
tar -xzf "rust/target/package/miilo-adapter-sdk-$version.crate" -C "$scratch/rust"
SDK_CONSUMER_ROOT="$scratch/rust" SDK_CONSUMER_VERSION="$version" python3 - <<'PY'
import json, os
from pathlib import Path
root=Path(os.environ['SDK_CONSUMER_ROOT'])
source=root/('miilo-adapter-sdk-'+os.environ['SDK_CONSUMER_VERSION'])
consumer=root/'consumer';(consumer/'src').mkdir(parents=True)
(consumer/'Cargo.toml').write_text('[package]\nname="miilo-runtime-consumer"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nmiilo-adapter-sdk={path='+json.dumps(str(source))+',features=["runtime"]}\n')
(consumer/'src/main.rs').write_text('fn main(){let runtime=miilo_adapter_sdk::runtime::ObservatoryRuntime::new("installed-consumer",8).unwrap();assert_eq!(runtime.cursor(),"installed-consumer:0");}\n')
PY
cargo run --manifest-path "$scratch/rust/consumer/Cargo.toml" --quiet -j2
echo 'Installed Rust runtime, wheel and npm SDK consumers passed.'
